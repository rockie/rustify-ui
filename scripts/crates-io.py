#!/usr/bin/env python3.12
"""Package and publish the explicit Rustify crates.io inventory using Cargo."""

import argparse
from contextlib import contextmanager
from email.utils import parsedate_to_datetime
import hashlib
import json
import os
from pathlib import Path
import re
import shutil
import subprocess
import sys
import tarfile
import tempfile
import time
import tomllib
import urllib.error
import urllib.request


ROOT = Path(__file__).resolve().parents[1]
CRATES_IO = "registry+https://github.com/rust-lang/crates.io-index"
CLI = "rustify-docs-cli"
FRAMEWORKS = ("rustify-docs", "rustify-dom", "rustify-components")


class ReleaseError(Exception):
    pass


class Waiting(ReleaseError):
    """Persisted state needs index visibility or a later retry, not a new build."""


def save_receipt(directory, receipt):
    directory = Path(directory)
    temporary = directory / "receipt.json.tmp"
    temporary.write_text(json.dumps(receipt, indent=2, sort_keys=True) + "\n")
    temporary.replace(directory / "receipt.json")


def read_receipt(directory):
    receipt = json.loads((Path(directory) / "receipt.json").read_text())
    if receipt.get("schema") != 1:
        raise ReleaseError("unsupported release receipt schema")
    return receipt


def sha256(path):
    with Path(path).open("rb") as stream:
        return hashlib.file_digest(stream, "sha256").hexdigest()


def archive_info(archive, name, version):
    archive = Path(archive)
    if archive.stat().st_size > 10_000_000:
        raise ReleaseError(f"{archive.name} exceeds 10,000,000 bytes; inspect its file list")
    prefix = f"{name}-{version}/"
    with tarfile.open(archive) as bundle:
        for member in bundle.getmembers():
            relative = Path(member.name.removeprefix(prefix))
            if (not member.name.startswith(prefix) or relative.is_absolute() or ".." in relative.parts
                    or member.issym() or member.islnk()
                    or any(part in ("target", ".git", ".cargo", "credentials.toml", "makepad") or part.startswith(".env") for part in relative.parts)
                    or (name == CLI and relative.parts[:2] == ("assets", "fonts"))):
                raise ReleaseError(f"unsafe or excluded archive member: {member.name}")
        def read(filename):
            member = bundle.extractfile(prefix + filename)
            if member is None:
                raise ReleaseError(f"archive omitted {filename}: {archive.name}")
            return member.read()
        lock_bytes = read("Cargo.lock")
        manifest = tomllib.loads(read("Cargo.toml").decode())
        vcs = json.loads(read(".cargo_vcs_info.json"))
        read("LICENSE")
        if (manifest["package"]["name"], manifest["package"]["version"]) != (name, version):
            raise ReleaseError(f"archive package identity mismatch: {archive.name}")
        tables = [manifest, *manifest.get("target", {}).values()]
        for table in tables:
            for kind in ("dependencies", "build-dependencies"):
                for key, dependency in table.get(kind, {}).items():
                    dependency = {"version":dependency} if isinstance(dependency, str) else dependency
                    actual_name = dependency.get("package", key)
                    if (not dependency.get("version") or any(field in dependency for field in ("path", "git", "workspace", "registry"))
                            or dependency.get("registry-index", "https://github.com/rust-lang/crates.io-index") != "https://github.com/rust-lang/crates.io-index"
                            or actual_name.startswith("makepad") or actual_name in ("rustify-ui", "rustify-makepad")):
                        raise ReleaseError(f"normalized archive manifest has a private/path/non-registry dependency: {actual_name}")
        locked = tomllib.loads(lock_bytes.decode())["package"]
        roots = [p for p in locked if "source" not in p]
        if len(roots) != 1 or (roots[0]["name"], roots[0]["version"]) != (name, version):
            raise ReleaseError(f"archive lock must contain only its own non-registry root: {archive.name}")
        for package in locked:
            if package is roots[0]:
                continue
            if package.get("source") != CRATES_IO or not re.fullmatch(r"[0-9a-f]{64}", package.get("checksum", "")):
                raise ReleaseError(f"archive lock contains a non-registry or unchecked dependency: {package['name']}")
    return {"sha256":sha256(archive), "lock_sha256":hashlib.sha256(lock_bytes).hexdigest(),
            "bytes":archive.stat().st_size, "source_rev":vcs["git"]["sha1"], "lock":locked}


def validate_archives(directory, receipt):
    directory = Path(directory).resolve()
    for package in receipt["packages"]:
        path = (directory / package["archive"]).resolve(strict=True)
        if not path.is_relative_to(directory):
            raise ReleaseError("archive path escapes the release directory")
        actual = archive_info(path, package["name"], package["version"])
        if actual["sha256"] != package["sha256"]:
            raise ReleaseError(f"archive checksum drift: {package['name']}")
        if actual["lock_sha256"] != package["lock_sha256"]:
            raise ReleaseError(f"archive lock digest drift: {package['name']}")
        revision = receipt["cli_rev"] if package["name"] == CLI else receipt["library_rev"]
        if actual["source_rev"] != revision:
            raise ReleaseError(f"archive VCS identity mismatch: {package['name']}")


def compare_archives(expected, actual):
    def identity(packages):
        return {name:(p["sha256"], p["lock_sha256"]) for name,p in packages.items()}
    if identity(expected) != identity(actual):
        raise ReleaseError("archive or Cargo.lock drift under a different index view; keep R/C and diagnose the inputs")


