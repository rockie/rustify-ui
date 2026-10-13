from pathlib import Path
import json
import subprocess

root = Path(__file__).resolve().parents[4]
out = root / 'docs/validation/docs-ssg-p2/m1'
runs = []
for package, label in [('rustify-ui', 'sdk'), ('rustify-components', 'components')]:
    for mode, flags in [('default', []), ('gpu', ['--features', 'gpu']), ('ssr', ['--no-default-features', '--features', 'ssr'])]:
        runs.append((f'{label}-{mode}-test', ['mbx', 'test', '--locked', '-p', package, '--lib', *flags]))
        runs.append((f'{label}-{mode}-clippy', ['mbx', 'clippy', '--locked', '-p', package, '--all-targets', *flags, '--', '-D', 'warnings']))
runs += [('fmt', ['cargo', 'fmt', '--all', '--', '--check']), ('sources', ['mbx', 'xtask', 'sources', 'verify']), ('catalog', ['mbx', 'xtask', 'catalog', '--write', 'docs/components.md', '--check'])]
results = []
for name, command in runs:
    with (out / f'{name}.log').open('w') as log:
        result = subprocess.run(command, cwd=root, stdout=log, stderr=subprocess.STDOUT)
    results.append({'name':name,'command':command,'exit_code':result.returncode})
    (out / 'modes.json').write_text(json.dumps(results, indent=2)+'\n')
    print(f'{name}: {result.returncode}', flush=True)
    if result.returncode:
        raise SystemExit(result.returncode)
