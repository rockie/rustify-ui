import { defineConfig, devices } from "@playwright/test";

const fusionPort = 4173;
const workbenchPort = 4174;
// A third server for the deployment checks: the same build, served under a
// sub-path, with a switch that makes it serve a broken one on request.
const deploymentPort = 4175;
const catalogPort = 4176;
// The workbench again, under a sub-path: the same deep-link checks have to
// pass at the root and below it, and the difference is the deployment.
const deepLinkPort = 4177;
const deploymentBase = "/tools/demo/";

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
            testMatch: ["m1-probes.spec.ts", "m2-runtime.spec.ts", "m3-state.spec.ts", "m4-geometry.spec.ts", "m4-overlay.spec.ts", "m6-mainpath.spec.ts", "p2-navigation.spec.ts"],
            use: { baseURL: `http://127.0.0.1:${fusionPort}/` },
        },
        {
            name: "property-workbench",
            testMatch: ["m3-workbench.spec.ts", "m5-text.spec.ts", "m5-semantics.spec.ts", "m6-theme.spec.ts", "m6-async.spec.ts", "m6-components.spec.ts", "m7-recovery.spec.ts", "m8-baseline.spec.ts", "m8-network.spec.ts", "m8-endurance.spec.ts", "p2-form.spec.ts", "p2-deeplink.spec.ts", "p2-workspace.spec.ts", "p2-drag.spec.ts", "p2-clipboard-files.spec.ts", "p2-zoom.spec.ts", "p2-ime.spec.ts"],
            use: { baseURL: `http://127.0.0.1:${workbenchPort}/` },
        },
        {
            name: "component-catalog",
            testMatch: ["p2-catalog.spec.ts", "p2-theme.spec.ts", "p2-semantics.spec.ts", "p2-i18n.spec.ts", "p2-reflow.spec.ts", "p2-a11y.spec.ts"],
            use: { baseURL: `http://127.0.0.1:${catalogPort}/` },
        },
        // The budget gate runs thirty cold loads and thirty hot ones, so it is
        // its own project rather than a slow tail on every workbench run. Same
        // build, same server: what makes it separate is how long it takes.
        {
            name: "budget",
            testMatch: ["p2-budget.spec.ts"],
            use: { baseURL: `http://127.0.0.1:${workbenchPort}/` },
        },
        {
            name: "workbench-deep",
            testMatch: ["p2-deeplink.spec.ts"],
            use: { baseURL: `http://127.0.0.1:${deepLinkPort}${deploymentBase}` },
        },
        {
            name: "deployment",
            testMatch: ["m7-deployment.spec.ts"],
            use: { baseURL: `http://127.0.0.1:${deploymentPort}${deploymentBase}` },
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
            command: `cargo xtask serve --example property-workbench --release --port ${workbenchPort} --spa`,
            url: `http://127.0.0.1:${workbenchPort}/`,
            reuseExistingServer: false,
            timeout: 120_000,
        },
        {
            command: `cargo xtask serve --example component-catalog --release --port ${catalogPort}`,
            url: `http://127.0.0.1:${catalogPort}/`,
            reuseExistingServer: false,
            timeout: 120_000,
        },
        {
            command: `cargo xtask build-web --example property-workbench --release --base ${deploymentBase} && cargo xtask serve --example property-workbench --release --port ${deepLinkPort} --base ${deploymentBase} --spa`,
            url: `http://127.0.0.1:${deepLinkPort}${deploymentBase}`,
            reuseExistingServer: false,
            timeout: 120_000,
        },
        {
            command: `cargo xtask build-web --example fusion-basic --release --base ${deploymentBase} && cargo xtask serve --example fusion-basic --release --port ${deploymentPort} --base ${deploymentBase} --spa`,
            url: `http://127.0.0.1:${deploymentPort}${deploymentBase}`,
            reuseExistingServer: false,
            timeout: 120_000,
        },
    ],
});
