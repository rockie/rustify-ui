// Web API transport and CSSOM ownership; compiler and commit authority live in Rust.
export class UtilityHost {
  constructor(root, base) {
    if (root.hasAttribute('data-rustify-style-owner')) throw JSON.stringify({ kind: 'protocol', message: 'utility root already owned' });
    this.owner = crypto.randomUUID();
    this.root = root;
    this.base = new URL(base || './', document.baseURI);
    this.pending = new Map();
    this.requests = new AbortController();
    this.closed = false;
    this.sheet = null;
    root.setAttribute('data-rustify-style-owner', this.owner);
    try { this.start(); }
    catch (error) {
      root.removeAttribute('data-rustify-style-owner');
      throw JSON.stringify({ kind: 'worker', message: String(error) });
    }
  }
  start() {
    this.worker = new Worker(new URL('utility-worker.js', this.base), { type: 'module' });
    this.worker.onmessage = ({ data }) => {
      const waiting = this.pending.get(data.request_id);
      if (!waiting) return;
      const header = waiting.header;
      if (data.runtime_id !== header.runtime_id || data.epoch !== header.epoch || data.protocol_version !== header.protocol_version) {
        this.fail({ kind: 'protocol', message: 'utility response identity mismatch' });
        return;
      }
      clearTimeout(waiting.timer);
      this.pending.delete(data.request_id);
      if (data.operation === 'error') waiting.reject(JSON.stringify(data.payload));
      else waiting.resolve(JSON.stringify(data));
    };
    this.worker.onerror = event => {
      event.preventDefault();
      this.fail({ kind: 'worker', message: event.message || 'utility Worker failed' });
    };
    this.worker.onmessageerror = () => this.fail({ kind: 'protocol', message: 'utility response could not be decoded' });
  }
  fail(error) {
    this.worker.terminate();
    for (const waiting of this.pending.values()) {
      clearTimeout(waiting.timer);
      waiting.reject(JSON.stringify(error));
    }
    this.pending.clear();
  }
  async request(serialized, timeout) {
    if (this.closed) throw JSON.stringify({ kind: 'disposed' });
    const envelope = JSON.parse(serialized);
    const deadline = performance.now() + timeout;
    if (envelope.operation === 'init') {
      let response;
      try { response = await fetch(new URL('build-manifest.json', this.base), { signal: AbortSignal.any([AbortSignal.timeout(timeout), this.requests.signal]) }); }
      catch (error) { throw JSON.stringify(error.name === 'TimeoutError' ? { kind: 'timeout' } : { kind: 'worker', message: String(error) }); }
      if (!response.ok) throw JSON.stringify({ kind: 'worker', message: `utility manifest: ${response.status}` });
      const manifest = await response.json();
      if (this.closed) throw JSON.stringify({ kind: 'disposed' });
      envelope.payload.build_id = manifest.build_id;
    }
    return new Promise((resolve, reject) => {
      const timer = setTimeout(() => this.fail({ kind: 'timeout' }), Math.max(0, deadline - performance.now()));
      this.pending.set(envelope.request_id, { header: envelope, timer, resolve, reject });
      try { this.worker.postMessage(envelope); }
      catch (error) { this.fail({ kind: 'worker', message: String(error) }); }
    });
  }
  install(cssJson) {
    if (this.closed) throw JSON.stringify({ kind: 'disposed' });
    const staged = new CSSStyleSheet();
    try { staged.replaceSync(JSON.parse(cssJson)); }
    catch (error) { throw JSON.stringify({ kind: 'install', message: String(error) }); }
    const sheets = [...document.adoptedStyleSheets];
    const index = sheets.indexOf(this.sheet);
    if (index < 0) sheets.push(staged);
    else sheets[index] = staged;
    document.adoptedStyleSheets = sheets;
    this.previousSheet = this.sheet;
    this.sheet = staged;
  }
  commitSheet() { this.previousSheet = null; }
  rollback() {
    if (this.closed) throw JSON.stringify({ kind: 'disposed' });
    const sheets = [...document.adoptedStyleSheets];
    const index = sheets.indexOf(this.sheet);
    if (index >= 0) {
      if (this.previousSheet) sheets[index] = this.previousSheet;
      else sheets.splice(index, 1);
    }
    document.adoptedStyleSheets = sheets;
    this.sheet = this.previousSheet;
    this.previousSheet = null;
  }
  restart() {
    if (this.closed) throw JSON.stringify({ kind: 'disposed' });
    this.fail({ kind: 'superseded' });
    this.requests.abort();
    this.requests = new AbortController();
    this.start();
  }
  classes(byteLimit) {
    const classes = new Set();
    let bytes = 2;
    const root = this.root;
    const elements = (function* () { yield root; yield* root.querySelectorAll('[class]'); })();
    for (const element of elements) {
      if (element.closest('[data-rustify-style-owner]') !== this.root) continue;
      for (const token of element.classList) {
        if (classes.has(token)) continue;
        bytes += new TextEncoder().encode(JSON.stringify(token)).length + 1;
        if (bytes > byteLimit) throw JSON.stringify({ kind: 'compile', error: { kind: 'limit', resource: 'queued_bytes', limit: byteLimit } });
        classes.add(token);
      }
    }
    return JSON.stringify([...classes]);
  }
  observe(callback, byteLimit) {
    const collect = elements => {
      const classes = new Set();
      let bytes = 2;
      for (const element of elements) {
        if (element.closest('[data-rustify-style-owner]') !== this.root) continue;
        for (const token of element.classList) {
          if (classes.has(token)) continue;
          bytes += new TextEncoder().encode(JSON.stringify(token)).length + 1;
          if (bytes > byteLimit) {
            callback(JSON.stringify({ error: { kind: 'compile', error: { kind: 'limit', resource: 'queued_bytes', limit: byteLimit } } }));
            return;
          }
          classes.add(token);
        }
      }
      if (classes.size) callback(JSON.stringify({ classes: [...classes] }));
    };
    this.observer = new MutationObserver(records => {
      function* touched() {
        for (const record of records) {
          if (record.type === 'attributes') yield record.target;
          else for (const node of record.addedNodes) {
            if (!(node instanceof Element)) continue;
            yield node;
            yield* node.querySelectorAll('[class]');
          }
        }
      }
      collect(touched());
    });
    this.observer.observe(this.root, { subtree: true, childList: true, attributes: true, attributeFilter: ['class'] });
    const root = this.root;
    collect((function* () { yield root; yield* root.querySelectorAll('[class]'); })());
  }
  cleanup() {
    if (this.closed) return;
    this.closed = true;
    this.requests.abort();
    this.observer?.disconnect();
    this.previousSheet = null;
    this.fail({ kind: 'disposed' });
    document.adoptedStyleSheets = document.adoptedStyleSheets.filter(sheet => sheet !== this.sheet);
    if (this.root.getAttribute('data-rustify-style-owner') === this.owner) this.root.removeAttribute('data-rustify-style-owner');
    this.sheet = null;
  }
  release() { return this.cleanup.bind(this); }
}
