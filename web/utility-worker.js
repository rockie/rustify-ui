// Browser API transport. Parsing, compilation, scoping and session state live in Rust.
const protocolVersion = 1;
let session;
let identity;
let chain = Promise.resolve();

async function verifiedAsset(path, digest, byteLimit) {
  if (!digest) throw new Error(`asset absent from manifest: ${path}`);
  const url = new URL(path, import.meta.url);
  url.searchParams.set('sha256', digest);
  const response = await fetch(url);
  if (!response.ok) throw new Error(`asset ${path}: ${response.status}`);
  let bytes;
  if (byteLimit === undefined) {
    bytes = await response.arrayBuffer();
  } else {
    const reader = response.body.getReader();
    const chunks = [];
    let length = 0;
    while (true) {
      const {done, value} = await reader.read();
      if (done) break;
      length += value.byteLength;
      if (length > byteLimit) {
        await reader.cancel();
        throw JSON.stringify({kind:'compile',error:{kind:'limit',resource:'queued_bytes',limit:byteLimit}});
      }
      chunks.push(value);
    }
    bytes = new Uint8Array(length);
    let offset = 0;
    for (const chunk of chunks) { bytes.set(chunk, offset); offset += chunk.byteLength; }
  }
  const actual = [...new Uint8Array(await crypto.subtle.digest('SHA-256', bytes))]
    .map(byte => byte.toString(16).padStart(2, '0')).join('');
  if (actual !== digest) throw new Error(`asset digest mismatch: ${path}`);
  return { url, bytes };
}

async function staticCandidates(manifest, limits) {
  if (manifest.static_candidates !== 'static-candidates.json') throw new Error('utility static candidate seed absent');
  const seed = await verifiedAsset(manifest.static_candidates, manifest.files_sha256[manifest.static_candidates], limits.queued_bytes);
  const candidates = JSON.parse(new TextDecoder().decode(seed.bytes));
  if (!Array.isArray(candidates) || candidates.some(value => typeof value !== 'string')) throw new Error('invalid utility static candidate seed');
  if (candidates.length > limits.candidate_count) throw JSON.stringify({kind:'compile',error:{kind:'limit',resource:'candidate_count',limit:limits.candidate_count}});
  return candidates;
}

self.onmessage = ({ data }) => {
  chain = chain.then(async () => {
    const header = { protocol_version: data.protocol_version, runtime_id: data.runtime_id,
      epoch: data.epoch, request_id: data.request_id };
    const respond = (operation, payload) => self.postMessage({ ...header, operation, payload });
    try {
      if (data.protocol_version !== protocolVersion || !data.runtime_id ||
          !Number.isSafeInteger(data.epoch) || data.epoch < 1 ||
          !Number.isSafeInteger(data.request_id) || data.request_id < 1) {
        throw new Error('invalid utility protocol envelope');
      }
      if (data.operation === 'init') {
        if (session) throw new Error('utility Worker already initialized');
        const response = await fetch(new URL('./build-manifest.json', import.meta.url));
        if (!response.ok) throw new Error(`utility manifest: ${response.status}`);
        const manifest = await response.json();
        if (manifest.protocol_version !== protocolVersion || manifest.build_id !== data.payload.build_id) {
          throw new Error('utility protocol or build identity mismatch');
        }
        await verifiedAsset('utility-worker.js', manifest.files_sha256['utility-worker.js']);
        const glue = await verifiedAsset('compiler/style.js', manifest.files_sha256['compiler/style.js']);
        const wasm = await verifiedAsset('compiler/style_bg.wasm', manifest.files_sha256['compiler/style_bg.wasm']);
        const facade = await import(glue.url.href);
        const exports = await facade.default({ module_or_path: wasm.bytes });
        const staged = new facade.CompilerSession(data.payload.config);
        const limits = JSON.parse(data.payload.config).limits;
        let candidates;
        try { candidates = await staticCandidates(manifest, limits); }
        catch (error) { staged.free(); throw error; }
        session = staged;
        identity = header;
        respond('ready', { build_id: manifest.build_id, scanner_enabled: 'scan_content' in facade,
          static_candidates: candidates,
          shared_memory: typeof SharedArrayBuffer !== 'undefined' && exports.memory.buffer instanceof SharedArrayBuffer });
        return;
      }
      if (!session || identity.runtime_id !== data.runtime_id || identity.epoch !== data.epoch) {
        throw new Error('utility runtime or epoch mismatch');
      }
      switch (data.operation) {
        case 'replace_config': respond('prepared', JSON.parse(session.replace_config(data.payload.config, data.payload.candidates))); break;
        case 'add_candidates': respond('prepared', JSON.parse(session.prepare(data.payload))); break;
        case 'scan_content': respond('prepared', JSON.parse(session.prepare_content(data.payload))); break;
        case 'scan_sources': respond('prepared', JSON.parse(session.prepare_sources(data.payload))); break;
        case 'ack': respond('committed', JSON.parse(session.commit(data.payload.token))); break;
        case 'nack': respond('discarded', JSON.parse(session.discard(data.payload.token))); break;
        case 'report': respond('committed', JSON.parse(session.report())); break;
        case 'dispose': session.free(); session = undefined; respond('disposed', null); self.close(); break;
        default: throw new Error(`unknown utility operation: ${data.operation}`);
      }
    } catch (error) {
      let detail;
      try { detail = typeof error === 'string' ? JSON.parse(error) : null; } catch { /* ordinary message */ }
      respond('error', detail || { kind: 'protocol', message: String(error?.message || error) });
    }
  });
};
