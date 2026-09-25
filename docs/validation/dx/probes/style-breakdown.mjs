// Groups the differences between two computed-style snapshots by property.
//
//   node docs/validation/dx/probes/style-breakdown.mjs <before.json> <after.json>
//
// `computed-style.mjs compare` prints the first 200 differing values; this
// prints every property that differs anywhere, how often and between which
// values. Tailwind's theme variables are named after the prefix (`--rui-text-sm`
// with it, `--text-sm` without), so a before-snapshot's `--rui-x` is compared
// as `--x` unless the element already has an `--x` of its own; the renamed
// names are listed first so the rename is visible rather than silent.

import { readFileSync } from "node:fs";

const [beforePath, afterPath] = process.argv.slice(2);
if (!beforePath || !afterPath) {
    console.error("usage: style-breakdown.mjs <before.json> <after.json>");
    process.exit(2);
}
const before = JSON.parse(readFileSync(beforePath, "utf8"));
const after = JSON.parse(readFileSync(afterPath, "utf8"));

const renamed = new Set();
for (const page of Object.values(before)) {
    for (const [element, values] of Object.entries(page)) {
        const next = {};
        for (const [name, value] of Object.entries(values)) {
            const bare = name.startsWith("--rui-") ? `--${name.slice(6)}` : name;
            if (bare !== name && !(bare in values)) {
                renamed.add(`${name} -> ${bare}`);
                next[bare] = value;
            } else {
                next[name] = value;
            }
        }
        page[element] = next;
    }
}

const byProperty = new Map();
let checked = 0;
let differences = 0;
let oneSided = 0;
for (const key of new Set([...Object.keys(before), ...Object.keys(after)])) {
    const left = before[key] ?? {};
    const right = after[key] ?? {};
    for (const element of new Set([...Object.keys(left), ...Object.keys(right)])) {
        if (!(element in left) || !(element in right)) {
            oneSided += 1;
            continue;
        }
        for (const property of new Set([...Object.keys(left[element]), ...Object.keys(right[element])])) {
            checked += 1;
            const a = left[element][property];
            const b = right[element][property];
            if (a === b) continue;
            differences += 1;
            const entry = byProperty.get(property) ?? { count: 0, pairs: new Map(), pages: new Set() };
            entry.count += 1;
            const pair = `${a} -> ${b}`;
            entry.pairs.set(pair, (entry.pairs.get(pair) ?? 0) + 1);
            entry.pages.add(key);
            byProperty.set(property, entry);
        }
    }
}

console.log(`compared as renamed: ${renamed.size ? [...renamed].sort().join(", ") : "none"}`);
console.log(`${checked} values compared, ${differences} differ, ${oneSided} elements on one side only`);
for (const [property, entry] of [...byProperty].sort((x, y) => y[1].count - x[1].count)) {
    console.log(`${property}: ${entry.count} values on ${entry.pages.size} page states`);
    for (const [pair, n] of [...entry.pairs].sort((x, y) => y[1] - x[1]).slice(0, 5)) {
        console.log(`    ${n} x ${pair}`);
    }
}
process.exit(differences === 0 && oneSided === 0 ? 0 : 1);
