#!/usr/bin/env python3
"""Compose the built official Docs output with a complete Theme Studio build.

Python 3.12+, standard library only. Validates both inputs against their own
manifests, copies Docs to <out> and Studio to <out>/theme-studio/, injects the
same meta CSP the Pages verifier requires into the Studio page, localizes the
one absolute public Studio URL in generated Docs HTML, refreshes the nested
Studio build-manifest file list and size report, and recomputes the root
manifest with the existing `publish.rs::Manifest::of` algorithm. It never
compiles anything and never writes into either input directory.
"""

from __future__ import annotations

import argparse
import hashlib
import json
from pathlib import Path, PurePosixPath
import shutil
import sys

# Kept byte-identical to scripts/docs-pages-verify.py::META_CSP; the composer
# never widens the policy the verifier enforces.
META_CSP = "default-src 'none'; script-src 'self' 'wasm-unsafe-eval'; style-src 'self'; img-src 'self' data:; font-src 'self'; connect-src 'self'; media-src 'self'; worker-src 'self'; object-src 'none'; base-uri 'none'; form-action 'none'"

# The single absolute link the site uses for the public Studio before it is
# deployed beside the docs; localized to the packaged copy, nothing else.
PUBLIC_STUDIO_URL = "https://rockie.github.io/rustify-ui/theme-studio/"
DOCS_MANIFEST = "rustify-docs-manifest.json"
STUDIO_MANIFEST = "build-manifest.json"


def fail(message: str) -> None:
    raise SystemExit(f"docs-pages-compose: {message}")


def relative_under(root: Path, listed: str) -> PurePosixPath:
    path = PurePosixPath(listed)
    if path.is_absolute() or ".." in path.parts or path.is_reserved():
        fail(f"manifest lists an unsafe path: {listed}")
    if path.as_posix() == ".":
        fail(f"manifest lists an empty path: {listed}")
    return path


