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
const docsV2Site = process.env.RUSTIFY_DOCS_V2_SITE || "tests/docs-ssg/target/theme-v2-author";
const docsV2Dist = process.env.RUSTIFY_DOCS_V2_DIST || `${docsV2Site}-dist`;
const docsV2Cli = process.env.RUSTIFY_DOCS_V2_CLI || "target/docs-cli/rustify-docs";
const shellQuote = (value: string) => `'${value.replaceAll("'", "'\\''")}'`;

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

const officialPorts = { "official-root": 4198, "official-subpath": 4199, "official-meta": 4200 } as const;
type Official = keyof typeof officialPorts;
const officialProjects = Object.keys(officialPorts) as Official[];
const officialVariant = (name: Official) => name === "official-root" ? "root" : name === "official-meta" ? "meta" : "subpath";
const officialBase = (name: Official) => name === "official-root" ? "/" : "/rustify-ui/";
const officialUrl = (name: Official) => `http://127.0.0.1:${officialPorts[name]}${officialBase(name)}`;

const i18nProjects = [
    ["docs-i18n-root", "i18n", "root", 4201, "i18n.spec.ts"],
    ["docs-i18n-subpath", "i18n", "subpath", 4202, "i18n.spec.ts"],
    ["docs-i18n-meta", "i18n", "meta", 4203, "i18n.spec.ts"],
    ["components-i18n-root", "i18n-components", "root", 4207, "i18n-islands.spec.ts"],
    ["components-i18n-subpath", "i18n-components", "subpath", 4208, "i18n-islands.spec.ts"],
    ["components-i18n-meta", "i18n-components", "meta", 4209, "i18n-islands.spec.ts"],
] as const;
const i18nUrl = (variant: string, port: number) => `http://127.0.0.1:${port}${variant === "root" ? "/" : "/rustify-ui/"}`;

const searchProjects = [
    ["docs-search-root", "root", 4204, "search.spec.ts"],
    ["docs-search-subpath", "subpath", 4205, "search*.spec.ts"],
    ["docs-search-meta", "meta", 4206, "search.spec.ts"],
    ["docs-search-perf", "subpath", 4210, "search-perf.spec.ts"],
] as const;

type Server = "dom-only" | "dom-hydrate" | "colors-static" | "components-static" | "docs-theme-v2" | Docs | Components | Official | typeof i18nProjects[number][0] | typeof searchProjects[number][0];
const docsIgnore = ["docs-ssg/dev*.spec.ts", "docs-ssg/islands.spec.ts", "docs-ssg/custom.spec.ts", "docs-ssg/official.spec.ts", "docs-ssg/i18n*.spec.ts", "docs-ssg/search*.spec.ts", "docs-ssg/theme-v2.spec.ts"];

