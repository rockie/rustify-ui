import os, sys, json, hashlib, tempfile, time, subprocess
from pathlib import Path
out=Path(__file__).resolve().parent
repo=out.parents[3]
tools=Path(tempfile.mkdtemp(prefix="rustify-gaps2-r2-tools-"))
(tools/"python3").symlink_to(sys.executable)
env=dict(os.environ, PATH=str(tools)+os.pathsep+os.environ["PATH"], GIT_CONFIG_COUNT="1", GIT_CONFIG_KEY_0="url.file://"+str(repo)+"/.insteadOf", GIT_CONFIG_VALUE_0="https://github.com/rockie/rustify-ui", CARGO_NET_GIT_FETCH_WITH_CLI="true", RUSTIFY_DOCS_CARGO="mbx")
installed=json.loads((repo/"docs/validation/docs-ssg-p2-gaps-2/apply-r1/installed-after-browser.json").read_text())
entry=Path(installed["entry"])
info=subprocess.check_output([str(entry),"--build-info"],text=True)
sha=hashlib.sha256(entry.read_bytes()).hexdigest()
assert sha==installed["sha256"] and info==installed["build_info"]
(out/"installed-before.json").write_text(json.dumps(dict(entry=str(entry),sha256=sha,build_info=info),indent=2)+"\n")
commands=[("fmt",["cargo","fmt","--all","--","--check"]),("cli-diagnostic-text",["mbx","test","--locked","-p","rustify-docs-cli","--test","diagnostic_text"])]
results=[]
for name,command in commands:
    start=time.monotonic()
    with (out/(name+".log")).open("w") as f:
        r=subprocess.run(command,cwd=repo,env=env,stdout=f,stderr=subprocess.STDOUT)
    item=dict(name=name,command=command,exit=r.returncode,seconds=round(time.monotonic()-start,2),target=subprocess.check_output(["git","rev-parse","HEAD"],cwd=repo,text=True).strip())
    results.append(item)
    (out/"checks.json").write_text(json.dumps(results,indent=2)+"\n")
    print(json.dumps(item),flush=True)
    if r.returncode: sys.exit(r.returncode)
env["RUSTIFY_DOCS_BIN"]=str(entry)
command=["npx","--no-install","playwright","test","-c","playwright.docs.config.ts","--project=docs-dev","dev-components.spec.ts","--grep","frontmatter language preserves|plain author paths|real DOM reader","--workers=1","--reporter=json","--output="+str(out/"browser-artifacts")]
env["RUSTIFY_ARTIFACT_DIR"]=str(out/"browser")
start=time.monotonic()
with (out/"browser-results.json").open("w") as output,(out/"browser.stderr").open("w") as err:
    r=subprocess.run(command,cwd=repo,env=env,stdout=output,stderr=err)
item=dict(name="browser",command=command,exit=r.returncode,seconds=round(time.monotonic()-start,2),entry=str(entry),sha256=sha)
(out/"browser-command.json").write_text(json.dumps(item,indent=2)+"\n")
print(json.dumps(item),flush=True)
(out/"installed-after.json").write_text(json.dumps(dict(entry=str(entry),sha256=hashlib.sha256(entry.read_bytes()).hexdigest(),build_info=subprocess.check_output([str(entry),"--build-info"],text=True)),indent=2)+"\n")
sys.exit(r.returncode)
