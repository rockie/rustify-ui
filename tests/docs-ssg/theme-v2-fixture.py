#!/usr/bin/env python3
"""Prepare and optionally build the complete v2 Docs source consumer, offline."""

from __future__ import annotations

import argparse
import hashlib
import json
from pathlib import Path
import runpy
import shutil
import subprocess


def prepare(destination: Path, cli: Path, base: str, build: bool) -> dict:
    repository = Path(__file__).resolve().parents[2]
    helpers = runpy.run_path(str(repository / "scripts/docs-pages-prepare.py"))
    identity = helpers["cli_identity"](cli)
    if identity["mode"] != "source" or identity["repository"] != repository:
        raise ValueError("v2 evidence requires the source CLI from this exact checkout")
    if not base.startswith("/") or not base.endswith("/") or any(value in base for value in ('"', "\\", "..")):
        raise ValueError("base must be an absolute directory URL path")
    destination.mkdir(parents=True, exist_ok=True)
    if any(destination.iterdir()):
        raise ValueError("the author fixture destination must be empty")
    source = repository / "tests/docs-ssg/fixtures/theme-v2"
    shutil.copytree(source, destination, dirs_exist_ok=True)
    manifest = destination / "Cargo.toml"
    manifest.write_bytes(helpers["project_manifest"](manifest.read_bytes(), identity, repository))
    config = destination / "rustify-docs.toml"
    config.write_text(config.read_text().replace("/theme-v2/", base))
    environment = helpers["source_environment"](repository)
    environment.update({"CARGO_NET_OFFLINE": "true", "RUSTIFY_DOCS_CARGO": "mbx"})
    subprocess.run(["mbx", "generate-lockfile", "--offline", "--manifest-path", str(manifest)], cwd=repository, env=environment, check=True)
    subprocess.run(["mbx", "run", "--locked", "--offline", "--manifest-path", str(manifest), "--profile", "docs-host", "--no-default-features", "--features", "ssr", "--bin", "theme-v2-data", "--", str(destination)], cwd=repository, env=environment, check=True)
    # A content-only negative case isolates the old CLI's JSON decoder from
    # Rust framework pairing and island/helper protocol differences.
    negative = destination.parent / (destination.name + "-old-cli")
    negative.mkdir(exist_ok=False)
    (negative / "content").mkdir()
    (negative / "content/index.md").write_text("---\ntitle: Old CLI v2 rejection\ndescription: The same v2 document is rejected by the retained old CLI.\n---\n\n# Old CLI v2 rejection\n\nOnly the theme format differs from a valid old content site.\n")
    (negative / "rustify-docs.toml").write_text('schema_version=1\n[site]\nname="Old CLI v2 rejection"\nurl="https://example.com/"\nbase="/"\n[theme]\ndocument="theme.json"\n')
    shutil.copyfile(destination / "theme.json", negative / "theme.json")
    dirty = bool(subprocess.check_output(["git", "status", "--porcelain"], cwd=repository))
    proof = {
        "site": str(destination), "negative": str(negative),
        "expected": str(destination / ".rustify-docs/theme-v2-expected.json"),
        "source": ("dirty@" if dirty else "") + identity["candidate_sha"],
        "cli_build_info": identity["info"], "framework": identity["framework"],
        "lock_sha256": hashlib.sha256((destination / "Cargo.lock").read_bytes()).hexdigest(),
        "direct_manifests": [str((repository / "crates" / name / "Cargo.toml").resolve()) for name in helpers["FRAMEWORKS"]],
    }
    if build:
        output = destination.parent / (destination.name + "-dist")
        subprocess.run([str(cli), "check", "--config", str(config)], cwd=repository, env=environment, check=True)
        subprocess.run([str(cli), "build", "--config", str(config), "--out", str(output)], cwd=repository, env=environment, check=True)
        proof["dist"] = str(output)
    (destination / ".rustify-docs/theme-v2-provenance.json").write_text(json.dumps(proof, ensure_ascii=False, indent=2) + "\n")
    return proof


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--out", type=Path, required=True)
    parser.add_argument("--cli", type=Path, required=True)
    parser.add_argument("--base", default="/theme-v2/")
    parser.add_argument("--build", action="store_true")
    args = parser.parse_args()
    print(json.dumps(prepare(args.out.resolve(), args.cli.resolve(), args.base, args.build), ensure_ascii=False, indent=2))
