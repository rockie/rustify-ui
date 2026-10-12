import json
import urllib.request
from pathlib import Path

url = json.loads(Path('target/docs-ssg/gaps/safari-bridge-state.json').read_text())['url']


def call(name, **arguments):
    request = urllib.request.Request(url, data=json.dumps({'name': name, 'arguments': arguments}).encode(),
                                    headers={'Content-Type': 'application/json'})
    with urllib.request.urlopen(request, timeout=58) as response:
        reply = json.load(response)
    if 'error' in reply or reply['result'].get('isError'):
        raise RuntimeError(reply)
    text = '\n'.join(c.get('text', '') for c in reply['result'].get('content', []))
    try:
        return json.loads(text)
    except json.JSONDecodeError:
        return text


def show(name, **arguments):
    value = call(name, **arguments)
    print(name, json.dumps(value, ensure_ascii=False), flush=True)
    return value
