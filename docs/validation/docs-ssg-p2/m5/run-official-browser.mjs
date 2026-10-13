import fs from "node:fs";
import path from "node:path";
import { spawn } from "node:child_process";

const [project, round, ...extraArgs] = process.argv.slice(2);
if (!/^official-(subpath|root|meta)$/.test(project || "") || !/^r\d+$/.test(round || "")) throw new Error("usage: run-official-browser.mjs official-{subpath,root,meta} rN");
const destination = path.resolve(`docs/validation/docs-ssg-p2/m5/${project}-${round}`);
fs.mkdirSync(destination, { recursive: false });
const log = fs.openSync(path.join(destination, "browser.log"), "wx");
const args = ["playwright", "test", "-c", "playwright.docs.config.ts", `--project=${project}`, "tests/docs-ssg/official.spec.ts", "--reporter=list,json", "--output", path.join(destination, "artifacts"), ...extraArgs];
fs.writeFileSync(path.join(destination, "command.json"), JSON.stringify({ command: ["npx", ...args], RUSTIFY_DOCS_BIN: path.resolve("target/docs-cli/rustify-docs"), started_at: new Date().toISOString() }, null, 2) + "\n");
const child = spawn("npx", args, { stdio: ["ignore", log, log], env: { ...process.env, RUSTIFY_DOCS_BIN: path.resolve("target/docs-cli/rustify-docs"), PLAYWRIGHT_JSON_OUTPUT_NAME: path.join(destination, "results.json") } });
const code = await new Promise((resolve, reject) => { child.on("error", reject); child.on("exit", resolve); });
fs.closeSync(log);
let stats;
if (fs.existsSync(path.join(destination, "results.json"))) stats = JSON.parse(fs.readFileSync(path.join(destination, "results.json"), "utf8")).stats;
console.log(JSON.stringify({ project, round, exit_code: code, destination, stats }));
process.exitCode = code ?? 1;
