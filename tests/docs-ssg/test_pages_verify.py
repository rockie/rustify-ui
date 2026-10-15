"""Local protocol fixtures; no GitHub or deployment calls are made here."""

import functools
import hashlib
from http.server import SimpleHTTPRequestHandler, ThreadingHTTPServer
import importlib.util
import json
from pathlib import Path
import tempfile
import threading
import unittest

SCRIPT = Path(__file__).resolve().parents[2] / "scripts/docs-pages-verify.py"
SPEC = importlib.util.spec_from_file_location("pages_verify", SCRIPT)
verify = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(verify)


def fixture():
    files = {}
    for lang, prefix in [("en", ""), ("zh", "zh/")]:
        files[prefix + "index.html"] = (f'<html lang="{lang}"><head><meta http-equiv="Content-Security-Policy" content="{verify.META_CSP}"></head>'
                                       '<body><main><h1>Docs</h1><h2 id="setup">Setup</h2></main></body></html>').encode()
        files[prefix + "llms.txt"] = f"# Docs\n\n- [Docs](/rustify-ui/{prefix}): A page.\n".encode()
        data = json.dumps({"schema_version": 1, "ui_lang": lang, "pages": [{"route": "/" + prefix, "title": "Docs", "source_lang": lang, "is_fallback": False, "nav_order": 0,
                           "sections": [{"anchor": "setup", "path": ["Docs", "Setup"], "text": "Public text."}]}]}).encode()
        files[f"assets/search.{lang}.{hashlib.sha256(data).hexdigest()[:10]}.json"] = data
    files[".nojekyll"] = b""
    return seal(files)


def seal(files):
    files = {name: content for name, content in files.items() if name != verify.MANIFEST}
    entries = [{"path": name, "sha256": hashlib.sha256(content).hexdigest(), "bytes": len(content)} for name, content in sorted(files.items())]
    digest = hashlib.sha256()
    for entry in entries:
        digest.update(entry["path"].encode() + b"\0" + entry["sha256"].encode() + b"\n")
    manifest = {"schema_version": 1, "mode": "production", "base": "/rustify-ui/", "cli_version": "fixture", "build_id": digest.hexdigest()[:16], "files": entries}
    files[verify.MANIFEST] = json.dumps(manifest).encode()
    identity = {"candidate_sha": "a" * 40, "framework_revision": "b" * 40, "cli_version": "fixture", "build_id": manifest["build_id"]}
    return files, identity


class PagesVerifierTests(unittest.TestCase):
    def test_complete_local_http_product_and_indexes_are_checked(self):
        files, identity = fixture()
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            for name, content in files.items():
                target = root / "rustify-ui" / name
                target.parent.mkdir(parents=True, exist_ok=True)
                target.write_bytes(content)
            class Quiet(SimpleHTTPRequestHandler):
                def log_message(self, *args):
                    pass
            server = ThreadingHTTPServer(("127.0.0.1", 0), functools.partial(Quiet, directory=directory))
            thread = threading.Thread(target=server.serve_forever, daemon=True)
            thread.start()
            try:
                url = f"http://127.0.0.1:{server.server_port}/rustify-ui/"
                result = verify.verify(url, files, identity)
                self.assertEqual(result["html"], 2)
                self.assertEqual(result["search_indexes"], 2)
                self.assertEqual(result["ai_indexes"], 2)
                original = next(name for name in files if name.startswith("assets/search.en."))
                data = json.loads(files[original])
                data["ui_lang"] = "fr"
                bad = json.dumps(data).encode()
                changed = {name: content for name, content in files.items() if name != original}
                changed[f"assets/search.en.{hashlib.sha256(bad).hexdigest()[:10]}.json"] = bad
                changed, changed_identity = seal(changed)
                for name, content in changed.items():
                    (root / "rustify-ui" / name).write_bytes(content)
                with self.assertRaisesRegex(ValueError, "language"):
                    verify.verify(url, changed, changed_identity)
                for name, content in files.items():
                    (root / "rustify-ui" / name).write_bytes(content)
                (root / "rustify-ui/zh/index.html").write_text("stale generation")
                with self.assertRaisesRegex(ValueError, "public bytes"):
                    verify.verify(url, files, identity)
            finally:
                server.shutdown()
                server.server_close()
                thread.join()

    def test_manifest_identity_and_complete_file_set_are_required(self):
        files, identity = fixture()
        verify.check_artifact(files, identity)
        for bad in [{**files, "extra.md": b"source"}, {name: content for name, content in files.items() if name != "zh/index.html"}, {**files, "index.html": b"stale"}]:
            with self.assertRaises(ValueError):
                verify.check_artifact(bad, identity)
        with self.assertRaisesRegex(ValueError, "identity disagree"):
            verify.check_artifact(files, {**identity, "build_id": "wrong"})
        self.assertEqual(verify.verify(verify.PUBLIC_URL, files, identity, online=False)["files"], len(files))

    def test_source_receipt_requires_the_same_candidate_framework_and_projection(self):
        candidate, framework = "a" * 40, "b" * 40
        projection = {"site/Cargo.toml": {}, "site/Cargo.lock": {}}
        identity = {"candidate_sha": candidate, "framework_revision": framework, "projection": projection}
        receipt = {**identity, "cli_build_info": f"rustify-docs fixture\npanic: unwind\nsource: {candidate}\n"}
        verify.check_identity(candidate, framework, identity, receipt)
        with self.assertRaises(ValueError):
            verify.check_identity(candidate, "c" * 40, identity, receipt)
        with self.assertRaises(ValueError):
            verify.check_identity(candidate, framework, identity, {**receipt, "cli_build_info": "panic: abort\n"})

    def test_addresses_resolve_once_and_cannot_escape_the_base(self):
        base = "https://example.test/rustify-ui/"
        self.assertEqual(verify.target(base + "zh/", base), "zh/index.html")
        self.assertEqual(verify.target(base + "literal%2520name/", base), "literal%20name/index.html")
        self.assertIsNone(verify.target("https://other.test/rustify-ui/", base))
        for url in ["https://example.test/outside/", base + "../secret", base + "%2Fsecret", base + "%5Csecret"]:
            with self.assertRaises(ValueError):
                verify.target(url, base)

    def test_html_template_ids_do_not_prove_a_visible_anchor(self):
        document = verify.Document('<html lang="en"><h1 id="real">Title</h1><template><h2 id="hidden">Hidden</h2></template></html>')
        self.assertEqual(document.ids, {"real"})


if __name__ == "__main__":
    unittest.main()
