import { defineConfig, devices } from "@playwright/test";

const port = 4173;

export default defineConfig({
    testDir: "./tests/browser",
    // A cold page compiles a 7.7 MB wasm module and boots four regions; the
    // 30 s default leaves the shorter probes no headroom over that.
    timeout: 120_000,
    fullyParallel: false,
    workers: 1,
    retries: 0,
    reporter: [["list"], ["json", { outputFile: "test-results/browser.json" }]],
    use: {
        baseURL: `http://127.0.0.1:${port}/`,
        trace: "retain-on-failure",
        ...devices["Desktop Chrome"],
    },
    webServer: {
        command: `cargo xtask serve --example fusion-basic --release --port ${port}`,
        url: `http://127.0.0.1:${port}/`,
        reuseExistingServer: false,
        timeout: 120_000,
    },
});
