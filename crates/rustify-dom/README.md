# rustify-dom

Shared Rustify state, DOM runtime, bindings and themes.

The default `csr` mode is for browser DOM applications. Use `default-features = false, features = ["ssr"]` for host rendering, or `hydrate` for a browser hydration build. Compile each rendering mode separately. There is no Makepad dependency. `utility` enables runtime class support; `external-host` is for an integrating SDK host.

Use the exact registry version in your manifest:

```toml
[dependencies]
rustify-dom = "=0.2.0"
```

docs.rs is configured for the host SSR mode on Linux. Browser-only APIs are absent there. The validated runtime environment is macOS Chrome; this metadata does not establish Linux or Windows runtime support.

See the [repository](https://github.com/rockie/rustify-ui) and [author guide](https://github.com/rockie/rustify-ui/blob/main/docs/docs-ssg.md).

Licensed under MIT; see [LICENSE](LICENSE).
