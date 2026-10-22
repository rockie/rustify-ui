//! Repository-only corpus availability check.
use std::io::Read;

#[test]
fn repo_corpus_sentinel() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    for file in [
        "tests/utility/cn/build-contract-fixtures.json",
        "tests/utility/cn/build-fixtures.json",
        "tests/utility/cn/compiler-fixtures.json",
        "tests/utility/cn/runtime-fixtures.json",
        "tests/utility/compiler/apply-fixtures.json",
        "tests/utility/compiler/arbitrary-properties-fixtures.json",
        "tests/utility/compiler/backgrounds-fixtures.json",
        "tests/utility/compiler/borders-fixtures.json",
        "tests/utility/compiler/colors-fixtures.json",
        "tests/utility/compiler/compile-ast-native.rs",
        "tests/utility/compiler/compiler-smoke.css",
        "tests/utility/compiler/css-scope-property-nesting.css",
        "tests/utility/compiler/current-walk-fixtures.json",
        "tests/utility/compiler/custom-utilities-fixtures.json",
        "tests/utility/compiler/custom-variants-fixtures.json",
        "tests/utility/compiler/design-system-native.rs",
        "tests/utility/compiler/dynamic-variant-fixtures.json",
        "tests/utility/compiler/effects-complete-fixtures.json",
        "tests/utility/compiler/effects-fixtures.json",
        "tests/utility/compiler/external-variables-fixtures.json",
        "tests/utility/compiler/filters-fixtures.json",
        "tests/utility/compiler/functional-spacing-fixtures.json",
        "tests/utility/compiler/functions-fixtures.json",
        "tests/utility/compiler/gradients-fixtures.json",
        "tests/utility/compiler/grid-fixtures.json",
        "tests/utility/compiler/imports-fixtures.json",
        "tests/utility/compiler/layout-fixtures.json",
        "tests/utility/compiler/legacy-theme-boundaries.json",
        "tests/utility/compiler/legacy-theme-fixtures.json",
        "tests/utility/compiler/masks-fixtures.json",
        "tests/utility/compiler/miscellaneous-fixtures.json",
        "tests/utility/compiler/motion-fixtures.json",
        "tests/utility/compiler/parser-errors-fixtures.json",
        "tests/utility/compiler/prefix-fixtures.json",
        "tests/utility/compiler/public-api-native.rs",
        "tests/utility/compiler/shadows-fixtures.json",
        "tests/utility/compiler/source-directives-fixtures.json",
        "tests/utility/compiler/source-pipeline.rs",
        "tests/utility/compiler/static-fixtures.json",
        "tests/utility/compiler/structural-spacing-fixtures.json",
        "tests/utility/compiler/theme-fixtures.json",
        "tests/utility/compiler/transforms-fixtures.json",
        "tests/utility/compiler/typography-fixtures.json",
        "tests/utility/compiler/utf16-ast-fields-native.rs",
        "tests/utility/compiler/utf16-declaration-native.rs",
        "tests/utility/compiler/utf16-error-native.rs",
        "tests/utility/compiler/utf16-family-native.rs",
        "tests/utility/compiler/utf16-fixtures.json",
        "tests/utility/compiler/utf16-function-fixtures.json",
        "tests/utility/compiler/utf16-parser-native.rs",
        "tests/utility/compiler/utf16-theme-native.rs",
        "tests/utility/compiler/utf16-value-native.rs",
        "tests/utility/compiler/variant-fixtures.json",
        "tests/utility/compiler/variant-parameters-fixtures.json",
        "tests/utility/scanner/fixtures.json",
        "tests/utility/scanner/inventory.json",
        "tests/utility/scanner/source-contracts.rs",
    ] {
        let path = root.join(file);
        let mut input = std::fs::File::open(&path)
            .unwrap_or_else(|error| panic!("{}: {error}", path.display()));
        input
            .read_exact(&mut [0])
            .unwrap_or_else(|error| panic!("{}: {error}", path.display()));
    }
}