def retry_time(value, now):
    if value is None:
        return None
    try:
        return now + max(0, int(value))
    except (ValueError, TypeError):
        try:
            return max(now, int(parsedate_to_datetime(value).timestamp()))
        except (ValueError, TypeError, OverflowError):
            return None


class Registry:
    def request(self, url):
        request = urllib.request.Request(url, headers={"User-Agent":"rustify-crates-io-release/1"})
        return urllib.request.urlopen(request, timeout=60)

    def lookup(self, name, version):
        # The API can acknowledge an upload before its sparse index is visible.
        try:
            with self.request(f"https://crates.io/api/v1/crates/{name}/{version}") as response:
                return json.load(response)["version"]["checksum"]
        except urllib.error.HTTPError as error:
            if error.code == 404:
                return None
            raise ReleaseError(f"registry lookup failed: HTTP {error.code}") from error

    def available(self, name, version, checksum):
        try:
            with self.request("https://index.crates.io/" + registry_key(name)) as response:
                entries = [json.loads(line) for line in response.read().splitlines()]
            matching = [entry for entry in entries if entry["vers"] == version]
            if not matching:
                return False
            if len(matching) != 1 or matching[0]["cksum"] != checksum:
                raise ReleaseError(f"public index checksum mismatch: {name}@{version}")
            with self.request(f"https://static.crates.io/crates/{name}/{name}-{version}.crate") as response:
                actual = hashlib.file_digest(response, "sha256").hexdigest()
            if actual != checksum:
                raise ReleaseError(f"public archive checksum mismatch: {name}@{version}")
            return True
        except urllib.error.HTTPError as error:
            if error.code == 404:
                return False
            raise ReleaseError(f"registry visibility failed: HTTP {error.code}") from error


def publish_packages(directory, receipt, group, registry, upload, now=None, retry_at=None):
    now = int(time.time()) if now is None else now
    validate_archives(directory, receipt)
    if group == "remaining" and receipt.get("verifications", {}).get("public_docs_install", {}).get("result") != "passed":
        raise ReleaseError("remaining group requires the real public Docs installation gate")
    for package in receipt["packages"]:
        if package["group"] != group:
            continue
        name, version, checksum = package["name"], package["version"], package["sha256"]

        def visible(existing):
            if existing is None:
                return False
            if existing != checksum:
                raise ReleaseError(f"published checksum differs for {name}@{version}; never overwrite this version")
            if not registry.available(name, version, checksum):
                package["state"] = "waiting-index"
                save_receipt(directory, receipt)
                raise Waiting(f"{name}@{version} was accepted; wait for its index and download before continuing")
            package["state"] = "published"
            package.pop("next_retry_at", None)
            save_receipt(directory, receipt)
            return True

        if visible(registry.lookup(name, version)):
            continue
        if package["state"] in ("waiting-index", "published"):
            raise Waiting(f"query {name}@{version} again before retrying an uncertain upload")
        if package["state"] == "waiting-rate-limit":
            deadline = retry_at if retry_at is not None else package.get("next_retry_at")
            if deadline is None or now < deadline:
                raise Waiting(f"{name}@{version} is rate limited; next_retry_at={deadline}; keep the same receipt")
        # A crash while uploading is treated like a timeout on resume.
        package["state"] = "waiting-index"
        save_receipt(directory, receipt)
        result = upload(package)
        if visible(registry.lookup(name, version)):
            continue
        if result["status"] == "rate-limit":
            package["state"] = "waiting-rate-limit"
            package["next_retry_at"] = retry_time(result.get("retry_after"), now)
            save_receipt(directory, receipt)
            raise Waiting(f"{name}@{version} HTTP 429; next_retry_at={package['next_retry_at']}")
        if result["status"] == "uploaded":
            if not registry.available(name, version, checksum):
                save_receipt(directory, receipt)
                raise Waiting(f"{name}@{version} upload succeeded; wait for index visibility")
            package["state"] = "published"
            save_receipt(directory, receipt)
        elif result["status"] == "timeout":
            save_receipt(directory, receipt)
            raise Waiting(f"upload of {name}@{version} timed out; check registry state before any retry")
        else:
            package["state"] = "pending"
            save_receipt(directory, receipt)
            raise ReleaseError(result.get("message", f"upload failed: {name}@{version}"))



def registry_key(name):
    if len(name) < 3:
        return f"{len(name)}/{name}"
    if len(name) == 3:
        return f"3/{name[0]}/{name}"
    return f"{name[:2]}/{name[2:4]}/{name}"


