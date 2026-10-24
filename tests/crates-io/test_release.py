"""Release receipts and injected registry/upload protocol failures."""
import hashlib
import importlib.util
import io
import json
from contextlib import nullcontext
from pathlib import Path
import re
import shutil
import subprocess
import tarfile
import tempfile
import unittest
import urllib.error
from unittest.mock import Mock, patch

ROOT = Path(__file__).resolve().parents[2]
SPEC = importlib.util.spec_from_file_location('crates_io', ROOT / 'scripts/crates-io.py')
release = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(release)


class ReleaseTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory(prefix='rustify-release-protocol-')
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        self.archive = self.root / 'core-0.2.0.crate'
        self.archive_bytes('registry+https://github.com/rust-lang/crates.io-index', 'a' * 64)
        self.package = {'name':'core', 'version':'0.2.0', 'group':'docs', 'dependencies':[],
                        'archive':self.archive.name, 'sha256':release.sha256(self.archive),
                        'lock_sha256':hashlib.sha256(self.lock.encode()).hexdigest(), 'state':'pending'}
        self.receipt = {'schema':1,'library_rev':'a'*40,'cli_rev':'b'*40,'packages':[self.package], 'verifications':{}}
        release.save_receipt(self.root, self.receipt)
        self.registry = Mock()
        self.registry.lookup.return_value = None
        self.registry.available.return_value = True
        self.upload = Mock(return_value={'status':'uploaded'})

    def archive_bytes(self, source, checksum, extra_manifest=''):
        self.lock = f'version = 4\n[[package]]\nname = "core"\nversion = "0.2.0"\n[[package]]\nname = "dep"\nversion = "1.0.0"\nsource = "{source}"\nchecksum = "{checksum}"\n'
        with tarfile.open(self.archive, 'w:gz') as bundle:
            for filename, content in {'Cargo.lock':self.lock, 'Cargo.toml':'[package]\nname="core"\nversion="0.2.0"\n' + extra_manifest, 'LICENSE':'MIT', '.cargo_vcs_info.json':json.dumps({'git':{'sha1':'a'*40}})}.items():
                payload = content.encode()
                entry = tarfile.TarInfo(f'core-0.2.0/{filename}')
                entry.size = len(payload)
                bundle.addfile(entry, io.BytesIO(payload))

    def publish(self, **kwargs):
        return release.publish_packages(self.root, self.receipt, 'docs', self.registry, self.upload, now=1000, **kwargs)

    def test_workflow_prefetch_covers_every_locked_registry_input(self):
        workflow = (ROOT / '.github/workflows/crates-io.yml').read_text()
        manifests = re.findall(r'^\s+mbx fetch --locked(?: --manifest-path (\S+))?$', workflow, re.MULTILINE)
        self.assertTrue(manifests, 'workflow must prefetch dependencies before offline packaging')
        prefetched = [(ROOT / (manifest or 'Cargo.toml')).with_name('Cargo.lock') for manifest in manifests]
        available = {(p['name'], p['version'], p['checksum']) for p in release.locked_registry_packages(prefetched)}
        locks = release.lock_inputs(ROOT, self.root, [])
        required = {(p['name'], p['version'], p['checksum']) for p in release.locked_registry_packages(locks)}
        self.assertEqual(required - available, set(), 'workflow prefetch omits locked registry inputs')

    def test_existing_same_checksum_resumes_without_repackaging_or_upload(self):
        self.registry.lookup.return_value = self.package['sha256']
        self.publish()
        self.upload.assert_not_called()
        self.assertEqual(self.package['state'], 'published')

    def test_existing_different_checksum_stops_before_upload(self):
        self.registry.lookup.return_value = 'f' * 64
        with self.assertRaisesRegex(release.ReleaseError, 'checksum'): self.publish()
        self.upload.assert_not_called()

    def test_timeout_queries_state_before_any_retry(self):
        self.upload.return_value = {'status':'timeout'}
        self.registry.lookup.side_effect = [None, self.package['sha256']]
        self.publish()
        self.assertEqual(self.upload.call_count, 1)
        self.assertEqual(self.package['state'], 'published')
        self.registry.lookup.side_effect = [None, None]
        self.package['state'] = 'pending'
        with self.assertRaises(release.Waiting): self.publish()
        self.assertEqual(self.package['state'], 'waiting-index')
        self.registry.lookup.side_effect = [None]
        before = self.upload.call_count
        with self.assertRaises(release.Waiting): self.publish()
        self.assertEqual(self.upload.call_count, before)

    def test_429_saves_retry_time_then_resumes_same_archive(self):
        self.upload.return_value = {'status':'rate-limit', 'retry_after':'600'}
        with self.assertRaises(release.Waiting): self.publish()
        stored = release.read_receipt(self.root)
        self.assertEqual(stored['packages'][0]['next_retry_at'], 1600)
        self.assertEqual(self.package['state'], 'waiting-rate-limit')
        self.upload.reset_mock()
        with self.assertRaises(release.Waiting): self.publish()
        self.upload.assert_not_called()
        self.upload.return_value = {'status':'uploaded'}
        self.registry.lookup.return_value = None
        release.publish_packages(self.root, self.receipt, 'docs', self.registry, self.upload, now=1600)
        self.assertEqual(self.package['state'], 'published')
        self.assertEqual(self.package['sha256'], release.sha256(self.archive))

    def test_unparseable_retry_after_requires_an_explicit_time(self):
        self.upload.return_value = {'status':'rate-limit', 'retry_after':None}
        with self.assertRaises(release.Waiting): self.publish()
        self.assertIsNone(self.package['next_retry_at'])
        with self.assertRaises(release.Waiting): self.publish()
        self.upload.return_value = {'status':'uploaded'}
        self.publish(retry_at=1000)
        self.assertEqual(self.package['state'], 'published')

    def test_retry_after_accepts_http_date(self):
        self.assertEqual(release.retry_time('Thu, 22 Oct 2026 11:00:00 GMT', 1000), 1792666800)

    def test_archive_receipt_detects_byte_lock_and_source_drift(self):
        release.validate_archives(self.root, self.receipt)
        self.archive_bytes('git+https://example.com/dep#abc', 'a' * 64)
        self.package['sha256'] = release.sha256(self.archive)
        with self.assertRaisesRegex(release.ReleaseError, 'registry'): release.validate_archives(self.root, self.receipt)
        self.archive_bytes(release.CRATES_IO, 'a' * 64)
        self.package['sha256'] = release.sha256(self.archive)
        self.package['lock_sha256'] = '0' * 64
        with self.assertRaisesRegex(release.ReleaseError, 'lock'): release.validate_archives(self.root, self.receipt)

    def test_archive_identity_comparison_rejects_index_induced_drift(self):
        first = {'core':{'sha256':'a'*64,'lock_sha256':'b'*64}}
        release.compare_archives(first, first)
        second = {'core':{'sha256':'c'*64,'lock_sha256':'b'*64}}
        with self.assertRaisesRegex(release.ReleaseError, 'drift'): release.compare_archives(first, second)

    def test_normalized_manifest_cannot_hide_private_or_target_path_dependencies(self):
        for extra in ('[dependencies]\nrustify-ui="0.2.0"\n',
                      '[target.\'cfg(target_arch = "wasm32")\'.dependencies]\ncore={version="0.2.0",path="../core"}\n'):
            self.archive_bytes(release.CRATES_IO, 'a' * 64, extra)
            with self.assertRaisesRegex(release.ReleaseError, 'manifest'):
                release.archive_info(self.archive, 'core', '0.2.0')

    def test_publish_checks_the_cargo_dry_run_archive_before_upload(self):
        cargo = Mock()
        cargo.version = 'cargo 1.99.0'
        cargo.env = {'CARGO_TARGET_DIR':str(self.root / 'target'), 'CARGO_HOME':str(self.root)}
        staged = self.root / 'target/package/tmp-registry' / self.archive.name
        staged.parent.mkdir(parents=True)
        shutil.copyfile(self.archive, staged)
        self.receipt['cargo_version'] = cargo.version
        self.receipt['verifications']['archive_install'] = {'result':'passed'}
        release.save_receipt(self.root, self.receipt)
        self.registry.lookup.side_effect = [None, self.package['sha256']]
        with (patch.object(release, 'validate_release_sources'),
              patch.object(release, 'checkout', return_value=nullcontext(self.root)),
              patch.object(release, 'Cargo', return_value=cargo),
              patch.object(release, 'Registry', return_value=self.registry),
              patch.object(release.subprocess, 'run', return_value=subprocess.CompletedProcess([], 0)) as upload):
            release.publish(self.root, 'docs')
        cargo.assert_called_once_with('publish', '--registry', 'crates-io', '--locked', '-p', 'core', '--dry-run', timeout=3600)
        upload.assert_called_once()
        self.assertEqual(release.read_receipt(self.root)['packages'][0]['state'], 'published')


