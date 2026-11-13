"""CLI identity and projection modes, without installing or uploading packages."""
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile
import tomllib
import unittest

ROOT = Path(__file__).resolve().parents[2]
SPEC = importlib.util.spec_from_file_location('docs_pairing', ROOT / 'scripts/docs-pages-prepare.py')
pairing = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(pairing)


class PairingTests(unittest.TestCase):
    def info(self, mode='registry'):
        framework = {
            'schema': 1, 'registry': 'crates-io', 'library_rev': 'a' * 40,
            'packages': {name: '0.3.0' for name in ('rustify-docs', 'rustify-dom', 'rustify-components')},
            'wasm_bindgen': '0.2.128',
        }
        revision = 'b' * 40
        if mode == 'source':
            revision = subprocess.check_output(['git', 'rev-parse', 'HEAD'], cwd=ROOT, text=True).strip()
            framework['development_checkout'] = hashlib.sha256(str(ROOT.resolve()).encode()).hexdigest()
        return f'rustify-docs 0.3.0\npanic: unwind\nsource: {revision}\ndistribution: {mode}\nframework: {json.dumps(framework)}\n'

    def test_registry_identity_does_not_require_a_local_git_commit(self):
        identity = pairing.read_identity(self.info(), ROOT)
        self.assertEqual(identity['mode'], 'registry')
        self.assertEqual(identity['framework_revision'], 'a' * 40)
        identity = pairing.read_identity(self.info().replace('source: ' + 'b' * 40, 'source: unknown'), ROOT)
        self.assertEqual(identity['candidate_sha'], 'unknown')

    def test_source_identity_requires_the_exact_checkout_and_manifest_versions(self):
        self.assertEqual(pairing.read_identity(self.info('source'), ROOT)['mode'], 'source')
        for text in (
            self.info('source').replace('development_checkout', 'wrong_checkout'),
            self.info('source').replace('"rustify-dom": "0.3.0"', '"rustify-dom": "0.1.0"'),
            self.info('source').replace('source: ' + subprocess.check_output(['git','rev-parse','HEAD'], cwd=ROOT, text=True).strip(), 'source: ' + '0' * 40),
        ):
            with self.assertRaises(ValueError): pairing.read_identity(text, ROOT)

    def test_projection_preserves_alias_features_and_unrelated_manifest_bytes(self):
        original = (ROOT / 'tests/docs-ssg/fixtures/components/Cargo.toml').read_bytes()
        original += b'\n[package.metadata.author]\nkeep = "byte for byte"\n'
        identity = pairing.read_identity(self.info(), ROOT)
        projected = pairing.project_manifest(original, identity, ROOT)
        document = tomllib.loads(projected.decode())
        alias = document['dependencies']['rustify-ui']
        self.assertEqual(alias, {'package':'rustify-dom', 'version':'=0.3.0', 'default-features':False})
        self.assertEqual(document['features'], tomllib.loads(original.decode())['features'])
        before = [line for line in original.splitlines(keepends=True) if not line.startswith((b'rustify-ui = ', b'rustify-docs = ', b'rustify-components = '))]
        after = [line for line in projected.splitlines(keepends=True) if not line.startswith((b'rustify-ui = ', b'rustify-docs = ', b'rustify-components = '))]
        self.assertEqual(before, after)
        source = pairing.project_manifest(original, pairing.read_identity(self.info('source'), ROOT), ROOT)
        deps = tomllib.loads(source.decode())['dependencies']
        self.assertEqual(deps['rustify-ui']['path'], str(ROOT / 'crates/rustify-dom'))
        self.assertNotIn('git', deps['rustify-ui'])

    def test_legacy_identity_reads_the_actual_frozen_c_record(self):
        candidate = '88566ba1ed08223a5834f794840fff5e48f4ba8a'
        identity = pairing.read_identity(f'rustify-docs 0.1.32\npanic: unwind\nsource: {candidate}\n', ROOT)
        expected = subprocess.check_output(['git', 'show', f'{candidate}:crates/rustify-docs-cli/assets/framework-revision.txt'], cwd=ROOT, text=True).strip()
        self.assertEqual(identity['mode'], 'legacy')
        self.assertEqual(identity['framework_revision'], expected)

    def test_legacy_projection_keeps_the_real_old_c_template_contract(self):
        candidate = '88566ba1ed08223a5834f794840fff5e48f4ba8a'
        def read(path):
            return subprocess.check_output(['git','show',f'{candidate}:{path}'], cwd=ROOT)
        original = read('tests/docs-ssg/fixtures/components/Cargo.toml')
        name = tomllib.loads(original.decode())['package']['name']
        revision = read('crates/rustify-docs-cli/assets/framework-revision.txt').strip()
        generated = read('crates/rustify-docs-cli/templates/components/Cargo.toml.in').replace(b'{{package}}', name.encode()).replace(b'{{framework-revision}}',revision)
        lock = read('crates/rustify-docs-cli/assets/components-Cargo.lock').replace(b'name = "docs-site"\nversion = "0.1.0"', f'name = "{name}"\nversion = "0.1.0"'.encode(), 1)
        projected = pairing.project_bytes(original, generated, lock, revision.decode())
        document = tomllib.loads(projected.decode())
        for package in pairing.LEGACY_FRAMEWORKS:
            self.assertEqual(document['dependencies'][package]['rev'], revision.decode())
        self.assertEqual(document['features'], tomllib.loads(original.decode())['features'])

    def test_new_identity_cannot_fall_back_to_legacy_on_incomplete_or_bad_json(self):
        for text in (self.info().replace('framework: ', 'framework: broken'), self.info().replace('distribution: registry\n',''), self.info().replace('"schema": 1','"schema": 2')):
            with self.assertRaises(ValueError): pairing.read_identity(text, ROOT)

    def test_release_readmes_teach_registry_installation_and_project_owned_fonts(self):
        cli = ROOT / 'crates/rustify-docs-cli'
        package = (cli / 'README.md').read_text()
        self.assertIn('cargo install rustify-docs-cli --version 0.3.0 --locked', package)
        for path in (cli / 'README.md', cli / 'templates/content/README.md', cli / 'templates/components/README.md'):
            text = path.read_text()
            self.assertIn('theme.example.json', text)
            self.assertIn('[[theme.fonts]]', text)
            self.assertIn('theme-font-system', text)
            self.assertNotIn('{{framework-revision}}', text)
            self.assertNotIn('requires a traceable CLI source revision', text)
        template = tomllib.loads((cli / 'templates/components/Cargo.toml.in').read_text().replace('{{package}}','docs-site'))
        for name in pairing.FRAMEWORKS:
            self.assertEqual(template['dependencies'][name], {'version':'=0.3.0', 'default-features':False})


