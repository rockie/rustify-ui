// A static host of the simplest kind, as GitHub Pages serves a site: files
// by path under one base, a directory's index.html, a redirect for a
// directory named without its slash, the site's 404.html with a real 404
// for anything else - and no response header of its own, so a page's only
// content security policy is the <meta> in it.
//
//   node static-host.mjs <dir> <port> <base>
import { createServer } from "node:http";
import { readFile, stat } from "node:fs/promises";
import { extname, join, resolve, sep } from "node:path";

const [dir, port, base] = process.argv.slice(2);
const root = resolve(dir);
const types = {
    ".html": "text/html; charset=utf-8",
    ".css": "text/css; charset=utf-8",
    ".js": "text/javascript; charset=utf-8",
    ".json": "application/json",
    ".svg": "image/svg+xml",
    ".png": "image/png",
    ".xml": "application/xml",
    ".txt": "text/plain; charset=utf-8",
};

async function isFile(path) {
    try {
        return (await stat(path)).isFile();
    } catch {
        return false;
    }
}

async function isDirectory(path) {
    try {
        return (await stat(path)).isDirectory();
    } catch {
        return false;
    }
}

async function notFound(response) {
    const body = await readFile(join(root, "404.html"));
    response.writeHead(404, { "Content-Type": types[".html"] });
    response.end(body);
}

createServer(async (request, response) => {
    let path;
    try {
        path = decodeURIComponent(new URL(request.url, "http://host").pathname);
    } catch {
        response.writeHead(400).end();
        return;
    }
    if (`${path}/` === base) {
        response.writeHead(301, { Location: base }).end();
        return;
    }
    if (!path.startsWith(base)) {
        response.writeHead(404, { "Content-Type": "text/plain" }).end("not found\n");
        return;
    }
    let file = resolve(root, `.${sep}${path.slice(base.length)}`);
    if (file !== root && !file.startsWith(root + sep)) {
        await notFound(response);
        return;
    }
    if (path.endsWith("/")) {
        file = join(file, "index.html");
    } else if (await isDirectory(file)) {
        response.writeHead(301, { Location: `${request.url.split("?")[0]}/` }).end();
        return;
    }
    if (!(await isFile(file))) {
        await notFound(response);
        return;
    }
    const body = await readFile(file);
    response.writeHead(200, { "Content-Type": types[extname(file)] || "application/octet-stream" });
    response.end(request.method === "HEAD" ? undefined : body);
}).listen(Number(port), "127.0.0.1", () => {
    console.log(`static host: http://127.0.0.1:${port}${base}`);
});
