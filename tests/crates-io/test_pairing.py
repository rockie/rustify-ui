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


SPEC_COMPOSE = importlib.util.spec_from_file_location('docs_compose', ROOT / 'scripts/docs-pages-compose.py')
compose = importlib.util.module_from_spec(SPEC_COMPOSE)
SPEC_COMPOSE.loader.exec_module(compose)


class ComposeTests(unittest.TestCase):
    """The Pages composer contract: valid composition plus the three refused
    inputs (missing resource, wrong base, output covering an input)."""

    def setUp(self):
        import tempfile
        self.temporary = tempfile.TemporaryDirectory()
        self.root = Path(self.temporary.name)
        self.docs = self.root / 'docs'
        self.studio = self.root / 'studio'
        self.make_inputs('/x/')

    def tearDown(self):
        self.temporary.cleanup()

    def make_inputs(self, base, studio_base=None, break_file=False):
        studio_base = studio_base or f'{base}theme-studio/'
        (self.docs / 'assets').mkdir(parents=True, exist_ok=True)
        (self.docs / 'index.html').write_bytes(
            b'<html><head></head><body><a href="https://rockie.github.io/rustify-ui/theme-studio/">Studio</a></body></html>'
        )
        (self.docs / 'assets/site.js').write_bytes(b'console.log(1)\n')
        files = {}
        for path in ('index.html', 'assets/site.js'):
            data = (self.docs / path).read_bytes()
            files[path] = {'path': path, 'sha256': hashlib.sha256(data).hexdigest(), 'bytes': len(data)}
        manifest = {
            'schema_version': 1, 'mode': 'production', 'base': base,
            'cli_version': '0.3.0', 'build_id': '0' * 16,
            'files': [files['index.html'], files['assets/site.js']],
            'locale_not_found': {'': '404.html'}, 'page_policies': {},
        }
        (self.docs / 'rustify-docs-manifest.json').write_text(json.dumps(manifest, indent=2) + '\n')
        self.studio.mkdir(parents=True, exist_ok=True)
        (self.studio / 'index.html').write_bytes(b'<html><head>\n<title>studio</title></head><body>app</body></html>')
        (self.studio / 'app.js').write_bytes(b'boot()\n')
        studio_manifest = {
            'example': 'theme-studio', 'profile': 'release', 'base': studio_base,
            'build_id': 'a' * 16, 'schema_hash': 'b' * 16, 'toolchain': '1.99.0',
            'files': {'index.html': (self.studio / 'index.html').stat().st_size,
                      'app.js': (self.studio / 'app.js').stat().st_size},
            'size_report': {'wasm': 0, 'js': 0, 'css': 0, 'fonts': 0, 'images': 0, 'data': 0, 'total': 0},
        }
        if break_file:
            (self.studio / 'app.js').unlink()
        (self.studio / 'build-manifest.json').write_text(json.dumps(studio_manifest, indent=2) + '\n')
        return manifest

    def compose(self, out, base='/x/', docs=None, studio=None):
        return compose.compose(docs or self.docs, studio or self.studio, out, base)

    def test_missing_resource_is_refused(self):
        (self.docs / 'assets/site.js').unlink()
        with self.assertRaises(SystemExit):
            self.compose(self.root / 'out')
        self.assertFalse((self.root / 'out').exists())
        self.assertFalse((self.root / 'out' / 'index.html').exists())

    def test_wrong_base_is_refused(self):
        with self.assertRaises(SystemExit):
            self.compose(self.root / 'out', base='/other/')

    def test_wrong_base_is_refused_docs_and_studio(self):
        self.make_inputs('/y/')
        with self.assertRaises(SystemExit):
            self.compose(self.root / 'out', base='/x/')
        self.assertFalse((self.root / 'out').exists())

    def test_output_covering_an_input_is_refused(self):
        with self.assertRaises(SystemExit):
            self.compose(self.docs)
        nested = self.docs / 'nested'
        with self.assertRaises(SystemExit):
            self.compose(nested)
        out = self.root / 'fresh'
        out.mkdir()
        (out / 'stale.txt').write_text('occupied')
        with self.assertRaises(SystemExit):
            self.compose(out)
        self.assertTrue((self.docs / 'index.html').exists())

    def test_studio_manifest_mismatch_is_refused(self):
        self.make_inputs('/x/', studio_base='/x/other/')
        with self.assertRaises(SystemExit):
            self.compose(self.root / 'out')

    def test_valid_compose_copies_csp_localizes_and_recomputes(self):
        out = self.root / 'composed'
        summary = self.compose(out)
        self.assertEqual(summary['base'], '/x/')
        docs_html = (out / 'index.html').read_text()
        self.assertIn('href="/x/theme-studio/"', docs_html)
        self.assertNotIn('https://rockie.github.io/rustify-ui/theme-studio/', docs_html)
        studio_html = (out / 'theme-studio/index.html').read_text()
        self.assertIn(compose.META_CSP, studio_html)
        nested = json.loads((out / 'theme-studio/build-manifest.json').read_text())
        self.assertEqual(nested['base'], '/x/theme-studio/')
        self.assertEqual(nested['build_id'], 'a' * 16)
        self.assertIn('index.html', nested['files'])
        self.assertGreater(nested['size_report']['data'], 0)
        self.assertEqual(nested['size_report']['js'], (out / 'theme-studio/app.js').stat().st_size)
        root_manifest = json.loads((out / 'rustify-docs-manifest.json').read_text())
        paths = sorted(entry['path'] for entry in root_manifest['files'])
        expected = sorted(['index.html', 'assets/site.js', 'theme-studio/index.html', 'theme-studio/app.js', 'theme-studio/build-manifest.json'])
        self.assertEqual(paths, expected)
        whole = hashlib.sha256()
        for entry in root_manifest['files']:
            whole.update(entry['path'].encode())
            whole.update(b'\0')
            whole.update(entry['sha256'].encode())
            whole.update(b'\n')
        self.assertEqual(root_manifest['build_id'], whole.hexdigest()[:16])
        self.assertEqual(root_manifest['locale_not_found'], {'': '404.html'})


if __name__ == '__main__': unittest.main()
