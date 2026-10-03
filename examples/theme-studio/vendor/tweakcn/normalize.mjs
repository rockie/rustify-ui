// Regenerate with Node 26: node --experimental-vm-modules normalize.mjs
// Add --check to verify committed outputs without writing them.
// Only the vendored snapshot is read; generated JSON is the Rust build input.
import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { readFileSync, writeFileSync } from "node:fs";
import { stripTypeScriptTypes } from "node:module";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { SourceTextModule, SyntheticModule } from "node:vm";

const root = dirname(fileURLToPath(import.meta.url));
const sourceRoot = join(root, "source");
const sha256 = (bytes) => createHash("sha256").update(bytes).digest("hex");
const json = (value) => `${JSON.stringify(value, null, 2)}\n`;
const modules = new Map();

function stub(name, exports) {
  return new SyntheticModule(Object.keys(exports), function () {
    for (const [key, value] of Object.entries(exports)) this.setExport(key, value);
  }, { identifier: name });
}

// TypeScript's eraser preserves imports even when they only describe types.
const typeImports = stub("type-imports", {
  ThemePreset: null, ThemeStyles: null, ThemeEditorState: null,
  InferSelectModel: null, theme: null,
});
const store = stub("unused-store", {
  useThemePresetStore: { getState() { throw new Error("Built-in normalization must not use the store"); } },
});

// Evaluate the upstream schema declaration to obtain required/optional keys.
// This shim records shape only; it does not claim to perform Zod validation.
const z = {
  string() {
    return {
      isOptional: false,
      describe() { return this; },
      optional() { return { ...this, isOptional: true }; },
    };
  },
  object(shape) {
    return {
      shape,
      omit(keys) { return z.object(Object.fromEntries(Object.entries(shape).filter(([key]) => !keys[key]))); },
    };
  },
};
const zod = stub("schema-shape", { z });

function load(relativePath) {
  const filename = resolve(sourceRoot, relativePath);
  if (!modules.has(filename)) {
    const source = stripTypeScriptTypes(readFileSync(filename, "utf8"), { mode: "strip" });
    modules.set(filename, new SourceTextModule(source, { identifier: filename }));
  }
  return modules.get(filename);
}

function link(specifier, referring) {
  if (specifier === "zod") return zod;
  if (specifier === "drizzle-orm" || specifier === "@/db/schema"
      || specifier === "../types/theme" || specifier === "../types/editor") return typeImports;
  if (specifier === "../store/theme-preset-store") return store;
  return load(`${resolve(dirname(referring.identifier), specifier)}.ts`);
}

const helper = load("utils/theme-preset-helper.ts");
await helper.link(link);
await helper.evaluate();
const schema = load("types/theme.ts");
await schema.link(link);
await schema.evaluate();
const presets = load("utils/theme-presets.ts").namespace.defaultPresets;
const config = load("config/theme.ts").namespace;
const shape = schema.namespace.themeStylePropsSchema.shape;
const tokens = Object.keys(shape);
const required = tokens.filter((key) => !shape[key].isOptional);
const optional = tokens.filter((key) => shape[key].isOptional);
const normalized = [];
const commonStyleDifferences = [];
const exceptionalMetricValues = [];
const numericTokens = ["radius", "spacing", "letter-spacing", "shadow-opacity", "shadow-blur",
  "shadow-spread", "shadow-offset-x", "shadow-offset-y"];

