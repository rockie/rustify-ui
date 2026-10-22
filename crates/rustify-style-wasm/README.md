# rustify-style-wasm

WebAssembly bindings for Rustify Style.

No features are enabled by default. Enable `cn`, `scanner`, `compiler` or `compiler-api` for the required browser API. Build for wasm32-unknown-unknown and process the output with the matching wasm-bindgen CLI. The docs.rs configuration selects the wasm target with compiler and scanner; it does not establish browser compatibility.

Use the exact registry version in your manifest:

```toml
[dependencies]
rustify-style-wasm = { version = "=0.2.0", features = ["compiler", "scanner"] }
```

See the [repository](https://github.com/rockie/rustify-ui) and [author guide](https://github.com/rockie/rustify-ui/blob/main/docs/docs-ssg.md).

Licensed under MIT; see [LICENSE](LICENSE).
