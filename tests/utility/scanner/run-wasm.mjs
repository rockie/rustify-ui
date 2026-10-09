import fs from 'node:fs';
import assert from 'node:assert/strict';
import { fileURLToPath } from 'node:url';

const modulePath = new URL('../../../target/wasm32-unknown-unknown/release/examples/scanner_probe.wasm', import.meta.url);
const bytes = fs.readFileSync(modulePath);
const module = new WebAssembly.Module(bytes);
const imports = WebAssembly.Module.imports(module);
assert.deepEqual(imports, [], 'pure scanner probe must not require host imports');
const instance = new WebAssembly.Instance(module);
const failures = instance.exports.run_scanner_fixtures();
assert.equal(failures, 0, 'native/shared fixed scanner corpus must agree in WASM');
const sharedMemory = instance.exports.memory.buffer instanceof SharedArrayBuffer;
assert.equal(sharedMemory, false, 'scanner probe must use ordinary single-threaded memory');
const corpus = JSON.parse(fs.readFileSync(new URL('./fixtures.json', import.meta.url), 'utf8'));
console.log(JSON.stringify({ module: fileURLToPath(modulePath), cases: corpus.length, failures, imports, sharedMemory, wasmBytes: bytes.byteLength }, null, 2));