def stage_registry(lockfile, archives, generated_index, output, cargo_home=None):
    """Copy locked registry inputs and Cargo's package index into a local registry."""
    cargo_home = Path(cargo_home or os.environ.get("CARGO_HOME", Path.home() / ".cargo"))
    output = Path(output)
    if output.exists():
        raise ReleaseError(f"local registry output already exists: {output}")
    locks = [lockfile] if isinstance(lockfile, (str, Path)) else lockfile
    packages = []
    for lock in locks:
        with Path(lock).open("rb") as stream:
            packages.extend(tomllib.load(stream)["package"])
    local = {}
    for archive in archives:
        name, version = Path(archive).name.removesuffix(".crate").rsplit("-", 1)
        local[name, version] = sha256(archive)
    cached_indexes = sorted((cargo_home / "registry/index").glob("index.crates.io-*"))
    cached_archives = sorted((cargo_home / "registry/cache").glob("index.crates.io-*"))
    entries = {}
    inputs = {}
    for package in packages:
        if not package.get("source", "").startswith("registry+"):
            continue
        name, version = package["name"], package["version"]
        if (name, version) in local:
            if package["checksum"] != local[name, version]:
                raise ReleaseError(f"locked local archive checksum mismatch: {name}@{version}")
            continue
        key = registry_key(name)
        matches = []
        for index in cached_indexes:
            cache = index / ".cache" / key
            if cache.is_file():
                for item in cache.read_bytes().split(b"\0"):
                    if item.startswith(b"{"):
                        entry = json.loads(item)
                        if entry["vers"] == version:
                            matches.append(entry)
        if not matches or any(e["cksum"] != package["checksum"] for e in matches):
            raise ReleaseError(f"missing or inconsistent cached index for {name} {version}; run mbx fetch --locked")
        filename = f"{name}-{version}.crate"
        candidates = [cache / filename for cache in cached_archives if (cache / filename).is_file()]
        if not candidates:
            raise ReleaseError(f"missing cached {filename}; run mbx fetch --locked")
        archive = candidates[0]
        if sha256(archive) != package["checksum"]:
            raise ReleaseError(f"cached archive checksum mismatch: {filename}")
        entries.setdefault(key, {})[version] = matches[0]
        inputs[archive.name] = archive
    for archive in archives:
        archive = Path(archive)
        # Cargo emits this entry while packaging the actual crate, including aliases and features2.
        name = archive.name.rsplit("-", 1)[0]
        index_file = Path(generated_index) / registry_key(name)
        if not index_file.is_file():
            raise ReleaseError(f"Cargo package index is missing {name}")
        for line in index_file.read_text().splitlines():
            entry = json.loads(line)
            if archive.name == f"{entry['name']}-{entry['vers']}.crate":
                if sha256(archive) != entry["cksum"]:
                    raise ReleaseError(f"package index checksum mismatch: {archive.name}")
                entries.setdefault(registry_key(name), {})[entry["vers"]] = entry
                break
        else:
            raise ReleaseError(f"Cargo package index has no matching entry for {archive.name}")
        inputs[archive.name] = archive
    (output / "index").mkdir(parents=True)
    for key, versions in entries.items():
        path = output / "index" / key
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text("".join(json.dumps(entry) + "\n" for entry in versions.values()))
    for archive in inputs.values():
        shutil.copyfile(archive, output / archive.name)
    return output


def inventory(root):
    with (root / "release/crates-io.toml").open("rb") as stream:
        packages = tomllib.load(stream)["packages"]
    seen = set()
    for package in packages:
        name = package["name"]
        if name in seen or not set(package["dependencies"]) <= seen:
            raise ReleaseError(f"inventory is not topological at {name}")
        path = (root / package["path"]).resolve()
        if not path.is_relative_to(root.resolve()) or package["group"] not in ("docs", "remaining"):
            raise ReleaseError(f"invalid inventory entry: {name}")
        seen.add(name)
    return packages


def cargo_metadata(root):
    cargo = Cargo(root, root / "target")
    return json.loads(cargo("metadata", "--locked", "--all-features", "--format-version", "1"))


def inspect(root=ROOT, metadata=None):
    root = Path(root).resolve()
    entries = inventory(root)
    metadata = metadata if metadata is not None else cargo_metadata(root)
    by_id = {p["id"]: p for p in metadata["packages"]}
    by_name = {}
    for entry in entries:
        manifest = root / entry["path"] / "Cargo.toml"
        matches = [p for p in by_id.values() if Path(p["manifest_path"]).resolve() == manifest]
        if len(matches) != 1 or matches[0]["name"] != entry["name"]:
            raise ReleaseError(f"inventory manifest/name mismatch: {entry['name']}")
        by_name[entry["name"]] = matches[0]
    allowed_ids = {p["id"] for p in by_name.values()}
    for identifier in metadata["workspace_members"]:
        package = by_id[identifier]
        if identifier not in allowed_ids and package["publish"] != []:
            raise ReleaseError(f"private workspace package permits publishing: {package['name']}")
    result = []
    for entry in entries:
        name = entry["name"]
        package = by_name[name]
        if package["publish"] != ["crates-io"]:
            raise ReleaseError(f"{name}: publish must explicitly allow only crates-io")
        for field in ("description", "license", "readme", "repository", "documentation"):
            if not package.get(field):
                raise ReleaseError(f"{name}: missing package {field}")
        directory = Path(package["manifest_path"]).parent
        for filename in (package["readme"], "LICENSE"):
            if not (directory / filename).is_file():
                raise ReleaseError(f"{name}: missing {filename}")
        with (directory / "Cargo.toml").open("rb") as stream:
            includes = tomllib.load(stream)["package"].get("include", [])
        if not includes or any(
            pattern in ("*", "**", "**/*") or ".." in Path(pattern).parts
            or any(part in pattern.lower() for part in ("credential", "secret", ".env", ".cargo", "target/"))
            for pattern in includes
        ):
            raise ReleaseError(f"{name}: missing or unsafe package content allowlist")
        if any("lib" in t["kind"] or "proc-macro" in t["kind"] or "rlib" in t["kind"] for t in package["targets"]):
            docs = package.get("metadata", {}).get("docs", {}).get("rs", {})
            if not docs.get("default-target") or docs.get("targets") != [] or docs.get("all-features"):
                raise ReleaseError(f"{name}: docs.rs must select a single target and rendering mode")
        internal = set()
        for dependency in package["dependencies"]:
            if dependency["kind"] == "dev":
                continue
            dep_name = dependency["name"]
            if dep_name.startswith("makepad") or dep_name in ("rustify-ui", "rustify-makepad"):
                raise ReleaseError(f"{name}: GPU dependency outside the publication inventory: {dep_name}")
            if dependency.get("path"):
                target = by_name.get(dep_name)
                if target is None:
                    raise ReleaseError(f"{name}: path dependency outside publication inventory: {dep_name}")
                if dependency["req"] != "=" + target["version"]:
                    raise ReleaseError(f"{name}: {dep_name} needs its exact version =" + target["version"])
                if Path(dependency["path"]).resolve() != Path(target["manifest_path"]).parent.resolve():
                    raise ReleaseError(f"{name}: wrong path for {dep_name}")
                internal.add(dep_name)
        if internal != set(entry["dependencies"]):
            raise ReleaseError(f"{name}: inventory dependencies differ from manifest: {sorted(internal)}")
        result.append({**entry, "version": package["version"]})

    nodes = {n["id"]: n for n in metadata["resolve"]["nodes"]}
    pending = list(allowed_ids)
    visited = set()
    while pending:
        identifier = pending.pop()
        if identifier in visited:
            continue
        visited.add(identifier)
        package = by_id[identifier]
        if identifier not in allowed_ids and not (package["source"] or "").startswith("registry+"):
            raise ReleaseError(f"publication closure contains a path/Git override: {package['name']}")
        for dependency in nodes[identifier]["deps"]:
            if any(k["kind"] != "dev" for k in dependency["dep_kinds"]):
                pending.append(dependency["pkg"])
    return {"packages": result, "registry_dependencies": len(visited - allowed_ids)}


