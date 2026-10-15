"""One identity line for the CLI a browser project runs.

identity.py <phase> <rustify-docs>

Run from the repository root in the plan's 9.2 shell, before and after each
project: the binary's path, SHA-256 and --build-info, and the C/R pair that
scripts/docs-pages-prepare.py reads from it for the projected sites.
"""

import hashlib, json, runpy, subprocess, sys
from pathlib import Path

phase, binary = sys.argv[1], Path(sys.argv[2])
info = subprocess.run([str(binary), "--build-info"], capture_output=True, text=True, check=True).stdout.splitlines()
candidate, framework, _ = runpy.run_path("scripts/docs-pages-prepare.py")["cli_pair"](binary)
print(json.dumps({"phase": phase, "binary": str(binary), "sha256": hashlib.sha256(binary.read_bytes()).hexdigest(),
                  "build_info": info, "prepare_pair": {"candidate": candidate, "framework": framework}}))
