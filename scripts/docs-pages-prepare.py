#!/usr/bin/env python3
"""Prepare the frozen official sources with the Git framework pair emitted by C.

Python 3.12+, standard library only. Does not build or deploy the site.
Uses a detached C worktree with real history; git archive supplies byte hashes.
Only site/Cargo.toml and site/Cargo.lock may differ from the frozen commit.
Project mode adapts test consumers with the same installed/under-test CLI pair.
"""

import argparse
import hashlib
import json
from pathlib import Path
import re
import shutil
import subprocess
import tarfile
import tempfile
import tomllib


REPOSITORY = "https://github.com/rockie/rustify-ui"
FRAMEWORKS = ("rustify-docs", "rustify-ui", "rustify-components")
PROJECTED = {"site/Cargo.toml", "site/Cargo.lock"}


def digest(stream):
    return hashlib.file_digest(stream, "sha256").hexdigest()


def fingerprint(path):
    if path.is_symlink():
        return "symlink:" + hashlib.sha256(path.readlink().as_posix().encode()).hexdigest()
    with path.open("rb") as stream:
        return digest(stream)


def verify_inputs(root, expected):
    for name, checksum in expected.items():
        if fingerprint(root / name) != checksum:
            raise ValueError(f"source changed after preparation: {name}")


def run(command, **kwargs):
    result = subprocess.run(command, stderr=subprocess.PIPE, **kwargs)
    if result.returncode:
        raise ValueError(f"{command[0]} failed: {result.stderr.decode(errors='replace')[-2400:]}")
    return result


def cli_pair(cli):
    """Read R from the real Git C named by this CLI, never a dirty asset."""
    cli = cli.resolve(strict=True)
    info = run([str(cli), "--build-info"], stdout=subprocess.PIPE).stdout.decode()
    sources = re.findall(r"^source: ([0-9a-f]{40})$", info, re.MULTILINE)
    if len(sources) != 1:
        raise ValueError("CLI --build-info must identify one traceable source commit C")
    candidate = sources[0]
    repository = Path(__file__).resolve().parents[1]
    try:
        framework = run(["git", "show", f"{candidate}:crates/rustify-docs-cli/assets/framework-revision.txt"],
                        cwd=repository, stdout=subprocess.PIPE).stdout.decode().strip()
    except ValueError as error:
        raise ValueError(f"cannot read paired R from CLI source C `{candidate}` in the local Git repository: {error}") from error
    if framework and not re.fullmatch(r"[0-9a-f]{40}", framework):
        raise ValueError("the CLI's Git C must contain an empty development R or a complete paired R")
    return candidate, framework, info


def project_bytes(original, generated, lock, framework):
    """Project three dependency lines; retain all other manifest bytes."""
    document = tomllib.loads(original.decode())
    paired = tomllib.loads(generated.decode())
    if paired["package"] != document["package"]:
        raise ValueError("new(C) package identity differs from the original consumer")
    projected = original
    for name in FRAMEWORKS:
        wanted = {"git": REPOSITORY, "rev": framework, "default-features": False}
        if paired["dependencies"][name] != wanted:
            raise ValueError(f"new(C) did not emit the exact Git R dependency for {name}")
        previous = document["dependencies"][name]
        if previous != wanted and (set(previous) != {"path", "default-features"}
                                   or previous["default-features"] is not False):
            raise ValueError(f"unexpected original framework dependency for {name}")
        old = [line for line in original.splitlines(keepends=True) if line.startswith(f"{name} = ".encode())]
        new = [line for line in generated.splitlines(keepends=True) if line.startswith(f"{name} = ".encode())]
        if len(old) != 1 or len(new) != 1:
            raise ValueError(f"expected one dependency line for {name}")
        projected = projected.replace(old[0], new[0], 1)
    packages = tomllib.loads(lock.decode())["package"]
    for name in FRAMEWORKS:
        matching = [package for package in packages if package["name"] == name]
        if len(matching) != 1 or matching[0].get("source") != f"git+{REPOSITORY}?rev={framework}#{framework}":
            raise ValueError(f"new(C) lock does not resolve one {name} at Git R")
    roots = [package for package in packages if package["name"] == document["package"]["name"] and "source" not in package]
    if len(roots) != 1 or roots[0]["version"] != document["package"]["version"]:
        raise ValueError("new(C) lock has the wrong consumer root package")
    bindings = [package["version"] for package in packages if package["name"] == "wasm-bindgen"]
    if not bindings or any(version != "0.2.128" for version in bindings):
        raise ValueError("new(C) lock must resolve wasm-bindgen exactly 0.2.128")
    return projected


def project_pair(cli, manifest, framework):
    original = manifest.read_bytes()
    package = tomllib.loads(original.decode())["package"]["name"]
    with tempfile.TemporaryDirectory(prefix="rustify-pages-new-") as scratch:
        generated = Path(scratch) / package
        run([str(cli), "new", str(generated), "--template", "components"], stdout=subprocess.PIPE)
        lock = (generated / "Cargo.lock").read_bytes()
        projected = project_bytes(original, (generated / "Cargo.toml").read_bytes(), lock, framework)
    manifest.write_bytes(projected)
    manifest.with_name("Cargo.lock").write_bytes(lock)


