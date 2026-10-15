"""Installed CLI: one new components project, CLI/SSE/terminal/browser equality.

Run from the repository root in the plan's verification shell. Pass the exact
installed --cli, --c and --r; --work must be an empty temporary directory.
"""
import argparse, hashlib, http.client, json, os, signal, subprocess, threading, time
from pathlib import Path
from urllib.parse import urlsplit

parser = argparse.ArgumentParser()
for name in ['cli', 'c', 'r', 'work', 'out', 'browser']:
    parser.add_argument('--' + name, required=True)
a = parser.parse_args()
work = Path(a.work); assert work.is_dir() and not any(work.iterdir())
out = Path(a.out); out.mkdir(parents=True, exist_ok=True)
site = work / 'author-site'
report = {'cli': a.cli, 'C': a.c, 'R': a.r, 'steps': [], 'cases': []}

def save(): (out/'acceptance.json').write_text(json.dumps(report, indent=2, ensure_ascii=False)+'\n')
def run(name, args, expected=0, cwd=None):
    result = subprocess.run(args, cwd=cwd or site, capture_output=True, text=True)
    (out/(name+'.stdout')).write_text(result.stdout); (out/(name+'.stderr')).write_text(result.stderr)
    report['steps'].append({'name': name, 'command': args, 'exit': result.returncode}); save()
    assert result.returncode == expected, (name, result.returncode, result.stdout[-4000:], result.stderr[-4000:])
    return result

def tree(root): return {str(p.relative_to(root)): hashlib.sha256(p.read_bytes()).hexdigest() for p in sorted(root.rglob('*')) if p.is_file()}
def errors(report): return [d for d in report['diagnostics'] if d['severity'] == 'error']
def warm(name):
    result = subprocess.run([a.cli,'check','--format','json'], cwd=site, capture_output=True, text=True)
    (out/(name+'.stdout')).write_text(result.stdout); (out/(name+'.stderr')).write_text(result.stderr)
    report['steps'].append({'name':name, 'exit':result.returncode}); save()
    if result.returncode:
        assert [d['rule'] for d in errors(json.loads(result.stdout))] == ['rust-generation-changed'], result.stdout
        run(name+'-retry', [a.cli,'check','--format','json'])

info = run('build-info', [a.cli,'--build-info'], cwd=work).stdout
assert info.startswith('rustify-docs 0.1.18\n') and f'source: {a.c}\n' in info and 'panic: unwind\n' in info, info
report['binary_sha256'] = hashlib.sha256(Path(a.cli).read_bytes()).hexdigest()
run('new', [a.cli,'new',str(site),'--template','components'], cwd=work)
pins = [line.strip() for line in (site/'Cargo.toml').read_text().splitlines() if 'github.com/rockie/rustify-ui' in line]
assert len(pins)==3 and all(f'rev = "{a.r}"' in pin for pin in pins),pins
lock=(site/'Cargo.lock').read_bytes(); report['pins']=pins; report['lock_sha256']=hashlib.sha256(lock).hexdigest()
run('fetch',['cargo','fetch','--locked'])
report['graphs']={}
for target,features,extra in [('host','ssr',[]),('wasm','hydrate',['--filter-platform','wasm32-unknown-unknown'])]:
    metadata=json.loads(run(target+'-metadata',['mbx','metadata','--locked','--offline','--format-version','1','--no-default-features','--features',features,*extra]).stdout)
    names={p['name'] for p in metadata['packages']}; assert not any('makepad' in name.lower() for name in names)
    sources={p['source'] for p in metadata['packages'] if p['name'] in ['rustify-docs','rustify-ui','rustify-components']}
    assert sources=={f'git+https://github.com/rockie/rustify-ui?rev={a.r}#{a.r}'}
    report['graphs'][target]={'packages':len(names),'makepad':0,'sources':sorted(sources)}