const serversOf: Record<string, Server[]> = {
    "colors-static": ["colors-static"],
    "components-static": ["components-static"],
    "docs-theme-v2": ["docs-theme-v2"],
    "dom-only": ["dom-only"],
    "dom-hydrate": ["dom-hydrate"],
    "docs-root": ["docs-root"],
    "docs-subpath": ["docs-subpath"],
    "docs-meta": ["docs-meta"],
    "components-root": ["components-root"],
    "components-subpath": ["components-subpath"],
    "components-meta": ["components-meta"],
    "components-webkit": ["components-subpath"],
    "official-root": ["official-root"],
    "official-subpath": ["official-subpath"],
    "official-meta": ["official-meta"],
    // Early engine regressions only: Playwright's Firefox and WebKit builds
    // are not the stable Firefox or Safari a reader runs (plan §9.3).
    "docs-firefox": ["docs-subpath"],
    "docs-webkit": ["docs-subpath"],
    // `rustify-docs dev`: each test starts its own on a copy of the fixture.
    "docs-dev": [],
    ...Object.fromEntries(i18nProjects.map(([name]) => [name, [name]])),
    ...Object.fromEntries(searchProjects.map(([name]) => [name, [name]])),
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
            name: "docs-theme-v2",
            testMatch: ["docs-ssg/theme-v2.spec.ts"],
            use: { baseURL: "http://127.0.0.1:4213/theme-v2/" },
        },
        {
            name: "components-static",
            testMatch: ["dom-components-ssr/static.spec.ts"],
            use: { baseURL: "http://127.0.0.1:4212/", javaScriptEnabled: false },
        },
        {
            name: "colors-static",
            testMatch: ["colors-static/colors.spec.ts"],
            use: { baseURL: "http://127.0.0.1:4211/" },
        },
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
            testIgnore: docsIgnore,
            use: { baseURL: docsUrl(name) },
        })),
        ...componentsProjects.map((name) => ({
            name,
            testMatch: ["docs-ssg/islands.spec.ts", "docs-ssg/custom.spec.ts"],
            use: { baseURL: componentsUrl(name) },
        })),
        ...officialProjects.map((name) => ({
            name,
            testMatch: ["docs-ssg/official.spec.ts"],
            use: { baseURL: officialUrl(name) },
        })),
        ...i18nProjects.map(([name, , variant, port, spec]) => ({
            name,
            testMatch: [`docs-ssg/${spec}`],
            use: { baseURL: i18nUrl(variant, port) },
        })),
        ...searchProjects.map(([name, variant, port, spec]) => ({
            name,
            testMatch: [`docs-ssg/${spec}`],
            testIgnore: name === "docs-search-subpath" ? ["docs-ssg/search-perf.spec.ts"] : [],
            use: { baseURL: i18nUrl(variant, port) },
        })),
        {
            name: "components-webkit",
            testMatch: ["docs-ssg/islands.spec.ts", "docs-ssg/custom.spec.ts"],
            use: { ...devices["Desktop Safari"], baseURL: componentsUrl("components-subpath"), launchOptions: {} },
        },
        {
            name: "docs-firefox",
            testMatch: ["docs-ssg/**/*.spec.ts"],
            testIgnore: docsIgnore,
            use: { ...devices["Desktop Firefox"], baseURL: docsUrl("docs-subpath"), launchOptions: {} },
        },
        {
            name: "docs-webkit",
            testMatch: ["docs-ssg/**/*.spec.ts"],
            testIgnore: docsIgnore,
            use: { ...devices["Desktop Safari"], baseURL: docsUrl("docs-subpath"), launchOptions: {} },
        },
        {
            name: "docs-dev",
            testMatch: ["docs-ssg/dev*.spec.ts"],
        },
    ],
    webServer: (
        [
            ["docs-theme-v2", `${shellQuote(docsV2Cli)} preview --dir ${shellQuote(docsV2Dist)} --port 4213`, "http://127.0.0.1:4213/theme-v2/"],
            ["components-static", `sh tests/dom-components-ssr/build.sh && ${xtask} serve --dir tests/dom-components-ssr/target/site --port 4212`, "http://127.0.0.1:4212/"],
            ["colors-static", `sh tests/colors-static/build.sh && ${xtask} serve --dir tests/colors-static/target/site --port 4211`, "http://127.0.0.1:4211/"],
            ["dom-only", `${xtask} serve --dir ${domOnlyDir} --port ${domOnlyPort}`, `http://127.0.0.1:${domOnlyPort}/`],
            ["dom-hydrate", `${xtask} serve --dir target/dom/hydration --port 4194`, "http://127.0.0.1:4194/"],
            ...docsProjects.map(
                (name) => [name, `sh tests/docs-ssg/serve.sh ${docsVariants[name]} ${docsPorts[name]}`, docsUrl(name)] as const,
            ),
            ...componentsProjects.map((name) => [name, `sh tests/docs-ssg/serve.sh components ${name === "components-root" ? "root" : name === "components-meta" ? "meta" : "subpath"} ${componentsPorts[name]}`, componentsUrl(name)] as const),
            ...officialProjects.map((name) => [name, `sh tests/docs-ssg/serve.sh official ${officialVariant(name)} ${officialPorts[name]}`, officialUrl(name)] as const),
            ...i18nProjects.map(([name, source, variant, port]) => [name, `sh tests/docs-ssg/serve.sh ${source} ${variant} ${port}`, i18nUrl(variant, port)] as const),
            ...searchProjects.map(([name, variant, port]) => [name, `sh tests/docs-ssg/serve.sh ${name === "docs-search-perf" ? "search-perf" : "search"} ${variant} ${port}`, i18nUrl(variant, port)] as const),
        ] as const
    )
        .filter(([server]) => wanted(server))
        // The docs servers may build the CLI first.
        .map(([, command, url]) => ({ command, url, reuseExistingServer: false, timeout: 600_000 })),
});
