import { defineConfig, devices } from "@playwright/test";

const fusionPort = 4173;
const workbenchPort = 4174;

export default defineConfig({
    testDir: "./tests/browser",
    // A cold page compiles a 7.7 MB wasm module and boots its regions; the
    // 30 s default leaves the shorter probes no headroom over that.
    timeout: 120_000,
    fullyParallel: false,
    workers: 1,
    retries: 0,
    reporter: [["list"], ["json", { outputFile: "test-results/browser.json" }]],
    use: {
        trace: "retain-on-failure",
        ...devices["Desktop Chrome"],
    },
    // One project per example: each has its own build directory and its own
    // static server, so a spec always talks to the app it was written for.
    projects: [
        {
            name: "fusion-basic",
            testMatch: ["m1-probes.spec.ts", "m2-runtime.spec.ts", "m3-state.spec.ts", "m4-geometry.spec.ts", "m4-overlay.spec.ts"],
            use: { baseURL: `http://127.0.0.1:${fusionPort}/` },
        },
        {
            name: "property-workbench",
            testMatch: ["m3-workbench.spec.ts", "m5-text.spec.ts", "m5-semantics.spec.ts", "m6-theme.spec.ts"],
            use: { baseURL: `http://127.0.0.1:${workbenchPort}/` },
        },
    ],
    webServer: [
        {
            command: `cargo xtask serve --example fusion-basic --release --port ${fusionPort}`,
            url: `http://127.0.0.1:${fusionPort}/`,
            reuseExistingServer: false,
            timeout: 120_000,
        },
        {
            command: `cargo xtask serve --example property-workbench --release --port ${workbenchPort}`,
            url: `http://127.0.0.1:${workbenchPort}/`,
            reuseExistingServer: false,
            timeout: 120_000,
        },
    ],
});
