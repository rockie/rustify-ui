"""Materialize Playwright's embedded attachments without changing the report."""
import base64
import json
from pathlib import Path

root = Path(__file__).resolve().parent
data = json.loads((root / "browser.json").read_text())
cases = []


def visit(suites):
    for suite in suites:
        for spec in suite.get("specs", []):
            for test in spec["tests"]:
                for result in test["results"]:
                    directory = root / "browser" / ("t%02d" % len(cases))
                    directory.mkdir(parents=True, exist_ok=True)
                    row = {"title": spec["title"], "status": result["status"], "attachments": []}
                    for i, attachment in enumerate(result.get("attachments", [])):
                        name = Path(attachment["name"]).name
                        target = directory / ("%02d-" % i + name)
                        assert "body" in attachment, attachment.keys()
                        target.write_bytes(base64.b64decode(attachment["body"]))
                        row["attachments"].append(str(target.relative_to(root)))
                    cases.append(row)
        visit(suite.get("suites", []))


visit(data["suites"])
audits = []
for path in (root / "browser").rglob("*browser-audit.json"):
    audit = json.loads(path.read_text())
    audits.append({"path": str(path.relative_to(root)), "console": audit["console"], "pageErrors": audit["pageErrors"],
                   "csp": audit["csp"], "failures": audit["failures"], "responses_400_plus": [r for r in audit["responses"] if r["status"] >= 400]})
summary = {"stats": data["stats"], "errors": data["errors"], "cases": cases, "audits": audits}
(root / "browser-summary.json").write_text(json.dumps(summary, ensure_ascii=False, indent=2) + "\n")
print(json.dumps({"stats": data["stats"], "attachments": sum(len(c["attachments"]) for c in cases), "audits": audits}, ensure_ascii=False))
