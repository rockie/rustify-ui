"""Read-only integrity verification for the completed manual R7 review."""
from pathlib import Path
from decimal import Decimal
from datetime import datetime, timezone
import hashlib
import json
import re
import struct
import subprocess
from urllib.parse import unquote

ROOT = Path(__file__).resolve().parents[4]
HERE = Path(__file__).resolve().parent
checks = {}

def read(name):
    return json.loads((HERE / name).read_text())

def digest(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()

def field(path, name):
    match = re.search(r'^' + re.escape(name) + r': (.+)$', path.read_text(), re.M)
    assert match, (str(path), name)
    return json.loads(match[1])

def command(*args):
    return subprocess.check_output(args, cwd=ROOT).decode()

images = read('image-integrity.json')
machines = read('machine-captures.json')['images']
for row in images + machines:
    path = HERE / row['file']
    assert digest(path) == row['sha256'], path
    header = path.read_bytes()[:24]
    assert header[:8] == b'\x89PNG\r\n\x1a\n' and header[12:16] == b'IHDR', path
    size = row.get('pixel_dimensions', [row.get('width'), row.get('height')])
    assert list(struct.unpack('>II', header[16:24])) == size, path
    if 'bytes' in row:
        assert path.stat().st_size == row['bytes'], path
assert len(images) == 278 and len(machines) == 14
checks['original_images_verified'] = {'scored': len(images), 'machine': len(machines)}

results = read('judge-results.json')
for name, sha in results['raw_sha256'].items():
    assert digest(HERE / name) == sha, name
for unit, row in results['scores'].items():
    assert sum(map(Decimal, row['dimensions'])) == Decimal(row['total']), unit
assert min(Decimal(row['total']) for row in results['scores'].values()) == Decimal('9.8')
assert results['minimum_unit'] == 'U5'
assert [(r['id'], r['severity']) for r in results['open_gaps']] == [('UI-218', 'P2')]
dispatch = read('judge-dispatch.json')['entries']
assert len(dispatch) == 10
opened = set()
packet_file_count = 0
for entry in dispatch:
    assert entry['fork_turns'] == 'none' and entry['read_only'] is True
    for key in ('task', 'packet'):
        assert digest(HERE / entry[key + '_file']) == entry[key + '_sha256'], entry['task']
    assert digest(HERE / entry['raw']) == entry['raw_sha256']
    packet = json.loads((HERE / entry['packet_file']).read_text())
    raw = (HERE / entry['raw']).read_text()
    for name, sha in packet['files'].items():
        source = Path(entry.get('archive_mapping', {}).get(name, name))
        assert digest(source) == sha, source
        packet_file_count += 1
        original = Path(name)
        if original.parent == HERE and original.suffix == '.png':
            assert original.name in raw, (entry['raw'], original.name)
            opened.add(original.name)
assert opened == {r['file'] for r in images}
checks['judging_verified'] = {'raw_conclusions': 10, 'packet_file_hashes': packet_file_count,
                             'current_originals_in_actual_open_records': len(opened),
                             'minimum': '9.8/U5', 'open_gap': 'UI-218/P2'}

target = read('target.json')
for name in ('target', 'frozen_snapshot', 'r7_snapshot'):
    output = command('python3', str(HERE.parent / 'r5/target-fingerprint.py'),
                     str(ROOT / target[name]), str(ROOT))
    assert output.splitlines()[-1] == 'fingerprint ' + target['fingerprint'], name
for name, key in [('target', 'current_target_sha256'), ('frozen_snapshot', 'target_sha256'),
                  ('r7_snapshot', 'r7_snapshot_sha256')]:
    assert digest(ROOT / target[name]) == target[key], name
current = (ROOT / target['target']).read_text()
previous = (ROOT / target['frozen_snapshot']).read_text()
assert current.split('## 采集')[0] == previous.split('## 采集')[0]
attachments = 0
for line in (HERE / 'target-files.txt').read_text().splitlines():
    sha, name = line.split(' ', 1)
    if sha != 'fingerprint':
        assert digest(ROOT / name) == sha, name
        attachments += 1
assert attachments == 13
checks['target_verified'] = {'same_normative_fingerprint': target['fingerprint'],
                             'attachments': attachments, 'changes_only_collection_and_log': True}

registration = read('sdlc-registration.json')
review_id = registration['review_id']
old_id = '4096c476-5ee3-44a5-a195-09a5183542ca'
sdlc = ROOT / '.xgent-ai/sdlc'
events = [sdlc / 'events/review' / review_id / (eid + '.md')
          for eid in registration['review_events']]
events.append(sdlc / 'events/review' / old_id / (registration['r6_closure_event'] + '.md'))
assert 'sha256:' + digest(ROOT / 'docs/plan/DOM-COMPONENTS-P1.ui-review.r7.md') == registration['report_sha256']
source_count = 0
for event in events:
    assert field(event, 'event_id') == event.stem
    for source in field(event, 'sources'):
        assert 'sha256:' + digest(ROOT / source['path']) == source['revision'], source['path']
        source_count += 1
    for predecessor in field(event, 'predecessors'):
        assert (event.parent / (predecessor + '.md')).is_file(), predecessor
    assert event.stem in (event.parent / 'index.md').read_text()
for oid in (review_id, old_id):
    state = sdlc / 'status/review' / (oid + '.md')
    included = set(field(state, 'included_events'))
    existing = {p.stem for p in (sdlc / 'events/review' / oid).glob('*.md') if p.stem != 'index'}
    assert included == existing, oid
    for index in ('status/review/index.md', 'events/review/index.md'):
        assert oid in (sdlc / index).read_text(), index
for name, sha in read('sdlc-immutable-before.json').items():
    assert digest(ROOT / name) == sha, name
checks['sdlc_verified'] = {'new_events': len(events), 'source_hashes': source_count,
                          'immutable_originals': len(read('sdlc-immutable-before.json'))}

docs = [ROOT / 'docs/plan/DOM-COMPONENTS-P1.ui-review.r7.md',
        ROOT / target['target'], ROOT / target['r7_snapshot'], HERE / 'capture.md', HERE / 'judge.md']
docs += sorted(HERE.glob('capture-U*.md')) + sorted(HERE.glob('judge-U?-?.md'))
docs += events + [e.parent / 'index.md' for e in events]
docs += [sdlc / 'status/review' / (oid + '.md') for oid in (review_id, old_id)]
docs += [sdlc / name for name in ('status/review/index.md', 'events/review/index.md', 'index.md',
         'status/plan/37a2c7b4-7052-4f85-96e9-135b6f7667a1.md')]
link_count = 0
for doc in set(docs):
    text = re.sub(r'```.*?```', '', doc.read_text(), flags=re.S)
    for link in re.findall(r'\]\(([^)]+)\)', text):
        url = unquote(link.split('#', 1)[0].strip('<>'))
        if not url or re.match(r'[a-zA-Z][a-zA-Z0-9+.-]*:', url):
            continue
        assert (doc.parent / url).exists(), (str(doc.relative_to(ROOT)), link)
        link_count += 1
checks['local_links_verified'] = link_count

version = read('tested-version.json')
assert command('git', 'rev-parse', 'HEAD').strip() == version['head']
assert command('git', 'diff', 'HEAD', '--', *version['product_paths']) == version['product_diff'] == ''
assert command('git', 'ls-files', '--others', '--exclude-standard', '--', *version['product_paths']) == version['product_untracked'] == ''
product_files = sorted(command('git', 'ls-files', '-z', '--', *version['product_paths']).strip('\0').split('\0'))
hash_ = hashlib.sha256()
for name in product_files:
    hash_.update(name.encode())
    hash_.update(b'\0')
    hash_.update(hashlib.sha256((ROOT / name).read_bytes()).digest())
assert hash_.hexdigest() == version['product_fingerprint_sha256']
assert read('version-final-check.json')['matches_initial'] is True
services = read('services.json')
assert services['stopped'] is True and services['remaining_listeners'] == ''
listeners = subprocess.run(['lsof', '-nP', '-iTCP:4181', '-iTCP:4176', '-sTCP:LISTEN'], capture_output=True, text=True)
assert listeners.stdout == '' and listeners.returncode == 1, listeners.stdout
checks['product_and_cleanup_verified'] = {'head': version['head'], 'product_files': len(product_files),
                                         'fingerprint': hash_.hexdigest(), 'services_stopped': True}
output = {'at': datetime.now(timezone.utc).isoformat(), 'passed': True, 'checks': checks}
(HERE / 'final-proof.json').write_text(json.dumps(output, indent=2, ensure_ascii=False) + '\n')
print(json.dumps(output, indent=2, ensure_ascii=False))
