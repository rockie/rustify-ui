import json
import os
import pathlib
import shutil
import subprocess
import sys
import tempfile
from datetime import datetime, timezone

evidence = pathlib.Path(__file__).resolve().parent
root = evidence.parents[3]
scratch = pathlib.Path(tempfile.gettempdir()) / 'rustify-docs-verify'
binary = scratch / 'cli-install/bin/rustify-docs'
info = subprocess.check_output([str(binary), '--build-info'], text=True)
assert 'source: 1b81de25f23a25702f61dbfd3d0ecdfe323d4d44' in info
assert info.startswith('rustify-docs 0.1.4\n')
env = dict(os.environ, RUSTIFY_DOCS_BIN=str(binary), DOM_ONLY_DIR=str(scratch / 'dom-consumer/public'))
results = []
for project in ['dom-only', 'docs-root', 'docs-subpath', 'docs-meta', 'docs-dev']:
    args = ['npx', 'playwright', 'test', '-c', 'playwright.docs.config.ts', '--project=' + project]
    print('Browser: ' + project, flush=True)
    with (evidence / (project + '.log')).open('w') as log:
        completed = subprocess.run(args, cwd=root, env=env, stdout=log, stderr=subprocess.STDOUT)
    report = root / 'test-results/docs/results.json'
    if report.exists():
        shutil.copy2(report, evidence / (project + '.json'))
    results.append({'project': project, 'command': args, 'source': info, 'observed_at': datetime.now(timezone.utc).isoformat(), 'exit_code': completed.returncode})
    (evidence / 'browser.json').write_text(json.dumps(results, indent=2) + '\n')
    print(project + ': exit ' + str(completed.returncode), flush=True)
    if completed.returncode:
        sys.exit(completed.returncode)
