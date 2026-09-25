import { defineConfig, devices } from "@playwright/test";
import { existsSync } from "node:fs";
import path from "node:path";
import { tierFilter } from "../tier";

const root = path.resolve(__dirname, "../..");
const reference = path.join(root, "ref/Vellum-main");
const artifactScope = process.env.VELLUM_ARTIFACT_SCOPE ?? "suite";

export default defineConfig({
    testDir: __dirname,
    outputDir: path.join(root, "test-results/vellum/artifacts", artifactScope),
    timeout: 120_000,
    fullyParallel: false,
    workers: 1,
    retries: 0,
    reporter: [["list"], ["json", { outputFile: path.join(root, "test-results/vellum", `${artifactScope}-results.json`) }]],
    ...tierFilter(),
    use: {
        ...devices["Desktop Chrome"],
        viewport: { width: 1600, height: 1000 },
        deviceScaleFactor: 1,
        trace: "retain-on-failure",
        screenshot: "only-on-failure",
        launchOptions: { args: ["--enable-unsafe-swiftshader", "--use-angle=swiftshader"] },
    },
    projects: [{ name: "vellum", use: { baseURL: "http://127.0.0.1:4179/" } }],
    webServer: [
        {
            command: "mbx xtask serve --example vellum --release --port 4179",
            cwd: root,
            url: "http://127.0.0.1:4179/",
            reuseExistingServer: false,
            timeout: 120_000,
        },
        ...(existsSync(reference) ? [{
            command: "python3 -m http.server 4180 --directory ref/Vellum-main --bind 127.0.0.1",
            cwd: root,
            url: "http://127.0.0.1:4180/",
            reuseExistingServer: false,
            timeout: 30_000,
        }] : []),
    ],
});
