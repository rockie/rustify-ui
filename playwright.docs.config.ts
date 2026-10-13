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

/// The documentation site's fixture, built by `tests/docs-ssg/serve.sh` and
/// served at the root and under a base by `rustify-docs preview`, and under
/// a base by a static host that sends no headers (`docs-meta`).
const docsPorts = { "docs-root": 4191, "docs-subpath": 4192, "docs-meta": 4193 } as const;
const docsBases = { "docs-root": "/", "docs-subpath": "/rustify-ui/", "docs-meta": "/rustify-ui/" } as const;
const docsVariants = { "docs-root": "root", "docs-subpath": "subpath", "docs-meta": "meta" } as const;
type Docs = keyof typeof docsPorts;
const docsProjects = Object.keys(docsPorts) as Docs[];
const docsUrl = (name: Docs) => `http://127.0.0.1:${docsPorts[name]}${docsBases[name]}`;

const componentsPorts = { "components-root": 4195, "components-subpath": 4196, "components-meta": 4197 } as const;
type Components = keyof typeof componentsPorts;
const componentsProjects = Object.keys(componentsPorts) as Components[];
const componentsBase = (name: Components) => name === "components-root" ? "/" : "/rustify-ui/";
const componentsUrl = (name: Components) => `http://127.0.0.1:${componentsPorts[name]}${componentsBase(name)}`;

type Server = "dom-only" | "dom-hydrate" | Docs | Components;

const serversOf: Record<string, Server[]> = {
    "dom-only": ["dom-only"],
    "dom-hydrate": ["dom-hydrate"],
    "docs-root": ["docs-root"],
    "docs-subpath": ["docs-subpath"],
    "docs-meta": ["docs-meta"],
    "components-root": ["components-root"],
    "components-subpath": ["components-subpath"],
    "components-meta": ["components-meta"],
    "components-webkit": ["components-subpath"],
    // Early engine regressions only: Playwright's Firefox and WebKit builds
    // are not the stable Firefox or Safari a reader runs (plan §9.3).
    "docs-firefox": ["docs-subpath"],
    "docs-webkit": ["docs-subpath"],
    // `rustify-docs dev`: each test starts its own on a copy of the fixture.
    "docs-dev": [],
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
            testIgnore: ["dom-only/hydration.spec.ts"],
            use: { baseURL: `http://127.0.0.1:${domOnlyPort}/` },
        },
        {
            name: "dom-hydrate",
            testMatch: ["dom-only/hydration.spec.ts"],
            use: { baseURL: "http://127.0.0.1:4194/" },
        },
        ...docsProjects.map((name) => ({
            name,
            testMatch: ["docs-ssg/**/*.spec.ts"],
            testIgnore: ["docs-ssg/dev*.spec.ts", "docs-ssg/islands.spec.ts", "docs-ssg/custom.spec.ts", "docs-ssg/official.spec.ts"],
            use: { baseURL: docsUrl(name) },
        })),
        ...componentsProjects.map((name) => ({
            name,
            testMatch: ["docs-ssg/islands.spec.ts", "docs-ssg/custom.spec.ts"],
            use: { baseURL: componentsUrl(name) },
        })),
        {
            name: "components-webkit",
            testMatch: ["docs-ssg/islands.spec.ts", "docs-ssg/custom.spec.ts"],
            use: { ...devices["Desktop Safari"], baseURL: componentsUrl("components-subpath"), launchOptions: {} },
        },
        {
            name: "docs-firefox",
            testMatch: ["docs-ssg/**/*.spec.ts"],
            testIgnore: ["docs-ssg/dev*.spec.ts", "docs-ssg/islands.spec.ts", "docs-ssg/custom.spec.ts", "docs-ssg/official.spec.ts"],
            use: { ...devices["Desktop Firefox"], baseURL: docsUrl("docs-subpath"), launchOptions: {} },
        },
        {
            name: "docs-webkit",
            testMatch: ["docs-ssg/**/*.spec.ts"],
            testIgnore: ["docs-ssg/dev*.spec.ts", "docs-ssg/islands.spec.ts", "docs-ssg/custom.spec.ts", "docs-ssg/official.spec.ts"],
            use: { ...devices["Desktop Safari"], baseURL: docsUrl("docs-subpath"), launchOptions: {} },
        },
        {
            name: "docs-dev",
            testMatch: ["docs-ssg/dev*.spec.ts"],
        },
    ],
    webServer: (
        [
            ["dom-only", `${xtask} serve --dir ${domOnlyDir} --port ${domOnlyPort}`, `http://127.0.0.1:${domOnlyPort}/`],
            ["dom-hydrate", `${xtask} serve --dir target/dom/hydration --port 4194`, "http://127.0.0.1:4194/"],
            ...docsProjects.map(
                (name) => [name, `sh tests/docs-ssg/serve.sh ${docsVariants[name]} ${docsPorts[name]}`, docsUrl(name)] as const,
            ),
            ...componentsProjects.map((name) => [name, `sh tests/docs-ssg/serve.sh components ${name === "components-root" ? "root" : name === "components-meta" ? "meta" : "subpath"} ${componentsPorts[name]}`, componentsUrl(name)] as const),
        ] as const
    )
        .filter(([server]) => wanted(server))
        // The docs servers may build the CLI first.
        .map(([, command, url]) => ({ command, url, reuseExistingServer: false, timeout: 600_000 })),
});
