import json
import sys
import time
from pathlib import Path
from urllib.parse import quote
from safari_client import call

script = sys.argv[1] if len(sys.argv) > 1 else 'on'
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
    js('location.assign(' + json.dumps(url) + '); return true;')
    for _ in range(40):
        value = js('return {url:location.href,ready:document.readyState};')
        if value['url'] == url and value['ready'] == 'complete':
            time.sleep(.25)
            return
        time.sleep(.1)
    raise RuntimeError(value)


def measure():
    return js("return {url:location.href,width:innerWidth,h1:[...document.querySelectorAll('h1')].map(e=>({id:e.id,text:e.textContent,top:e.getBoundingClientRect().top})),headerBottom:document.querySelector('body>header').getBoundingClientRect().bottom,header:document.querySelectorAll('body>header').length,main:document.querySelectorAll('main').length,footer:document.querySelectorAll('body>footer').length,nav:document.querySelectorAll('nav').length,overflow:document.documentElement.scrollWidth-innerWidth,modeHidden:document.querySelector('[data-rd-mode-switch]').hidden,article:document.querySelector('article').textContent.replace(/\\s+/g,' ').trim(),pageLang:document.documentElement.lang,actionLang:document.querySelector('.rd-actions a')?.lang,footerHref:document.querySelector('body>footer a').getAttribute('href')};")


def save(label, value):
    results.append({'check': label, 'result': value})
    (evidence / ('home-checks-script-' + script + '.json')).write_text(json.dumps(results, ensure_ascii=False, indent=2) + '\n')
    print(label, json.dumps(value, ensure_ascii=False), flush=True)


for site in sites:
    if not site['name'].startswith('home-'):
        continue
    base = '/' if site['name'] == 'home-root' else '/rustify-ui/'
    for width in (1280, 375):
        call('set_viewport_size', width=width, height=740)
        for route, anchor in (('', 'home'), ('explicit/', '入口'), ('raw/', '</h1>')):
            goto(site['url'] + route + '#' + quote(anchor, safe=''))
            value = measure()
            assert value['width'] == width and value['overflow'] <= 0, value
            assert value['header'] == value['main'] == value['footer'] == 1 and value['nav'] > 0, value
            assert len(value['h1']) == 1 and value['h1'][0]['id'] == anchor, value
            assert value['h1'][0]['top'] >= value['headerBottom'], value
            assert value['modeHidden'] is (script == 'off'), value
            if route == 'raw/':
                assert value['article'] == 'Same Automatic' and value['actionLang'] == 'fr', value
            value['manifest_sha256'] = site['manifest_sha256']
            save(site['name'] + '/' + (route or 'automatic/') + str(width), value)
            if width == 375 and route == 'raw/':
                call('screenshot', full_page=True, savePath=str((evidence / (site['name'] + '-raw-' + script + '-375.png')).absolute()))
            js("document.querySelector('article a').click(); return true;")
            time.sleep(.25)
            same = measure()
            assert same['h1'][0]['top'] >= same['headerBottom'], same
            assert js('return document.querySelectorAll("h1").length;') == 1
            js("document.querySelector('article a:last-child').click(); return true;")
            time.sleep(.3)
            cross = measure()
            assert len(cross['h1']) == 1 and cross['h1'][0]['top'] >= cross['headerBottom'], cross
            save(site['name'] + '/' + (route or 'automatic/') + str(width) + '/links', {'same': same['url'], 'cross': cross['url'], 'crossId': cross['h1'][0]['id']})
        goto(site['url'] + 'empty/')
        value = measure()
        assert value['header'] == value['main'] == value['footer'] == 1 and value['nav'] > 0 and value['footerHref'] == base, value
        assert value['width'] == width and value['overflow'] <= 0 and value['modeHidden'] is (script == 'off'), value
        save(site['name'] + '/empty/' + str(width), value)
        if width == 375:
            call('screenshot', full_page=True, savePath=str((evidence / (site['name'] + '-empty-' + script + '-375.png')).absolute()))
        js("document.querySelector('body>footer a').click(); return true;")
        time.sleep(.3)
        assert js('return location.href;') == site['url']
        save(site['name'] + '/footer/' + str(width), {'url': js('return location.href;')})
print('Home boundary checks completed with script ' + script, flush=True)
