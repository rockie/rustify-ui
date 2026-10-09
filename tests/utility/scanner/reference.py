#!/usr/bin/env python3
"""Check the frozen scanner corpus against the fixed, isolated upstream oracle."""
import hashlib
import json
from pathlib import Path
import subprocess

ROOT = Path(__file__).resolve().parents[3]
SCANNER = Path(__file__).resolve().parent
inventory = json.loads((SCANNER / "inventory.json").read_text())
for entry in inventory["files"] + inventory.get("adapted_sources", []):
    source = ROOT / "ref/tailwindcss" / entry["source"]
    if not source.is_file():
        raise SystemExit(f"fixed upstream source missing: {source}")
    if hashlib.sha256(source.read_bytes()).hexdigest() != entry["sha256"]:
        raise SystemExit(f"fixed upstream source differs: {source}")
    if "local" in entry:
        local = ROOT / entry["local"]
        if hashlib.sha256(local.read_bytes()).hexdigest() != entry.get("local_sha256", entry["sha256"]):
            raise SystemExit(f"vendored source differs: {local}")
result = subprocess.run(
    ["mbx", "run", "--locked", "--manifest-path", str(SCANNER / "oracle/Cargo.toml"),
     "--", str(SCANNER / "fixtures.json")], cwd=ROOT, check=True,
    text=True, stdout=subprocess.PIPE,
)
expected = json.loads((SCANNER / "fixtures.json").read_text())
actual = json.loads(result.stdout)
if expected != actual:
    raise SystemExit("scanner corpus differs from fixed upstream; inspect the difference")
print(f"scanner oracle: {len(actual)} fixed cases agree")

result = subprocess.run(
    ["mbx", "run", "--locked", "--manifest-path", str(SCANNER / "oracle/Cargo.toml"),
     "--", str(SCANNER / "source-fixtures.json"), "--sources"], cwd=ROOT, check=True,
    text=True, stdout=subprocess.PIPE,
)
expected = json.loads((SCANNER / "source-fixtures.json").read_text())
actual = json.loads(result.stdout)
if expected != actual:
    raise SystemExit("source corpus differs from fixed upstream; inspect the difference")
print(f"source oracle: {len(actual)} fixed cases and {sum(len(case['steps']) for case in actual)} operations agree")
