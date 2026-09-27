import { defineConfig, devices } from "@playwright/test";
import { TIER, tierFilter } from "./tests/tier";

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

type Server = "fusion" | "workbench" | "catalog" | "data" | "deep" | "deployment";

// The servers each project talks to. A run that names its projects starts only
// these: six servers for a one-project run is most of its start-up. The
// fusion server is on the data project's list for the probe that boots a
// second example, which is an evidence test.
const serversOf: Record<string, Server[]> = {
    "fusion-basic": ["fusion"],
    "property-workbench": ["workbench"],
    "component-catalog": ["catalog"],
    "data-workbench": TIER === "regression" ? ["data"] : ["data", "fusion"],
    budget: ["workbench"],
    "budget-minimal": ["fusion"],
    "budget-data": ["data"],
    "budget-scene": ["data"],
    "workbench-deep": ["deep"],
    deployment: ["deployment"],
};

/// The projects named on the command line, in either of the forms the CLI
/// takes (`--project=a` and `--project a b`), or nothing if none are named.
function requestedProjects(argv: string[]): string[] {
    const names: string[] = [];
    for (let i = 0; i < argv.length; i++) {
        const arg = argv[i];
        if (arg.startsWith("--project=")) {
            names.push(arg.slice("--project=".length));
        } else if (arg === "--project") {
            while (i + 1 < argv.length && !argv[i + 1].startsWith("-")) {
                names.push(argv[++i]);
            }
        }
    }
    return names;
}

/// Every server, unless the run names projects this file knows; then only
/// theirs. An unknown name starts everything and lets the runner report it.
function neededServers(): Set<Server> | null {
    const names = requestedProjects(process.argv);
    if (names.length === 0 || names.some((name) => !(name in serversOf))) {
        return null;
    }
    return new Set(names.flatMap((name) => serversOf[name]));
}

const needed = neededServers();
const wanted = (server: Server) => needed === null || needed.has(server);

// Measurement projects exist only where measurements run.
const measuring = TIER !== "regression";

export default defineConfig({
    testDir: "./tests/browser",
    // A cold page compiles a 7.7 MB wasm module and boots its regions; the
    // 30 s default leaves the shorter probes no headroom over that.
    timeout: 120_000,
    fullyParallel: false,
    workers: 1,
    retries: 0,
    reporter: [["list"], ["json", { outputFile: "test-results/browser.json" }]],
    ...tierFilter(),
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
        ...(measuring ? [{
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
        }] : []),
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
    // The two sub-path servers serve builds made for their base, which a
    // plain `build-web` does not produce; `serve` refuses to start without
    // one and prints the command that makes it.
    webServer: ([
        ["fusion", `mbx xtask serve --example fusion-basic --release --port ${fusionPort}`, `http://127.0.0.1:${fusionPort}/`],
        ["workbench", `mbx xtask serve --example property-workbench --release --port ${workbenchPort} --spa`, `http://127.0.0.1:${workbenchPort}/`],
        ["catalog", `mbx xtask serve --example component-catalog --release --port ${catalogPort}`, `http://127.0.0.1:${catalogPort}/`],
        ["data", `mbx xtask serve --example data-workbench --release --port ${dataPort} --spa`, `http://127.0.0.1:${dataPort}/`],
        ["deep", `mbx xtask serve --example property-workbench --release --port ${deepLinkPort} --base ${deploymentBase} --spa`, `http://127.0.0.1:${deepLinkPort}${deploymentBase}`],
        ["deployment", `mbx xtask serve --example fusion-basic --release --port ${deploymentPort} --base ${deploymentBase} --spa`, `http://127.0.0.1:${deploymentPort}${deploymentBase}`],
    ] as const)
        .filter(([server]) => wanted(server))
        .map(([, command, url]) => ({ command, url, reuseExistingServer: false, timeout: 120_000 })),
});
