#!/usr/bin/env python3
"""Prepare consumers with the registry, source or historical Git CLI pair.

Python 3.12+, standard library only. Does not build or deploy the site.
Uses a detached C worktree with real history; git archive supplies byte hashes.
Only site/Cargo.toml and site/Cargo.lock may differ from the frozen commit.
Project mode adapts test consumers with the same installed/under-test CLI pair.
"""

import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import shutil
import subprocess
import tarfile
import tempfile
import tomllib


REPOSITORY = "https://github.com/rockie/rustify-ui"
FRAMEWORKS = ("rustify-docs", "rustify-dom", "rustify-components")
LEGACY_FRAMEWORKS = ("rustify-docs", "rustify-ui", "rustify-components")
CRATES_IO = "registry+https://github.com/rust-lang/crates.io-index"
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


def read_identity(info, repository):
    repository = repository.resolve(strict=True)
    sources = re.findall(r"^source: (\S+)$", info, re.MULTILINE)
    versions = re.findall(r"^rustify-docs (\S+)$", info, re.MULTILINE)
    distributions = re.findall(r"^distribution: (\S+)$", info, re.MULTILINE)
    records = re.findall(r"^framework: (.+)$", info, re.MULTILINE)
    if len(sources) != 1 or len(versions) != 1:
        raise ValueError("CLI --build-info must identify one version and source")
    candidate = sources[0]
    if not distributions and not records:
        if not re.fullmatch(r"[0-9a-f]{40}", candidate):
            raise ValueError("historical Git CLI must identify a complete source C")
        framework = run(["git", "show", f"{candidate}:crates/rustify-docs-cli/assets/framework-revision.txt"],
                        cwd=repository, stdout=subprocess.PIPE).stdout.decode().strip()
        if framework and not re.fullmatch(r"[0-9a-f]{40}", framework):
            raise ValueError("historical Git C has an invalid paired R")
        return {"mode":"legacy", "candidate_sha":candidate, "framework_revision":framework, "info":info}
    if len(distributions) != 1 or len(records) != 1 or distributions[0] not in ("registry", "source"):
        raise ValueError("CLI distribution/framework identity is incomplete or unsupported")
    record = json.loads(records[0])
    if (record.get("schema") != 1 or record.get("registry") != "crates-io"
            or set(record.get("packages", {})) != set(FRAMEWORKS)
            or any(not isinstance(v, str) or not v for v in record["packages"].values())
            or record.get("wasm_bindgen") != "0.2.128"):
        raise ValueError("invalid CLI framework release record")
    mode = distributions[0]
    framework = record.get("library_rev")
    if mode == "registry" and not re.fullmatch(r"[0-9a-f]{40}", framework or ""):
        raise ValueError("registry CLI must have a frozen library R")
    if mode == "source":
        actual = run(["git", "rev-parse", "HEAD"], cwd=repository, stdout=subprocess.PIPE).stdout.decode().strip()
        if candidate != actual or record.get("development_checkout") != hashlib.sha256(str(repository).encode()).hexdigest():
            raise ValueError("source CLI must match this exact canonical checkout and HEAD")
        for name, version in {**record["packages"], "rustify-docs-cli":versions[0]}.items():
            with (repository / "crates" / name / "Cargo.toml").open("rb") as stream:
                actual_version = tomllib.load(stream)["package"]["version"]
                if actual_version == {"workspace": True}:
                    with (repository / "Cargo.toml").open("rb") as workspace:
                        actual_version = tomllib.load(workspace)["workspace"]["package"]["version"]
                if actual_version != version:
                    raise ValueError(f"source CLI version mismatch for {name}")
    return {"mode":mode, "candidate_sha":candidate, "framework_revision":framework or "", "framework":record, "info":info}


def cli_identity(cli):
    info = run([str(cli.resolve(strict=True)), "--build-info"], stdout=subprocess.PIPE).stdout.decode()
    repository = Path(__file__).resolve().parents[1]
    if "\ndistribution: source\n" in info:
        records = re.findall(r"^framework: (.+)$", info, re.MULTILINE)
        if len(records) != 1:
            raise ValueError("source CLI omitted its framework JSON")
        fingerprint = json.loads(records[0]).get("development_checkout")
        cargo_home = Path(os.environ.get("CARGO_HOME", Path.home() / ".cargo"))
        candidates = [repository, *(cargo_home / "git/checkouts").glob("*/*")]
        matches = {path.resolve() for path in candidates if path.is_dir()
                   and hashlib.sha256(str(path.resolve()).encode()).hexdigest() == fingerprint}
        if len(matches) != 1:
            raise ValueError("cannot locate the exact checkout compiled into this source CLI")
        repository = matches.pop()
    identity = read_identity(info, repository)
    identity["repository"] = repository.resolve()
    return identity


def cli_pair(cli):
    identity = cli_identity(cli)
    return identity["candidate_sha"], identity["framework_revision"], identity["info"]


