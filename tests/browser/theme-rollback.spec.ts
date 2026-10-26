import {test, expect} from "./support";
import type {Page} from "@playwright/test";
import {createHash} from "node:crypto";
import {mkdir, readFile, writeFile} from "node:fs/promises";
import {extname, join, resolve, sep} from "node:path";

test.use({fresh:true});
const legacyKey = "rustify-ui.theme-studio.v1";
const key = "rustify-ui.theme-studio.v2";
const snapshot = (page:Page) => page.evaluate(() => window.__theme_studio.snapshot());
const raw = (page:Page, name:string) => page.evaluate(name => localStorage.getItem(name), name);
async function ready(page:Page) {
  await expect(page.getByTestId("status")).toHaveAttribute("data-status", "ready", {timeout:60_000});
}
async function edit(page:Page, value:string) {
  await page.getByTestId("editor-colors").click();
  await page.getByTestId("color-search").fill("");
  await page.getByTestId("color-token-primary").click();
  await page.getByTestId("color-value").fill(value);
  await page.getByTestId("color-value").press("Enter");
  await expect.poll(async () => {
    const state = await snapshot(page);
    return state.values?.light.primary ?? state.document.styles.light.primary;
  }).toBe(value);
}
async function save(page:Page, name:string) {
  await page.getByTestId("theme-save").click();
  await page.getByTestId("theme-name").fill(name);
  await page.getByTestId("theme-name-confirm").click();
  await expect(page.getByTestId("storage-status")).toContainText("Saved on this device");
}
async function download(page:Page, id:string) {
  const pending = page.waitForEvent("download");
  await page.getByTestId(id).click();
  return readFile((await (await pending).path())!, "utf8");
}

test("an actual old deployment retains v1 through upgrade, rollback, original restoration and return to v2", async ({page}, info) => {
  const oldDirectory = process.env.RUSTIFY_THEME_OLD_SITE;
  test.skip(!oldDirectory, "Requires a real pre-upgrade release product");
  test.setTimeout(180_000);
  const oldSite = resolve(oldDirectory!);
  const oldManifest = JSON.parse(await readFile(join(oldSite, "build-manifest.json"), "utf8"));
  const currentManifest = await (await page.request.get("./build-manifest.json")).json();
  const response = await page.request.get("./");
  const policy = response.headers()["content-security-policy"];
  expect(oldManifest.toolchain).toBe(currentManifest.toolchain);
  expect(oldManifest.build_id).not.toBe(currentManifest.build_id);
  let oldDeployment = true;
  const errors:string[] = [], failures:string[] = [];
  page.on("pageerror", error => errors.push(error.message));
  page.on("response", response => {if (response.status() >= 400) failures.push(`${response.status()} ${response.url()}`);});
  // Serve the retained, complete release bytes at the same origin. Only the
  // deployed directory changes; no SDK, WASM, loader or storage API is mocked.
  await page.route("**/*", async route => {
    if (!oldDeployment) return route.continue();
    const url = new URL(route.request().url());
    const path = resolve(oldSite, "." + (url.pathname === "/" ? "/index.html" : decodeURIComponent(url.pathname)));
    expect(path.startsWith(oldSite + sep)).toBe(true);
    const types:Record<string,string> = {".html":"text/html", ".js":"text/javascript", ".css":"text/css", ".wasm":"application/wasm", ".json":"application/json", ".ttf":"font/ttf", ".otf":"font/otf", ".svg":"image/svg+xml"};
    await route.fulfill({path, contentType:types[extname(path)] || "application/octet-stream", headers:{"content-security-policy":policy, "cache-control":"no-store"}});
  });
  await page.goto("./"); await ready(page);
  await page.getByTestId("renderer-dom").click();
  await edit(page, "#123456"); await save(page, "Legacy rollback 中文");
  const original = (await raw(page, legacyKey))!;
  expect(JSON.parse(original).draft.schema_version).toBe(1);
  expect(await raw(page, key)).toBeNull();

  oldDeployment = false;
  await page.reload(); await ready(page);
  await expect(page.getByTestId("upgrade-notice")).toBeVisible();
  expect((await snapshot(page)).values.light.primary).toBe("#123456");
  expect(await raw(page, key)).toBeNull();
  const backup = await download(page, "legacy-download");
  expect(backup).toBe(original);
  await page.getByTestId("upgrade-review-open").click();
  await page.getByTestId("upgrade-confirm").click();
  await expect.poll(async () => (await snapshot(page)).storage.permission).toBe("Ready");
  const backupKey = legacyKey + ".backup." + createHash("sha256").update(original).digest("hex");
  expect(await raw(page, backupKey)).toBe(original);
  await page.getByTestId("studio-config-scaling").click();
  await page.getByRole("option", {name:"110%", exact:true}).click();
  await edit(page, "#abcdef"); await save(page, "New configured work");
  await page.getByTestId("theme-export").click();
  const exported = await download(page, "export-download");
  expect(JSON.parse(exported)).toMatchObject({schema_version:2, config:{scaling:110}});
  await page.keyboard.press("Escape");
  const v2 = (await raw(page, key))!, current = (await snapshot(page)).document;
  expect(await raw(page, legacyKey)).toBe(original);

  oldDeployment = true;
  await page.reload(); await ready(page);
  expect((await snapshot(page)).document).toMatchObject({schema_version:1, name:"Legacy rollback 中文", styles:{light:{primary:"#123456"}}});
  await edit(page, "#765432");
  await expect.poll(() => raw(page, legacyKey)).not.toBe(original);
  expect(await raw(page, key)).toBe(v2);
  // This is the documented explicit storage-inspector restoration, exercised
  // through the same-origin storage API while no Studio instance is mounted.
  await page.goto("./build-manifest.json");
  await page.evaluate(({legacyKey, backup}) => localStorage.setItem(legacyKey, backup), {legacyKey, backup});
  await page.goto("./"); await ready(page);
  expect((await snapshot(page)).document.styles.light.primary).toBe("#123456");
  expect(await raw(page, key)).toBe(v2);

  oldDeployment = false;
  await page.reload(); await ready(page);
  expect((await snapshot(page)).document).toEqual(current);
  expect((await snapshot(page)).values.light.primary).toBe("#abcdef");
  expect(await download(page, "legacy-download")).toBe(original);
  expect(errors).toEqual([]); expect(failures).toEqual([]);
  const directory = "docs/validation/dom-components-p1/acceptance";
  await mkdir(directory, {recursive:true});
  await writeFile(join(directory, "storage-rollback.json"), JSON.stringify({oldManifest, currentManifest, steps:["old v1 save", "explicit source upgrade and original download", "v2 configured save and export", "same-origin complete old deployment", "explicit original-envelope restore outside Studio", "return prefers unchanged v2"], originalSha256:createHash("sha256").update(original).digest("hex"), exported:JSON.parse(exported), errors, failures}, null, 2));
  await page.screenshot({path:join(directory, "storage-rollback.png")});
});
