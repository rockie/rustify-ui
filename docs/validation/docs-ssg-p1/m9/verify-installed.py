import hashlib
import json
import os
import pathlib
import subprocess
import tempfile
from datetime import datetime, timezone
from html.parser import HTMLParser

evidence = pathlib.Path(__file__).resolve().parent
scratch = pathlib.Path(tempfile.gettempdir()) / 'rustify-docs-verify'
installed = scratch / 'cli-install'
binary = installed / 'bin/rustify-docs'
source = '1b81de25f23a25702f61dbfd3d0ecdfe323d4d44'
previous = '42757c627cb69456d241ba0de51b34f1ad85be3d'
sysroot = pathlib.Path(subprocess.check_output(['rustc', '--print', 'sysroot'], text=True).strip())
author_env = {'PATH': str(sysroot / 'bin') + ':/usr/bin:/bin'}
work = pathlib.Path(tempfile.mkdtemp(prefix='rustify-docs-m9-'))
site = work / 'site'

def cli(args, cwd=work):
    result = subprocess.run([str(binary), *args], cwd=cwd, env=author_env, capture_output=True, text=True, check=True)
    return result.stdout

assert cli(['--version']).strip() == 'rustify-docs 0.1.4'
help_text = cli(['--help'])
info = cli(['--build-info'])
assert 'source: ' + source in info and 'panic: unwind' in info
cli(['new', 'site'])
readme = (site / 'README.md').read_text()
command = 'cargo install --git https://github.com/rockie/rustify-ui --rev ' + source + ' --locked --profile docs-cli rustify-docs-cli'
assert command in help_text and command in readme and 'Install version 0.1.4' in readme
assert '--force' in readme and previous in readme and '{{' not in readme
(evidence / 'installed-help.txt').write_text(help_text)
(evidence / 'generated-README.md').write_text(readme)

def built(version):
    cli(['build'], site)
    manifest = json.loads((site / 'dist/rustify-docs-manifest.json').read_text())
    assert manifest['cli_version'] == version
    for entry in manifest['files']:
        content = (site / 'dist' / entry['path']).read_bytes()
        assert len(content) == entry['bytes'] and hashlib.sha256(content).hexdigest() == entry['sha256']
    assert not any(entry['path'] == 'README.md' for entry in manifest['files'])
    for page in ['index.html', 'guide/getting-started/index.html', '404.html']:
        class Landmarks(HTMLParser):
            def __init__(self):
                super().__init__()
                self.footers = 0
            def handle_starttag(self, tag, attrs):
                if tag == 'footer' and 'rd-site-footer' in dict(attrs).get('class', '').split():
                    self.footers += 1
        landmarks = Landmarks()
        landmarks.feed((site / 'dist' / page).read_text())
        assert landmarks.footers == 1, page
    return manifest

first = built('0.1.4')
results = []
repo = evidence.parents[3]
url = repo.as_uri()
install_env = dict(author_env, HOME=os.environ['HOME'], TMPDIR=tempfile.gettempdir())
for label, revision, version in [('rollback', previous, '0.1.3'), ('upgrade', source, '0.1.4')]:
    args = [str(sysroot / 'bin/cargo'), 'install', '--git', url, '--rev', revision, '--locked', '--profile', 'docs-cli', '--offline', '--root', str(installed), 'rustify-docs-cli', '--force']
    print(label + ': installing ' + version, flush=True)
    with (evidence / (label + '-install.log')).open('w') as log:
        subprocess.run(args, env=install_env, stdout=log, stderr=subprocess.STDOUT, check=True)
    version_text = cli(['--version']).strip()
    assert version_text == 'rustify-docs ' + version
    listed = subprocess.check_output([str(sysroot / 'bin/cargo'), 'install', '--list', '--root', str(installed)], env=install_env, text=True)
    assert revision in listed and 'rustify-docs-cli v' + version in listed
    manifest = built(version)
    assert manifest['files'] == first['files']
    results.append({'action': label, 'source': revision, 'version': version_text, 'installed_source': listed, 'manifest_cli_version': manifest['cli_version'], 'published_files_identical': True})
    print(label + ': verified version, source and fresh manifest', flush=True)

assert 'source: ' + source in cli(['--build-info'])
proof = {'observed_at': datetime.now(timezone.utc).isoformat(), 'source_candidate': source, 'cli_version': '0.1.4', 'binary': str(binary), 'binary_sha256': hashlib.sha256(binary.read_bytes()).hexdigest(), 'build_info': info, 'help_template_same_install_command': True, 'site': str(site), 'manifest': first, 'rollback_upgrade': results, 'result': 'passed'}
(evidence / 'installed-source.json').write_text(json.dumps(proof, indent=2) + '\n')
print('Installation help, README, rollback and upgrade: passed', flush=True)
