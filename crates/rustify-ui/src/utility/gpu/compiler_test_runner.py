#!/usr/bin/env python3
"""Run native compiler -> GPU tests with explicit compiler dependency unification.

Uses a temporary Cargo workspace, the repository lock, and the shared target cache.
Production rustify-ui continues to depend only on lightweight protocol/cn features.
"""
from pathlib import Path
import json
import os
import shutil
import subprocess
import tempfile

root = Path(__file__).resolve().parents[5]
with tempfile.TemporaryDirectory(prefix="rustify-gpu-compiler-") as temporary:
    project = Path(temporary)
    (project / "src").mkdir()
    manifest = '''[package]
name = "rustify-gpu-compiler-contract"
version = "0.0.0"
edition = "2021"
[workspace]
[dependencies]
rustify-ui = { path = UI_PATH, features = ["utility"] }
rustify-style = { path = STYLE_PATH, default-features = false, features = ["compiler"] }
'''.replace("UI_PATH", json.dumps(str(root / "crates/rustify-ui"))).replace(
        "STYLE_PATH", json.dumps(str(root / "crates/rustify-style")))
    manifest += '\n[patch.crates-io]\n'
    for name in ("bitflags", "smallvec"):
        manifest += name + " = { path = " + json.dumps(str(root / "makepad/libs" / name)) + " }\n"
    (project / "Cargo.toml").write_text(manifest)
    shutil.copyfile(root / "Cargo.lock", project / "Cargo.lock")
    shutil.copyfile(Path(__file__).with_name("compiler_tests.rs"), project / "src/lib.rs")
    environment = dict(os.environ)
    environment.setdefault("CARGO_TARGET_DIR", str(root / "target"))
    subprocess.run(["mbx", "test", "--offline", "--manifest-path", str(project / "Cargo.toml")],
                   cwd=root, env=environment, check=True)