for (const [id, preset] of Object.entries(presets)) {
  assert.ok(id.length > 0, "Preset identity must not be empty");
  const expected = {
    light: { ...config.defaultThemeState.styles.light, ...preset.styles.light },
    dark: { ...config.defaultThemeState.styles.dark, ...preset.styles.light, ...preset.styles.dark },
  };
  const actual = helper.namespace.getBuiltInThemeStyles(id);
  assert.deepEqual(actual.styles, expected, `${id}: normalization differs from upstream helper`);
  assert.equal(actual.name, preset.label || id);
  assert.equal(typeof actual.name, "string", `${id}: name must be a string`);
  assert.ok(actual.name.length > 0, `${id}: name must not be empty`);
  for (const [mode, styles] of Object.entries(actual.styles)) {
    assert.deepEqual(Object.keys(styles).sort(), [...tokens].sort(), `${id}/${mode}: token set differs`);
    for (const key of required) assert.equal(typeof styles[key], "string", `${id}/${mode}/${key}`);
    for (const [key, value] of Object.entries(styles)) {
      assert.equal(typeof value, "string", `${id}/${mode}/${key}`);
      assert.ok(value.length > 0, `${id}/${mode}/${key}: empty value`);
      const numeric = /^[+-]?(?:\d+(?:\.\d*)?|\.\d+)(?:px|rem|em)?$/.test(value);
      const letterSpacingEm = /^[+-]?(?:\d+(?:\.\d*)?|\.\d+)em$/.test(value);
      if (numericTokens.includes(key) && (!numeric || (key === "letter-spacing" && !letterSpacingEm))) {
        exceptionalMetricValues.push({ id, mode, token: key, value });
      }
    }
    for (const [key, value] of Object.entries(preset.styles[mode] || {})) {
      assert.equal(styles[key], value, `${id}/${mode}/${key}: authored value was changed`);
    }
  }
  for (const token of config.COMMON_STYLES) {
    if (actual.styles.light[token] !== actual.styles.dark[token]) {
      commonStyleDifferences.push({ id, token, light: actual.styles.light[token], dark: actual.styles.dark[token] });
    }
  }
  normalized.push({ id, name: actual.name, styles: actual.styles });
}
assert.equal(new Set(normalized.map(({ id }) => id)).size, normalized.length);
assert.deepEqual(normalized.map(({ id }) => id), Object.keys(presets));
assert.equal(helper.namespace.getBuiltInThemeStyles("not-a-built-in-theme"), null);

const output = json(normalized);
const inputFiles = ["utils/theme-presets.ts", "utils/theme-preset-helper.ts", "config/theme.ts", "types/theme.ts"];
const provenance = {
  source: {
    repository: "https://github.com/jnsahaj/tweakcn",
    snapshot_root: "ref/tweakcn-main",
    upstream_commit: null,
    identity: "Local source snapshot identified by the input SHA256 digests; upstream commit is not established",
    license: "Apache-2.0",
  },
  inputs: [
    ...inputFiles.map((path) => ({ source_path: `ref/tweakcn-main/${path}`, vendored_path: `source/${path}`,
      sha256: sha256(readFileSync(join(sourceRoot, path))) })),
    { source_path: "ref/tweakcn-main/LICENSE", vendored_path: "LICENSE", sha256: sha256(readFileSync(join(root, "LICENSE"))) },
  ],
  preset_keys: Object.keys(presets),
  merge_order: { light: ["default.light", "preset.light"], dark: ["default.dark", "preset.light", "preset.dark"] },
  required_tokens: required,
  optional_tokens: optional,
  common_styles: [...config.COMMON_STYLES],
  normalization: {
    script: "normalize.mjs",
    command: "node --experimental-vm-modules examples/theme-studio/vendor/tweakcn/normalize.mjs --check",
    method: "Evaluate the complete vendored TypeScript modules with Node's type eraser; compare every preset with getBuiltInThemeStyles; inspect upstream schema shape",
    scope: "Built-in presets only; unused type/database/store imports use shims; schema shim records keys only",
    author_values_preserved: true,
    rustify_extension_tokens_added: false,
    output_path: "presets.json",
    output_sha256: sha256(output),
  },
  audit: {
    preset_count: normalized.length,
    tokens_per_mode: tokens.length,
    missing_required_tokens: [],
    unknown_tokens: [],
    exceptional_metric_values: exceptionalMetricValues,
    common_style_differences: commonStyleDifferences,
  },
};

for (const [filename, bytes] of [["presets.json", output], ["provenance.json", json(provenance)]]) {
  if (process.argv.includes("--check")) {
    assert.equal(readFileSync(join(root, filename), "utf8"), bytes, `${filename}: regenerate the vendored data`);
  } else {
    writeFileSync(join(root, filename), bytes);
  }
}
console.log(JSON.stringify({ presets: normalized.length, tokens_per_mode: tokens.length,
  required_tokens: required.length, optional_tokens: optional,
  output_sha256: sha256(output), exceptional_metric_values: exceptionalMetricValues,
  common_style_differences: commonStyleDifferences }, null, 2));
