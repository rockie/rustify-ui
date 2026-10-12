"""Isolated reproduction fixtures for DOCS-SSG-P1 code review R1.

Run after building rustify-docs-cli with --profile docs-cli:
  python3 reproduce.py --binary /path/to/rustify-docs
The printed work directory also works with browser-cases.cjs.
Only synthetic sentinels are used; no actual private files are read.
"""

import argparse
import json
from pathlib import Path
import re
import subprocess
import tempfile

parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument("--binary", required=True)
parser.add_argument("--work", help="An empty temporary directory; created if absent")
args = parser.parse_args()
binary = Path(args.binary).resolve()
root = Path(args.work or tempfile.mkdtemp(prefix="rustify-docs-review-r1-")).resolve()
root.mkdir(parents=True, exist_ok=True)
if any(root.iterdir()):
    parser.error("--work must be empty")

CONFIG = 'schema_version = 1\n[site]\nname = "Review Docs"\nurl = "https://example.net/docs/"\nbase = "/docs/"\n'
PAGE = '---\ntitle: Review page\ndescription: Review fixture\n---\n# Review page\n\nBody.\n'
SVG = '<svg xmlns="http://www.w3.org/2000/svg" width="40" height="40"><rect width="40" height="40" fill="red"/></svg>'


def site(name, body=PAGE, config=CONFIG):
    path = root / "cases" / name
    (path / "content").mkdir(parents=True)
    (path / "static").mkdir()
    (path / "rustify-docs.toml").write_text(config)
    if body is not None:
        (path / "content/index.md").write_text(body)
    return path


def invoke(name, command="build", config_name="rustify-docs.toml", extra=()):
    path = root / "cases" / name
    argv = [str(binary), command, "--config", str(path / config_name), "--format", "json", *extra]
    done = subprocess.run(argv, capture_output=True, text=True)
    (path / (command + ".stdout")).write_text(done.stdout)
    (path / (command + ".stderr")).write_text(done.stderr)
    item = {"case": name, "command": command, "argv": argv, "exit": done.returncode,
            "stdout": done.stdout, "stderr": done.stderr}
    try:
        item["report"] = json.loads(done.stdout)
    except ValueError:
        pass
    results.append(item)
    return path, item


def html(path):
    return (path / "dist/index.html").read_text()


def published_sentinel(path, sentinel):
    return [str(file.relative_to(path / "dist")) for file in (path / "dist").rglob("*")
            if file.is_file() and sentinel.encode() in file.read_bytes()]


results = []
p = site("header-links", config=CONFIG + '\n[[nav.top]]\nlabel = "Top link visible"\nlink = "guide.md"\n[[nav.social]]\nlabel = "Social link visible"\nlink = "https://example.net/community"\n')
(p / "content/guide.md").write_text(PAGE.replace("Review page", "Guide"))
p, item = invoke("header-links")
item["header_labels_in_html"] = {label: label in html(p) for label in ("Top link visible", "Social link visible")}

p = site("home-resource", '---\ntitle: Review home\ndescription: Review fixture\nlayout: home\nactions:\n  - text: Download manual\n    link: manual.pdf\n---\n# Review home\n\nBody.\n')
(p / "content/manual.pdf").write_bytes(b"%PDF-1.4\nREVIEW_MANUAL_FIXTURE\n")
p, item = invoke("home-resource")
item["download_href"] = re.findall(r'href="([^"]+)"[^>]*>Download manual', html(p))
item["manual_published"] = published_sentinel(p, "REVIEW_MANUAL_FIXTURE")

outside = root / "outside"
outside.mkdir()
(outside / "outside.txt").write_text("OUTSIDE_REVIEW_FIXTURE\n")
p = site("external-static-root")
(p / "static").rmdir()
(p / "static").symlink_to(outside, target_is_directory=True)
p, item = invoke("external-static-root")
item["outside_file_published"] = published_sentinel(p, "OUTSIDE_REVIEW_FIXTURE")

p = site("private-symlink")
(p / ".env").write_text("PRIVATE_SYMLINK_REVIEW_FIXTURE=1\n")
(p / "static/public.txt").symlink_to(p / ".env")
p, item = invoke("private-symlink")
item["private_bytes_published"] = published_sentinel(p, "PRIVATE_SYMLINK_REVIEW_FIXTURE")

