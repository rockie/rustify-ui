// Validation only. Root supplies an already-serving installed-C official host.
// No Cargo/server startup, Safari settings, existing-tab discovery or screenshots.
import { spawn } from "node:child_process";
import { createHash } from "node:crypto";
import { readFile, writeFile, mkdir } from "node:fs/promises";
import { dirname } from "node:path";
import { createInterface } from "node:readline";
import { fileURLToPath } from "node:url";

const root = new URL("../../../../", import.meta.url);
const marker = "__RUSTIFY_SAFARI_VALUE__";
const sleep = (ms) => new Promise((resolve) => setTimeout(resolve, ms));
const sha256 = (bytes) => createHash("sha256").update(bytes).digest("hex");

// Standard MCP content may be JSON text or a plain action acknowledgement.
export function toolValue(result) {
  if (result?.isError) throw new Error(JSON.stringify(result));
  if (result?.structuredContent !== undefined) return result.structuredContent;
  const text = (result?.content || []).filter((part) => part.type === "text").map((part) => part.text).join("\n");
  try { return JSON.parse(text); } catch { return text; }
}

function markedValue(value) {
  if (typeof value === "string") {
    if (value.startsWith(marker)) return { found: true, value: JSON.parse(value.slice(marker.length)) };
    try { return markedValue(JSON.parse(value)); } catch { return { found: false }; }
  }
  if (value && typeof value === "object") {
    for (const child of Object.values(value)) {
      const answer = markedValue(child); if (answer.found) return answer;
    }
  }
  return { found: false };
}

export class SafariMcp {
  constructor() {
    this.responses = []; this.stderr = ""; this.pending = new Map(); this.issued = new Map(); this.sequence = 0; this.cancelled = false;
    this.child = spawn("/usr/bin/safaridriver", ["--mcp"], { stdio: ["pipe", "pipe", "pipe"] });
    this.lines = createInterface({ input: this.child.stdout });
    this.child.stdin.on("error", (error) => this.cancel(error));
    this.child.stderr.on("data", (chunk) => { this.stderr = (this.stderr + chunk).slice(-65536); });
    this.lines.on("line", (line) => {
      let response;
      try { response = JSON.parse(line); } catch { this.cancel(new Error(`invalid MCP NDJSON: ${line.slice(0, 200)}`)); return; }
      const issued = this.issued.get(response.id);
      if (!issued) return; // Do not retain unsolicited tab events or content.
      this.responses.push({ method: issued.method, params: issued.params, response });
      const request = this.pending.get(response.id);
      if (!request) return;
      this.pending.delete(response.id); clearTimeout(request.timer);
      if (response.error) request.reject(new Error(JSON.stringify(response.error)));
      else request.resolve(response.result);
    });
    this.child.on("error", (error) => this.cancel(error));
    this.child.on("exit", (code, signal) => this.cancel(new Error(`Safari MCP exited: ${code ?? signal}`)));
  }

  request(method, params, { cleanup = false, timeout = 30000 } = {}) {
    if (this.cancelled && !cleanup) return Promise.reject(new Error("Safari validation cancelled"));
    if (this.child.exitCode !== null || this.child.signalCode !== null) return Promise.reject(new Error("Safari MCP is no longer running"));
    const id = ++this.sequence;
    this.issued.set(id, { method, params });
    return new Promise((resolve, reject) => {
      const timer = setTimeout(() => this.cancel(new Error(`MCP timeout: ${method}/${params?.name || ""}`)), timeout);
      this.pending.set(id, { method, params, resolve, reject, timer });
      this.child.stdin.write(JSON.stringify({ jsonrpc: "2.0", id, method, params }) + "\n", (error) => {
        if (error) { clearTimeout(timer); this.pending.delete(id); reject(error); }
      });
    });
  }

  async initialize() {
    const result = await this.request("initialize", { protocolVersion: "2024-11-05", capabilities: {}, clientInfo: { name: "rustify-native-safari-validation", version: "1" } });
    this.child.stdin.write(JSON.stringify({ jsonrpc: "2.0", method: "notifications/initialized" }) + "\n");
    return result;
  }

