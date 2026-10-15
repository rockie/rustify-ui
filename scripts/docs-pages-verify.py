#!/usr/bin/env python3
"""Compare the official Pages files with one successful workflow's artifact.

Read-only: downloads Actions evidence and public HTTP; never enables or deploys
Pages. Native-browser, device, public Git installation and rollback checks are
separate release gates.
"""

from __future__ import annotations

import argparse
import base64
import hashlib
from html.parser import HTMLParser
import io
import json
from pathlib import Path, PurePosixPath
import re
import subprocess
import tarfile
from urllib.parse import quote, unquote, urljoin, urlsplit
from urllib.request import Request, urlopen
import zipfile

REPOSITORY = "rockie/rustify-ui"
PUBLIC_URL = "https://rockie.github.io/rustify-ui/"
MANIFEST = "rustify-docs-manifest.json"
META_CSP = "default-src 'none'; script-src 'self' 'wasm-unsafe-eval'; style-src 'self'; img-src 'self' data:; font-src 'self'; connect-src 'self'; media-src 'self'; worker-src 'self'; object-src 'none'; base-uri 'none'; form-action 'none'"


def gh(endpoint, binary=False):
    result = subprocess.run(["gh", "api", endpoint], stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    if result.returncode:
        raise ValueError(f"gh api {endpoint}: {result.stderr.decode(errors='replace')[-2000:]}")
    return result.stdout if binary else json.loads(result.stdout)


def zip_member(archive, name):
    matches = [entry for entry in archive.namelist() if PurePosixPath(entry).name == name]
    if len(matches) != 1:
        raise ValueError(f"artifact needs exactly one {name}")
    return archive.read(matches[0])


def check_identity(candidate, framework, identity, receipt):
    if not re.fullmatch(r"[0-9a-f]{40}", framework) or identity["framework_revision"] != framework or receipt["framework_revision"] != framework:
        raise ValueError("workflow identity does not match the framework R in source C")
    if identity["candidate_sha"] != candidate or receipt["candidate_sha"] != candidate or f"\nsource: {candidate}\n" not in receipt["cli_build_info"] or "\npanic: unwind\n" not in receipt["cli_build_info"]:
        raise ValueError("prepare receipt does not identify the installed CLI C/profile")
    if set(identity["projection"]) != {"site/Cargo.toml", "site/Cargo.lock"} or identity["projection"] != receipt["projection"]:
        raise ValueError("unexpected official-source projection")


def workflow_artifact(candidate, run_id=None):
    runs = [gh(f"repos/{REPOSITORY}/actions/runs/{run_id}")] if run_id else gh(
        f"repos/{REPOSITORY}/actions/workflows/docs-pages.yml/runs?event=workflow_dispatch&status=completed&per_page=30"
    )["workflow_runs"]
    for run in runs:
        if run["status"] != "completed" or run["conclusion"] != "success":
            continue
        artifacts = gh(f"repos/{REPOSITORY}/actions/runs/{run['id']}/artifacts")["artifacts"]
        evidence = next((item for item in artifacts if item["name"] == f"docs-pages-evidence-{run['id']}" and not item["expired"]), None)
        pages = next((item for item in artifacts if item["name"] == "github-pages" and not item["expired"]), None)
        if not evidence or not pages:
            continue
        with zipfile.ZipFile(io.BytesIO(gh(f"repos/{REPOSITORY}/actions/artifacts/{evidence['id']}/zip", True))) as archive:
            identity = json.loads(zip_member(archive, "identity.json"))
            if identity.get("candidate_sha") != candidate:
                continue
            receipt = json.loads(zip_member(archive, "pages-prepare.json"))
            manifest = zip_member(archive, MANIFEST)
        raw = gh(f"repos/{REPOSITORY}/contents/crates/rustify-docs-cli/assets/framework-revision.txt?ref={candidate}")
        framework = base64.b64decode(raw["content"]).decode().strip()
        check_identity(candidate, framework, identity, receipt)
        with zipfile.ZipFile(io.BytesIO(gh(f"repos/{REPOSITORY}/actions/artifacts/{pages['id']}/zip", True))) as archive:
            members = [name for name in archive.namelist() if name.endswith((".tar", ".tar.gz", ".tgz"))]
            if len(members) != 1:
                raise ValueError("Pages artifact needs one complete tar archive")
            files = {}
            with tarfile.open(fileobj=io.BytesIO(archive.read(members[0])), mode="r:*") as output:
                for member in output:
                    if member.isdir():
                        continue
                    name = PurePosixPath(member.name)
                    if not member.isfile() or name.is_absolute() or ".." in name.parts or name.as_posix() in files:
                        raise ValueError(f"unsafe or duplicate Pages artifact path: {member.name}")
                    files[name.as_posix()] = output.extractfile(member).read()
        if files.get(MANIFEST) != manifest:
            raise ValueError("Pages and evidence artifacts have different manifests")
        return run["id"], identity, files
    raise ValueError("no successful matching run with both unexpired evidence and complete Pages artifacts; select --run-id or rerun the approved candidate")


class Document(HTMLParser):
    def __init__(self, text):
        super().__init__(convert_charrefs=True)
        self.ids, self.links, self.policies = set(), [], []
        self.lang = None
        self.templates = 0
        self.feed(text)

    def handle_starttag(self, tag, attributes):
        attributes = dict(attributes)
        if tag == "template":
            self.templates += 1
        if self.templates:
            return
        if attributes.get("id"):
            self.ids.add(attributes["id"])
        if tag == "html":
            self.lang = attributes.get("lang")
        if tag == "meta" and attributes.get("http-equiv", "").lower() == "content-security-policy":
            self.policies.append(attributes.get("content", ""))
        for key in ("href", "src", "poster", "data-rd-search-index", "data-rd-search-module", "data-rd-search-worker"):
            if attributes.get(key):
                self.links.append((attributes[key], tag == "a"))

    def handle_endtag(self, tag):
        if tag == "template" and self.templates:
            self.templates -= 1


def target(url, base):
    destination, root = urlsplit(url), urlsplit(base)
    if (destination.scheme, destination.netloc) != (root.scheme, root.netloc):
        return None
    if not destination.path.startswith(root.path):
        raise ValueError(f"same-origin address is outside the deployment base: {url}")
    relative = unquote(destination.path[len(root.path):])
    if ".." in PurePosixPath(relative).parts or relative.startswith("/") or "\\" in relative:
        raise ValueError(f"unsafe address: {url}")
    return relative + "index.html" if not relative or relative.endswith("/") else relative


def check_artifact(files, identity):
    manifest = json.loads(files[MANIFEST])
    if manifest["schema_version"] != 1 or manifest["mode"] != "production" or manifest["base"] != "/rustify-ui/":
        raise ValueError("wrong official manifest schema, mode or base")
    if (manifest["build_id"], manifest["cli_version"]) != (identity["build_id"], identity["cli_version"]):
        raise ValueError("artifact manifest and workflow identity disagree")
    entries = manifest["files"]
    names = [entry["path"] for entry in entries]
    if names != sorted(set(names)) or set(files) != set(names) | {MANIFEST}:
        raise ValueError("artifact file set is incomplete, duplicated or has unexpected extras")
    digest = hashlib.sha256()
    for entry in entries:
        name, checksum = entry["path"], entry["sha256"]
        if PurePosixPath(name).is_absolute() or ".." in PurePosixPath(name).parts or "\\" in name:
            raise ValueError(f"unsafe manifest file: {name}")
        if name.endswith(".md") or name.endswith("llms-full.txt"):
            raise ValueError(f"source Markdown or full-text AI copy was published: {name}")
        if len(files[name]) != entry["bytes"] or hashlib.sha256(files[name]).hexdigest() != checksum:
            raise ValueError(f"artifact bytes disagree with the manifest: {name}")
        if name.startswith(("assets/search.", "assets/search-worker.")) and name.rsplit(".", 2)[-2] != checksum[:10]:
            raise ValueError(f"search fingerprint disagrees with its bytes: {name}")
        digest.update(name.encode() + b"\0" + checksum.encode() + b"\n")
    if digest.hexdigest()[:16] != manifest["build_id"]:
        raise ValueError("manifest build_id is not the complete file-list identity")
    return manifest


def verify(url, files, identity, online=True):
    manifest = check_artifact(files, identity)
    documents = {}
    for name, expected in files.items():
        if online:
            address = urljoin(url, quote(name, safe="/"))
            with urlopen(Request(address, headers={"Accept-Encoding": "identity", "User-Agent": "rustify-docs-verifier"}), timeout=30) as response:
                if response.status != 200 or target(response.url, url) != name or response.read() != expected:
                    raise ValueError(f"public bytes/status/address differ from the same-run artifact: {address}")
                mime = response.headers.get_content_type()
            wanted = {".html": {"text/html"}, ".css": {"text/css"}, ".js": {"text/javascript", "application/javascript"},
                      ".json": {"application/json"}, ".txt": {"text/plain"}, ".wasm": {"application/wasm"}}.get(PurePosixPath(name).suffix)
            if wanted and mime not in wanted:
                raise ValueError(f"wrong MIME for {name}: {mime}")
        if name.endswith(".html"):
            document = Document(expected.decode())
            if document.policies != [META_CSP]:
                raise ValueError(f"meta CSP differs from the strict production policy: {name}")
            documents[name] = document
    def reference(value, source, external=False):
        if value.startswith("data:"):
            return
        if urlsplit(value).scheme.lower() in {"javascript", "file", "vbscript", "blob"}:
            raise ValueError(f"unsafe reference in {source}: {value}")
        destination = urljoin(urljoin(url, quote(source, safe="/")), value)
        name = target(destination, url)
        if name is None:
            if not external:
                raise ValueError(f"external resource in {source}: {value}")
            return
        if name not in files:
            raise ValueError(f"missing target in {source}: {value}")
        anchor = unquote(urlsplit(destination).fragment)
        if anchor and name in documents and anchor not in documents[name].ids:
            raise ValueError(f"missing anchor in {source}: {value}")
    for name, document in documents.items():
        for value, external in document.links:
            reference(value, name, external)
    indexes, ai_files = 0, 0
    for name, content in files.items():
        language_index = re.fullmatch(r"assets/search\.([A-Za-z0-9-]+)\.[0-9a-f]+\.json", name)
        if language_index:
            indexes += 1
            data = json.loads(content)
            if data["schema_version"] != 1:
                raise ValueError(f"unsupported search schema: {name}")
            if data["ui_lang"] != language_index[1]:
                raise ValueError(f"search index language disagrees with its asset name: {name}")
            for page in data["pages"]:
                address = urljoin(url, quote(page["route"].lstrip("/"), safe="/"))
                reference(address, name)
                page_file = target(address, url)
                if page_file not in documents or documents[page_file].lang != data["ui_lang"]:
                    raise ValueError(f"search page language disagrees with its index: {page['route']}")
                for section in page["sections"]:
                    if section["anchor"] is not None:
                        reference(address + "#" + quote(section["anchor"], safe=""), name)
        if PurePosixPath(name).name == "llms.txt":
            ai_files += 1
            text = content.decode()
            lines = [line for line in text.splitlines() if line.startswith("- [")]
            for line in lines:
                match = re.fullmatch(r"- \[.*\]\(([^)]+)\): (.+)", line)
                if not match or not match[2].strip():
                    raise ValueError(f"invalid or empty AI entry in {name}")
                reference(match[1], name)
    languages = {document.lang for document in documents.values()}
    if languages != {"en", "zh"} or indexes != 2 or ai_files != 2:
        raise ValueError("official HTML, search and AI outputs must cover en and zh")
    return {"candidate_sha": identity["candidate_sha"], "framework_revision": identity["framework_revision"],
            "build_id": manifest["build_id"], "files": len(files), "html": len(documents), "search_indexes": indexes, "ai_indexes": ai_files}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    mode = parser.add_mutually_exclusive_group(required=True)
    mode.add_argument("--url")
    mode.add_argument("--dir", type=Path)
    parser.add_argument("--identity", type=Path)
    parser.add_argument("--candidate-sha", required=True)
    parser.add_argument("--run-id", type=int)
    parser.add_argument("--report", type=Path)
    args = parser.parse_args()
    if args.url and args.url != PUBLIC_URL or not re.fullmatch(r"[0-9a-f]{40}", args.candidate_sha):
        parser.error("use the existing official Pages URL and a complete lowercase candidate C SHA")
    if args.dir and not args.identity:
        parser.error("--dir needs the same-run --identity JSON and its adjacent pages-prepare.json")
    try:
        if args.dir:
            identity = json.loads(args.identity.read_text())
            receipt = json.loads((args.identity.parent / "pages-prepare.json").read_text())
            framework = subprocess.check_output(["git", "show", f"{args.candidate_sha}:crates/rustify-docs-cli/assets/framework-revision.txt"], cwd=Path(__file__).resolve().parents[1], text=True).strip()
            check_identity(args.candidate_sha, framework, identity, receipt)
            files = {}
            for path in args.dir.rglob("*"):
                if path.is_symlink():
                    raise ValueError(f"unexpected symlink in complete output: {path}")
                if path.is_file():
                    files[path.relative_to(args.dir).as_posix()] = path.read_bytes()
            report = {"verification": "local artifact and graph", "passed": True, **verify(PUBLIC_URL, files, identity, online=False)}
        else:
            run, identity, files = workflow_artifact(args.candidate_sha, args.run_id)
            report = {"verification": "public HTTP and same-run artifact", "run_id": run, "url": args.url, "passed": True, **verify(args.url, files, identity)}
        if args.report:
            args.report.parent.mkdir(parents=True, exist_ok=True)
            args.report.write_text(json.dumps(report, indent=2) + "\n")
        print(json.dumps(report, indent=2))
    except (ValueError, OSError, KeyError, subprocess.CalledProcessError, tarfile.TarError, zipfile.BadZipFile) as error:
        raise SystemExit(f"Pages verification failed: {error}") from error


if __name__ == "__main__":
    main()
