# rustify-style

Pure Rust class merging, content scanning and utility compilation.

Default feature: `cn` for class merging. Enable `scanner`, `scanner-fs`, `protocol` or `compiler` as needed. No DOM or GPU runtime is linked. The scanner includes Tailwind Oxide source; upstream MIT notices are included under upstream/.

Use the exact registry version in your manifest:

```toml
[dependencies]
rustify-style = "=0.2.0"
```

See the [repository](https://github.com/rockie/rustify-ui) and [author guide](https://github.com/rockie/rustify-ui/blob/main/docs/docs-ssg.md).

Licensed under MIT; see [LICENSE](LICENSE).
Third-party notices are in [upstream/README.md](upstream/README.md).
