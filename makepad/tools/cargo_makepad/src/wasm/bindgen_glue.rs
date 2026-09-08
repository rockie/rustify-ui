//! Adapts the wasm-bindgen `--target web` glue so Makepad's `env` imports are
//! supplied by the caller instead of an ES module named `env`.
//!
//! Every rewrite is asserted against the exact shape wasm-bindgen 0.2.128
//! emits; a drifted layout fails the build instead of producing glue that
//! silently loads nothing.

const ENV_IMPORT_PREFIX: &str = "import * as ";
const ENV_IMPORT_SUFFIX: &str = " from \"env\"";
const ENV_TABLE_PREFIX: &str = "\"env\": ";
const INIT_SIGNATURE: &str = "async function __wbg_init(module_or_path) {";
const INIT_SYNC_SIGNATURE: &str = "function initSync(module) {";
const GET_IMPORTS: &str = "const imports = __wbg_get_imports();";
const FINALIZE_RETURN: &str = "    return wasm;\n}";

pub fn patch_bindgen_glue(js: &str) -> Result<String, String> {
    let mut removed_imports = 0;
    let mut removed_table_entries = 0;
    let kept: Vec<&str> = js
        .lines()
        .filter(|line| {
            let trimmed = line.trim();
            if trimmed.starts_with(ENV_IMPORT_PREFIX) && trimmed.ends_with(ENV_IMPORT_SUFFIX) {
                removed_imports += 1;
                return false;
            }
            if trimmed.starts_with(ENV_TABLE_PREFIX) && trimmed.ends_with(',') {
                removed_table_entries += 1;
                return false;
            }
            true
        })
        .collect();
    if removed_imports == 0 || removed_table_entries == 0 {
        return Err(mismatch("no `env` module imports found"));
    }
    if removed_imports != removed_table_entries {
        return Err(mismatch("`env` import lines and import table entries differ"));
    }

    let mut out = kept.join("\n");
    out.push('\n');
    out = replace_exactly(
        &out,
        INIT_SIGNATURE,
        "async function __wbg_init(module_or_path, env) {",
        1,
    )?;
    out = replace_exactly(&out, INIT_SYNC_SIGNATURE, "function initSync(module, env) {", 1)?;
    out = replace_exactly(
        &out,
        GET_IMPORTS,
        "const imports = __wbg_get_imports();\n    imports.env = env;",
        2,
    )?;
    out = replace_exactly(&out, FINALIZE_RETURN, "    return instance;\n}", 1)?;
    Ok(out)
}

fn replace_exactly(text: &str, from: &str, to: &str, expected: usize) -> Result<String, String> {
    let found = text.matches(from).count();
    if found != expected {
        return Err(mismatch(&format!(
            "expected {expected} occurrence(s) of `{from}`, found {found}"
        )));
    }
    Ok(text.replace(from, to))
}

fn mismatch(detail: &str) -> String {
    format!("BuildContractMismatch: wasm-bindgen glue layout changed ({detail}); update cargo-makepad's glue patch")
}

#[cfg(test)]
mod tests {
    use super::*;

    const GLUE: &str = r#"import * as import1 from "env"
import * as import2 from "env"

let wasm;

function __wbg_get_imports() {
    return {
        "./bindgen_bg.js": {},
        "env": import1,
        "env": import2,
    };
}

function __wbg_finalize_init(instance, module) {
    wasm = instance.exports;
    return wasm;
}

function initSync(module) {
    if (wasm !== undefined) return wasm;
    const imports = __wbg_get_imports();
    const instance = new WebAssembly.Instance(module, imports);
    return __wbg_finalize_init(instance, module);
}

async function __wbg_init(module_or_path) {
    if (wasm !== undefined) return wasm;
    const imports = __wbg_get_imports();
    const { instance, module } = await __wbg_load(await module_or_path, imports);
    return __wbg_finalize_init(instance, module);
}

export { initSync, __wbg_init as default };
"#;

    #[test]
    fn env_imports_come_from_the_caller_and_init_returns_the_instance() {
        let patched = patch_bindgen_glue(GLUE).unwrap();
        assert!(!patched.contains("from \"env\""));
        assert!(!patched.contains("\"env\": import"));
        assert!(patched.contains("async function __wbg_init(module_or_path, env) {"));
        assert!(patched.contains("function initSync(module, env) {"));
        assert_eq!(patched.matches("imports.env = env;").count(), 2);
        assert!(patched.contains("    return instance;\n}"));
        assert!(!patched.contains("    return wasm;\n}"));
        assert!(patched.contains("if (wasm !== undefined) return wasm;"));
    }

    #[test]
    fn a_drifted_layout_is_a_build_contract_mismatch() {
        let without_env = GLUE.replace("from \"env\"", "from \"other\"");
        assert!(patch_bindgen_glue(&without_env)
            .unwrap_err()
            .starts_with("BuildContractMismatch"));
        let renamed_init = GLUE.replace("__wbg_init(module_or_path)", "__wbg_init(path)");
        assert!(patch_bindgen_glue(&renamed_init).is_err());
    }
}
