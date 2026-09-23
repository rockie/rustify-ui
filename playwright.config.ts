import { defineConfig, devices } from "@playwright/test";

// Exported so a probe that has to reach a second example's server names the
// port once rather than repeating it.
export const fusionPort = 4173;
const workbenchPort = 4174;
// A third server for the deployment checks: the same build, served under a
// sub-path, with a switch that makes it serve a broken one on request.
const deploymentPort = 4175;
const catalogPort = 4176;
// The workbench again, under a sub-path: the same deep-link checks have to
// pass at the root and below it, and the difference is the deployment.
const deepLinkPort = 4177;
// The fourth example: a hundred thousand rows and ten thousand objects.
const dataPort = 4178;
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
            testMatch: ["p3-restart.spec.ts", "m1-probes.spec.ts", "m2-runtime.spec.ts", "m3-state.spec.ts", "m4-geometry.spec.ts", "m4-overlay.spec.ts", "m6-mainpath.spec.ts", "p2-navigation.spec.ts", "p3-instances.spec.ts", "p3-b0.spec.ts", "p3-idle.spec.ts", "p3-memory.spec.ts", "p3-endurance.spec.ts"],
            use: { baseURL: `http://127.0.0.1:${fusionPort}/` },
        },
        {
            name: "property-workbench",
            testMatch: ["p3-restart.spec.ts", "m3-workbench.spec.ts", "m5-text.spec.ts", "m5-semantics.spec.ts", "m6-theme.spec.ts", "m6-async.spec.ts", "m6-components.spec.ts", "m7-recovery.spec.ts", "m8-baseline.spec.ts", "m8-network.spec.ts", "m8-endurance.spec.ts", "p2-form.spec.ts", "p2-deeplink.spec.ts", "p2-workspace.spec.ts", "p2-drag.spec.ts", "p2-clipboard-files.spec.ts", "p2-zoom.spec.ts", "p2-ime.spec.ts", "p3-policy.spec.ts", "p3-faults.spec.ts", "p3-diagnostics.spec.ts"],
            use: { baseURL: `http://127.0.0.1:${workbenchPort}/` },
        },
        {
            name: "component-catalog",
            testMatch: ["p3-restart.spec.ts", "p2-catalog.spec.ts", "p2-theme.spec.ts", "p2-semantics.spec.ts", "p2-i18n.spec.ts", "p2-reflow.spec.ts", "p2-a11y.spec.ts", "p3-policy.spec.ts"],
            use: { baseURL: `http://127.0.0.1:${catalogPort}/` },
        },
        {
            name: "data-workbench",
            testMatch: ["p3-restart.spec.ts", "p3-probes.spec.ts", "p3-table.spec.ts", "p3-jobs.spec.ts", "p3-scene.spec.ts", "p3-memory.spec.ts"],
            use: { baseURL: `http://127.0.0.1:${dataPort}/` },
        },
        // The budget gate runs thirty cold loads and thirty hot ones, so it is
        // its own project rather than a slow tail on every workbench run. Same
        // build, same server: what makes it separate is how long it takes.
        {
            name: "budget",
            testMatch: ["p2-budget.spec.ts"],
            use: { baseURL: `http://127.0.0.1:${workbenchPort}/` },
        },
        // R29's B0 column, on the example B0 actually is. Its own project for
        // the same reason `budget` is: sixty loads take a quarter of an hour,
        // which has no business being a slow tail on the fusion-basic run.
        {
            name: "budget-minimal",
            testMatch: ["p3-budget-minimal.spec.ts"],
            use: { baseURL: `http://127.0.0.1:${fusionPort}/` },
        },
        // R30 AC2 over B2 and AC3 over the jobs. Headless: B2 is DOM work, so
        // the software rasteriser is not what is under test.
        {
            name: "budget-data",
            testMatch: ["p3-budget-data.spec.ts"],
            use: { baseURL: `http://127.0.0.1:${dataPort}/` },
        },
        // R30 AC2 over B3, on headed Chrome and nowhere else (A-3, M1 probe
        // 2): the same scene measures p95 50.10 ms under SwiftShader against
        // 17.60 ms here, so a result from the software rasteriser would be
        // about the rasteriser. CI does not run this project - see
        // `.github/workflows` - and the file skips itself if it is ever
        // started headless rather than reporting a rasteriser as a red build.
        {
            name: "budget-scene",
            testMatch: ["p3-budget-scene.spec.ts"],
            use: {
                baseURL: `http://127.0.0.1:${dataPort}/`,
                channel: "chrome",
                headless: false,
            },
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
            command: `mbx xtask serve --example fusion-basic --release --port ${fusionPort}`,
            url: `http://127.0.0.1:${fusionPort}/`,
            reuseExistingServer: false,
            timeout: 120_000,
        },
        {
            command: `mbx xtask serve --example property-workbench --release --port ${workbenchPort} --spa`,
            url: `http://127.0.0.1:${workbenchPort}/`,
            reuseExistingServer: false,
            timeout: 120_000,
        },
        {
            command: `mbx xtask serve --example component-catalog --release --port ${catalogPort}`,
            url: `http://127.0.0.1:${catalogPort}/`,
            reuseExistingServer: false,
            timeout: 120_000,
        },
        {
            command: `mbx xtask serve --example data-workbench --release --port ${dataPort} --spa`,
            url: `http://127.0.0.1:${dataPort}/`,
            reuseExistingServer: false,
            timeout: 120_000,
        },
        {
            command: `mbx xtask build-web --example property-workbench --release --base ${deploymentBase} && mbx xtask serve --example property-workbench --release --port ${deepLinkPort} --base ${deploymentBase} --spa`,
            url: `http://127.0.0.1:${deepLinkPort}${deploymentBase}`,
            reuseExistingServer: false,
            timeout: 120_000,
        },
        {
            command: `mbx xtask build-web --example fusion-basic --release --base ${deploymentBase} && mbx xtask serve --example fusion-basic --release --port ${deploymentPort} --base ${deploymentBase} --spa`,
            url: `http://127.0.0.1:${deploymentPort}${deploymentBase}`,
            reuseExistingServer: false,
            timeout: 120_000,
        },
    ],
});
