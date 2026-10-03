import {test,expect,waitForReady} from "./support";
import type {Page} from "@playwright/test";
import {readFile,mkdir,writeFile} from "node:fs/promises";

test.use({fresh:true});
const key="rustify-ui.theme-studio.v1";
const state=(page:Page)=>page.evaluate(()=>window.__theme_studio.snapshot());
const raw=(page:Page)=>page.evaluate(key=>localStorage.getItem(key),key);
async function edit(page:Page,value:string) {
  await page.getByTestId("editor-colors").click();
  await page.getByTestId("color-token-primary").click();
  await page.getByTestId("color-value").fill(value);
  await page.getByTestId("color-value").press("Enter");
  await expect.poll(async()=> (await state(page)).document.styles.light.primary).toBe(value);
}
async function save(page:Page,name:string) {
  await page.getByTestId("theme-save").click();
  await page.getByTestId("theme-name").fill(name);
  await page.getByTestId("theme-name-confirm").click();
}
async function confirm(page:Page) {await page.getByTestId("theme-confirm").click();}
async function other(page:Page) {
  const second=await page.context().newPage();
  await second.goto(page.url());
  await expect(second.getByTestId("status")).toHaveAttribute("data-status","ready");
  return second;
}

test("named save, refresh, copy, rename, overwrite and delete retain real checkpoints",async({page})=> {
  await waitForReady(page);
  await edit(page,"#00a88a");
  await save(page,"Ocean 中文");
  await expect(page.getByTestId("storage-status")).toContainText("Saved on this device");
  expect((await state(page)).modified).toBe(false);
  const saved=JSON.parse((await raw(page))!);
  expect(saved.saved_themes).toHaveLength(1);expect(saved.draft.name).toBe("Ocean 中文");
  await edit(page,"#224488");await page.reload();
  await expect(page.getByTestId("status")).toHaveAttribute("data-status","ready");
  expect((await state(page)).document.styles.light.primary).toBe("#224488");
  expect((await state(page)).modified).toBe(true);
  await page.getByTestId("theme-reset").click();
  await expect.poll(async()=> (await state(page)).document.styles.light.primary).toBe("#00a88a");
  await page.getByTestId("theme-library").click();
  const row=page.locator(`li[data-theme-id="${saved.saved_themes[0].id}"]`);
  await row.getByTestId("local-copy").click();await page.getByTestId("theme-name").fill("Ocean copy");await page.getByTestId("theme-name-confirm").click();
  await expect(page.locator("li[data-theme-id]")).toHaveCount(2);
  await row.getByTestId("local-rename").click();await page.getByTestId("theme-name").fill("Ocean renamed");await page.getByTestId("theme-name-confirm").click();
  await expect(row).toContainText("Ocean renamed");
  await row.getByTestId("local-load").click();
  expect((await state(page)).modified).toBe(false);
  await edit(page,"#cc8844");await save(page,"Ocean renamed");
  await expect(page.getByTestId("theme-confirm-dialog")).toBeVisible();await confirm(page);
  expect(JSON.parse((await raw(page))!).saved_themes).toHaveLength(2);
  await page.getByTestId("theme-library").click();
  await row.getByTestId("local-delete").click();await page.getByTestId("theme-confirm-cancel").click();await expect(row).toBeVisible();
  await row.getByTestId("local-delete").click();await confirm(page);await expect(row).toHaveCount(0);
  expect(JSON.parse((await raw(page))!).saved_themes).toHaveLength(1);
});

test("bad cache is preserved and downloadable; explicit clear touches only the studio key",async({page})=> {
  await waitForReady(page);const broken="{\"envelope_version\":999,\"private\":\"保留原文\"}";
  await page.evaluate(({key,broken})=> {localStorage.setItem(key,broken);localStorage.setItem("unrelated-app","keep");},{key,broken});
  await page.reload();await expect(page.getByTestId("corrupt-cache")).toBeVisible();
  await edit(page,"#552288");expect(await raw(page)).toBe(broken);
  const downloadPromise=page.waitForEvent("download");await page.getByTestId("cache-download").click();
  const download=await downloadPromise;expect(await readFile((await download.path())!,"utf8")).toBe(broken);
  await page.getByTestId("cache-clear").click();await page.getByTestId("theme-confirm-cancel").click();expect(await raw(page)).toBe(broken);
  await page.getByTestId("cache-clear").click();await confirm(page);
  expect(await raw(page)).toBeNull();expect(await page.evaluate(()=>localStorage.getItem("unrelated-app"))).toBe("keep");
  expect((await state(page)).document.styles.light.primary).toBe("#552288");
  await mkdir("docs/validation/theme-studio/M5",{recursive:true});await writeFile("docs/validation/theme-studio/M5/cache.json",JSON.stringify({downloaded:broken,otherKey:"keep",draft:(await state(page)).document},null,2));
});

