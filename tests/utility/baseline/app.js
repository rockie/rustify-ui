const candidates = ["w-[137px]", "bg-[#123456]", "hover:p-7"];

window.utilityProbe = {
  async compile() {
    const manifestResponse = await fetch('./build-manifest.json');
    if (!manifestResponse.ok) throw new Error(`manifest: ${manifestResponse.status}`);
    const manifest = await manifestResponse.json();
    const worker = new Worker(new URL('./utility-worker.js', import.meta.url), { type: 'module' });
    const runtime_id = crypto.randomUUID();
    let request_id = 0;
    const request = (operation, payload) => new Promise((resolve, reject) => {
      const id = ++request_id;
      const timer = setTimeout(() => { worker.terminate(); reject(new Error('compiler Worker deadline')); }, 10000);
      const cleanup = () => { clearTimeout(timer); worker.removeEventListener('message', receive); worker.removeEventListener('error', fail); };
      const fail = event => { cleanup(); reject(new Error(event.message)); };
      const receive = ({ data }) => {
        if (data.protocol_version !== 1 || data.runtime_id !== runtime_id || data.epoch !== 1 || data.request_id !== id) return;
        cleanup();
        data.operation === 'error' ? reject(new Error(JSON.stringify(data.payload))) : resolve(data.payload);
      };
      worker.addEventListener('message', receive);
      worker.addEventListener('error', fail);
      worker.postMessage({ protocol_version: 1, runtime_id, epoch: 1, request_id: id, operation, payload });
    });
    const original = [...document.adoptedStyleSheets];
    const root = document.querySelector('[data-utility-probe]');
    const boundary = document.querySelector('[data-probe="boundary"]');
    const sample = document.querySelector('#sample');
    const beforeWidth = getComputedStyle(sample).width;
    const violations = [];
    const record = event => violations.push(event.violatedDirective);
    document.addEventListener('securitypolicyviolation', record);
    root.setAttribute('data-rustify-style-owner', runtime_id);
    boundary.setAttribute('data-rustify-style-owner', 'nested');
    try {
      const config = JSON.stringify({ input: { css: '@tailwind utilities;', entry: 'memory:input.css' },
        assets: [], owner: runtime_id,
        limits: { input_bytes: 65536, asset_bytes: 65536, candidate_count: 1024,
          ast_depth: 32, css_bytes: 262144, queued_bytes: 65536, cache_bytes: 1048576 } });
      const ready = await request('init', { build_id: manifest.build_id, config });
      const prepared = await request('add_candidates', JSON.stringify(candidates.slice(0, 2)));
      const uncommitted = await request('report', null);
      const sheet = new CSSStyleSheet();
      sheet.replaceSync(prepared.report.css);
      for (const id of ['sample', 'outside', 'boundary']) document.querySelector(`#${id}`).className = candidates.slice(0, 2).join(' ');
      document.adoptedStyleSheets = [...original, sheet];
      const installed = { width: getComputedStyle(sample).width, background: getComputedStyle(sample).backgroundColor };
      const committed = await request('ack', { token: prepared.token });
      const repeated = await request('add_candidates', JSON.stringify(candidates.slice(0, 2)));
      await request('ack', { token: repeated.token });
      const outsideWidth = getComputedStyle(document.querySelector('#outside')).width;
      const boundaryWidth = getComputedStyle(document.querySelector('#boundary')).width;
      document.adoptedStyleSheets = original;
      const cleanedWidth = getComputedStyle(sample).width;
      await request('dispose', null);
      return { ready, prepared: prepared.report, uncommittedRevision: uncommitted.revision,
        committed, repeated: repeated.report, installed, outsideWidth, boundaryWidth, beforeWidth, cleanedWidth, violations };
    } finally {
      worker.terminate();
      document.adoptedStyleSheets = original;
      root.removeAttribute('data-rustify-style-owner');
      boundary.removeAttribute('data-rustify-style-owner');
      document.removeEventListener('securitypolicyviolation', record);
    }
  },
  async official() {
    const sheet = new CSSStyleSheet();
    const original = [...document.adoptedStyleSheets];
    const violations = [];
    const record = event => violations.push(event.violatedDirective);
    document.addEventListener("securitypolicyviolation", record);
    const sample = document.querySelector("#sample");
    const response = await fetch(new URL("./official.css", import.meta.url));
    if (!response.ok) throw new Error(`official.css: ${response.status}`);
    const css = await response.text();
    try {
      const start = performance.now();
      sheet.replaceSync(css);
      const stagingMs = performance.now() - start;
      sample.className = 'w-[137px] bg-[#123456] ring-[3px] before:content-["probe"]';
      document.querySelector("#outside").className = sample.className;
      const installStart = performance.now();
      document.adoptedStyleSheets = [...original, sheet];
      const installMs = performance.now() - installStart;
      const style = getComputedStyle(sample);
      const report = { width: style.width, background: style.backgroundColor, boxShadow: style.boxShadow,
        beforeContent: getComputedStyle(sample, "::before").content,
        outsideWidth: getComputedStyle(document.querySelector("#outside")).width,
        stagingMs, installMs, cssBytes: new TextEncoder().encode(css).length };
      document.adoptedStyleSheets = original;
      await new Promise(resolve => requestAnimationFrame(resolve));
      return { ...report, cleanedWidth: getComputedStyle(sample).width, violations };
    } finally {
      document.adoptedStyleSheets = original;
      document.removeEventListener("securitypolicyviolation", record);
    }
  },
  async scanner() {
    const facade = await import("./extractor/style.js");
    await facade.default({ module_or_path: new URL("./extractor/style_bg.wasm", import.meta.url) });
    const fixtures = await fetch(new URL("./scanner-fixtures.json", import.meta.url)).then(response => {
      if (!response.ok) throw new Error(`scanner fixtures: ${response.status}`);
      return response.json();
    });
    return {
      actual: fixtures.map(({ content, extension }) => JSON.parse(facade.scan_content(content, extension))),
      expected: fixtures.map(({ candidates, variables }) => ({ candidates, variables })),
      sharedMemory: typeof SharedArrayBuffer !== "undefined",
    };
  },
  worker() {
    const worker = new Worker(new URL("./worker.js", import.meta.url), { type: "module" });
    return new Promise((resolve, reject) => {
      const timer = setTimeout(() => { worker.terminate(); reject(new Error("Worker probe timed out")); }, 10000);
      const finish = () => { clearTimeout(timer); worker.terminate(); };
      worker.onerror = event => { finish(); reject(new Error(event.message)); };
      worker.onmessage = event => { finish(); event.data.error ? reject(new Error(event.data.error)) : resolve(event.data); };
      worker.postMessage({ requestId: 1, candidates });
    });
  },
  async css() {
    const violations = [];
    const record = event => violations.push(event.violatedDirective);
    document.addEventListener("securitypolicyviolation", record);
    const sample = document.querySelector("#sample");
    const snapshot = element => ({ width: getComputedStyle(element).width, background: getComputedStyle(element).backgroundColor });
    const original = [...document.adoptedStyleSheets];
    const before = snapshot(sample);
    const sheet = new CSSStyleSheet();
    const css = await fetch(new URL("./probe.css", import.meta.url)).then(response => {
      if (!response.ok) throw new Error(`probe.css: ${response.status}`);
      return response.text();
    });
    try {
      sheet.replaceSync(css);
      for (const id of ["sample", "outside", "boundary"]) {
        document.querySelector(`#${id}`).className = "w-[137px] bg-[#123456]";
      }
      const staged = snapshot(sample);
      document.adoptedStyleSheets = [...original, sheet];
      const installed = { ...snapshot(sample), nestedColor: getComputedStyle(sample.firstElementChild).color,
        propertyWidth: getComputedStyle(document.querySelector("#property")).width };
      const outside = snapshot(document.querySelector("#outside"));
      const boundary = snapshot(document.querySelector("#boundary"));
      document.adoptedStyleSheets = original;
      await new Promise(resolve => requestAnimationFrame(resolve));
      return { before, staged, installed, outside, boundary, cleaned: snapshot(sample),
        sheetCountBefore: original.length, sheetCountAfter: document.adoptedStyleSheets.length, violations };
    } finally {
      document.adoptedStyleSheets = original;
      document.removeEventListener("securitypolicyviolation", record);
    }
  },
};

