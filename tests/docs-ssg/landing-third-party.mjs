// A stand-in for the third parties a landing page talks to, on two local
// origins: one a profile lists, one it does not. It implements only what
// the landing spec needs of them:
//   listed  GET  /tag.js     a measurement script with a dataLayer-style
//                            queue (`window.fakeTagQueue`); each event is
//                            sent by fetch, sendBeacon and an image, and it
//                            tries to load a script and a frame from the
//                            other origin, which the page's policy blocks
//           GET|POST /collect, GET /pixel.gif   what the script sends
//           POST /lead       a lead endpoint: records the form, then 303s
//                            back to `<Origin><sitePath>thanks/`
//           GET  /embed      an embeddable form, recording its query
//           GET  /log        everything recorded, as JSON; POST /reset clears it
//   blocked GET  /blocked.js, /frame   recorded if a browser ever asks
//
//   node landing-third-party.mjs <listedPort> <blockedPort> <sitePath>
// Port 0 picks a free port; the ports in use are printed as
// `third-party: <listed> <blocked>` once both listen.
import { createServer } from "node:http";

const [listedPort, blockedPort, sitePath] = process.argv.slice(2);
const log = [];
const record = (entry) => log.push({ at: Date.now(), ...entry });
let listed;
let blocked;

const tag = () => `(function () {
  var listed = "http://127.0.0.1:${listed}";
  var blocked = "http://127.0.0.1:${blocked}";
  function report(event) {
    var query = "?e=" + encodeURIComponent(event) + "&page=" + encodeURIComponent(location.pathname);
    fetch(listed + "/collect" + query + "&via=fetch", { mode: "no-cors" });
    navigator.sendBeacon(listed + "/collect" + query + "&via=beacon", event);
    new Image().src = listed + "/pixel.gif" + query + "&via=pixel";
    var script = document.createElement("script");
    script.src = blocked + "/blocked.js" + query;
    document.head.appendChild(script);
    var frame = document.createElement("iframe");
    frame.title = "blocked";
    frame.src = blocked + "/frame" + query;
    document.body.appendChild(frame);
  }
  var queue = window.fakeTagQueue = window.fakeTagQueue || [];
  var pending = queue.slice();
  queue.push = function (event) { report(event); return 0; };
  pending.forEach(report);
})();
`;

const pixel = Buffer.from("R0lGODlhAQABAIAAAAAAAP///yH5BAEAAAAALAAAAAABAAEAAAIBRAA7", "base64");

function body(request) {
    return new Promise((resolve) => {
        let text = "";
        request.on("data", (chunk) => (text += chunk));
        request.on("end", () => resolve(text));
    });
}

const listedServer = createServer(async (request, response) => {
    const url = new URL(request.url, "http://host");
    const query = Object.fromEntries(url.searchParams);
    if (url.pathname === "/tag.js") {
        response.writeHead(200, { "Content-Type": "text/javascript; charset=utf-8", "Cache-Control": "no-store" });
        response.end(tag());
    } else if (url.pathname === "/collect") {
        await body(request);
        record({ kind: query.via || "collect", event: query.e, page: query.page });
        response.writeHead(204, { "Access-Control-Allow-Origin": "*" }).end();
    } else if (url.pathname === "/pixel.gif") {
        record({ kind: "pixel", event: query.e, page: query.page });
        response.writeHead(200, { "Content-Type": "image/gif", "Cache-Control": "no-store" });
        response.end(pixel);
    } else if (url.pathname === "/lead" && request.method === "POST") {
        const form = Object.fromEntries(new URLSearchParams(await body(request)));
        const origin = request.headers.origin && request.headers.origin !== "null"
            ? request.headers.origin
            : new URL(request.headers.referer || "http://invalid").origin;
        record({ kind: "lead", form, origin });
        response.writeHead(303, { Location: `${origin}${sitePath}thanks/` }).end();
    } else if (url.pathname === "/embed") {
        record({ kind: "embed", query });
        response.writeHead(200, { "Content-Type": "text/html; charset=utf-8" });
        response.end(`<!doctype html><html lang="en"><title>Embedded form</title>
<form method="post" action="/lead"><label>Email <input name="email" type="email"></label>
<button>Sign up</button></form></html>`);
    } else if (url.pathname === "/log") {
        response.writeHead(200, { "Content-Type": "application/json", "Access-Control-Allow-Origin": "*" });
        response.end(JSON.stringify(log));
    } else if (url.pathname === "/reset" && request.method === "POST") {
        log.length = 0;
        response.writeHead(204).end();
    } else {
        response.writeHead(404).end();
    }
});

const blockedServer = createServer((request, response) => {
    const url = new URL(request.url, "http://host");
    if (url.pathname === "/blocked.js") {
        record({ kind: "blocked-script" });
        response.writeHead(200, { "Content-Type": "text/javascript" }).end("");
    } else if (url.pathname === "/frame") {
        record({ kind: "blocked-frame" });
        response.writeHead(200, { "Content-Type": "text/html" }).end("<!doctype html><title>blocked</title>");
    } else {
        response.writeHead(404).end();
    }
});

listedServer.listen(Number(listedPort), "127.0.0.1", () => {
    listed = listedServer.address().port;
    blockedServer.listen(Number(blockedPort), "127.0.0.1", () => {
        blocked = blockedServer.address().port;
        console.log(`third-party: ${listed} ${blocked}`);
    });
});