test("quota failure preserves the previous bytes, in-memory draft and unsaved named library",async({page})=> {
  await waitForReady(page);await edit(page,"#552288");const previous=await raw(page);
  await page.evaluate(()=> {Storage.prototype.setItem=function(){throw new DOMException("Test capacity reached","QuotaExceededError");};});
  await edit(page,"#882255");await expect(page.getByTestId("storage-status")).toContainText(/failed|quota|Quota/);
  expect(await raw(page)).toBe(previous);expect((await state(page)).document.styles.light.primary).toBe("#882255");
  await save(page,"Cannot save");expect(await raw(page)).toBe(previous);
  await page.getByTestId("theme-library").click();await expect(page.getByTestId("library-empty")).toBeVisible();
  expect((await state(page)).modified).toBe(true);
});

test("denied localStorage leaves the editor and export available",async({page})=> {
  await page.addInitScript(()=>Object.defineProperty(window,"localStorage",{get(){throw new DOMException("Test access refused","SecurityError");}}));
  await waitForReady(page);await expect(page.getByTestId("storage-status")).toContainText(/unavailable|refused|Security/);
  await edit(page,"#552288");await page.getByTestId("theme-export").click();
  expect(JSON.parse(await page.getByTestId("export-source").inputValue()).styles.light.primary).toBe("#552288");
});

test("a real other-tab save freezes the draft until explicit load, keep or save-copy",async({page})=> {
  await waitForReady(page);await edit(page,"#552288");const second=await other(page);
  await edit(second,"#224488");await expect(page.getByTestId("storage-conflict")).toBeVisible();
  expect((await state(page)).document.styles.light.primary).toBe("#552288");
  await edit(page,"#775533");expect(JSON.parse((await raw(page))!).draft.styles.light.primary).toBe("#224488");
  await page.getByTestId("conflict-load").click();await page.getByTestId("theme-confirm-cancel").click();
  expect((await state(page)).document.styles.light.primary).toBe("#775533");
  await page.getByTestId("conflict-load").click();await confirm(page);
  await expect(page.getByTestId("storage-conflict")).toHaveCount(0);
  expect((await state(page)).document.styles.light.primary).toBe("#224488");expect((await state(page)).modified).toBe(true);
  await page.getByTestId("theme-reset").click();await expect.poll(async()=> (await state(page)).document.styles.light.primary).toBe("#1570ef");
  await second.getByTestId("conflict-keep").click();await expect(page.getByTestId("storage-conflict")).toBeVisible();
  await page.getByTestId("conflict-keep").click();await expect(page.getByTestId("storage-conflict")).toHaveCount(0);
  await expect(second.getByTestId("storage-conflict")).toBeVisible();await second.getByTestId("conflict-load").click();await confirm(second);
  await edit(second,"#aabb44");await expect(page.getByTestId("storage-conflict")).toBeVisible();
  await page.getByTestId("conflict-copy").click();await page.getByTestId("theme-name").fill("Retained branch");await page.getByTestId("theme-name-confirm").click();
  await expect(page.getByTestId("storage-conflict")).toHaveCount(0);
  const envelope=JSON.parse((await raw(page))!);expect(envelope.saved_themes).toHaveLength(1);
  expect(envelope.saved_themes[0].styles.light.primary).toBe("#1570ef");
  expect((await state(page)).document.name).toBe("Retained branch");
  await second.close();
});

test("failed keep and duplicate conflict-copy retain the unresolved conflict and library",async({page})=> {
  await waitForReady(page);await save(page,"Existing");const second=await other(page);
  await edit(second,"#224488");await expect(page.getByTestId("storage-conflict")).toBeVisible();const before=await raw(page);
  await page.getByTestId("conflict-copy").click();await page.getByTestId("theme-name").fill("Existing");await page.getByTestId("theme-name-confirm").click();
  await expect(page.getByTestId("name-error")).toBeVisible();expect(await raw(page)).toBe(before);
  await page.getByTestId("theme-name-cancel").click();
  await page.evaluate(()=> {Storage.prototype.setItem=function(){throw new DOMException("Test capacity reached","QuotaExceededError");};});
  await page.getByTestId("conflict-keep").click();await expect(page.getByTestId("storage-conflict")).toBeVisible();
  expect(await raw(page)).toBe(before);await second.close();
});
