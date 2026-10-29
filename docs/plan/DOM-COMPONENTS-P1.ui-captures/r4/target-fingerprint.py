# Manual target fingerprint for the review-ui loop (no Phoenix engine installed).
# Hashes the "## 单元" and "## 规范要点" sections (LF) plus the local files they
# link directly, sorted by repo-relative path, each part length-prefixed.
import hashlib, re, struct, sys, pathlib
target = pathlib.Path(sys.argv[1]); root = pathlib.Path(sys.argv[2])
text = target.read_text(encoding="utf-8").replace("\r\n", "\n")
def section(name):
    m = re.search(r"^## " + re.escape(name) + r"\n(.*?)(?=^## |\Z)", text, re.S | re.M)
    assert m, name
    return m.group(1)
h = hashlib.sha256(b"manual-sha256-normative-sections-v1\0")
def part(b): h.update(struct.pack(">Q", len(b))); h.update(b)
files = set()
for name in ("单元", "规范要点"):
    body = section(name); part(body.encode())
    for link in re.findall(r"\]\(([^)#]+)\)", body):
        p = (target.parent / link).resolve()
        if p.is_file(): files.add(p.relative_to(root.resolve()).as_posix())
for rel in sorted(files):
    digest = hashlib.sha256((root / rel).read_bytes()).hexdigest()
    part(rel.encode()); part(digest.encode()); print(digest, rel)
print("fingerprint", h.hexdigest())