// The cn JS facade crosses into WASM from the page realm under the same strict
// CSP as the product: every fixed runtime case, with live JS callbacks.
window.utilityProbe.cn = async () => {
  const violations = [];
  const record = event => violations.push(event.violatedDirective);
  document.addEventListener('securitypolicyviolation', record);
  try {
    const wasm = await import('./cn/style.js');
    const exports = await wasm.default({ module_or_path: new URL('./cn/style_bg.wasm', import.meta.url) });
    const { runFixedCases } = await import('./cn/runtime-check.mjs');
    const response = await fetch('./cn/runtime-fixtures.json');
    if (!response.ok) throw new Error(`fixtures: ${response.status}`);
    const fixtures = (await response.json()).cases;
    const limits = { maxDepth: 128, maxNodes: 100000, maxUtf16Units: 1000000, maxTokens: 100000, cacheBytes: 10000000 };
    const started = performance.now();
    const result = runFixedCases(wasm, fixtures, limits);
    const sentinel = { kind: 'validator' };
    const session = new wasm.CnSession({ extend: { classGroups: { probe: [{ probe: [() => { throw sentinel; }] }] } } }, limits);
    let identity = false;
    try { session.merge(['probe-a']); } catch (error) { identity = error === sentinel; }
    session.free();
    return { ...result, durationMs: performance.now() - started, callbackExceptionIdentity: identity,
      sharedMemory: typeof SharedArrayBuffer !== 'undefined' && exports.memory.buffer instanceof SharedArrayBuffer, violations };
  } finally {
    document.removeEventListener('securitypolicyviolation', record);
  }
};
