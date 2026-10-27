"""Snapshot the actual author-fix sources, builds and current evidence; no acceptance scoring."""
from pathlib import Path
from datetime import datetime
import hashlib
import json
import subprocess

ROOT = Path.cwd()
OUT = Path('docs/plan/DOM-COMPONENTS-P1.ui-captures/r2-apply')

def digest(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()

def write(path, value):
    Path(path).write_text(json.dumps(value, ensure_ascii=False, indent=2) + '\n')

baseline = json.loads((OUT / 'baseline.json').read_text())
changed = [p for p, h in baseline['protected'].items() if not Path(p).is_file() or digest(p) != h]
assert not changed, changed
source_paths = subprocess.check_output(['git', 'diff', 'HEAD', '--name-only'], text=True).splitlines()
source_paths = [p for p in source_paths if p.startswith(('crates/', 'examples/', 'tests/browser/', 'web/', 'xtask/')) or p in ['.gitattributes', '.impeccable/config.json', 'DESIGN.md', 'sources.lock.json']]
source_paths += [str(p) for p in Path('public').rglob('*') if p.is_file()]
sources = {p: digest(p) for p in sorted(set(source_paths))}
artifacts = {}
for app in ['theme-studio', 'component-catalog']:
    directory = Path('target/makepad-wasm-app/release') / app
    artifacts[app] = {
        'directory': str(directory),
        'manifest': json.loads((directory / 'build-manifest.json').read_text()),
        'files': {str(p.relative_to(directory)): {'sha256': digest(p), 'bytes': p.stat().st_size} for p in sorted(directory.rglob('*')) if p.is_file()},
    }
frames = sorted(p for p in (OUT / 'confirm').glob('*.png') if p.name.startswith(('U6', 'U7')))
frames += sorted(p for p in (OUT / 'confirm-final').glob('*.png') if p.name != 'U5-inspector-picked-1440-light.png')
frames += sorted((OUT / 'confirm-final-inspector').glob('*.png'))
assert len(frames) == 120, len(frames)
slider = sorted((OUT / 'slider-confirm').glob('*.png'))
assert len(slider) == 4
centers = sorted((OUT / 'slider-center').glob('*.png'))
assert len(centers) == 4
at = datetime.now().astimezone().isoformat(timespec='seconds')
value = {
    'at': at,
    'baseline': {'head': baseline['head'], 'branch': 'main', 'worktree': 'dirty'},
    'source_hash_algorithm': 'SHA-256 of UTF-8 canonical JSON mapping sorted repository-relative paths to per-file SHA-256; tracked task changes plus new public assets; unrelated user files excluded',
    'source_fingerprint': hashlib.sha256(json.dumps(sources, sort_keys=True, separators=(',', ':'), ensure_ascii=False).encode()).hexdigest(),
    'source_files': sources,
    'artifacts': artifacts,
    'capture_reuse': 'Studio U1–U5/SUP and named-reference/configuration/Inspector states recaptured on the final build. The 40 Catalog U6/U7 frames reuse confirm: their markup and visible styles are unchanged by the later Studio fixes and Slider-only CSS correction. Main DOM picked frame is superseded by confirm-final-inspector; GPU picked and v2 configuration are included. Slider has four additional final frames.',
    'current_captures': {str(p): digest(p) for p in frames},
    'supplementary_slider_captures': {str(p): digest(p) for p in slider},
    'supplementary_slider_center_captures': {str(p): digest(p) for p in centers},
    'protected_files': len(baseline['protected']),
    'protected_changed': changed,
    'target_file_sha256': digest('docs/plan/DOM-COMPONENTS-P1.ui-target.md'),
    'target_normative_fingerprint': '567117725f56f04104860e0b77173cb2e37913b14014c530158f7d989b1dd5d8',
}
write(OUT / 'current-version.json', value)
write(OUT / 'checks/protected-check.json', {'at': at, 'protected_files': len(baseline['protected']), 'changed': changed})
print(json.dumps({'sources': len(sources), 'fingerprint': value['source_fingerprint'], 'matrix_frames': len(frames), 'slider_frames': len(slider), 'protected': len(baseline['protected']), 'changed': changed}))
