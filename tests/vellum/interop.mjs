import { readFile, writeFile, mkdir } from 'node:fs/promises';
import { fileURLToPath } from 'node:url';
import assert from 'node:assert/strict';

const fixture = new URL('./fixtures/forma.vellum', import.meta.url);
const source = new URL('../../ref/Vellum-main/src/document.js', import.meta.url);
const { DocumentModel, makeStarter } = await import(source.href);

if (process.argv[2] === '--generate') {
    let serial = 0;
    Object.defineProperty(globalThis.crypto, 'randomUUID', {
        value: () => `00000000-0000-4000-8000-${String(++serial).padStart(12, '0')}`,
    });
    await mkdir(new URL('./fixtures/', import.meta.url), { recursive: true });
    await writeFile(fixture, new DocumentModel(makeStarter()).serialize());
}

const starter = process.argv[2] === '--check-starter';
const input = starter ? process.argv[3] : process.argv[2] && process.argv[2] !== '--generate'
    ? process.argv[2] : fileURLToPath(fixture);
let text = '';
if (input === '-') {
    for await (const chunk of process.stdin) text += chunk;
} else {
    text = await readFile(input, 'utf8');
}
const document = DocumentModel.parse(text);
const counts = document.pages.map(page => page.nodes.length);
console.log(JSON.stringify({ pages: counts.length, layers: counts.reduce((a, b) => a + b, 0), counts }));
if (starter || process.argv[2] === '--generate') {
    assert.deepEqual(counts, [171, 31, 0]);
    const expected = JSON.parse(await readFile(fixture, 'utf8'));
    function compare(actual, expected, path) {
        if (expected === null || typeof expected !== 'object') {
            assert.deepEqual(actual, expected, path);
        } else {
            if (Array.isArray(expected)) assert.equal(actual.length, expected.length, path);
            for (const [key, value] of Object.entries(expected)) compare(actual[key], value, `${path}.${key}`);
        }
    }
    compare(document, expected, 'document');
    console.log('All reference starter fields match.');
}