  async tool(name, args, options) {
    const result = await this.request("tools/call", { name, arguments: args }, options);
    if (result?.isError) throw new Error(JSON.stringify(result));
    return result;
  }
  cancel(error) {
    this.cancelled = true;
    for (const request of this.pending.values()) { clearTimeout(request.timer); request.reject(error); }
    this.pending.clear();
  }
  async stop() {
    this.cancel(new Error("Safari MCP cleanup")); this.lines.close(); this.child.stdin.end();
    if (this.child.exitCode === null && this.child.signalCode === null) {
      this.child.kill("SIGTERM");
      await Promise.race([new Promise((resolve) => this.child.once("exit", resolve)), sleep(2000)]);
      if (this.child.exitCode === null && this.child.signalCode === null) this.child.kill("SIGKILL");
    }
  }
}

async function inputs() {
  const formal = JSON.parse(await readFile(new URL("formal-a-official-islands-c4-r1.json", import.meta.url), "utf8"));
  if (formal.categories.length !== 24 || new Set(formal.categories.map((entry) => entry.route)).size !== 24) throw new Error("formal A must supply 24 unique categories");
  const sourceHashes = {};
  for (const [path, expected] of Object.entries(formal.source_sha256)) {
    const actual = sha256(await readFile(new URL(path, root))); sourceHashes[path] = actual;
    if (actual !== expected) throw new Error(`formal A source drift: ${path}`);
  }
  const source = await readFile(new URL("measure-official-islands.mjs", import.meta.url), "utf8");
  const start = source.indexOf("  const until ="), end = source.indexOf("  let coordinator;", start);
  if (start < 0 || end < 0) throw new Error("cannot extract until/runProbes from the existing formal meter");
  const probes = source.slice(start, end);
  const config = await readFile(new URL("rustify-docs.toml", root), "utf8");
  const selected = config.match(/\[content\][\s\S]*?include\s*=\s*\[([\s\S]*?)\]/)?.[1];
  if (!selected) throw new Error("official Guides need the actual include list");
  const guides = [];
  for (const match of selected.matchAll(/"([^"/]+)\.md"/g)) {
    const file = `docs/${match[1]}.md`, bytes = await readFile(new URL(file, root));
    const title = bytes.toString("utf8").match(/^# ([^\r\n]+)$/m)?.[1];
    if (!title) throw new Error(`guide has no source H1: ${file}`);
    guides.push({ route: `${match[1]}/`, title }); sourceHashes[file] = sha256(bytes);
  }
  if (guides.length !== 12) throw new Error("official include list must supply twelve Guides");
  const categories = [];
  for (const entry of formal.categories) {
    const full = await readFile(new URL(entry.file, root), "utf8");
    const fn = full.match(new RegExp(`(?:\\/\\/\\/[^\\r\\n]*\\r?\\n)*pub fn ${entry.function}\\([^)]*\\)[\\s\\S]*?^\\}(?:\\r?\\n|$)`, "m"))?.[0];
    if (!fn) throw new Error(`missing formal source function: ${entry.file}::${entry.function}`);
    categories.push({ ...entry, source: [fn, full] });
  }
  const readme = await readFile(new URL("README.md", root), "utf8");
  const introduction = readme.slice(readme.indexOf("\n", readme.indexOf("# ")) + 1, readme.indexOf("\n## Quick start")).trim();
  const paragraphs = introduction.split(/\n\s*\n/).map((paragraph) => paragraph.replace(/\[([^\]]+)\]\([^)]*\)/g, "$1").replace(/`([^`]+)`/g, "$1").replace(/\s+/g, " ").trim());
  sourceHashes["README.md"] = sha256(readme);
  const shellPath = "crates/rustify-docs/src/shell.rs", shell = await readFile(new URL(shellPath, root), "utf8");
  const icons = [...shell.matchAll(/<link rel="icon" href="([^"]+)" \/>/g)].map((match) => match[1]);
  if (icons.length !== 1 || !icons[0].startsWith("data:image/svg+xml,")) throw new Error("official shell must declare its one inline SVG head icon");
  sourceHashes[shellPath] = sha256(shell);
  const statePath = "target/docs-ssg/m6-installed-c.json", stateBytes = await readFile(new URL(statePath, root));
  const installed = JSON.parse(stateBytes), version = installed.source_identity.match(/^rustify-docs (\d+\.\d+\.\d+)$/m)?.[1];
  if (!version || !installed.source_identity.includes(`source: ${installed.candidate_sha}`)) throw new Error("installed CLI source identity is incomplete");
  if (sha256(await readFile(installed.binary)) !== installed.binary_sha256) throw new Error("installed CLI binary differs from the canonical state");
  sourceHashes[statePath] = sha256(stateBytes);
  const pair = { candidate_sha: installed.candidate_sha, framework_revision: installed.framework_revision, cli_version: version, binary_sha256: installed.binary_sha256 };
  return { categories, guides, paragraphs, probes, sourceHashes, pair, icon: icons[0] };
}

// Inspector output is intentionally strict. Unknown/plain-text evidence is
// retained in raw responses and fails the gate, never treated as an empty list.
function rows(result, name) {
  const value = toolValue(result);
  if (Array.isArray(value)) return value;
  if (value && typeof value === "object") {
    const arrays = Object.values(value).filter(Array.isArray);
    if (arrays.length === 1) return arrays[0];
  }
  throw new Error(`unverified ${name} output shape; inspect raw MCP response`);
}

export async function verifyOfficialSafari({ origin, out }) {
  const base = new URL(origin);
  if (!/^https?:$/.test(base.protocol) || base.username || base.password || base.search || base.hash) throw new Error("origin must be an HTTP(S) host/base without credentials, query or fragment");
  if (!base.pathname.endsWith("/")) base.pathname += "/";
  const data = await inputs();
  const report = { scope: "native Safari post-hydration compatibility and public input results only; no original SSR identity/layout/performance claim", origin: base.href,
    pair: data.pair, source_sha256: data.sourceHashes, pages: [], small_checks: [], passed: false,
    limitations: ["No physical-phone, native-zoom, VoiceOver or full accessibility acceptance", "Early CSP/page errors rely on Inspector buffers; page event listeners start after navigation", "Saved owned-tab schema probe validates response wrappers only, not this run's compatibility"], cleanup: {} };
  let client, handle, viewport, themeBefore, resized = false, themeTouched = false, deadline;
  const save = async () => {
    report.raw_mcp_responses = client?.responses || []; report.mcp_stderr = client?.stderr || "";
    await mkdir(dirname(out), { recursive: true }); await writeFile(out, JSON.stringify(report, null, 2) + "\n");
  };
  const owned = async (name, args = {}, options) => {
    if (!handle) throw new Error("no owned Safari tab");
    await client.tool("switch_tab", { handle }, options);
    return client.tool(name, args, options);
  };
  const evaluate = async (body, options) => {
    const result = await owned("evaluate_javascript", { expression: `return ${JSON.stringify(marker)} + JSON.stringify((function(){${body}\n})());` }, options);
    const answer = markedValue(toolValue(result));
    if (!answer.found) throw new Error("unverified evaluate output shape: own return marker missing; inspect raw MCP response");
    return answer.value;
  };
  const key = async (value) => owned("page_interactions", { interactions: [{ type: "keyPress", value, purpose: "Verify the owned official page's native keyboard public result" }] });
  const inspect = async (expectedUrl, expectedStatus, category, headIcons) => {
    const requests = rows(await owned("list_network_requests", { tab_handle: handle, clear: true }), "network");
    const messages = rows(await owned("browser_console_messages", { tab_handle: handle, clear: true, level_filter: ["error"], limit: 500 }), "console");
    if (JSON.stringify(headIcons) !== JSON.stringify([data.icon])) throw new Error("actual head icon differs from the official shell declaration");
    const inline_resources = [];
    for (const request of requests) {
      if (typeof request.url !== "string" || !Number.isInteger(request.status)) throw new Error("unverified network URL/status schema; inspect raw MCP response");
      const url = new URL(request.url);
      if (url.protocol === "data:") {
        if (request.url !== data.icon || request.status !== 200 || request.mime_type !== "image/svg+xml") throw new Error(`unexpected inline resource: ${JSON.stringify(request)}`);
        inline_resources.push(request); continue;
      }
      if (!/^https?:$/.test(url.protocol) || url.origin !== base.origin || /makepad/i.test(request.url)) throw new Error(`foreign/Makepad request: ${request.url}`);
      if (request.status >= 400 && !(request.url === expectedUrl && expectedStatus === 404 && request.status === 404)) throw new Error(`failed request ${request.status}: ${request.url}`);
    }
    if (!requests.some((request) => request.url === expectedUrl && request.status === expectedStatus)) throw new Error("Inspector did not retain the expected main document request");
    const errors = messages.filter((message) => !(expectedStatus === 404 && message.url === expectedUrl && /404/.test(String(message.message))));
    if (errors.length) throw new Error(`Safari console errors: ${JSON.stringify(errors)}`);
    const wasm = requests.filter((request) => /\.wasm(?:[?#]|$)/.test(request.url));
    if (category ? wasm.length === 0 : wasm.length !== 0) throw new Error("wrong official-page wasm request presence");
    const details = [];
    for (const request of wasm) {
      const id = request.request_id ?? request.id;
      if (typeof id !== "string") throw new Error("unverified Inspector request id shape");
      const detail = toolValue(await owned("get_network_request", { request_id: id })); details.push(detail);
      const response = detail?.request, headers = response?.response_headers;
      if (!headers || typeof headers !== "object" || Array.isArray(headers)) throw new Error("unverified Inspector response-header shape; raw response retained");
      const mime = Object.entries(headers).find(([name]) => name.toLowerCase() === "content-type")?.[1];
      if (request.status !== 200 || response.status !== 200 || typeof mime !== "string" || mime.split(";")[0].trim() !== "application/wasm" || response.response_mime_type !== "application/wasm") throw new Error(`bad observed wasm MIME/status: ${JSON.stringify(detail)}`);
    }
    return { requests: requests.filter((request) => /^https?:/.test(request.url)), inline_resources, messages, wasm_details: details };
  };
  try {
    client = new SafariMcp();
    deadline = setTimeout(() => client.cancel(new Error("native Safari run exceeded the 15-minute hard deadline")), 15 * 60000);
    report.initialize = await client.initialize();
    const created = toolValue(await client.tool("create_tab", { url: "about:blank" }));
    if (typeof created?.handle !== "string") throw new Error("create_tab did not return its owned handle");
    handle = created.handle; report.owned_tab = handle;
    viewport = await evaluate("return {width:innerWidth,height:innerHeight};");
    const manifestResponse = await fetch(new URL("rustify-docs-manifest.json", base), { signal: AbortSignal.timeout(10000) });
    if (!manifestResponse.ok) throw new Error(`host manifest: ${manifestResponse.status}`);
    report.manifest = await manifestResponse.json();
    if (report.manifest.cli_version !== data.pair.cli_version) throw new Error("serving manifest differs from the runtime frozen pair's CLI version");
    const pages = [{ route: "./", title: "Rustify UI", home: true }, ...data.guides, ...data.categories, { route: "missing/native/safari/deep/", title: "Page not found", status: 404 }];
    for (const entry of pages) {
      if (client.cancelled) break;
      const url = new URL(entry.route, base).href, row = { route: entry.route, url, title: entry.title, category: entry.name ?? null, passed: false };
      report.pages.push(row); console.log(`Safari: ${entry.route}`);
      try {
        // Subscribe/clear while still on owned about:blank before the first navigation.
        await owned("list_network_requests", { tab_handle: handle, clear: true });
        await owned("browser_console_messages", { tab_handle: handle, clear: true, level_filter: ["error"], limit: 500 });
        await owned("navigate_to_url", { tab_uuid: handle, url });
        const readyUntil = Date.now() + 30000;
        do {
          row.page = await evaluate(`const blocks=[...document.querySelectorAll('[data-rustify-island]')],coordinator=window.__rustify_docs_islands;
