#!/usr/bin/env python3
"""Inventory built A/B first-load files without Cargo, browsers or servers.

Python 3.12 standard library. Based on formal-resource-selection-r1's closure:
HTML declarations, static modules, the eager islands boot and CSS references.
Omitted B arguments remain absent; this tool never substitutes an older dist.
"""

import argparse
import gzip
import hashlib
from html.parser import HTMLParser
import json
from pathlib import Path, PurePosixPath
import re
import sys
from urllib.parse import unquote, urljoin, urlsplit
import zlib


ROOT = Path(__file__).resolve().parents[4]
IDENTITY = ROOT / "target/docs-ssg/m6-installed-c.json"


def sha256(data):
    return hashlib.sha256(data).hexdigest()


class Html(HTMLParser):
    def __init__(self, text):
        super().__init__()
        self.skip = 0
        self.refs = []
        self.lang = self.category = None
        self.islands = 0
        self.excluded = dict.fromkeys(("navigation", "download", "metadata", "lazy", "template", "inline_data"), 0)
        self.feed(text)

    def ref(self, url, reason, conditional=False):
        if url.startswith("data:"):
            self.excluded["inline_data"] += 1
        elif url and not url.startswith("#"):
            self.refs.append((url, reason, conditional))

    def handle_starttag(self, tag, attributes):
        attrs = dict(attributes)
        if tag in ("template", "noscript"):
            self.skip += 1
            self.excluded["template"] += 1
            return
        if self.skip:
            return
        if tag == "html":
            self.lang = attrs.get("lang")
        if "data-component-category" in attrs:
            self.category = attrs["data-component-category"]
        if "data-rustify-island" in attrs:
            self.islands += 1
        if tag == "a" and "href" in attrs:
            self.excluded["download" if "download" in attrs else "navigation"] += 1
        if tag == "meta" and attrs.get("property", "").startswith("og:"):
            self.excluded["metadata"] += 1
        if tag == "link":
            rel = set(attrs.get("rel", "").split())
            if "canonical" in rel:
                self.excluded["metadata"] += 1
            elif rel & {"stylesheet", "modulepreload", "preload", "icon"}:
                self.ref(attrs.get("href", ""), "html-link-" + attrs.get("rel", ""), bool(attrs.get("media")) or "icon" in rel)
        if tag == "script" and attrs.get("type", "").lower() in ("", "module", "text/javascript", "application/javascript"):
            self.ref(attrs.get("src", ""), "html-script")
        if tag == "img":
            if attrs.get("loading") == "lazy":
                self.excluded["lazy"] += 1
            else:
                self.ref(attrs.get("src", ""), "html-eager-image")
                for candidate in attrs.get("srcset", "").split(","):
                    if candidate.strip():
                        self.ref(candidate.strip().split()[0], "html-srcset-candidate", True)
        if tag in ("image", "use"):
            self.ref(attrs.get("href") or attrs.get("xlink:href", ""), "svg-reference")
        if tag in ("video", "audio", "iframe") and attrs.get("preload") != "none" and attrs.get("loading") != "lazy":
            self.ref(attrs.get("src", ""), "html-" + tag, True)

    def handle_endtag(self, tag):
        if tag in ("template", "noscript") and self.skip:
            self.skip -= 1


def route(file):
    if file == "index.html":
        return "/"
    return "/" + (file[:-10] if file.endswith("/index.html") else file)


def maximum(pages, metric):
    value = max(page[metric] for page in pages)
    return {metric: value, "candidates": [{key: page[key] for key in ("file", "route", "category")} for page in pages if page[metric] == value]}