def run(command, cwd=ROOT, env=None, timeout=None):
    result = subprocess.run([str(arg) for arg in command], cwd=cwd, env=env, timeout=timeout,
                            text=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    if result.returncode:
        raise ReleaseError(f"{command[0]} {command[1]} failed:\n{(result.stdout + result.stderr)[-12000:]}")
    return result.stdout


class Cargo:
    def __init__(self, root, target, cargo_home=None, config=None):
        self.root = Path(root)
        with (self.root / "mise.toml").open("rb") as stream:
            self.toolchain = tomllib.load(stream)["tools"]["rust"]["version"]
        self.sysroot = Path(run(["rustc", "--print", "sysroot"], cwd=self.root).strip())
        self.env = {**os.environ, "RUSTC":str(self.sysroot / "bin/rustc"),
                    "RUSTDOC":str(self.sysroot / "bin/rustdoc"), "RUSTUP_TOOLCHAIN":self.toolchain,
                    "PATH":str(self.sysroot / "bin") + os.pathsep + os.environ.get("PATH", ""),
                    "CARGO_TARGET_DIR":str(Path(target).resolve()), "PYTHONDONTWRITEBYTECODE":"1"}
        actual = run([self.env["RUSTC"], "--version"], cwd=self.root).strip()
        if not actual.startswith(f"rustc {self.toolchain} "):
            raise ReleaseError(f"pinned compiler mismatch: expected {self.toolchain}, found {actual}")
        if cargo_home is not None:
            self.env["CARGO_HOME"] = str(Path(cargo_home).resolve())
        self.config = config
        self.version = run([self.sysroot / "bin/cargo", "--version"], cwd=self.root).strip()

    def command(self, *args):
        # PATH selects this sysroot's real Cargo. +toolchain belongs to the
        # rustup proxy and is rejected when mbx starts Cargo directly.
        return ["mbx", *args, *(["--config", str(self.config)] if self.config else [])]

    def __call__(self, *args, cwd=None, timeout=None):
        return run(self.command(*args), cwd=cwd or self.root, env=self.env, timeout=timeout)


@contextmanager
def checkout(revision, repository=ROOT):
    if not re.fullmatch(r"[0-9a-f]{40}", revision):
        raise ReleaseError("revision must be a complete lowercase commit SHA")
    if run(["git", "rev-parse", f"{revision}^{{commit}}"], cwd=repository).strip() != revision:
        raise ReleaseError("revision did not resolve to the requested commit")
    with tempfile.TemporaryDirectory(prefix="rustify-release-worktree-") as scratch:
        worktree = Path(scratch) / "source"
        run(["git", "worktree", "add", "--detach", worktree, revision], cwd=repository)
        try:
            if run(["git", "status", "--porcelain"], cwd=worktree).strip():
                raise ReleaseError("release worktree must be clean")
            yield worktree
        finally:
            # Only this tool-created worktree is removed; never the working checkout.
            run(["git", "worktree", "remove", "--force", worktree], cwd=repository)


def release_directory(directory):
    directory = Path(directory).absolute()
    if directory.is_symlink() or directory.resolve().is_relative_to(ROOT.resolve()):
        raise ReleaseError("release directory must be persistent and outside the repository")
    return directory


def registry_config(registry, path):
    Path(path).parent.mkdir(parents=True, exist_ok=True)
    Path(path).write_text('[source.crates-io]\nreplace-with = "rustify-release"\n[source.rustify-release]\nlocal-registry = '
                          + json.dumps(str(Path(registry).resolve())) + "\n")
    return Path(path)


def locked_registry_packages(lockfiles, archives=()):
    packages = {}
    locks = []
    for path in lockfiles:
        locks.append(tomllib.loads(Path(path).read_text())["package"])
    for package, archive in archives:
        locks.append(archive_info(archive, package["name"], package["version"])["lock"])
    for locked in locks:
        for package in locked:
            if not package.get("source"):
                continue
            key = package["name"], package["version"]
            if package["source"] != CRATES_IO or not re.fullmatch(r"[0-9a-f]{64}", package.get("checksum", "")):
                raise ReleaseError(f"non-canonical locked registry input: {key}")
            entry = {field:package[field] for field in ("name", "version", "checksum")}
            if key in packages and packages[key] != entry:
                raise ReleaseError(f"inconsistent locked checksum: {key}")
            packages[key] = entry
    return sorted(packages.values(), key=lambda p:(p["name"], p["version"]))


def lock_inputs(root, output, packages):
    directory = output / "locks"
    directory.mkdir(exist_ok=True)
    paths = [root / "Cargo.lock", root / "site/Cargo.lock", root / "makepad/Cargo.lock",
             root / "makepad/platform/script/Cargo.lock"]
    paths.extend(sorted((root / "tests/docs-ssg/fixtures").rglob("Cargo.lock")))
    paths.extend(sorted((root / "tests/dom-only").rglob("Cargo.lock")))
    # Only committed author fixtures are inputs, never an ignored dev/cache lock.
    tracked = set(run(["git", "ls-files"], cwd=root).splitlines())
    copied = []
    for index, path in enumerate(path for path in paths if path.relative_to(root).as_posix() in tracked):
        target = directory / f"input-{index}.Cargo.lock"
        shutil.copyfile(path, target)
        copied.append(target)
    for package in packages:
        with tarfile.open(output / package["archive"]) as bundle:
            data = bundle.extractfile(f"{package['name']}-{package['version']}/Cargo.lock").read()
        target = directory / f"{package['name']}.Cargo.lock"
        target.write_bytes(data)
        copied.append(target)
    return copied


def collect_archives(root, entries, cargo, output, revision):
    flags = [flag for entry in entries for flag in ("-p", entry["name"])]
    cargo("package", "--locked", "--no-verify", *flags)
    result = []
    for entry in entries:
        filename = f"{entry['name']}-{entry['version']}.crate"
        source = Path(cargo.env["CARGO_TARGET_DIR"]) / "package" / filename
        actual = archive_info(source, entry["name"], entry["version"])
        if actual["source_rev"] != revision:
            raise ReleaseError(f"Cargo packaged the wrong source commit: {filename}")
        target = output / "crates" / filename
        target.parent.mkdir(exist_ok=True)
        shutil.copyfile(source, target)
        result.append({**entry, **{k:v for k,v in actual.items() if k != "lock"},
                       "archive":target.relative_to(output).as_posix(), "state":"pending"})
    return result


def package_libraries(revision, output):
    output = release_directory(output)
    if output.exists():
        raise ReleaseError("library release output already exists; keep or explicitly choose another directory")
    output.mkdir(parents=True)
    with checkout(revision) as root:
        cargo = Cargo(root, output / "work/libraries")
        metadata = json.loads(cargo("metadata", "--locked", "--all-features", "--format-version", "1"))
        entries = inspect(root, metadata)["packages"]
        libraries = [entry for entry in entries if entry["name"] != CLI]
        packages = collect_archives(root, libraries, cargo, output, revision)
        locks = lock_inputs(root, output, packages)
        stage_registry(locks, [output / p["archive"] for p in packages],
                       Path(cargo.env["CARGO_TARGET_DIR"]) / "package/tmp-registry/index", output / "registry")
        own = {p["name"] for p in packages}
        third_party = [p for p in locked_registry_packages(locks) if p["name"] not in own]
        receipt = {"schema":1, "cargo_version":cargo.version, "rust_toolchain":cargo.toolchain,
                   "library_rev":revision, "cli_rev":None, "packages":packages,
                   "inventory":entries, "registry_packages":third_party,
                   "template_lock_sha256":None, "verifications":{}}
        save_receipt(output, receipt)
    return receipt


def library_paths(receipt):
    return [p["path"] for p in receipt["inventory"] if p["name"] != CLI] + ["Cargo.toml", "Cargo.lock", "mise.toml", "release/crates-io.toml"]


def ensure_libraries_unchanged(receipt, revision=None, root=ROOT):
    command = ["git", "diff", "--name-only", receipt["library_rev"]]
    if revision is not None:
        command.append(revision)
    changed = run([*command, "--", *library_paths(receipt)], cwd=root).strip()
    if changed:
        raise ReleaseError(f"library inputs changed after R; refreeze the libraries before CLI: {changed}")


def freeze_cli(libraries, revision):
    libraries = release_directory(libraries)
    receipt = read_receipt(libraries)
    validate_archives(libraries, receipt)
    if revision != receipt["library_rev"] or receipt["cli_rev"] is not None:
        raise ReleaseError("freeze-cli needs the original, library-only R receipt")
    ensure_libraries_unchanged(receipt)
    record = {"schema":1, "registry":"crates-io", "library_rev":revision, "wasm_bindgen":"0.2.128",
              "packages":{p["name"]:p["version"] for p in receipt["packages"] if p["name"] in FRAMEWORKS}}
    with tempfile.TemporaryDirectory(prefix="rustify-freeze-cli-") as scratch:
        site = Path(scratch) / "docs-site"
        site.mkdir()
        template = (ROOT / "crates/rustify-docs-cli/templates/components/Cargo.toml.in").read_text()
        (site / "Cargo.toml").write_text(template.replace("{{package}}", "docs-site"))
        (site / "src/bin").mkdir(parents=True)
        (site / "src/lib.rs").write_text("")
        (site / "src/bin/docs-render.rs").write_text("fn main() {}\n")
        config = registry_config(libraries / "registry", Path(scratch) / "config.toml")
        cargo = Cargo(ROOT, libraries / "work/freeze", config=config)
        cargo("generate-lockfile", "--offline", "--manifest-path", site / "Cargo.toml")
        lock = (site / "Cargo.lock").read_bytes()
        document = tomllib.loads(lock.decode())
        for name, version in record["packages"].items():
            matches = [p for p in document["package"] if p["name"] == name]
            archived = next(p for p in receipt["packages"] if p["name"] == name)
            if len(matches) != 1 or matches[0]["version"] != version or matches[0].get("source") != CRATES_IO or matches[0].get("checksum") != archived["sha256"]:
                raise ReleaseError(f"resolved template lock does not match the real archive: {name}")
    assets = ROOT / "crates/rustify-docs-cli/assets"
    (assets / "components-Cargo.lock").write_bytes(lock)
    (assets / "framework-release.json").write_text(json.dumps(record, indent=2) + "\n")
    return {"library_rev":revision, "template_lock_sha256":hashlib.sha256(lock).hexdigest()}


def package_cli(revision, libraries, output):
    libraries, output = release_directory(libraries), release_directory(output)
    if libraries != output:
        if output.exists():
            raise ReleaseError("CLI output already exists")
        shutil.copytree(libraries, output, ignore=shutil.ignore_patterns("work"))
    receipt = read_receipt(output)
    validate_archives(output, receipt)
    if receipt["cli_rev"] is not None:
        raise ReleaseError("this receipt already contains CLI C; choose a fresh library copy for a new C")
    ensure_libraries_unchanged(receipt, revision)
    with checkout(revision) as root:
        config = registry_config(output / "registry", output / "work/package-config.toml")
        cargo = Cargo(root, output / "work/cli", config=config)
        if cargo.version != receipt["cargo_version"]:
            raise ReleaseError("Cargo version differs from the frozen library receipt")
        assets = root / "crates/rustify-docs-cli/assets"
        record = json.loads((assets / "framework-release.json").read_text())
        expected = {p["name"]:p["version"] for p in receipt["packages"] if p["name"] in FRAMEWORKS}
        if record.get("library_rev") != receipt["library_rev"] or record.get("packages") != expected:
            raise ReleaseError("CLI C does not contain the frozen R/version pair")
        entry = next(p for p in receipt["inventory"] if p["name"] == CLI)
        packaged = collect_archives(root, [entry], cargo, output, revision)[0]
        actual = archive_info(output / packaged["archive"], entry["name"], entry["version"])
        known = {(p["name"], p["version"]):p["checksum"] for p in receipt["registry_packages"]}
        known.update({(p["name"], p["version"]):p["sha256"] for p in receipt["packages"]})
        for package in actual["lock"]:
            if package.get("source") and known.get((package["name"], package["version"])) != package["checksum"]:
                raise ReleaseError(f"CLI archive selected a new registry input after R: {package['name']}")
        receipt["packages"].append(packaged)
        order = {p["name"]:i for i,p in enumerate(receipt["inventory"])}
        receipt["packages"].sort(key=lambda p:order[p["name"]])
        receipt["cli_rev"] = revision
        receipt["template_lock_sha256"] = sha256(assets / "components-Cargo.lock")
        save_receipt(output, receipt)
    return receipt


def validate_release_sources(receipt):
    if not receipt.get("cli_rev") or len(receipt["packages"]) != len(receipt["inventory"]):
        raise ReleaseError("release is incomplete: freeze and package CLI C first")
    with checkout(receipt["library_rev"]) as root:
        actual = inspect(root)["packages"]
        if receipt["inventory"] != actual:
            raise ReleaseError("receipt inventory differs from frozen R")
        for expected, package in zip(actual, receipt["packages"]):
            if any(package.get(key) != value for key,value in expected.items()):
                raise ReleaseError(f"receipt topology/package identity mismatch: {expected['name']}")
    ensure_libraries_unchanged(receipt, receipt["cli_rev"])


def alternate_registry(registry, output):
    """Expose one extra real compatible version in a second index view."""
    shutil.copytree(registry, output)
    key = registry_key("bitflags")
    current = [json.loads(line) for line in (output / "index" / key).read_text().splitlines()]
    versions = {entry["vers"] for entry in current}
    with Registry().request("https://index.crates.io/" + key) as response:
        candidates = [json.loads(line) for line in response.read().splitlines()]
    additional = [e for e in candidates if re.fullmatch(r"2\.\d+\.\d+", e["vers"]) and e["vers"] not in versions and not e.get("yanked")]
    if not additional:
        raise ReleaseError("no additional compatible version for the index stability probe")
    entry = additional[-1]
    archive = output / f"bitflags-{entry['vers']}.crate"
    with Registry().request(f"https://static.crates.io/crates/bitflags/{archive.name}") as response:
        archive.write_bytes(response.read())
    if sha256(archive) != entry["cksum"]:
        raise ReleaseError("alternate index input checksum mismatch")
    (output / "index" / key).write_text("".join(json.dumps(e) + "\n" for e in [*current, entry]))
    return {"name":entry["name"], "version":entry["vers"], "checksum":entry["cksum"]}


def verify_index_stability(directory, receipt):
    work = directory / "work"
    alternate = Path(tempfile.mkdtemp(prefix="index-view-", dir=work)) / "registry"
    extra = alternate_registry(directory / "registry", alternate)
    expected = {p["name"]:p for p in receipt["packages"] if p["name"] != CLI}
    with checkout(receipt["library_rev"]) as root:
        for index, registry in enumerate((directory / "registry", alternate)):
            scratch = Path(tempfile.mkdtemp(prefix=f"repackage-{index}-", dir=work))
            config = registry_config(registry, scratch / "config.toml")
            cargo = Cargo(root, scratch / "target", config=config)
            if cargo.version != receipt["cargo_version"]:
                raise ReleaseError("index stability check must use the frozen Cargo version")
            actual = collect_archives(root, list(expected.values()), cargo, scratch, receipt["library_rev"])
            compare_archives(expected, {p["name"]:p for p in actual})
    return {"result":"passed", "extra_visible_registry_package":extra, "packages":len(expected)}


def verify(directory, public=False, public_docs=False):
    directory = release_directory(directory)
    receipt = read_receipt(directory)
    validate_archives(directory, receipt)
    validate_release_sources(receipt)
    work = directory / "work"
    work.mkdir(exist_ok=True)
    scratch = Path(tempfile.mkdtemp(prefix="public-" if public or public_docs else "verify-", dir=work))
    cargo_home = scratch / "cargo-home"
    cargo_home.mkdir()
    staging = not public and not public_docs
    if staging:
        registry_config(directory / "registry", cargo_home / "config.toml")
    cargo = Cargo(ROOT, scratch / "target", cargo_home=cargo_home)
    cargo.root = scratch
    if not staging:
        if any(key.startswith("CARGO_SOURCE_") or key in ("CARGO_REGISTRY_INDEX", "CARGO_REGISTRIES_CRATES_IO_INDEX") for key in cargo.env):
            raise ReleaseError("public installation cannot use source/index overrides")
        cargo.env.pop("CARGO_NET_OFFLINE", None)
    if cargo.version != receipt["cargo_version"]:
        raise ReleaseError("verification Cargo version differs from the frozen receipt")
    cargo.env["RUSTIFY_DOCS_CARGO"] = "mbx"
    if staging:
        cargo.env["CARGO_NET_OFFLINE"] = "true"
    packages = scratch / "packages"
    packages.mkdir()
    checked = []
    if staging:
        for package in receipt["packages"]:
            with tarfile.open(directory / package["archive"]) as bundle:
                bundle.extractall(packages, filter="data")
            root = packages / f"{package['name']}-{package['version']}"
            if package["name"] == CLI:
                cli_root = root
                continue
            manifest = tomllib.loads((root / "Cargo.toml").read_text())
            docs = manifest["package"]["metadata"]["docs"]["rs"]
            flags = ["--no-default-features"] if docs.get("no-default-features") else []
            if docs.get("features"):
                flags.extend(["--features", ",".join(docs["features"])])
            for command in ("check", "doc"):
                args = [command, "--locked", "--offline", "--manifest-path", root / "Cargo.toml", "--target", docs["default-target"], *flags]
                if command == "doc":
                    args.append("--no-deps")
                cargo(*args)
            checked.append(package["name"])
        cargo("install", "--locked", "--offline", "--path", cli_root, "--root", scratch / "installed", timeout=3600)
    else:
        entry = next(p for p in receipt["packages"] if p["name"] == CLI)
        cargo("install", CLI, "--version", entry["version"], "--locked", "--root", scratch / "installed", timeout=3600)
    binary = scratch / "installed/bin/rustify-docs"
    info = run([binary, "--build-info"], cwd=scratch, env=cargo.env)
    if "\npanic: unwind\n" not in info or "\ndistribution: registry\n" not in info or f"\nsource: {receipt['cli_rev']}\n" not in info:
        raise ReleaseError("archive installation did not report registry C with unwind")
    records = re.findall(r"^framework: (.+)$", info, re.MULTILINE)
    if len(records) != 1:
        raise ReleaseError("archive CLI omitted the frozen framework JSON")
    record = json.loads(records[0])
    versions = {p["name"]:p["version"] for p in receipt["packages"] if p["name"] in FRAMEWORKS}
    if record.get("library_rev") != receipt["library_rev"] or record.get("packages") != versions:
        raise ReleaseError("installed CLI framework identity differs from receipt")
    if run([binary, "__panic-probe"], cwd=scratch, env=cargo.env).strip() != "recovered":
        raise ReleaseError("registry release installation cannot recover a panic")
    author = scratch / "author"
    author.mkdir()
    for template in ("content", "components"):
        site = author / template
        run([binary, "new", site, "--template", template], cwd=author, env=cargo.env)
        if template == "components":
            lock = site / "Cargo.lock"
            before = sha256(lock)
            if staging:
                embedded = (cli_root / "assets/components-Cargo.lock").read_text()
                expected = embedded.replace('name = "docs-site"\nversion = "0.1.0"', 'name = "components"\nversion = "0.1.0"', 1)
                if lock.read_text() != expected:
                    raise ReleaseError("new components changed more than the embedded lock root name")
            for features, target in (("ssr", None), ("hydrate", "wasm32-unknown-unknown")):
                args = ["metadata", "--locked", "--manifest-path", site / "Cargo.toml", "--no-default-features", "--features", features, "--format-version", "1"]
                if target:
                    args.extend(["--filter-platform", target])
                metadata = json.loads(cargo(*args))
                for name, version in versions.items():
                    found = [p for p in metadata["packages"] if p["name"] == name]
                    if len(found) != 1 or found[0]["version"] != version or found[0]["source"] != CRATES_IO:
                        raise ReleaseError(f"component {features} graph is not the registry pair: {name}")
            if sha256(lock) != before:
                raise ReleaseError("component metadata changed the supplied lock")
        for command in ("check", "build"):
            run([binary, command], cwd=site, env=cargo.env, timeout=3600)
    result = {"result":"passed", "cli_build_info":info,
              "listing":cargo("install", "--list", "--root", scratch / "installed"),
              "cargo_version":cargo.version, "checked_libraries":checked, "binary":str(binary),
              "cargo_home":str(cargo_home), "author_projects":str(author), "completed_at":int(time.time())}
    key = "public_docs_install" if public_docs else "public_install" if public else "archive_install"
    if staging:
        result["index_stability"] = verify_index_stability(directory, receipt)
    evidence = scratch / "verification.json"
    evidence.write_text(json.dumps(result, indent=2) + "\n")
    receipt["verifications"][key] = {"result":"passed", "path":str(evidence), "sha256":sha256(evidence)}
    save_receipt(directory, receipt)
    return result


def reject_staging_sources(root, cargo):
    cargo_home = Path(cargo.env.get("CARGO_HOME", Path.home() / ".cargo"))
    paths = [cargo_home / "config.toml", cargo_home / "config"]
    for parent in (root, *root.parents):
        paths.extend((parent / ".cargo/config.toml", parent / ".cargo/config"))
    for path in paths:
        if path.is_file():
            with path.open("rb") as stream:
                if any("replace-with" in value for value in tomllib.load(stream).get("source", {}).values()):
                    raise ReleaseError("production publish cannot use Cargo source replacement")
    if any(key.startswith("CARGO_SOURCE_") for key in cargo.env):
        raise ReleaseError("production publish cannot use source replacement environment overrides")


def publish(directory, group, retry_at=None):
    directory = release_directory(directory)
    receipt = read_receipt(directory)
    validate_archives(directory, receipt)
    validate_release_sources(receipt)
    if receipt.get("verifications", {}).get("archive_install", {}).get("result") != "passed":
        raise ReleaseError("publish requires successful frozen archive verification")

    def upload(package):
        revision = receipt["cli_rev"] if package["name"] == CLI else receipt["library_rev"]
        with checkout(revision) as root:
            cargo = Cargo(root, directory / "work/publish" / package["name"])
            reject_staging_sources(root, cargo)
            if cargo.version != receipt["cargo_version"]:
                raise ReleaseError("production Cargo version differs from frozen receipt")
            args = ["publish", "--registry", "crates-io", "--locked", "-p", package["name"]]
            try:
                cargo(*args, "--dry-run", timeout=3600)
                archive = Path(cargo.env["CARGO_TARGET_DIR"]) / "package/tmp-registry" / Path(package["archive"]).name
                actual = archive_info(archive, package["name"], package["version"])
                compare_archives({package["name"]:package}, {package["name"]:actual})
            except ReleaseError as error:
                return {"status":"error", "message":str(error)}
            try:
                result = subprocess.run(cargo.command(*args), cwd=root, env=cargo.env, timeout=3600,
                                        text=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
            except subprocess.TimeoutExpired:
                return {"status":"timeout"}
            if result.returncode == 0:
                return {"status":"uploaded"}
            message = result.stdout + result.stderr
            if re.search(r"\b429\b|too many requests|rate limit", message, re.IGNORECASE):
                retry = re.search(r"retry-after\s*:\s*([^\r\n]+)", message, re.IGNORECASE)
                return {"status":"rate-limit", "retry_after":retry[1].strip() if retry else None}
            return {"status":"error", "message":message[-8000:]}

    publish_packages(directory, receipt, group, Registry(), upload, retry_at=retry_at)
    return receipt


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    commands = parser.add_subparsers(dest="command", required=True)
    commands.add_parser("inspect")
    libraries = commands.add_parser("package-libraries")
    libraries.add_argument("--rev", required=True)
    libraries.add_argument("--out", required=True, type=Path)
    freeze = commands.add_parser("freeze-cli")
    freeze.add_argument("--libraries", required=True, type=Path)
    freeze.add_argument("--library-rev", required=True)
    cli = commands.add_parser("package-cli")
    cli.add_argument("--rev", required=True)
    cli.add_argument("--libraries", required=True, type=Path)
    cli.add_argument("--out", required=True, type=Path)
    verification = commands.add_parser("verify")
    verification.add_argument("--release", required=True, type=Path)
    mode = verification.add_mutually_exclusive_group()
    mode.add_argument("--public-docs", action="store_true")
    mode.add_argument("--public", action="store_true")
    publishing = commands.add_parser("publish")
    publishing.add_argument("--release", required=True, type=Path)
    publishing.add_argument("--group", required=True, choices=("docs", "remaining"))
    publishing.add_argument("--retry-at", type=int, help="explicit Unix retry time when the server supplied none")
    args = parser.parse_args()
    if args.command == "inspect":
        result = inspect()
    elif args.command == "package-libraries":
        result = package_libraries(args.rev, args.out)
    elif args.command == "freeze-cli":
        result = freeze_cli(args.libraries, args.library_rev)
    elif args.command == "package-cli":
        result = package_cli(args.rev, args.libraries, args.out)
    elif args.command == "verify":
        result = verify(args.release, args.public, args.public_docs)
    else:
        result = publish(args.release, args.group, args.retry_at)
    print(json.dumps(result, indent=2))


if __name__ == "__main__":
    try:
        main()
    except Waiting as error:
        print(f"crates-io: waiting: {error}", file=sys.stderr)
        sys.exit(75)
    except (ReleaseError, OSError, ValueError, KeyError, subprocess.TimeoutExpired, tarfile.TarError) as error:
        print(f"crates-io: {error}", file=sys.stderr)
        sys.exit(1)