COMPOSER = ROOT / 'scripts/docs-pages-compose.py'
REAL_DOCS = os.environ.get('RUSTIFY_COMPOSE_DOCS_DIR')
REAL_STUDIO = os.environ.get('RUSTIFY_COMPOSE_STUDIO_DIR')


def tree(directory):
    return sorted((path.relative_to(directory).as_posix(), path.stat().st_size)
                  for path in directory.rglob('*') if path.is_file())


@unittest.skipUnless(REAL_DOCS and REAL_STUDIO,
                     'set RUSTIFY_COMPOSE_DOCS_DIR and RUSTIFY_COMPOSE_STUDIO_DIR to a real /rustify-ui/ Docs build and its Studio build')
class ComposeTests(unittest.TestCase):
    """The Pages composer CLI over real Docs and Studio builds: a valid
    composition plus the refused inputs (missing resource, wrong base, output
    inside or over an input), none of which may change an input."""

    BASE = '/rustify-ui/'

    @classmethod
    def setUpClass(cls):
        cls.temporary = tempfile.TemporaryDirectory()
        cls.root = Path(cls.temporary.name)
        cls.docs = cls.root / 'docs'
        cls.studio = cls.root / 'studio'
        shutil.copytree(REAL_DOCS, cls.docs)
        shutil.copytree(REAL_STUDIO, cls.studio)
        cls.inputs = (tree(cls.docs), tree(cls.studio))

    @classmethod
    def tearDownClass(cls):
        cls.temporary.cleanup()

    def compose(self, out, base=BASE, docs=None):
        return subprocess.run(
            [sys.executable, str(COMPOSER), '--docs-dir', str(docs or self.docs),
             '--studio-dir', str(self.studio), '--out', str(out), '--base', base],
            capture_output=True, text=True)

    def assertRefused(self, run, out, reason):
        self.assertNotEqual(run.returncode, 0, run.stdout)
        self.assertIn(reason, run.stderr)
        self.assertFalse(out.exists(), f'{out} was created by a refused composition')
        self.assertEqual((tree(self.docs), tree(self.studio)), self.inputs)

    def test_valid_composition_packages_studio_with_its_policy_and_one_manifest(self):
        out = self.root / 'composed'
        run = self.compose(out)
        self.assertEqual(run.returncode, 0, run.stderr)
        self.addCleanup(shutil.rmtree, out)
        summary = json.loads(run.stdout)
        self.assertEqual(summary['base'], self.BASE)
        studio_html = (out / 'theme-studio/index.html').read_text()
        self.assertIn(compose_csp(), studio_html)
        studio_page = (out / 'studio/index.html').read_text()
        self.assertIn('href="/rustify-ui/theme-studio/"', studio_page)
        self.assertNotIn('https://rockie.github.io/rustify-ui/theme-studio/', studio_page)
        nested = json.loads((out / 'theme-studio/build-manifest.json').read_text())
        original = json.loads((self.studio / 'build-manifest.json').read_text())
        self.assertEqual((nested['base'], nested['build_id']), (original['base'], original['build_id']))
        self.assertEqual(nested['files']['index.html'], (out / 'theme-studio/index.html').stat().st_size)
        root_manifest = json.loads((out / 'rustify-docs-manifest.json').read_text())
        listed = sorted(entry['path'] for entry in root_manifest['files'])
        self.assertEqual(listed, sorted(path for path, _ in tree(out) if path != 'rustify-docs-manifest.json'))
        whole = hashlib.sha256()
        for entry in root_manifest['files']:
            data = (out / entry['path']).read_bytes()
            self.assertEqual(entry['sha256'], hashlib.sha256(data).hexdigest(), entry['path'])
            whole.update(entry['path'].encode() + b'\0' + entry['sha256'].encode() + b'\n')
        self.assertEqual((root_manifest['build_id'], summary['build_id']), (whole.hexdigest()[:16],) * 2)
        self.assertEqual((tree(self.docs), tree(self.studio)), self.inputs)

    def test_missing_resource_is_refused(self):
        docs = self.root / 'docs-missing'
        shutil.copytree(self.docs, docs)
        self.addCleanup(shutil.rmtree, docs)
        (docs / 'colors/index.html').unlink()
        self.assertRefused(self.compose(self.root / 'out', docs=docs), self.root / 'out', 'colors/index.html')

    def test_wrong_base_is_refused(self):
        self.assertRefused(self.compose(self.root / 'out', base='/other/'), self.root / 'out', '/other/')

    def test_output_inside_or_over_an_input_is_refused(self):
        self.assertRefused(self.compose(self.docs / 'nested'), self.docs / 'nested', 'inside an input')
        self.assertRefused(self.compose(self.studio / 'nested'), self.studio / 'nested', 'inside an input')
        run = self.compose(self.docs)
        self.assertNotEqual(run.returncode, 0)
        self.assertIn('one of the input directories', run.stderr)
        self.assertEqual((tree(self.docs), tree(self.studio)), self.inputs)
        occupied = self.root / 'occupied'
        occupied.mkdir()
        self.addCleanup(shutil.rmtree, occupied)
        (occupied / 'stale.txt').write_text('occupied')
        run = self.compose(occupied)
        self.assertNotEqual(run.returncode, 0)
        self.assertIn('not empty', run.stderr)
        self.assertEqual(tree(occupied), [('stale.txt', 8)])


def compose_csp():
    text = (ROOT / 'scripts/docs-pages-verify.py').read_text()
    return text.split('META_CSP = "', 1)[1].split('"', 1)[0]


if __name__ == '__main__': unittest.main()
