import json
import sys
import time
from pathlib import Path
from safari_client import call

evidence = Path('docs/validation/docs-ssg-p1-gaps/m2/safari')
sites = json.loads(Path('target/docs-ssg/gaps/safari-preview.json').read_text())
results = []


def js(expression):
    try:
        return call('evaluate_javascript', expression=expression)
    except RuntimeError as error:
        if 'Callback was not called before the unload event.' not in str(error):
            raise


def goto(url):
    js('location.assign(' + json.dumps(url) + ');return true;')
    for _ in range(40):
        value = js('return {url:location.href,ready:document.readyState};')
        if value['url'] == url and value['ready'] == 'complete':
            time.sleep(.3)
            return
        time.sleep(.1)
    raise RuntimeError(value)


def save(label, value):
    results.append({'check': label, 'result': value})
    (evidence / 'full-checks.json').write_text(json.dumps(results, ensure_ascii=False, indent=2) + '\n')
    print(label, json.dumps(value, ensure_ascii=False), flush=True)


def measure():
    return js("return {url:location.href,width:innerWidth,theme:document.documentElement.dataset.theme,h1:document.querySelectorAll('h1').length,header:document.querySelectorAll('body>header').length,main:document.querySelectorAll('main').length,footer:document.querySelectorAll('body>footer').length,current:document.querySelector('.rd-sidebar-wide [aria-current=page]')?.textContent,headerCurrent:[...document.querySelectorAll('header nav [aria-current]')].map(e=>e.textContent),overflow:document.documentElement.scrollWidth-innerWidth,modeHidden:document.querySelector('[data-rd-mode-switch]').hidden};")


for site in sites:
    if not site['name'].startswith('full-'):
        continue
    call('set_viewport_size', width=1280, height=800)
    goto(site['url'])
    js("document.querySelector('.rd-actions a').click();return true;")
    time.sleep(.35)
    value = measure()
    assert value['url'] == site['url'] + 'guide/' and value['h1'] == value['header'] == value['main'] == value['footer'] == 1, value
    save(site['name'] + '/home-action', value)
    js("document.querySelector('[data-rd-mode=dark]').click();document.querySelector('.rd-pager [rel=next]').click();return true;")
    time.sleep(.35)
    value = measure()
    assert value['url'] == site['url'] + 'guide/setup/' and value['theme'] == 'dark' and value['current'] == 'Setup' and value['headerCurrent'] == ['Guide'] and value['overflow'] <= 0, value
    save(site['name'] + '/setup-dark', value)
    js("document.querySelector('.rd-toc-wide a[href=\"#check-the-version\"]').click();return true;")
    time.sleep(.5)
    value = js("return {hash:location.hash,top:document.getElementById('check-the-version').getBoundingClientRect().top,headerBottom:document.querySelector('body>header').getBoundingClientRect().bottom};")
    assert value['hash'] == '#check-the-version' and value['top'] >= value['headerBottom'], value
    save(site['name'] + '/ordinary-anchor', value)
    call('set_viewport_size', width=375, height=740)
    value = measure()
    assert value['width'] == 375 and value['overflow'] <= 0, value
    save(site['name'] + '/narrow', value)
    call('screenshot', full_page=True, savePath=str((evidence / (site['name'] + '-loopback-setup-375.png')).absolute()))
    js("document.querySelector('[data-rd-mode=light]').click();document.querySelector('.rd-pager [rel=next]').click();return true;")
    time.sleep(.35)
    js("document.querySelector('.rd-toc-narrow summary').click();document.querySelector('.rd-toc-narrow a').click();return true;")
    time.sleep(.5)
    value = js("return {url:location.href,lang:document.documentElement.lang,theme:document.documentElement.dataset.theme,hash:decodeURIComponent(location.hash),top:document.getElementById('数据-api').getBoundingClientRect().top,headerBottom:document.querySelector('body>header').getBoundingClientRect().bottom,overflow:document.documentElement.scrollWidth-innerWidth};")
    assert value['lang'] == 'zh-CN' and value['theme'] == 'light' and value['hash'] == '#数据-api' and value['top'] >= value['headerBottom'] and value['overflow'] <= 0, value
    save(site['name'] + '/Chinese-anchor-light', value)
    goto(site['url'] + 'a/b/c/missing/')
    value = measure()
    assert value['h1'] == value['header'] == value['main'] == value['footer'] == 1 and value['overflow'] <= 0, value
    save(site['name'] + '/404', value)
    network = call('list_network_requests', clear=False)
    bad = [r for r in network['requests'] if r.get('status', 0) >= 400 or r.get('status', 0) == 0]
    save(site['name'] + '/network', {'total': network['count'], 'nonSuccess': bad})
    assert all(r['url'].endswith(('/a/b/c/missing/', '/favicon.ico')) for r in bad), bad
    js("document.querySelector('main a[href]').click();return true;")
    time.sleep(.35)
    assert js('return location.href;') == site['url']
    save(site['name'] + '/404-return', {'url': js('return location.href;')})
save('console', call('browser_console_messages', clear=False, level_filter=['error', 'warn']))
save('motion', js("return {reduce:matchMedia('(prefers-reduced-motion: reduce)').matches,scrollBehavior:getComputedStyle(document.documentElement).scrollBehavior};"))
print('Full site checks completed', flush=True)
