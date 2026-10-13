import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import { spawnSync } from "node:child_process";

const round = process.argv[2] || "r1"; assert(/^r\d+$/.test(round));
const out = path.resolve(`docs/validation/docs-ssg-p2/m5/official-graphs-${round}`); fs.mkdirSync(out);
const source = fs.readFileSync("xtask/src/docs.rs", "utf8");
const constants = name => { const text = source.match(new RegExp(`const ${name}: [^=]+ = &?\\[([\\s\\S]*?)\\];`))?.[1]; assert(text, name); return [...text.matchAll(/"([^"\n]+)"/g)].map(match => match[1]); };
const privatePackages = constants("SITE_PIPELINE_PACKAGES"), toolPackages = constants("TOOL_PACKAGES");
const reports = [];
for (const mode of ["ssr", "hydrate"]) {
  const args = ["tree", "--locked", "--offline", "--manifest-path", "site/Cargo.toml", "--package", "rustify-official-site", "--edges", "normal,build", "--target", "all", "--prefix", "none", "--no-dedupe", "--format", "{p}\t{f}", "--no-default-features", "--features", mode];
  const result = spawnSync("mbx", args, { encoding: "utf8", maxBuffer: 64 * 1024 * 1024 }); fs.writeFileSync(path.join(out, `${mode}.log`), result.stdout + result.stderr); if (result.error) throw result.error; assert.equal(result.status, 0);
  const lines = [...new Set(result.stdout.trim().split("\n"))]; const packages = new Set(lines.map(line => line.split(/\s+/)[0]));
  assert(![...packages].some(name => name.startsWith("makepad") || name === "rustify-makepad" || name === "cargo-makepad"));
  assert(![...privatePackages, ...toolPackages].some(name => packages.has(name)));
  const forbidden = new Map([["rustify-docs", ["pipeline"]], ["rustify-style", ["compiler", "scanner", "scanner-fs"]], ["rustify-ui", ["gpu"]], ["rustify-components", ["gpu"]]]);
  for (const name of ["leptos", "rustify-ui", "rustify-components"]) forbidden.set(name, [...(forbidden.get(name) || []), ...(mode === "ssr" ? ["csr", "hydrate"] : ["csr", "ssr"])]);
  if (mode === "ssr") forbidden.set("reactive_graph", ["effects"]); else forbidden.get("rustify-docs").push("site-ssr");
  for (const line of lines) { const name = line.split(/\s+/)[0], features = (line.split("\t")[1] || "").split(","); for (const feature of forbidden.get(name) || []) assert(!features.includes(feature), `${mode}: ${line} has forbidden ${feature}`); }
  assert(lines.some(line => line.startsWith("rustify-docs ") && line.split("\t")[1]?.split(",").includes(mode === "ssr" ? "site-ssr" : "hydrate")));
  reports.push({ mode, command: ["mbx", ...args], target: "all", edges: "normal,build", packages: packages.size, forbidden_packages_from: "xtask/src/docs.rs SITE_PIPELINE_PACKAGES/TOOL_PACKAGES", passed: true });
}
fs.writeFileSync(path.join(out, "report.json"), JSON.stringify(reports, null, 2) + "\n"); console.log(JSON.stringify(reports));