def prepare_project(cli, manifest, out=None):
    candidate, framework, _ = cli_pair(cli)
    manifest = manifest.resolve(strict=True)
    if framework:
        if out is not None:
            if out.is_symlink() or (out.exists() and (not out.is_dir() or any(out.iterdir()))):
                raise ValueError("--out must be an absent or empty consumer directory")
            out.mkdir(parents=True, exist_ok=True)
            for name in ("src", "content", "static", "Cargo.toml", "Cargo.lock", "rustify-docs.toml", "theme.json"):
                source = manifest.parent / name
                if source.is_dir():
                    shutil.copytree(source, out / name, symlinks=True)
                elif source.exists():
                    shutil.copy2(source, out / name)
            manifest = out / manifest.name
        project_pair(cli.resolve(strict=True), manifest, framework)
    return {"candidate_sha": candidate, "framework_revision": framework,
            "manifest": str(manifest), "project": str(manifest.parent)}


def prepare(candidate, cli, out):
    if not re.fullmatch(r"[0-9a-f]{40}", candidate):
        raise ValueError("--candidate-sha must be the complete lowercase C commit SHA")
    if out.is_symlink() or (out.exists() and (not out.is_dir() or any(out.iterdir()))):
        raise ValueError("--out must be an absent or empty directory, without a symlink")
    cli = cli.resolve(strict=True)
    source, framework, info = cli_pair(cli)
    if source != candidate:
        raise ValueError("installed CLI --build-info source must equal candidate C")
    if "\npanic: unwind\n" not in info:
        raise ValueError("install the candidate CLI with --profile docs-cli")
    repository = Path(__file__).resolve().parents[1]
    resolved = run(["git", "rev-parse", f"{candidate}^{{commit}}"], cwd=repository,
                   stdout=subprocess.PIPE).stdout.decode().strip()
    if resolved != candidate:
        raise ValueError("candidate must identify a commit in the local repository")
    out.mkdir(parents=True, exist_ok=True)
    original = {}
    with tempfile.TemporaryFile() as stream:
        run(["git", "archive", "--format=tar", candidate], cwd=repository, stdout=stream)
        stream.seek(0)
        with tarfile.open(fileobj=stream) as archive:
            for member in archive.getmembers():
                if member.isfile():
                    with archive.extractfile(member) as source:
                        original[member.name] = digest(source)
                elif member.issym():
                    original[member.name] = "symlink:" + hashlib.sha256(member.linkname.encode()).hexdigest()
    run(["git", "worktree", "add", "--detach", str(out), candidate], cwd=repository,
        stdout=subprocess.PIPE)
    verify_inputs(out, original)
    if not re.fullmatch(r"[0-9a-f]{40}", framework):
        raise ValueError("candidate C must contain a complete paired framework revision R")
    official = out / "site/Cargo.toml"
    official_document = tomllib.loads(official.read_text())
    for name in FRAMEWORKS:
        if official_document["dependencies"][name] != {"path": f"../crates/{name}", "default-features": False}:
            raise ValueError(f"unexpected frozen official dependency for {name}")
    project_pair(cli, official, framework)
    protected = {name: checksum for name, checksum in original.items() if name not in PROJECTED}
    verify_inputs(out, protected)
    inputs = dict(protected)
    inputs.update({name: fingerprint(out / name) for name in PROJECTED})
    evidence = {
        "schema_version": 1, "candidate_sha": candidate, "framework_revision": framework,
        "cli_build_info": info, "cli_sha256": fingerprint(cli),
        "projection": {name: {"original_sha256": original[name], "projected_sha256": inputs[name]} for name in sorted(PROJECTED)},
        "protected_files": len(protected), "inputs": inputs,
        "scope": "detached C worktree; only three dependency lines and the paired lock are projected",
    }
    receipt = out / ".rustify-docs/pages-prepare.json"
    receipt.parent.mkdir(parents=True, exist_ok=True)
    receipt.write_text(json.dumps(evidence, indent=2, sort_keys=True) + "\n")
    print(json.dumps({"candidate_sha": candidate, "framework_revision": framework,
                      "protected_files": len(protected), "receipt": str(receipt)}))


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    mode = parser.add_mutually_exclusive_group(required=True)
    mode.add_argument("--candidate-sha")
    mode.add_argument("--project", type=Path, help="Cargo.toml of an explicit test consumer")
    parser.add_argument("--cli", required=True, type=Path)
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()
    try:
        if args.project is not None:
            print(json.dumps(prepare_project(args.cli, args.project,
                                            args.out.absolute() if args.out is not None else None)))
        elif args.out is None:
            parser.error("--candidate-sha requires --out")
        else:
            prepare(args.candidate_sha, args.cli, args.out.absolute())
    except (ValueError, OSError, KeyError, tarfile.TarError) as error:
        parser.exit(1, f"docs-pages-prepare: {error}\n")


if __name__ == "__main__":
    main()
