import { defineConfig, devices } from "@playwright/test";
import { tierFilter } from "./tests/tier";

// The DOM-only SDK and the documentation tool, kept apart from the GPU
// examples' config: none of these projects needs a GPU build, and none of
// their servers shares a port with `playwright.config.ts` (4173-4187).
//
// One project per command, and one command at a time, as for the root config.

/// The DOM consumer's product. `mbx xtask build-dom` (or `docs-verify --group
/// features`) writes the in-repository build here; `DOM_ONLY_DIR` points the
/// same checks at another build of the same fixture, such as the one built
/// outside the repository from a git commit.
const domOnlyDir = process.env.DOM_ONLY_DIR || "target/dom/dom-consumer";
const domOnlyPort = Number(process.env.DOM_ONLY_PORT || 4190);
const xtask = process.env.RUSTIFY_XTASK || "mbx xtask";

type Server = "dom-only";

const serversOf: Record<string, Server[]> = {
    "dom-only": ["dom-only"],
};

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

const requested = requestedProjects(process.argv);
const needed =
    requested.length === 0 || requested.some((name) => !(name in serversOf))
        ? null
        : new Set(requested.flatMap((name) => serversOf[name]));
const wanted = (server: Server) => needed === null || needed.has(server);

export default defineConfig({
    testDir: "./tests",
    timeout: 60_000,
    expect: { timeout: 10_000 },
    fullyParallel: false,
    workers: 1,
    retries: 0,
    outputDir: "test-results/docs",
    reporter: [["list"], ["json", { outputFile: "test-results/docs/results.json" }]],
    ...tierFilter(),
    use: {
        trace: "retain-on-failure",
        ...devices["Desktop Chrome"],
        launchOptions: { executablePath: process.env.RUSTIFY_CHROMIUM || undefined },
    },
    projects: [
        {
            name: "dom-only",
            testMatch: ["dom-only/**/*.spec.ts"],
            use: { baseURL: `http://127.0.0.1:${domOnlyPort}/` },
        },
    ],
    webServer: ([["dom-only", `${xtask} serve --dir ${domOnlyDir} --port ${domOnlyPort}`, `http://127.0.0.1:${domOnlyPort}/`]] as const)
        .filter(([server]) => wanted(server))
        .map(([, command, url]) => ({ command, url, reuseExistingServer: false, timeout: 120_000 })),
});
