// Summarises Playwright JSON reports: per project, per spec file, counts by outcome,
// and the titles of every test that did not pass.
import { readFileSync, readdirSync } from "node:fs";
import { join, basename } from "node:path";
const dir = process.argv[2];
const rows = [];
for (const file of readdirSync(dir).filter((f) => f.endsWith(".json")).sort()) {
  const report = JSON.parse(readFileSync(join(dir, file), "utf8"));
  const specs = new Map();
  const notPassed = [];
  const walk = (suite, path) => {
    for (const spec of suite.specs ?? []) {
      for (const t of spec.tests ?? []) {
        const outcome = t.status === "expected" ? "passed" : t.status; // expected|unexpected|skipped|flaky
        const key = spec.file;
        const entry = specs.get(key) ?? { passed: 0, unexpected: 0, skipped: 0, flaky: 0 };
        entry[outcome] = (entry[outcome] ?? 0) + 1;
        specs.set(key, entry);
        if (outcome !== "passed") notPassed.push(`${outcome}: ${spec.file} › ${spec.title}`);
      }
    }
    for (const child of suite.suites ?? []) walk(child, path);
  };
  for (const suite of report.suites ?? []) walk(suite, []);
  const total = [...specs.values()].reduce((a, e) => ({ passed: a.passed + e.passed, unexpected: a.unexpected + e.unexpected, skipped: a.skipped + e.skipped, flaky: a.flaky + (e.flaky ?? 0) }), { passed: 0, unexpected: 0, skipped: 0, flaky: 0 });
  rows.push({ project: basename(file, ".json"), specs: specs.size, ...total, notPassed, perSpec: Object.fromEntries(specs) });
}
console.log(JSON.stringify(rows, null, 1));
