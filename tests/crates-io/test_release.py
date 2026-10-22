"""Release receipts and injected registry/upload protocol failures."""
import hashlib
import importlib.util
import io
import json
from pathlib import Path
import shutil
import subprocess
import tarfile
import tempfile
import unittest
from unittest.mock import Mock

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


if __name__ == '__main__': unittest.main()
