# rustify-classification-macros

Byte classification derive used by the Rustify Tailwind scanner.

A host procedural macro vendored from Tailwind CSS 4.1.13. Its library import is `classification_macros::ClassifyBytes`; Rustify Style uses a dependency alias. This package preserves the upstream MIT notice in LICENSE.

Use the exact registry version in your manifest:

```toml
[dependencies]
classification_macros = { package = "rustify-classification-macros", version = "=0.2.0" }
```

See the [repository](https://github.com/rockie/rustify-ui) and [author guide](https://github.com/rockie/rustify-ui/blob/main/docs/docs-ssg.md).

Licensed under MIT; see [LICENSE](LICENSE).
