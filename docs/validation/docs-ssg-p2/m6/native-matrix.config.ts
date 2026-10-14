import path from "node:path";
import base from "../../../../playwright.docs.config";

const root = path.resolve(__dirname, "../../../..");
const port = Number(process.env.RUSTIFY_M6_META_PORT);
if (!Number.isInteger(port) || port <= 0) throw new Error("RUSTIFY_M6_META_PORT must name the allocated validation port");
const replacePort = (url: string) => url.replace("127.0.0.1:4200", `127.0.0.1:${port}`);
export default {
    ...base,
    testDir: path.join(root, "tests"),
    outputDir: path.join(root, "test-results/docs"),
    reporter: [["list"], ["json", { outputFile: path.join(root, "test-results/docs/results.json") }]],
    projects: base.projects?.map(project => project.name === "official-meta"
        ? { ...project, use: { ...project.use, baseURL: replacePort(project.use!.baseURL!) } }
        : project),
    webServer: (Array.isArray(base.webServer) ? base.webServer : [base.webServer]).filter(Boolean).map(server => ({
        ...server!, cwd: root,
        command: server!.command.endsWith("official meta 4200") ? server!.command.replace(/4200$/, String(port)) : server!.command,
        url: replacePort(server!.url!),
    })),
};
