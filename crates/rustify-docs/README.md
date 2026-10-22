# rustify-docs

Static Markdown sites and optional Leptos islands.

The default `pipeline` feature parses content and renders HTML on the host. A components site uses `default-features = false` and forwards `site-ssr` for its host helper or `hydrate` for its browser module. Do not combine host SSR and browser CSR/hydrate modes. Content-only authors use the rustify-docs CLI.

Use the exact registry version in your manifest:

```toml
[dependencies]
rustify-docs = "=0.2.0"
```

docs.rs is configured for the host SSR mode on Linux. Browser-only APIs are absent there. The validated runtime environment is macOS Chrome; this metadata does not establish Linux or Windows runtime support.

See the [repository](https://github.com/rockie/rustify-ui) and [author guide](https://github.com/rockie/rustify-ui/blob/main/docs/docs-ssg.md).

Licensed under MIT; see [LICENSE](LICENSE).