run('host-check',['mbx','check','--locked','--offline','--no-default-features','--features','ssr','--profile','docs-host','--bin','docs-render'])
run('wasm-check',['mbx','check','--locked','--offline','--no-default-features','--features','hydrate','--profile','docs-wasm','--lib','--target','wasm32-unknown-unknown'])
assert (site/'Cargo.lock').read_bytes()==lock, 'locked metadata/check changed the generated lock'
# The explicit cargo update gate is the pre-C pair-audit on docs-site.
# Keep this renamed author's original lock for every required installed exit.
registry=(site/'src/lib.rs').read_text(); plain='label: String = "Counter".to_string()'; assert registry.count(plain)==1
(site/'src/lib.rs').write_text(registry.replace(plain,'label: String = "a`b".to_string()'))
(site/'content/a`b`c.md').write_text('---\ndescription: Target\n---\n# Target\n')
config=(site/'rustify-docs.toml').read_text(); assert config.count('lang = "en"')==1
warm('check-good')
run('build-good',[a.cli,'build','--format','json']); good=tree(site/'dist'); report['good_dist']=good

dev=subprocess.Popen([a.cli,'dev','--port','0'],cwd=site,stdout=subprocess.PIPE,stderr=subprocess.PIPE,text=True,start_new_session=True)
log=[]; threading.Thread(target=lambda:log.extend(dev.stderr),daemon=True).start()
node=None;states=[];stop_stream=False
try:
    line=dev.stdout.readline();assert line.startswith('dev: '),line
    url=line.split(' ',1)[1].strip();parsed=urlsplit(url);report['dev_url']=url
    def connect():return http.client.HTTPConnection(parsed.hostname, parsed.port, timeout=30)
    def follow():
        connection=connect();connection.request('GET','/__rustify-docs/events?path=/&good=0');response=connection.getresponse()
        try:
            while not stop_stream:
                line=response.readline()
                if not line:break
                if line.startswith(b'data: '):states.append(json.loads(line[6:]))
        except (OSError,TimeoutError):
            if not stop_stream:raise
        finally:connection.close()
    threading.Thread(target=follow,daemon=True).start()
    def until(wanted):
        start=time.monotonic()
        while time.monotonic()-start<900:
            if states and wanted(states[-1]):return states[-1]
            if dev.poll() is not None:raise AssertionError(('dev exited',dev.returncode,''.join(log)[-4000:]))
            time.sleep(.1)
        raise AssertionError(('state timeout',states[-1:],''.join(log)[-4000:]))
    first=until(lambda s:not s['building'] and s['ok'])
    def page_bytes():
        connection=connect();connection.request('GET','/');response=connection.getresponse();assert response.status==200
        data=response.read();connection.close();return data
    good_page=page_bytes(); (out/'last-good.html').write_bytes(good_page)
    browser_errors=(out/'browser.stderr').open('w')
    node=subprocess.Popen(['node',a.browser,url,str(out/'browser')],stdin=subprocess.PIPE,stdout=subprocess.PIPE,stderr=browser_errors,text=True)
    def browser(command=None):
        if command:node.stdin.write(json.dumps(command)+'\n');node.stdin.flush()
        line=node.stdout.readline(); assert line, ('browser exited',node.poll(),(out/'browser.stderr').read_text())
        return json.loads(line)
    assert browser()['ready']
    good_md='---\ntitle: Gaps\ndescription: Refused embeds\n---\n# Gaps\n\n'
    fields='`initial` (`i32`; default expression `2`), `label` (`String`; default expression `` "a`b".to_string() ``)'
    phases=[
      ('tick-unknown',good_md+'```rustify-component\nname: Counter\nprops:\n  un`known: 1\n```\n',config,'unknown-property','component `Counter` has no property `` un`known ``',f'Use one of {fields} for `Counter`; rename or remove `` un`known ``.',{'message':'component Counter has no property un`known','messageCodes':['Counter','un`known'],'suggestion':'Use one of initial (i32; default expression 2), label (String; default expression "a`b".to_string()) for Counter; rename or remove un`known.','suggestionCodes':['initial','i32','2','label','String','"a`b".to_string()','Counter','un`known']}),
      ('tick-typed',good_md+'```rustify-component\nname: Counter\nprops:\n  label: [1]\n```\n',config,'invalid_props','invalid type: sequence, expected a string','Write `label: "text"` for declared type `String` (default expression `` "a`b".to_string() ``). Reported props path: `label`.',{'message':'invalid type: sequence, expected a string','messageCodes':[],'suggestion':'Write label: "text" for declared type String (default expression "a`b".to_string()). Reported props path: label.','suggestionCodes':['label: "text"','String','"a`b".to_string()','label']}),
      ('tick-anchor',good_md+'[Target](a%60b%60c.md#a%60b)\n',config,'broken-anchor','`` content/a`b`c.md `` has no heading `` #a`b ``',"link to an existing heading's id; ids are the heading text in lower case with spaces as `-`",{'message':'content/a`b`c.md has no heading #a`b','messageCodes':['content/a`b`c.md','#a`b'],'suggestion':"link to an existing heading's id; ids are the heading text in lower case with spaces as -",'suggestionCodes':['-']}),
      ('empty-language',good_md,config.replace('lang = "en"','lang = ""'),'config-lang','``  `` is not a language tag','use a tag like `en` or `zh-CN`',{'message':' is not a language tag','messageCodes':[''],'suggestion':'use a tag like en or zh-CN','suggestionCodes':['en','zh-CN']}),
    ]
    previous=first
    for name,markdown,current_config,rule,message,suggestion,display in phases:
        (site/'content/gaps.md').write_text(markdown);(site/'rustify-docs.toml').write_text(current_config)
        checked=json.loads(run(name+'-check',[a.cli,'check','--format','json'],1).stdout)
        built=json.loads(run(name+'-build',[a.cli,'build','--format','json'],1).stdout)
        assert errors(checked)==errors(built),(checked,built)
        diagnostics=errors(checked);assert len(diagnostics)==1,diagnostics
        d=diagnostics[0];assert (d['rule'],d['message'],d['suggestion'])==(rule,message,suggestion),d
        assert d['line']>0 and d['column']>0 and d['file']==('rustify-docs.toml' if rule=='config-lang' else 'content/gaps.md'),d
        terminal=run(name+'-text',[a.cli,'check'],1).stderr
        location=f"  --> {d['file']}:{d['line']}:{d['column']}\n  = help: {suggestion}\n"
        assert message in terminal and location in terminal,terminal
        assert tree(site/'dist')==good
        failed=until(lambda s:s['revision']>previous['revision'] and not s['building'] and not s['ok'] and errors(s)==diagnostics)
        assert failed['good']==first['good'] and not failed['reload'] and page_bytes()==good_page
        deadline=time.monotonic()+10
        while time.monotonic()<deadline and not (message in ''.join(log) and location in ''.join(log)):time.sleep(.05)
        assert message in ''.join(log) and location in ''.join(log)
        result=browser({'action':'show','label':name,'revision':failed['revision'],'rule':rule,'expected':display})
        report['cases'].append({'name':name,'diagnostic':d,'SSE':failed,'browser':result,'dist_unchanged':True,'last_good_bytes_unchanged':True,'terminal_same':True});save();previous=failed
    (site/'content/gaps.md').unlink();(site/'rustify-docs.toml').write_text(config)
    fixed=until(lambda s:s['revision']>previous['revision'] and not s['building'] and s['ok']);assert fixed['errors']==0
    report['recovered']=fixed;report['browser_recovered']=browser({'action':'clear','revision':fixed['revision']})
    assert browser({'action':'stop'})['stopped'];node.stdin.close();assert node.wait(timeout=30)==0
    run('check-fixed',[a.cli,'check','--format','json']);assert tree(site/'dist')==good
    assert (site/'Cargo.lock').read_bytes()==lock
    report['passed']=True;save();print(json.dumps({'passed':True,'C':a.c,'R':a.r,'cases':len(report['cases']),'browser_views':sum(case['browser']['views'] for case in report['cases'])}))
finally:
    stop_stream=True
    if node is not None and node.poll() is None:node.terminate();node.wait(timeout=30)
    if dev.poll() is None:os.killpg(dev.pid,signal.SIGTERM)
    dev.wait(timeout=30)
    (out/'dev.stderr').write_text(''.join(log));(out/'states.json').write_text(json.dumps(states,indent=2)+'\n');save()