def project_manifest(original, identity, repository):
    """Keep author bytes and dependency aliases; change only framework sources."""
    document = tomllib.loads(original.decode())
    projected = original
    found = set()
    for key, dependency in document["dependencies"].items():
        actual = dependency.get("package", key) if isinstance(dependency, dict) else key
        name = "rustify-dom" if actual == "rustify-ui" else actual
        if name not in FRAMEWORKS:
            continue
        if name in found or not isinstance(dependency, dict):
            raise ValueError(f"consumer needs one table dependency for {name}")
        found.add(name)
        wanted = {k:v for k,v in dependency.items() if k not in ("path", "git", "rev", "branch", "tag", "registry", "version", "package")}
        if key != name:
            wanted["package"] = name
        wanted["version"] = "=" + identity["framework"]["packages"][name]
        if identity["mode"] == "source":
            wanted["path"] = str((repository / "crates" / name).resolve(strict=True))
        lines = [line for line in projected.splitlines(keepends=True) if line.startswith(f"{key} = ".encode())]
        if len(lines) != 1:
            raise ValueError(f"expected one dependency line for {key}")
        fields = ", ".join(f"{k} = {json.dumps(v)}" for k,v in wanted.items())
        newline = b"\r\n" if lines[0].endswith(b"\r\n") else b"\n"
        projected = projected.replace(lines[0], f"{key} = {{ {fields} }}".encode() + newline, 1)
    if found != set(FRAMEWORKS):
        raise ValueError("consumer must directly depend on all three framework packages")
    return projected


def validate_registry_lock(lock, identity, original):
    packages = tomllib.loads(lock.decode())["package"]
    root = tomllib.loads(original.decode())["package"]
    roots = [p for p in packages if not p.get("source")]
    if len(roots) != 1 or (roots[0]["name"], roots[0]["version"]) != (root["name"], root["version"]):
        raise ValueError("new(C) lock has the wrong consumer root package")
    for package in packages:
        if package is roots[0]:
            continue
        if package.get("source") != CRATES_IO or not re.fullmatch(r"[0-9a-f]{64}", package.get("checksum", "")):
            raise ValueError("new(C) lock must contain only canonical registry dependencies with checksums")
    for name, version in {**identity["framework"]["packages"], "wasm-bindgen":identity["framework"]["wasm_bindgen"]}.items():
        matching = [p for p in packages if p["name"] == name]
        if len(matching) != 1 or matching[0]["version"] != version:
            raise ValueError(f"new(C) lock must resolve one {name}@{version}")


def project_bytes(original, generated, lock, framework):
    """Project three dependency lines; retain all other manifest bytes."""
    document = tomllib.loads(original.decode())
    paired = tomllib.loads(generated.decode())
    if paired["package"] != document["package"]:
        raise ValueError("new(C) package identity differs from the original consumer")
    projected = original
    for name in LEGACY_FRAMEWORKS:
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
    for name in LEGACY_FRAMEWORKS:
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


def source_environment(repository):
    with (repository / "mise.toml").open("rb") as stream:
        toolchain = tomllib.load(stream)["tools"]["rust"]["version"]
    sysroot = Path(run(["rustc", "--print", "sysroot"], cwd=repository, stdout=subprocess.PIPE).stdout.decode().strip())
    compiler = sysroot / "bin/rustc"
    if not run([str(compiler), "--version"], stdout=subprocess.PIPE).stdout.decode().startswith(f"rustc {toolchain} "):
        raise ValueError("source projection requires the checkout's mise-pinned compiler")
    return {**os.environ, "RUSTC":str(compiler), "RUSTDOC":str(sysroot / "bin/rustdoc"),
            "RUSTUP_TOOLCHAIN":toolchain, "PATH":str(sysroot / "bin") + os.pathsep + os.environ.get("PATH", "")}


def prepare_project(cli, manifest, out=None):
    identity = cli_identity(cli)
    repository = identity["repository"]
    manifest = manifest.resolve(strict=True)
    if out is not None:
        if out.is_symlink() or (out.exists() and (not out.is_dir() or any(out.iterdir()))):
            raise ValueError("--out must be an absent or empty consumer directory")
        out.mkdir(parents=True, exist_ok=True)
        for name in ("src", "content", "static", "fonts", "Cargo.toml", "Cargo.lock", "rustify-docs.toml", "theme.json", "theme.example.json"):
            source = manifest.parent / name
            if source.is_dir():
                shutil.copytree(source, out / name, symlinks=True)
            elif source.exists():
                shutil.copy2(source, out / name)
        manifest = out / manifest.name
    if identity["mode"] == "legacy":
        if identity["framework_revision"]:
            project_pair(cli.resolve(strict=True), manifest, identity["framework_revision"])
    else:
        original = manifest.read_bytes()
        projected = project_manifest(original, identity, repository)
        if identity["mode"] == "registry":
            package = tomllib.loads(original.decode())["package"]["name"]
            with tempfile.TemporaryDirectory(prefix="rustify-registry-new-") as scratch:
                generated = Path(scratch) / package
                run([str(cli.resolve(strict=True)), "new", str(generated), "--template", "components"], stdout=subprocess.PIPE)
                generated_manifest = tomllib.loads((generated / "Cargo.toml").read_text())
                for name, version in identity["framework"]["packages"].items():
                    if generated_manifest["dependencies"][name] != {"version":"="+version, "default-features":False}:
                        raise ValueError(f"new(C) manifest mismatch for {name}")
                lock = (generated / "Cargo.lock").read_bytes()
                validate_registry_lock(lock, identity, original)
            manifest.write_bytes(projected)
            manifest.with_name("Cargo.lock").write_bytes(lock)
        else:
            manifest.write_bytes(projected)
            # Source fixtures have their own genuine path lock. Never edit the
            # embedded registry lock or substitute source strings in it.
            compiler = os.environ.get("RUSTIFY_DOCS_CARGO", "mbx")
            run([compiler, "metadata", "--offline", "--format-version", "1", "--manifest-path", str(manifest)],
                cwd=repository, env=source_environment(repository), stdout=subprocess.PIPE)
    return {"candidate_sha":identity["candidate_sha"], "framework_revision":identity["framework_revision"],
            "distribution":identity["mode"], "manifest":str(manifest), "project":str(manifest.parent)}


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
    official = out / "site/Cargo.toml"
    prepare_project(cli, official)
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