class RegistryRequestTests(unittest.TestCase):
    def test_transient_connection_failure_retries_the_read(self):
        response = Mock()
        with (patch.object(release.urllib.request, 'urlopen', side_effect=[urllib.error.URLError('TLS EOF'), response]) as request,
              patch.object(release.time, 'sleep')):
            self.assertIs(release.Registry().request('https://crates.io/api/v1/crates/core/0.2.0'), response)
        self.assertEqual(request.call_count, 2)

    def test_connection_retries_are_bounded(self):
        with (patch.object(release.urllib.request, 'urlopen', side_effect=urllib.error.URLError('TLS EOF')) as request,
              patch.object(release.time, 'sleep')):
            with self.assertRaises(urllib.error.URLError):
                release.Registry().request('https://crates.io/api/v1/crates/core/0.2.0')
        self.assertEqual(request.call_count, 3)

    def test_missing_version_is_not_retried(self):
        error = urllib.error.HTTPError('https://crates.io', 404, 'Not found', {}, None)
        with (patch.object(release.urllib.request, 'urlopen', side_effect=error) as request,
              patch.object(release.time, 'sleep') as sleep):
            self.assertIsNone(release.Registry().lookup('core', '0.2.0'))
        request.assert_called_once()
        sleep.assert_not_called()