def sha256_of(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for block in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(block)
    return digest.hexdigest()


def size_report(files: dict[str, int]) -> dict[str, int]:
    buckets = {"wasm": 0, "js": 0, "css": 0, "fonts": 0, "images": 0, "data": 0}
    total = 0
    for name, size in files.items():
        ext = name.rsplit(".", 1)[-1] if "." in name else ""
        if ext == "wasm":
            key = "wasm"
        elif ext in ("js", "mjs"):
            key = "js"
        elif ext == "css":
            key = "css"
        elif ext in ("ttf", "otf", "woff", "woff2"):
            key = "fonts"
        elif ext in ("png", "jpg", "jpeg", "svg", "webp", "gif"):
            key = "images"
        else:
            key = "data"
        buckets[key] += size
        total += size
    buckets["total"] = total
    return buckets


def load_docs_manifest(docs_dir: Path, base: str) -> dict:
    manifest_path = docs_dir / DOCS_MANIFEST
    if not manifest_path.is_file():
        fail(f"{docs_dir} has no {DOCS_MANIFEST}; it is not a built Docs output")
    manifest = json.loads(manifest_path.read_text())
    if manifest.get("base") != base:
        fail(f"Docs manifest base {manifest.get('base')!r} does not match --base {base!r}")
    if manifest.get("mode") != "production":
        fail("Docs manifest must describe a production build")
    for entry in manifest.get("files", []):
        relative = relative_under(docs_dir, entry["path"])
        source = docs_dir / relative
        if not source.is_file():
            fail(f"Docs manifest lists a missing file: {entry['path']}")
        if source.stat().st_size != entry["bytes"]:
            fail(f"Docs manifest size mismatch for {entry['path']}")
        if sha256_of(source) != entry["sha256"]:
            fail(f"Docs manifest digest mismatch for {entry['path']}")
    for entry in manifest.get("files", []):
        if PurePosixPath(entry["path"]).parts[0] == "theme-studio":
            fail("Docs output already contains a theme-studio entry")
    return manifest


def load_studio_manifest(studio_dir: Path, base: str) -> dict:
    manifest_path = studio_dir / STUDIO_MANIFEST
    if not manifest_path.is_file():
        fail(f"{studio_dir} has no {STUDIO_MANIFEST}; it is not a built Studio product")
    if not (studio_dir / "index.html").is_file():
        fail("Studio build has no index.html")
    manifest = json.loads(manifest_path.read_text())
    expected = f"{base}theme-studio/"
    if manifest.get("base") != expected:
        fail(
            f"Studio build-manifest base {manifest.get('base')!r} does not match {expected!r}"
        )
    for listed in manifest.get("files", {}):
        if listed == STUDIO_MANIFEST:
            # An incremental rebuild can list the manifest itself with the
            # previous build's size. The refreshed nested manifest excludes
            # it, matching a fresh build, so its recorded size is not a fact
            # about this product.
            continue
        relative = relative_under(studio_dir, listed)
        source = studio_dir / relative
        if not source.is_file():
            fail(f"Studio build-manifest lists a missing file: {listed}")
        if source.stat().st_size != manifest["files"][listed]:
            fail(f"Studio build-manifest size mismatch for {listed}")
    return manifest


def prepare_out(out: Path, docs_dir: Path, studio_dir: Path) -> None:
    if out.exists():
        if not out.is_dir():
            fail(f"--out {out} exists and is not a directory")
        if any(out.iterdir()):
            fail(f"--out {out} exists and is not empty")
    else:
        out.mkdir(parents=True)
    resolved = {docs_dir.resolve(), studio_dir.resolve()}
    probe = out.resolve()
    if probe in resolved:
        fail("--out must not be one of the input directories")
    for source in resolved:
        if probe == source or source in probe.parents:
            fail("--out must not contain an input directory")


def inject_meta_csp(page: Path) -> None:
    text = page.read_text()
    if "Content-Security-Policy" in text:
        fail("Studio index.html already carries a Content-Security-Policy")
    marker = "<head>"
    index = text.find(marker)
    if index < 0:
        fail("Studio index.html has no <head> to hold the policy")
    meta = (
        f'{marker}\n    <meta http-equiv="Content-Security-Policy" content="{META_CSP}">'
    )
    page.write_text(text[:index] + meta + text[index + len(marker):])


def localize_studio_links(out: Path, base: str) -> None:
    replacement = f'"{base}theme-studio/"'
    needle = f'"{PUBLIC_STUDIO_URL}"'
    for page in sorted(out.rglob("*.html")):
        if "theme-studio" in page.relative_to(out).parts[:-1]:
            continue
        text = page.read_text()
        if needle in text:
            page.write_text(text.replace(needle, replacement))


def refresh_studio_manifest(studio_out: Path, manifest: dict) -> None:
    files: dict[str, int] = {}
    for path in sorted(studio_out.rglob("*")):
        if not path.is_file():
            continue
        relative = path.relative_to(studio_out).as_posix()
        if relative == STUDIO_MANIFEST:
            continue
        files[relative] = path.stat().st_size
    manifest["files"] = files
    manifest["size_report"] = size_report(files)
    (studio_out / STUDIO_MANIFEST).write_text(
        json.dumps(manifest, indent=2, sort_keys=True) + "\n"
    )


def recompute_root_manifest(out: Path, docs_manifest: dict, base: str) -> dict:
    files: dict[str, Path] = {}
    for path in sorted(out.rglob("*")):
        if not path.is_file():
            continue
        relative = path.relative_to(out).as_posix()
        if relative == DOCS_MANIFEST:
            continue
        files[relative] = path
    whole = hashlib.sha256()
    entries = []
    for relative in sorted(files):
        digest = sha256_of(files[relative])
        whole.update(relative.encode())
        whole.update(b"\0")
        whole.update(digest.encode())
        whole.update(b"\n")
        entries.append(
            {
                "path": relative,
                "sha256": digest,
                "bytes": files[relative].stat().st_size,
            }
        )
    build_id = whole.hexdigest()[:16]
    manifest = {
        "schema_version": docs_manifest.get("schema_version", 1),
        "mode": docs_manifest.get("mode", "production"),
        "base": base,
        "cli_version": docs_manifest["cli_version"],
        "build_id": build_id,
        "files": entries,
        "locale_not_found": docs_manifest.get("locale_not_found", {}),
        "page_policies": docs_manifest.get("page_policies", {}),
    }
    (out / DOCS_MANIFEST).write_text(json.dumps(manifest, indent=2) + "\n")
    return manifest


def compose(docs_dir: Path, studio_dir: Path, out: Path, base: str) -> dict:
    if not (base.startswith("/") and base.endswith("/")):
        fail("--base must look like /rustify-ui/")
    docs_manifest = load_docs_manifest(docs_dir, base)
    studio_manifest = load_studio_manifest(studio_dir, base)
    prepare_out(out, docs_dir, studio_dir)

    for entry in docs_manifest["files"]:
        relative = relative_under(docs_dir, entry["path"])
        destination = out / relative
        destination.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(docs_dir / relative, destination)
    studio_out = out / "theme-studio"
    studio_out.mkdir()
    for listed in studio_manifest["files"]:
        relative = relative_under(studio_dir, listed)
        destination = studio_out / relative
        destination.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(studio_dir / relative, destination)

    inject_meta_csp(studio_out / "index.html")
    localize_studio_links(out, base)
    refresh_studio_manifest(studio_out, studio_manifest)
    root_manifest = recompute_root_manifest(out, docs_manifest, base)
    return {
        "files": len(root_manifest["files"]),
        "build_id": root_manifest["build_id"],
        "cli_version": root_manifest["cli_version"],
        "base": base,
        "studio_build_id": studio_manifest.get("build_id"),
        "studio_base": studio_manifest.get("base"),
    }


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--docs-dir", required=True, type=Path)
    parser.add_argument("--studio-dir", required=True, type=Path)
    parser.add_argument("--out", required=True, type=Path)
    parser.add_argument("--base", required=True)
    args = parser.parse_args()
    summary = compose(args.docs_dir, args.studio_dir, args.out, args.base)
    print(json.dumps(summary, indent=2))


if __name__ == "__main__":
    try:
        main()
    except (OSError, ValueError, KeyError, json.JSONDecodeError) as error:
        fail(str(error))
        sys.exit(1)
