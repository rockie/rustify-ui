"""Restore evicted previews and require the original production fingerprints."""

import hashlib
import json
import shutil
import subprocess
from datetime import datetime, timezone
from pathlib import Path

repo = Path(__file__).resolve().parents[4]
evidence = Path(__file__).parent
expected = json.loads((evidence / "matrix-artifacts.json").read_text())
binary = repo / "target/docs-cli/rustify-docs"
durable = Path("/tmp/rustify-ui-docs-ssg-gaps-42757c6")
durable.mkdir(exist_ok=True)
recovered = durable / "rustify-docs-recovered"
shutil.copy2(binary, recovered)
entries = []
for entry in expected["sites"]:
    name = entry["name"]
    site = repo / "target/docs-ssg/gaps/matrix" / name / "site"
    out = repo / entry["out"]
    if site.exists() or out.exists():
        raise SystemExit(f"{name} already exists; inspect before replacing it")
    fixture = "home" if name.startswith("home-") else "content"
    shutil.copytree(repo / "tests/docs-ssg/fixtures" / fixture, site)
    config = site / "rustify-docs.toml"
    config.write_text(config.read_text().replace("/docs/", entry["base"]))
    subprocess.run([str(recovered), "build", "--config", str(config), "--out", str(out)], check=True)
    manifest = out / "rustify-docs-manifest.json"
    actual = hashlib.sha256(manifest.read_bytes()).hexdigest()
    if actual != entry["manifest_sha256"]:
        raise SystemExit(f"Production fingerprint changed for {name}: {actual}")
    saved = durable / name / "dist"
    if saved.exists():
        raise SystemExit(f"{saved} already exists")
    shutil.copytree(out, saved)
    entries.append({**entry, "recovered_out": str(saved), "matches_original": True})
report = {
    "observed_at": datetime.now(timezone.utc).isoformat(),
    "source_candidate": "42757c627cb69456d241ba0de51b34f1ad85be3d",
    "reason": "The shared target cache was removed while its preview processes remained alive.",
    "original_binary_sha256": expected["binary_sha256"],
    "recovered_binary": str(recovered),
    "recovered_binary_sha256": hashlib.sha256(recovered.read_bytes()).hexdigest(),
    "sites": entries,
}
(evidence / "preview-recovery.json").write_text(json.dumps(report, indent=2) + "\n")
print(json.dumps(report, indent=2))
