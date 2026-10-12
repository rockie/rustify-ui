import json
import os
import select
import subprocess
import time
from datetime import datetime, timezone
from http.server import BaseHTTPRequestHandler, HTTPServer
from pathlib import Path

root = Path.cwd()
evidence = root / 'docs/validation/docs-ssg-p1-gaps/m2/safari'
driver = subprocess.Popen(['/usr/bin/safaridriver', '--mcp'], stdin=subprocess.PIPE,
                          stdout=subprocess.PIPE, stderr=subprocess.PIPE)
pending = b''
sequence = 0
log = (evidence / 'protocol.jsonl').open('x')


def record(value):
    log.write(json.dumps({'at': datetime.now(timezone.utc).isoformat(), **value}) + '\n')
    log.flush()


def rpc(method, params):
    global pending, sequence
    sequence += 1
    request = {'jsonrpc': '2.0', 'id': sequence, 'method': method, 'params': params}
    record({'request': request})
    driver.stdin.write((json.dumps(request) + '\n').encode())
    driver.stdin.flush()
    deadline = time.monotonic() + 55
    while time.monotonic() < deadline:
        while b'\n' in pending:
            line, pending = pending.split(b'\n', 1)
            reply = json.loads(line)
            record({'reply': reply})
            if reply.get('id') == sequence:
                return reply
        ready, _, _ = select.select([driver.stdout, driver.stderr], [], [], .5)
        for stream in ready:
            data = os.read(stream.fileno(), 65536)
            if not data:
                raise RuntimeError('Safari MCP closed its stream')
            if stream is driver.stdout:
                pending += data
            else:
                record({'stderr': data.decode(errors='replace')})
    raise TimeoutError(method)


rpc('initialize', {'protocolVersion': '2024-11-05', 'capabilities': {},
                   'clientInfo': {'name': 'rustify-docs-validation', 'version': '1'}})
driver.stdin.write(b'{"jsonrpc":"2.0","method":"notifications/initialized"}\n')
driver.stdin.flush()


class Handler(BaseHTTPRequestHandler):
    def log_message(self, *args):
        pass

    def do_POST(self):
        try:
            args = json.loads(self.rfile.read(int(self.headers['Content-Length'])))
            reply = rpc('tools/call', args)
        except Exception as error:
            reply = {'error': str(error)}
        body = json.dumps(reply).encode()
        self.send_response(200)
        self.send_header('Content-Type', 'application/json')
        self.send_header('Content-Length', str(len(body)))
        self.end_headers()
        self.wfile.write(body)


server = HTTPServer(('127.0.0.1', 0), Handler)
print(json.dumps({'url': 'http://127.0.0.1:' + str(server.server_port),
                  'pid': os.getpid()}), flush=True)
try:
    server.serve_forever()
finally:
    driver.terminate()
    driver.wait(timeout=5)
    log.close()
