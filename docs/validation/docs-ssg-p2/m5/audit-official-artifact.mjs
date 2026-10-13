import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import { createHash } from "node:crypto";

const [directory = "site/dist", destination] = process.argv.slice(2);
if (!destination) throw new Error("usage: audit-official-artifact.mjs <dist> <new-report.json>");
const sha = bytes => createHash("sha256").update(bytes).digest("hex");
const manifest = JSON.parse(fs.readFileSync(path.join(directory, "rustify-docs-manifest.json"), "utf8"));
const declared = new Set(manifest.files.map(file => file.path)); const actual = [];
function files(dir) { for (const entry of fs.readdirSync(dir, { withFileTypes: true })) { const file = path.join(dir, entry.name); if (entry.isDirectory()) files(file); else actual.push(path.relative(directory, file).replaceAll("\\", "/")); } } files(directory);
assert.deepEqual(actual.sort(), [...declared, "rustify-docs-manifest.json"].sort());
const pages = [], modules = [];
for (const file of manifest.files) {
  const bytes = fs.readFileSync(path.join(directory, file.path)); assert.equal(bytes.length, file.bytes); assert.equal(sha(bytes), file.sha256);
  assert(!/\.(md|rs|toml|lock)$/.test(file.path)); assert(!/makepad|embedded\.js|\.gitkeep/i.test(file.path));
  if (file.path.endsWith(".html")) {
    const html = bytes.toString(); const h1 = [...html.matchAll(/<h1(?:\s|>)/g)].length, main = [...html.matchAll(/<main(?:\s|>)/g)].length;
    assert(/<html\b[^>]*\blang="en"/.test(html)); assert.equal(h1, 1); assert.equal(main, 1);
    const scripts = [...html.matchAll(/<script\b([^>]*)>([\s\S]*?)<\/script>/g)]; assert(scripts.every(match => /\bsrc="[^"]+"/.test(match[1]) && !match[2].trim()));
    const islands = scripts.filter(match => /\btype="module"/.test(match[1]) && /\/assets\/islands\//.test(match[1])).length;
    assert.equal(islands, /^components\/[^/]+\/index\.html$/.test(file.path) ? 1 : 0); pages.push({ path: file.path, h1, main, island_modules: islands });
  }
  if (file.path.endsWith(".js")) for (const line of bytes.toString().split("\n").filter(line => /^\s*import\b/.test(line) || line.includes(" from ") || /import\s*\(/.test(line))) assert(!/makepad/i.test(line));
  if (file.path.endsWith("app_bg.wasm")) {
    const names = [...new Set(WebAssembly.Module.imports(new WebAssembly.Module(bytes)).map(entry => entry.module))]; assert(names.every(name => name === "wbg" || name === "./app_bg.js" || name.startsWith("./snippets/rustify-ui-"))); modules.push({ path: file.path, import_modules: names });
  }
}
assert.equal(pages.length, 38); assert.equal(pages.filter(page => page.island_modules).length, 24); assert.equal(modules.length, 1);
const report = { scope: "static final published bytes and wasm import audit; does not replace browser", directory, cli_version: manifest.cli_version, build_id: manifest.build_id, base: manifest.base, file_digests_verified: manifest.files.length, files_including_manifest: actual.length, pages_including_404: pages.length, modules, pages, passed: true };
fs.writeFileSync(destination, JSON.stringify(report, null, 2) + "\n", { flag: "wx" }); console.log(JSON.stringify({ passed: true, files: actual.length, pages: pages.length, imports: modules }));
