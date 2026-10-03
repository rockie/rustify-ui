# Pure Makepad text checks

Run from the repository root:

```sh
sh tests/theme-text/run.sh --lib
sh tests/theme-text/run.sh --lib letter_spacing_
```

This independent test crate compiles the actual `makepad/draw/src/text` source files. It supplies only `SharedBytes` and a log macro in place of the platform module, so CPU font shaping, layout, caching and hit testing can run on macOS without the fork's removed native backend. It has no second implementation of the text algorithms.

The script sets the source font directory used by the existing real-font tests. Keep the same bundled resources when comparing results. These tests do not exercise WebGL drawing, browser font loading or DOM/GPU pixel parity; those still require the browser checks.
