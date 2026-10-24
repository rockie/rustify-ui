"""CLI identity and projection modes, without installing or uploading packages."""
import hashlib
import importlib.util
import json
from pathlib import Path
import subprocess
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
            'packages': {name: '0.2.1' for name in ('rustify-docs', 'rustify-dom', 'rustify-components')},
            'wasm_bindgen': '0.2.128',
        }
        revision = 'b' * 40
        if mode == 'source':
            revision = subprocess.check_output(['git', 'rev-parse', 'HEAD'], cwd=ROOT, text=True).strip()
            framework['development_checkout'] = hashlib.sha256(str(ROOT.resolve()).encode()).hexdigest()
        return f'rustify-docs 0.2.1\npanic: unwind\nsource: {revision}\ndistribution: {mode}\nframework: {json.dumps(framework)}\n'

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
            self.info('source').replace('"rustify-dom": "0.2.1"', '"rustify-dom": "0.1.0"'),
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
        self.assertEqual(alias, {'package':'rustify-dom', 'version':'=0.2.1', 'default-features':False})
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
        self.assertIn('cargo install rustify-docs-cli --version 0.2.1 --locked', package)
        for path in (cli / 'README.md', cli / 'templates/content/README.md', cli / 'templates/components/README.md'):
            text = path.read_text()
            self.assertIn('theme.example.json', text)
            self.assertIn('[[theme.fonts]]', text)
            self.assertIn('theme-font-system', text)
            self.assertNotIn('{{framework-revision}}', text)
            self.assertNotIn('requires a traceable CLI source revision', text)
        template = tomllib.loads((cli / 'templates/components/Cargo.toml.in').read_text().replace('{{package}}','docs-site'))
        for name in pairing.FRAMEWORKS:
            self.assertEqual(template['dependencies'][name], {'version':'=0.2.1', 'default-features':False})


if __name__ == '__main__': unittest.main()