p = site("private-markdown")
(p / "content/.env.md").write_text(PAGE.replace("Review page", "PRIVATE_MARKDOWN_REVIEW_FIXTURE"))
p, item = invoke("private-markdown")
item["private_page_published"] = published_sentinel(p, "PRIVATE_MARKDOWN_REVIEW_FIXTURE")

site("css-external", PAGE + '\n<div class="bg-[url(https://example.invalid/review.png)] h-20">CSS resource</div>\n')
p, item = invoke("css-external")
item["external_url_in_css"] = any("example.invalid/review.png" in f.read_text() for f in (p / "dist").rglob("*.css"))

p = site("svg-fragment", PAGE + '\n<svg width="40" height="40"><use href="/sprite.svg#mark"></use></svg>\n')
(p / "static/sprite.svg").write_text('<svg xmlns="http://www.w3.org/2000/svg"><symbol id="mark"><rect width="40" height="40" fill="red"/></symbol></svg>')
p, item = invoke("svg-fragment")
item["use_href"] = re.findall(r'<use[^>]*href="([^"]+)"', html(p))

site("frontmatter-no-newline", "---")
invoke("frontmatter-no-newline", "check")

p = site("alternate-config")
(p / "custom.toml").write_text(CONFIG)
(p / "rustify-docs.toml").write_text("NOT TOML")
invoke("alternate-config", "check", "custom.toml")

site("no-content", None)
invoke("no-content")
site("json-output-error")
invoke("json-output-error", extra=("--out", str(root / "cases/json-output-error/content")))

site("toc-percent", PAGE + '\n## Completion {#100%}\n\n' + '\n\n'.join("Review paragraph " + str(i) for i in range(20)) + '\n\n## Finish\n\nEnd.\n')
invoke("toc-percent")

p = site("css-local", PAGE + '\n<div class="bg-[url(/bg.svg)] h-20">Local CSS resource</div>\n')
(p / "static/bg.svg").write_text(SVG)
invoke("css-local")

p = site("external-image", PAGE + '\n![External file](../images/public.svg)\n')
(p / "images").mkdir()
(root / "outside-image.svg").write_text(SVG.replace("red", "OUTSIDE_IMAGE_REVIEW_FIXTURE"))
(p / "images/public.svg").symlink_to(root / "outside-image.svg")
p, item = invoke("external-image")
item["outside_image_published"] = published_sentinel(p, "OUTSIDE_IMAGE_REVIEW_FIXTURE")

p = site("watch-resource", PAGE + '\n![Watch resource](../images/mark.svg)\n')
(p / "images").mkdir()
(p / "images/mark.svg").write_text(SVG)
invoke("watch-resource")

p = site("private-logo", config=CONFIG + '\n[theme]\nlogo = ".env"\n')
(p / ".env").write_text("PRIVATE_LOGO_REVIEW_FIXTURE=1\n")
p, item = invoke("private-logo")
item["private_bytes_published"] = published_sentinel(p, "PRIVATE_LOGO_REVIEW_FIXTURE")

p = site("private-include-alias", PAGE + '\n<!-- rustify:include path="public.txt" -->\n')
(p / ".env").write_text("PRIVATE_INCLUDE_REVIEW_FIXTURE=1\n")
(p / "public.txt").symlink_to(p / ".env")
p, item = invoke("private-include-alias")
item["private_bytes_in_html"] = "PRIVATE_INCLUDE_REVIEW_FIXTURE" in html(p)

site("missing-footnote-anchor", PAGE + '\n[Missing footnote](#fn-never-defined)\n')
p, item = invoke("missing-footnote-anchor")
item["link_published"] = 'href="#fn-never-defined"' in html(p)
item["target_exists"] = 'id="fn-never-defined"' in html(p)

(root / "reproductions.json").write_text(json.dumps(results, ensure_ascii=False, indent=2) + "\n")
print(json.dumps({"work": str(root), "cases": len(results), "results": [
    {k: v for k, v in item.items() if k not in ("argv", "stdout", "stderr", "report")}
    | {"success": item.get("report", {}).get("success"),
       "errors": item.get("report", {}).get("summary", {}).get("errors")}
    for item in results]}, ensure_ascii=False, indent=2))
