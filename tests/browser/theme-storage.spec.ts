import {test,expect,waitForReady} from "./support";
import type {Page} from "@playwright/test";
import {readFile} from "node:fs/promises";

test.use({fresh:true});
const key="rustify-ui.theme-studio.v2";
const state=(page:Page)=>page.evaluate(()=>window.__theme_studio.snapshot());
const raw=(page:Page)=>page.evaluate(key=>localStorage.getItem(key),key);
const primary=(document:any)=>document.overrides?.light?.primary ?? document.styles?.light?.primary ?? document.base?.light?.primary;
async function settled(page:Page) {await expect.poll(async()=>{const s=await state(page);return s.storage.pending===0||s.storage.paused||s.storage.permission!=="Ready";}).toBe(true);}
async function edit(page:Page,value:string) {
  await page.getByTestId("editor-colors").click();
  await page.getByTestId("color-token-primary").click();
  await page.getByTestId("color-value").fill(value);
  await page.getByTestId("color-value").press("Enter");
  await expect.poll(async()=> (await state(page)).values.light.primary).toBe(value);
  await settled(page);
}
async function save(page:Page,name:string) {
  await page.getByTestId("theme-save").click();
  await page.getByTestId("theme-name").fill(name);
  await page.getByTestId("theme-name-confirm").click();
  await settled(page);
}
async function confirm(page:Page) {await page.getByTestId("theme-confirm").click();await settled(page);}
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
  await expect(page.getByTestId("theme-modified")).toHaveText("Matches reference document");
  const saved=JSON.parse((await raw(page))!);
  expect(saved.saved_themes).toHaveLength(1);expect(saved.draft.name).toBe("Ocean 中文");
  await edit(page,"#224488");
  await expect(page.getByTestId("theme-modified")).toHaveText("Modified from reference document");
  await expect(page.getByTestId("toolbar-storage-status")).toHaveText("Saved on this device");
  await page.reload();
  await expect(page.getByTestId("status")).toHaveAttribute("data-status","ready");
  expect((await state(page)).values.light.primary).toBe("#224488");
  expect((await state(page)).modified).toBe(true);
  await page.getByTestId("theme-reset").click();
  await expect.poll(async()=> (await state(page)).values.light.primary).toBe("#00a88a");
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
  await expect.poll(async()=>primary(JSON.parse((await raw(page))!).saved_themes.find((theme:any)=>theme.id===saved.saved_themes[0].id))).toBe("#cc8844");
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
  await expect.poll(()=>raw(page)).toBeNull();expect(await page.evaluate(()=>localStorage.getItem("unrelated-app"))).toBe("keep");
  expect((await state(page)).values.light.primary).toBe("#552288");
  await test.info().attach("protected-cache", {body:JSON.stringify({downloaded:broken,otherKey:"keep",draft:(await state(page)).document},null,2),contentType:"application/json"});
});

test("quota failure preserves the previous bytes, in-memory draft and unsaved named library",async({page})=> {
  await waitForReady(page);await edit(page,"#552288");const previous=await raw(page);
  await page.evaluate(()=> {Storage.prototype.setItem=function(){throw new DOMException("Test capacity reached","QuotaExceededError");};});
  await edit(page,"#882255");await expect(page.locator(".tools-storage-state > strong")).toHaveText("Storage full · saving in memory");
  await expect(page.getByTestId("storage-status")).toContainText("Your current draft stays in memory");
  expect(await raw(page)).toBe(previous);expect((await state(page)).values.light.primary).toBe("#882255");
  await save(page,"Cannot save");expect(await raw(page)).toBe(previous);
  await page.getByTestId("theme-library").click();await expect(page.getByTestId("library-empty")).toBeVisible();
  expect((await state(page)).modified).toBe(true);
});