return {url:location.href,lang:document.documentElement.lang,title:[...document.querySelectorAll('h1')].map(n=>n.textContent.trim()),policy:document.querySelector('meta[http-equiv="Content-Security-Policy"]')?.content,ua:navigator.userAgent,ready:document.readyState==='complete'&&blocks.every(n=>n.getAttribute('data-rd-island-state')==='ready'),islands:blocks.length,icons:[...document.head.querySelectorAll('link[rel~="icon"]')].map(n=>n.getAttribute('href')),
lifecycle:{visibility_state:document.visibilityState,hidden:document.hidden,has_focus:document.hasFocus(),ready_state:document.readyState,coordinator:coordinator?{state:coordinator.state(),failure:coordinator.failure(),stats:coordinator.stats()}:null,
blocks:blocks.map(n=>{const field=n.querySelector('fieldset[data-rd-island-controls]');return {key:n.getAttribute('data-rustify-island'),state:n.getAttribute('data-rd-island-state'),mounted:field?.firstElementChild?.getAttribute('data-rustify-mounted'),inert:field?.inert,disabled:field?.disabled,aria_busy:field?.getAttribute('aria-busy')};})}};`);
          if (row.page.lifecycle.hidden || row.page.lifecycle.visibility_state === "hidden") {
            row.environment_gap = "owned Safari document is hidden; a real visible tab is required for rAF and native input";
            throw new Error(`native Safari environment: ${row.environment_gap}`);
          }
          if (row.page.lifecycle.coordinator?.state === "failed") throw new Error(`island coordinator failed: ${row.page.lifecycle.coordinator.failure}`);
          if (row.page.ready) break;
          if (Date.now() >= readyUntil) throw new Error("official document/island did not become ready within 30 seconds");
          await sleep(100);
        } while (!client.cancelled);
        if (!row.page.ready || row.page.lang !== "en" || row.page.title.length !== 1 || row.page.title[0] !== entry.title || row.page.islands !== (entry.name ? 1 : 0)) throw new Error("wrong actual English official page/island");
        if (!row.page.policy?.includes("script-src") || row.page.policy.includes("'unsafe-inline'") || row.page.policy.includes("frame-ancestors")) throw new Error("bad rendered meta CSP");
        await evaluate("window.__officialSafariErrors=[];for(const type of ['error','unhandledrejection','securitypolicyviolation'])window.addEventListener(type,e=>window.__officialSafariErrors.push({type,message:String(e.message||e.reason||e.violatedDirective),blocked:e.blockedURI||null}));return true;");
        row.links = await evaluate(`const links=[...document.querySelectorAll('.rd-sidebar-wide a')];return links.map(n=>({href:n.getAttribute('href'),title:n.textContent.trim(),current:n.getAttribute('aria-current')}));`);
        if (!entry.home) {
          const expected = [...data.guides, ...data.categories].map((item) => new URL(item.route, base).pathname);
          if (JSON.stringify(row.links.map((link) => link.href)) !== JSON.stringify(expected)) throw new Error("ordinary navigation differs from source-derived 36 destinations");
          if (entry.status !== 404 && row.links.filter((link) => link.current === "page").length !== 1) throw new Error("current ordinary destination is not unique");
        }
        if (entry.home) {
          themeBefore = await evaluate(`const key=${JSON.stringify(`rustify-docs:mode:${base.pathname}`)};try{return {key,value:localStorage.getItem(key)};}catch{return {unavailable:true};}`);
          const home = await evaluate(`return {paragraphs:[...document.querySelectorAll('.rd-home-prose p')].map(n=>n.textContent.replace(/\\s+/g,' ').trim()),actions:[...document.querySelectorAll('.rd-actions a')].map(n=>({text:n.textContent.trim(),href:n.getAttribute('href')})),islands:document.querySelectorAll('[data-rustify-island]').length};`);
          if (JSON.stringify(home.paragraphs) !== JSON.stringify(data.paragraphs) || home.actions.length !== 2 || home.actions[0].href !== new URL("quickstart/", base).pathname || home.actions[1].href !== new URL(data.categories[0].route, base).pathname) throw new Error("home README / ordinary actions differ from original sources");
          row.home = home;
        }
        if (entry.name) {
          const actual = await evaluate("return {path:document.querySelector('[data-rd-source-path]')?.textContent.trim(),sources:[...document.querySelectorAll('.rd-demo-source template.rd-code-raw')].map(n=>n.content.textContent)};");
          if (actual.path !== entry.file || JSON.stringify(actual.sources) !== JSON.stringify(entry.source)) throw new Error("function/full-file source differs from original bytes");
          row.source_exact = true;
          await evaluate(`const entry=${JSON.stringify({ name: entry.name, demo: entry.demo })},meter={probes:[],metadata:null,done:false,failure:null,hydrationWindowEnd:null};const queue=[],pending=new Map();let sequence=0;window.__officialNativeKey=value=>new Promise((resolve,reject)=>{const id=++sequence;pending.set(id,{resolve,reject});queue.push({id,value});});window.__officialSafari={snapshot:()=>({done:meter.done,failure:meter.failure,probes:meter.probes,key:queue[0]||null}),resolveKey:(id,error)=>{if(queue[0]?.id!==id)throw new Error('native key queue changed');queue.shift();const promise=pending.get(id);pending.delete(id);if(error)promise.reject(new Error(error));else promise.resolve();}};${data.probes}\ndocument.fonts.ready.then(()=>requestAnimationFrame(()=>requestAnimationFrame(()=>{meter.hydrationWindowEnd=performance.now();runProbes();})));return true;`);
          const inputUntil = Date.now() + 90000;
          do {
            row.input = await evaluate("return window.__officialSafari.snapshot();");
            if (row.input.done) break;
            if (Date.now() >= inputUntil) throw new Error("public native-Safari input probes exceeded 90 seconds");
            if (row.input.key) {
              const pending = row.input.key;
              try { await key(pending.value); await evaluate(`window.__officialSafari.resolveKey(${pending.id},null);return true;`); }
              catch (error) { await evaluate(`window.__officialSafari.resolveKey(${pending.id},${JSON.stringify(String(error))});return true;`); throw error; }
            } else await sleep(50);
          } while (!client.cancelled);
          if (!row.input.done || row.input.failure || row.input.probes.some((probe) => probe.failure)) throw new Error(row.input.failure || "incomplete public input probes");
        }
        row.audit_events = await evaluate("return window.__officialSafariErrors;");
        if (row.audit_events.length) throw new Error(`page/CSP events: ${JSON.stringify(row.audit_events)}`);
        row.inspector = await inspect(url, entry.status || 200, Boolean(entry.name), row.page.icons);
        row.passed = true;
      } catch (error) {
        row.failure = String(error);
        if (!client.cancelled) {
          try { row.failure_inspector = { network: toolValue(await owned("list_network_requests", { tab_handle: handle, clear: true })), console: toolValue(await owned("browser_console_messages", { tab_handle: handle, clear: true, level_filter: ["error"], limit: 500 })) }; }
          catch (inspectionError) { row.inspection_error = String(inspectionError); }
        }
      }
      await save(); console.log(`Safari: ${entry.route} ${row.passed ? "passed" : row.failure}`);
      if (!row.passed) {
        report.stopped_after_failure = entry.route;
        report.unrun_routes = pages.slice(report.pages.length).map((page) => page.route);
        throw new Error(`stopped after first verifier failure at ${entry.route}: ${row.failure}`);
      }
    }
    // Necessary compatibility checks only, without a new visual/screenshot pass.
    await owned("navigate_to_url", { tab_uuid: handle, url: base.href });
    await owned("page_interactions", { interactions: [{ type: "click", text: "Quick start", purpose: "Follow the owned official home's ordinary reading link" }] });
    const followed = await evaluate("return {url:location.href,title:document.querySelector('h1')?.textContent.trim()};");
    report.small_checks.push({ kind: "ordinary home link", ...followed, passed: followed.url === new URL(data.guides[0].route, base).href && followed.title === data.guides[0].title });
    await owned("navigate_to_url", { tab_uuid: handle, url: new URL("components/data-table/", base).href });
    if (!themeBefore) throw new Error("no owned-origin theme baseline captured; refuse changing persisted theme");
    for (const mode of ["light", "dark"]) {
      themeTouched = true;
      await evaluate(`document.querySelector('[data-rd-mode="${mode}"]').click();return true;`); await sleep(200);
      const colors = await evaluate(`const field=document.querySelector('fieldset[data-rd-island-controls]'),grid=field?.querySelector('[role="grid"]'),cell=grid?.querySelector('[role="gridcell"]');return {mode:document.documentElement.getAttribute('data-theme'),body:getComputedStyle(document.body).backgroundColor,foreground:getComputedStyle(document.body).color,grid:grid?getComputedStyle(grid).backgroundColor:null,cell:cell?getComputedStyle(cell).color:null};`);
      report.small_checks.push({ kind: "theme", ...colors, passed: colors.mode === mode && colors.body === colors.grid && colors.foreground === colors.cell && colors.foreground !== colors.body });
    }
    await owned("navigate_to_url", { tab_uuid: handle, url: new URL(data.guides[0].route, base).href });
    for (const width of [375, 320]) {
      resized = true; await owned("set_viewport_size", { width, height: 812 });
      const layout = await evaluate("return {width:innerWidth,overflow:document.documentElement.scrollWidth-innerWidth,sidebarDetails:!!document.querySelector('.rd-sidebar details')};");
      report.small_checks.push({ kind: "narrow no-overflow", requested_width: width, ...layout, passed: layout.width === width && layout.overflow <= 0 && layout.sidebarDetails });
    }
    await evaluate("document.querySelector('.rd-sidebar summary').focus();return true;"); await key("Enter");
    report.small_checks.push({ kind: "native details Enter", passed: await evaluate("return document.querySelector('.rd-sidebar details')?.open===true;") });
    report.passed = report.pages.length === 38 && report.pages.every((page) => page.passed) && report.small_checks.every((check) => check.passed);
  } catch (error) { report.failure = String(error); }
  finally {
    if (deadline) clearTimeout(deadline);
    if (client) {
      client.cancel(new Error("validation finished; only owned-tab cleanup is allowed"));
      if (handle) {
        if (themeTouched && themeBefore && !themeBefore.unavailable) {
          try { await evaluate(`const backup=${JSON.stringify(themeBefore)};if(backup.value===null)localStorage.removeItem(backup.key);else localStorage.setItem(backup.key,backup.value);return true;`, { cleanup: true, timeout: 2000 }); report.cleanup.theme_storage_restored = true; }
          catch (error) { report.cleanup.theme_restore_error = String(error); }
        }
        if (resized && viewport) {
          try { await owned("set_viewport_size", viewport, { cleanup: true, timeout: 2000 }); report.cleanup.viewport_restored = true; }
          catch (error) { report.cleanup.viewport_restore_error = String(error); }
        }
        try {
          const closed = toolValue(await client.tool("close_tab", { handle }, { cleanup: true, timeout: 3000 }));
          if (closed !== `Closed tab ${handle}`) throw new Error("owned-tab close acknowledgement differs from the saved native probe");
          report.cleanup.owned_tab_closed = true;
        }
        catch (error) { report.cleanup.close_error = String(error); }
      }
      await client.stop(); report.raw_mcp_responses = client.responses; report.mcp_stderr = client.stderr;
    }
    if (report.cleanup.close_error || report.cleanup.viewport_restore_error || report.cleanup.theme_restore_error) report.passed = false;
    await save();
  }
  return report;
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  const args = process.argv.slice(2), options = {};
  for (let index = 0; index < args.length; index += 2) {
    if (!["--origin", "--out"].includes(args[index]) || !args[index + 1]) throw new Error("usage: --origin <already-serving-host/base/> --out <report.json>");
    options[args[index].slice(2)] = args[index + 1];
  }
  if (!options.origin || !options.out) throw new Error("--origin and --out are required");
  const report = await verifyOfficialSafari(options);
  console.log(`Native Safari compatibility: ${report.passed ? "passed" : "failed/unverified"}; ${report.pages.filter((page) => page.passed).length}/${report.pages.length} pages; owned-tab cleanup ${report.cleanup.owned_tab_closed === true}`);
  if (!report.passed) process.exitCode = 1;
}
