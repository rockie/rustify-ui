# rustify-components

Controlled Leptos DOM components and their styles.

The default `csr` mode renders in the browser. Host rendering uses `default-features = false, features = ["ssr"]`; hydration uses `hydrate`. Compile each mode separately. Import CSS through `CSS` and `styles::{SDK, SLIDER, CONTROLS, THEME_V4}`. GPU applications enable gpu on the Git SDK and must unify the rustify-dom source, as described in the repository quick start. The former component gpu feature is removed.

Use the exact registry version in your manifest:

```toml
[dependencies]
rustify-components = "=0.2.0"
```

docs.rs is configured for the host SSR mode on Linux. Browser-only APIs are absent there. The validated runtime environment is macOS Chrome; this metadata does not establish Linux or Windows runtime support.

See the [repository](https://github.com/rockie/rustify-ui) and [author guide](https://github.com/rockie/rustify-ui/blob/main/docs/docs-ssg.md).

Licensed under MIT; see [LICENSE](LICENSE).
Rust/UI notices are preserved in [LICENSE-RUST-UI](LICENSE-RUST-UI).