test("a failed named save keeps the acknowledged draft status and retries its retained action",async({page})=> {
  await waitForReady(page);await edit(page,"#552288");
  await expect(page.getByTestId("storage-status")).toHaveText("Saved on this device");
  const previous=await raw(page);
  await page.evaluate(()=> {
    (window as any).__storageSetItem=Storage.prototype.setItem;
    Storage.prototype.setItem=function(){throw new DOMException("Test capacity reached","QuotaExceededError");};
  });
  await save(page,"Retained name");
  await expect(page.getByTestId("toolbar-storage-status")).toHaveText("Draft saved · action failed");
  await expect(page.getByTestId("storage-status")).toContainText("The current draft is saved on this device");
  await expect(page.getByTestId("named-storage-feedback")).toHaveAttribute("data-result","failed");
  await expect(page.getByTestId("named-storage-feedback")).toContainText("Retained name");
  await expect(page.getByTestId("named-storage-feedback").getByRole("button",{name:'Retry “Retained name”',exact:true})).toBeVisible();
  await expect(page.locator(".tools-storage-state > strong")).toHaveText("Storage full · saving in memory");
  await expect(page.getByTestId("storage-callout-retry")).toHaveCount(0);
  await expect(page.getByTestId("storage-retry")).toHaveText("Retry all retained saves");
  await expect(page.getByTestId("theme-save")).toHaveCSS("background-color","rgb(255, 255, 255)");
  expect(await raw(page)).toBe(previous);
  await expect(page.getByTestId("storage-issue").locator("..")).toHaveClass(/tools-state-error/);
  await page.evaluate(()=>Storage.prototype.setItem=(window as any).__storageSetItem);
  await page.getByTestId("named-storage-feedback").getByRole("button",{name:'Retry “Retained name”',exact:true}).click();
  await expect.poll(async()=>JSON.parse((await raw(page))!).saved_themes.map((theme:any)=>theme.name)).toEqual(["Retained name"]);
});

test("denied localStorage leaves the editor and export available",async({page})=> {
  await page.addInitScript(()=>Object.defineProperty(window,"localStorage",{get(){throw new DOMException("Test access refused","SecurityError");}}));
  await waitForReady(page);await expect(page.locator(".tools-storage-state > strong")).toHaveText("Memory only · local saving unavailable");
  await edit(page,"#552288");await page.getByTestId("theme-export").click();
  expect(primary(JSON.parse(await page.getByTestId("export-source").inputValue()))).toBe("#552288");
});

test("a real other-tab save freezes the draft until explicit load, keep or save-copy",async({page})=> {
  await waitForReady(page);await edit(page,"#552288");const second=await other(page);
  await edit(second,"#224488");await expect(page.getByTestId("storage-conflict")).toBeVisible();
  expect((await state(page)).values.light.primary).toBe("#552288");
  await edit(page,"#775533");expect(primary(JSON.parse((await raw(page))!).draft)).toBe("#224488");
  await page.getByTestId("conflict-load").click();await page.getByTestId("theme-confirm-cancel").click();
  expect((await state(page)).values.light.primary).toBe("#775533");
  await page.getByTestId("conflict-load").click();await confirm(page);
  await expect(page.getByTestId("storage-conflict")).toHaveCount(0);
  expect((await state(page)).values.light.primary).toBe("#224488");expect((await state(page)).modified).toBe(true);
  await page.getByTestId("theme-reset").click();await expect.poll(async()=> (await state(page)).values.light.primary).toBe("#1570ef");
  await second.getByTestId("conflict-keep").click();await expect(page.getByTestId("storage-conflict")).toBeVisible();
  await page.getByTestId("conflict-keep").click();await expect(page.getByTestId("storage-conflict")).toHaveCount(0);
  await expect(second.getByTestId("storage-conflict")).toBeVisible();await second.getByTestId("conflict-load").click();await confirm(second);
  await edit(second,"#aabb44");await expect(page.getByTestId("storage-conflict")).toBeVisible();
  await page.getByTestId("conflict-copy").click();await page.getByTestId("theme-name").fill("Retained branch");await page.getByTestId("theme-name-confirm").click();
  await expect(page.getByTestId("storage-conflict")).toHaveCount(0);
  const envelope=JSON.parse((await raw(page))!);expect(envelope.saved_themes).toHaveLength(1);
  expect(primary(envelope.saved_themes[0])).toBe("#1570ef");
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