def inventory(dist, gzip_cache):
    dist = dist.resolve(strict=True)
    manifest_path = dist / "rustify-docs-manifest.json"
    original = manifest_path.read_bytes()
    manifest = json.loads(original)
    if manifest["mode"] != "production":
        raise ValueError(f"{dist}: production output required")
    data, metrics = {}, {}
    for file in manifest["files"]:
        name = file["path"]
        if PurePosixPath(name).is_absolute() or ".." in PurePosixPath(name).parts or str(PurePosixPath(name)) != name or name in data:
            raise ValueError(f"unsafe or duplicate manifest path {name}")
        content = (dist / name).read_bytes()
        digest = sha256(content)
        if len(content) != file["bytes"] or digest != file["sha256"]:
            raise ValueError(f"{dist}/{name}: manifest SHA/bytes mismatch")
        if digest not in gzip_cache:
            gzip_cache[digest] = len(gzip.compress(content, compresslevel=9, mtime=0))
        data[name] = content
        metrics[name] = {"bytes": len(content), "gzip9": gzip_cache[digest], "sha256": digest}
    base = manifest["base"]
    dependency_graph, dependency_sets, pages = {}, {}, []
    exclusions = dict.fromkeys(("navigation", "download", "metadata", "lazy", "template", "inline_data"), 0)

    def resolve(href, source):
        if href.startswith("#") or re.match(r"(?:data|blob|javascript|mailto|tel):", href, re.I):
            return None
        parsed = urlsplit(urljoin("https://first-load.invalid" + base + source, href))
        if parsed.netloc != "first-load.invalid" or not parsed.path.startswith(base):
            raise ValueError(f"external/outside-base first-load URL {href}")
        name = unquote(parsed.path[len(base):])
        if not name or name.endswith("/"):
            name += "index.html"
        if name not in data:
            raise ValueError(f"unpublished first-load URL {href} from {source}")
        return name

    def dependencies(name):
        if name in dependency_graph:
            return dependency_graph[name]
        refs, unexecuted, inline = [], [], 0
        if name.endswith(".js"):
            text = data[name].decode()
            for match in re.finditer(r"\bfrom\s*['\"]([^'\"]+)['\"]|\bimport\s*['\"]([^'\"]+)['\"]", text):
                refs.append((match[1] or match[2], "static-module-import", False))
            urls = re.findall(r"new URL\(\s*['\"]([^'\"]+)['\"]\s*,\s*import\.meta\.url\s*\)", text)
            if name.endswith("/islands.js"):
                if "await boot({" not in text or "if (blocks.length" not in text:
                    raise ValueError("the audited eager islands bootstrap changed")
                refs.extend((url, "eager-island-boot-option", False) for url in urls)
            else:
                unexecuted.extend(urls)
            unexecuted.extend(re.findall(r"\bimport\(\s*['\"]([^'\"]+)['\"]", text))
        elif name.endswith(".css"):
            text = data[name].decode()
            for match in re.finditer(r"url\(\s*(?:\"([^\"]*)\"|'([^']*)'|([^)]*))\s*\)", text):
                url = next(value for value in match.groups() if value is not None).strip()
                if url.startswith(("data:", "#")):
                    inline += 1
                else:
                    refs.append((url, "css-url-conservative-selector-or-font", True))
            refs.extend((url, "css-import", True) for url in re.findall(r"@import\s+['\"]([^'\"]+)['\"]", text))
        dependency_graph[name] = {"refs": refs, "inline_css_urls": inline, "unexecuted_url_literals": unexecuted}
        return dependency_graph[name]

    for name in sorted(file for file in data if file.endswith(".html")):
        html = Html(data[name].decode())
        selected, pending, conditional = {name}, [name], set()
        html_edges = []
        while pending:
            source = pending.pop()
            refs = html.refs if source == name else dependencies(source)["refs"]
            for href, reason, possible in refs:
                target = resolve(href, source)
                if target is None:
                    continue
                if reason == "eager-island-boot-option" and not html.islands:
                    raise ValueError(f"island bootstrap on static page {name}")
                if possible:
                    conditional.add(target)
                if source == name:
                    html_edges.append({"to": target, "reason": reason})
                if target not in selected:
                    selected.add(target)
                    pending.append(target)
        shared = tuple(sorted(selected - {name}))
        if shared not in dependency_sets:
            dependency_sets[shared] = {"id": f"set-{len(dependency_sets) + 1}", "files": shared,
                "bytes": sum(metrics[file]["bytes"] for file in shared), "gzip9": sum(metrics[file]["gzip9"] for file in shared),
                "sha256": sha256("".join(f"{file} {metrics[file]['sha256']}\n" for file in shared).encode()),
                "conditional_files": sorted(conditional), "html_references": html_edges}
        deps = dependency_sets[shared]
        pages.append({"file": name, "route": base.rstrip("/") + route(name), "lang": html.lang, "category": html.category,
            "islands": html.islands, "dependency_set": deps["id"], "bytes": metrics[name]["bytes"] + deps["bytes"],
            "gzip9": metrics[name]["gzip9"] + deps["gzip9"],
            "closure_sha256": sha256("".join(f"{file} {metrics[file]['sha256']}\n" for file in sorted(selected)).encode())})
        for reason, count in html.excluded.items():
            exclusions[reason] += count
    groups = {}

    def group(name, selected):
        if selected:
            groups[name] = {"count": len(selected), "max_raw": maximum(selected, "bytes"), "max_gzip9": maximum(selected, "gzip9")}

    group("home", [page for page in pages if page["file"] == "index.html"])
    group("404", [page for page in pages if page["file"] == "404.html"])
    group("islands_all", [page for page in pages if page["islands"]])
    guides = [page for page in pages if not page["islands"] and page["file"] not in ("index.html", "404.html")]
    group("guides_all", guides)
    for lang in ("en", "zh"):
        group("guides_" + lang, [page for page in guides if page["lang"] == lang])
    for page in pages:
        if page["category"]:
            group("category:" + page["category"], [page])
    used = {page["file"] for page in pages} | {file for files in dependency_sets for file in files}
    if manifest_path.read_bytes() != original:
        raise ValueError(f"{dist}: manifest changed during inventory")
    return {"dist": str(dist), "base": base, "build_id": manifest["build_id"], "cli_version": manifest["cli_version"],
        "manifest_sha256": sha256(original), "verification": {"files": len(data), "every_SHA_and_bytes_match": True,
            "total_declared_bytes": sum(item["bytes"] for item in metrics.values())},
        "inventory_sha256": sha256("".join(f"{file} {metrics[file]['sha256']}\n" for file in sorted(metrics)).encode()),
        "files": metrics, "pages": pages, "groups": groups, "first_load_dependency_sets": list(dependency_sets.values()),
        "excluded_HTML_references_totals": exclusions, "dependency_graph": dependency_graph,
        "unused_first_load_manifest_files": sorted(set(data) - used),
        "referenced_network_font_files": sorted(file for file in used if re.search(r"\.(?:ttf|otf|woff2?)$", file)),
        "island_non_HTML_dependency_sets": sorted({page["dependency_set"] for page in pages if page["islands"]})}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--a-dist", type=Path, required=True)
    parser.add_argument("--b-p2-dist", type=Path)
    parser.add_argument("--b-p1-dist", type=Path)
    parser.add_argument("--out", type=Path, required=True)
    args = parser.parse_args()
    try:
        if args.out.exists():
            raise ValueError("choose a new output path; preserved reports are never overwritten")
        identity_bytes = IDENTITY.read_bytes()
        identity = json.loads(identity_bytes)
        version = re.search(r"^rustify-docs (\S+)", identity["source_identity"])[1]
        sites, cache = {}, {}
        for name, dist in (("A_official", args.a_dist), ("B_P2", args.b_p2_dist), ("B_P1", args.b_p1_dist)):
            if dist is not None:
                sites[name] = inventory(dist, cache)
                if sites[name]["cli_version"] != version:
                    raise ValueError(f"{name}: CLI version differs from current installed-C identity")
        if IDENTITY.read_bytes() != identity_bytes:
            raise ValueError("installed-C identity changed during inventory")
        report = {"schema_version": 1, "scope": "static declared first-load inventory only; no browser observation or release/performance verdict",
            "candidate_C": identity["candidate_sha"], "framework_R": identity["framework_revision"],
            "installed_identity_input": {"path": str(IDENTITY.relative_to(ROOT)), "sha256": sha256(identity_bytes)},
            "script_sha256": sha256(Path(__file__).read_bytes()),
            "method": {"compression": {"algorithm": "Python gzip.compress(level=9, mtime=0), per-file sum; identical SHA content shared",
                "python": sys.version.split()[0], "zlib": zlib.ZLIB_RUNTIME_VERSION,
                "meaning": "normalized gzip sizes, not HTTP compression; Node/zlib may produce different gzip bytes"},
                "closure": "HTML executable scripts/styles/preload/eager images; static imports and audited eager islands boot glue/wasm; CSS urls/imports conservative",
                "excluded": "navigation/download/OG/canonical, templates/noscript, lazy images, unexecuted dynamic import/new URL literals",
                "inline_icon": "data URI icon has no network edge; its bytes are already in the verified HTML",
                "actual_vs_declared": "implicit native-browser requests (e.g. favicon.ico), viewport/media-dependent CSS/fonts and srcset choices require actual response cross-check; this tool does not fabricate those observations",
                "page_totals": "each verified HTML file plus its shared non-HTML dependency set; raw and gzip maxima retain ties"},
            "sites": sites}
        args.out.write_text(json.dumps(report, indent=2) + "\n")
        print(json.dumps({"out": str(args.out), "candidate_C": report["candidate_C"], "framework_R": report["framework_R"],
            "sites": {name: {"files_verified": site["verification"]["files"], "html_pages": len(site["pages"]),
                "island_maxima": site["groups"].get("islands_all")} for name, site in sites.items()}}))
    except (OSError, ValueError, KeyError, TypeError) as error:
        parser.exit(1, f"inventory-first-load: {error}\n")


if __name__ == "__main__":
    main()
