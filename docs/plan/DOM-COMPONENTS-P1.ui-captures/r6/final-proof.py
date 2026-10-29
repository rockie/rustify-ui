from pathlib import Path
from decimal import Decimal
import json, hashlib, re, subprocess, urllib.parse, struct

root = Path.cwd()
p = root / 'docs/plan/DOM-COMPONENTS-P1.ui-captures/r6'
report = root / 'docs/plan/DOM-COMPONENTS-P1.ui-review.r6.md'
correction = json.loads((p / 'sdlc-correction.json').read_text())
sha = lambda f: hashlib.sha256(f.read_bytes()).hexdigest()
assert correction['report_sha256'] == 'sha256:' + sha(report)
results = json.loads((p / 'judge-results.json').read_text())
totals = [Decimal(v['total']) for v in results['scores'].values()]
assert min(totals) == Decimal('9.5')
for value in results['scores'].values():
    assert sum(map(Decimal, value['dimensions'])) == Decimal(value['total'])
assert '**需要修改**' in report.read_text()
assert set(re.findall(r'^### (UI-\d+)', report.read_text(), re.M)) == {'UI-216', 'UI-217'}
for key, checksum in results['raw_sha256'].items():
    assert sha(p / f'judge-{key}.md') == checksum
original_dispatch = json.loads((p / 'revisions/judge-dispatch-before-matrix-supplement.json').read_text())
for unit, record in original_dispatch['results'].items():
    assert sha(p / record['raw']) == record['sha256']
for path, checksum in json.loads((p / 'target-hashes.json').read_text()).items():
    assert sha(root / path) == checksum
images = json.loads((p / 'image-integrity-final.json').read_text())
assert len(images) == 236
for image in images:
    assert sha(p / image['file']) == image['sha256']
runs = json.loads((p / 'capture-log.json').read_text())['runs']
records = [r for run in runs for r in run['records']]
for r in records:
    if r['kind'] == 'viewport':
        width, height = struct.unpack('>II', (p / r['file']).read_bytes()[16:24])
        expected = tuple(int(n) * r['dpr'] for n in r['viewport'].split('x'))
        assert (width, height) == expected, r['file']
machine = json.loads((p / 'machine-captures.json').read_text())
assert len(machine) == 15
for image in machine:
    assert sha(p / image['file']) == image['sha256']
proof = json.loads((p / 'judge-evidence-check.json').read_text())
assert proof['unique_current_images'] == 236 and not proof['missing']

def field(text, key):
    return json.loads(re.search(r'^' + key + r': (.+)$', text, re.M).group(1))

objects = ['4096c476-5ee3-44a5-a195-09a5183542ca', '2cc80246-5d61-4370-bbd7-462827ecbce3']
event_files = []
for object_id in objects:
    d = root / '.xgent-ai/sdlc/events/review' / object_id
    files = sorted(f for f in d.glob('*.md') if f.name != 'index.md')
    state = (root / '.xgent-ai/sdlc/status/review' / f'{object_id}.md').read_text()
    actual = {f.stem for f in files}
    assert set(field(state, 'included_events')) == actual
    index = (d / 'index.md').read_text()
    for f in files:
        text = f.read_text()
        assert field(text, 'event_id') == f.stem
        assert field(text, 'object_id') == object_id
        assert len(re.findall(r'^\| ' + f.stem + r' \|', index, re.M)) == 1
        for predecessor in field(text, 'predecessors'):
            prior = (d / f'{predecessor}.md').read_text()
            assert field(prior, 'scope') == field(text, 'scope')
            assert field(prior, 'facet') == field(text, 'facet')
    event_files += files
for event_id in correction['ids'].values():
    f = next(f for f in event_files if f.stem == event_id)
    for source in field(f.read_text(), 'sources'):
        assert source['revision'] == 'sha256:' + sha(root / source['path'])
for path, checksum in correction['immutable_original_events'].items():
    assert checksum == 'sha256:' + sha(root / path)

# Verbatim historical snapshots retain original link bases to preserve their hashes.
snapshots = []
for f in (p / 'revisions').glob('*before-matrix-supplement*'):
    original_name = f.name.replace('-before-matrix-supplement', '')
    original = report if original_name == 'report.md' else p / original_name
    snapshots.append({'snapshot': str(f.relative_to(root)), 'original_artifact': str(original.relative_to(root)), 'sha256': sha(f), 'relative_link_base': str(original.parent.relative_to(root))})
(p / 'revisions/snapshot-context.json').write_text(json.dumps(snapshots, ensure_ascii=False, indent=2) + '\n')
files = [report, p / 'capture.md', p / 'judge.md', *p.glob('capture-U*.md')]
files += [root / '.xgent-ai/sdlc/status/review' / f'{i}.md' for i in objects]
files += [root / '.xgent-ai/sdlc/status/plan/37a2c7b4-7052-4f85-96e9-135b6f7667a1.md', root / '.xgent-ai/sdlc/index.md']
files += event_files
files += [root / '.xgent-ai/sdlc/events/review' / i / 'index.md' for i in objects]
missing = []
links = 0
for f in files:
    for link in re.findall(r'\[[^\]]*\]\(([^)]+)\)', f.read_text()):
        raw = link.strip('<>')
        if raw.startswith(('http:', 'https:', 'mailto:', '#')):
            continue
        target = urllib.parse.unquote(raw.split('#', 1)[0])
        if not target:
            continue
        links += 1
        if not (f.parent / target).exists():
            missing.append({'source': str(f.relative_to(root)), 'target': target})
assert not missing, missing
head = subprocess.check_output(['git', 'rev-parse', 'HEAD']).decode().strip()
assert head == 'd7ff86ce2111991f0a4e0b0ea3e131cb4a6bea03'
paths = ['crates', 'examples', 'web', 'public', 'makepad', 'Cargo.toml', 'Cargo.lock', 'mise.toml', 'package.json', 'package-lock.json', 'xtask']
assert not subprocess.check_output(['git', 'diff', 'HEAD', '--', *paths])
assert not subprocess.check_output(['git', 'ls-files', '--others', '--exclude-standard', '--', *paths])
ports = subprocess.run(['lsof', '-nP', '-iTCP:4181', '-iTCP:4176', '-sTCP:LISTEN'], capture_output=True)
assert ports.returncode == 1 and not ports.stdout
status = subprocess.check_output(['git', 'status', '--short']).decode()
(p / 'final-worktree-status.txt').write_text(status)
final = {'report': str(report.relative_to(root)), 'report_sha256': 'sha256:' + sha(report), 'verdict': '需要修改', 'scores': [str(x) for x in totals], 'minimum': '9.5', 'minimum_unit': 'U1', 'threshold': '8', 'p0': 0, 'p1': 1, 'p2': 1, 'open_gaps': ['UI-217', 'UI-216'], 'review_images': 236, 'machine_images': 15, 'fresh_reviewers': 14, 'local_links_checked': links, 'missing_links': missing, 'event_sets_match_states': True, 'current_event_sources_hash_valid': True, 'original_events_unchanged': True, 'initial_154_images_unchanged': True, 'target_hashes_unchanged': True, 'head_unchanged': True, 'product_diff_empty': True, 'services_stopped': True}
(p / 'final-verification.json').write_text(json.dumps(final, ensure_ascii=False, indent=2) + '\n')
print(json.dumps(final, ensure_ascii=False))