class CargoReproducibilityTests(unittest.TestCase):
    def test_locked_real_archives_stay_identical_when_an_extra_compatible_version_is_visible(self):
        with tempfile.TemporaryDirectory(prefix='rustify-index-contract-') as temporary:
            work = Path(temporary)
            cargo = release.Cargo(ROOT, work / 'target')
            def git(root, *args):
                return subprocess.check_output(['git', *args], cwd=root, text=True).strip()
            def commit(root):
                git(root, 'add', '.')
                git(root, '-c', 'user.name=Rustify fixture', '-c', 'user.email=fixture@example.invalid', 'commit', '--quiet', '-m', 'Freeze real package inputs')
            def project(name):
                root = work / name
                (root / 'src').mkdir(parents=True)
                (root / 'src/lib.rs').write_text('pub fn value() -> u8 { 1 }\n')
                (root / 'LICENSE').write_text('MIT fixture\n')
                git(root, 'init', '--quiet')
                return root
            dependency = project('dependency')
            entries = []
            for version in ('1.0.0', '1.1.0'):
                (dependency / 'Cargo.toml').write_text(f'[package]\nname="release-probe-dep"\nversion="{version}"\nedition="2021"\n')
                cargo('generate-lockfile', '--offline', cwd=dependency)
                commit(dependency)
                cargo('package', '--locked', '--no-verify', cwd=dependency)
                archive = work / 'target/package' / f'release-probe-dep-{version}.crate'
                entries.append((archive, {'name':'release-probe-dep','vers':version,'deps':[], 'features':{}, 'yanked':False,'cksum':release.sha256(archive)}))
            key = release.registry_key('release-probe-dep')
            views = []
            for index in range(2):
                registry = work / f'index-{index}'
                (registry / 'index' / key).parent.mkdir(parents=True)
                (registry / 'index' / key).write_text(''.join(json.dumps(entry) + '\n' for _,entry in entries[:index+1]))
                for archive,_ in entries[:index+1]: shutil.copyfile(archive, registry / archive.name)
                views.append(registry)
            root = project('consumer')
            (root / 'Cargo.toml').write_text('[package]\nname="release-probe"\nversion="1.0.0"\nedition="2021"\n[dependencies]\nrelease-probe-dep="1"\n')
            cargo.config = release.registry_config(views[0], work / 'source.toml')
            cargo('generate-lockfile', '--offline', cwd=root)
            commit(root)
            identities = []
            for registry in views:
                cargo.config = release.registry_config(registry, work / 'source.toml')
                cargo('package', '--locked', '--no-verify', '--offline', cwd=root)
                archive = work / 'target/package/release-probe-1.0.0.crate'
                identities.append(release.archive_info(archive, 'release-probe', '1.0.0'))
            release.compare_archives({'probe':identities[0]}, {'probe':identities[1]})
            dependency_versions = {p['version'] for p in identities[1]['lock'] if p['name'] == 'release-probe-dep'}
            self.assertEqual(dependency_versions, {'1.0.0'})


class PerPackageRevisionTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory(prefix='rustify-release-revisions-')
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)

    def library_archive(self, name, version, revision, dependency='dep'):
        lock = (f'version = 4\n[[package]]\nname = "{name}"\nversion = "{version}"\n'
                f'[[package]]\nname = "{dependency}"\nversion = "1.0.0"\nsource = "{release.CRATES_IO}"\nchecksum = "{"b" * 64}"\n')
        path = self.root / f'{name}-{version}.crate'
        with tarfile.open(path, 'w:gz') as bundle:
            files = {'Cargo.lock':lock, 'Cargo.toml':f'[package]\nname="{name}"\nversion="{version}"\n',
                     'LICENSE':'MIT', '.cargo_vcs_info.json':json.dumps({'git':{'sha1':revision}})}
            for filename, content in files.items():
                payload = content.encode()
                entry = tarfile.TarInfo(f'{name}-{version}/{filename}')
                entry.size = len(payload)
                bundle.addfile(entry, io.BytesIO(payload))
        info = release.archive_info(path, name, version)
        return {'name':name, 'version':version, 'path':f'crates/{name}', 'group':'docs', 'dependencies':[],
                'archive':path.name, 'sha256':info['sha256'], 'lock_sha256':info['lock_sha256'],
                'bytes':info['bytes'], 'source_rev':revision, 'state':'pending'}

    def test_a_cli_version_change_does_not_move_a_library_closure(self):
        library = {'name':'core', 'path':'crates/core', 'version':'0.2.0', 'source_rev':'a' * 40}
        receipt = {'library_rev':'a' * 40, 'cli_rev':None, 'packages':[library]}
        before = [{'name':'core','version':'0.2.0','dependencies':['dep']}, {'name':'dep','version':'1.0.0'},
                  {'name':'cli','version':'0.2.0','dependencies':['core']}]
        after = [{'name':'core','version':'0.2.0','dependencies':['dep']}, {'name':'dep','version':'1.0.0'},
                 {'name':'cli','version':'0.2.1','dependencies':['core']}]
        with patch.object(release, 'run', return_value=''), \
             patch.object(release, 'locked_at', side_effect=lambda root, revision: before if revision == 'a' * 40 else after):
            self.assertFalse(release.library_inputs_changed(receipt, library, 'b' * 40))

    def test_a_third_party_bump_under_a_library_closure_marks_it_changed(self):
        library = {'name':'core', 'path':'crates/core', 'version':'0.2.0', 'source_rev':'a' * 40}
        receipt = {'library_rev':'a' * 40, 'cli_rev':None, 'packages':[library]}
        before = [{'name':'core','version':'0.2.0','dependencies':['dep']}, {'name':'dep','version':'1.0.0','checksum':'1' * 64}]
        after = [{'name':'core','version':'0.2.0','dependencies':['dep']}, {'name':'dep','version':'1.0.1','checksum':'2' * 64}]
        with patch.object(release, 'run', return_value=''), \
             patch.object(release, 'locked_at', side_effect=lambda root, revision: before if revision == 'a' * 40 else after):
            self.assertTrue(release.library_inputs_changed(receipt, library, 'b' * 40))

    def test_plan_reuses_unchanged_archives_and_packages_version_bumps(self):
        previous = {'library_rev':'a' * 40, 'cli_rev':None,
                    'packages':[{'name':'core','path':'crates/core','version':'0.2.0','source_rev':'a' * 40}]}
        with patch.object(release, 'library_inputs_changed', return_value=False):
            plan = release.library_packaging_plan(previous, [{'name':'core','version':'0.2.0','path':'crates/core'}], 'b' * 40)
            self.assertEqual([(item['action'], item['source_rev']) for item in plan], [('reuse', 'a' * 40)])
            plan = release.library_packaging_plan(previous, [{'name':'core','version':'0.2.1','path':'crates/core'}], 'b' * 40)
            self.assertEqual(plan[0]['action'], 'package')
        with patch.object(release, 'library_inputs_changed', return_value=True):
            with self.assertRaises(release.ReleaseError):
                release.library_packaging_plan(previous, [{'name':'core','version':'0.2.0','path':'crates/core'}], 'b' * 40)

    def test_the_framework_trio_moves_together(self):
        def prior(name):
            return {'name':name, 'path':f'crates/{name}', 'version':'0.2.1', 'source_rev':'a' * 40}
        previous = {'library_rev':'a' * 40, 'cli_rev':None,
                    'packages':[prior(name) for name in release.FRAMEWORKS]}
        entries = [{'name':name, 'version':'0.2.1', 'path':f'crates/{name}'} for name in release.FRAMEWORKS]
        moved = [dict(entry) for entry in entries]
        for entry in moved:
            if entry['name'] != 'rustify-dom':
                entry['version'] = '0.2.2'

        def changed(receipt, package, revision, root=None):
            return package['name'] != 'rustify-dom'

        with patch.object(release, 'library_inputs_changed', side_effect=changed):
            with self.assertRaises(release.ReleaseError) as raised:
                release.library_packaging_plan(previous, moved, 'b' * 40)
            self.assertIn('rustify-dom', str(raised.exception))
            bumped = [{'name':name, 'version':'0.2.2' if name != 'rustify-style-wasm' else '0.2.1', 'path':f'crates/{name}'}
                      for name in release.FRAMEWORKS]
            self.assertTrue(all(item['action'] == 'package'
                                for item in release.library_packaging_plan(previous, bumped, 'b' * 40)))

    def test_archives_may_carry_different_source_revisions(self):
        first = self.library_archive('core', '0.2.0', 'a' * 40)
        second = self.library_archive('other', '0.2.1', 'c' * 40)
        receipt = {'library_rev':'c' * 40, 'cli_rev':'b' * 40, 'packages':[first, second]}
        release.validate_archives(self.root, receipt)
        receipt['packages'][0]['source_rev'] = 'c' * 40
        with self.assertRaises(release.ReleaseError):
            release.validate_archives(self.root, receipt)

    def test_the_framework_trio_must_share_one_revision(self):
        packages = [{'name':name, 'version':'0.2.1', 'source_rev':revision}
                    for name, revision in zip(release.FRAMEWORKS, ['a' * 40, 'a' * 40, 'b' * 40])]
        with self.assertRaises(release.ReleaseError):
            release.framework_revision({'library_rev':None, 'cli_rev':None, 'packages':packages})
        packages[2]['source_rev'] = 'a' * 40
        self.assertEqual(release.framework_revision({'library_rev':None, 'cli_rev':None, 'packages':packages}), 'a' * 40)


if __name__ == '__main__': unittest.main()
