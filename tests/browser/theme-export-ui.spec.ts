import {test,expect,waitForReady} from "./support";
import type {Page} from "@playwright/test";
import {mkdir,readFile,writeFile} from "node:fs/promises";

test.use({fresh:true});
const evidence=process.env.RUSTIFY_UI_EVIDENCE_OUT ? `${process.env.RUSTIFY_UI_EVIDENCE_OUT}/export` : "docs/validation/dom-components-p1/export";
const snapshot=(page:Page)=>page.evaluate(()=>window.__theme_studio.snapshot());
async function outputKind(page:Page,name:string){
  await page.getByTestId("export-kind").click();
  await page.getByRole("option",{name,exact:true}).click();
}
async function download(page:Page,id="export-download"){
  const pending=page.waitForEvent("download");await page.getByTestId(id).click();
  const file=await pending;
  return {name:file.suggestedFilename(),text:await readFile((await file.path())!,"utf8")};
}

test("v2 export preserves intent and explicitly reviewed v1 loses configuration, with actual clipboard/download bytes",async({page})=>{
  const errors:string[]=[];page.on("pageerror",error=>errors.push(error.message));
  await page.context().grantPermissions(["clipboard-read","clipboard-write"]);
  await waitForReady(page);
  const before=await snapshot(page),candidate=structuredClone(before.document);
  candidate.name="Configured export 中文";
  candidate.config={appearance:"dark",accent_color:"pink",gray_color:"auto",panel_background:"translucent",radius:"full",scaling:110};
  candidate.overrides.light.primary="#123456";candidate.overrides.dark.primary="#bbaaee";
  candidate.base.light["future-author-string"]="保留作者原文";
  await page.getByTestId("theme-import").click();
  await page.getByTestId("import-source").fill(JSON.stringify(candidate));
  await page.getByTestId("import-review").click();
  await expect(page.getByTestId("import-version")).toHaveText("JSON v2 → v2");
  await expect(page.getByTestId("import-config").locator("div").filter({has:page.getByText("Scaling",{exact:true})})).toContainText("100% → 110%");
  expect((await snapshot(page)).document).toEqual(before.document);
  await page.getByTestId("import-apply").click();
  const applied=await snapshot(page);expect(applied.document).toEqual(candidate);
  await page.getByTestId("theme-export").click();
  const source=page.getByTestId("export-source");
  expect(JSON.parse(await source.inputValue())).toEqual(candidate);
  const full=await download(page);expect(full.name).toBe("theme.json");expect(JSON.parse(full.text)).toEqual(candidate);
  await outputKind(page,"Compatible JSON v1 · lossy");
  const losses=page.getByTestId("export-v1-losses");await expect(losses).toBeVisible();
  for(const field of ["config.accent_color","config.gray_color","config.appearance","config.radius","config.scaling","sources","derived_roles"]){
    await expect(losses.locator(`[data-loss="${field}"]`)).toBeVisible();
  }
  const lossFields=await losses.locator("[data-loss]").evaluateAll(nodes=>nodes.map(node=>node.getAttribute("data-loss")));
  const rawLegacy=await source.inputValue(),legacy=JSON.parse(rawLegacy);
  expect(legacy.schema_version).toBe(1);expect(legacy.config).toBeUndefined();expect(legacy.overrides).toBeUndefined();
  expect(legacy.styles.light["future-author-string"]).toBe("保留作者原文");
  expect(legacy.styles.light.primary).toBe("rgb(18 52 86)");
  const compatible=await download(page);expect(compatible.name).toBe("theme-v1.json");expect(compatible.text).toBe(rawLegacy);
  await page.getByTestId("export-copy").click();
  await expect(page.getByTestId("export-feedback")).toContainText("Copied to clipboard");
  expect(await page.evaluate(()=>navigator.clipboard.readText())).toBe(rawLegacy);
  await outputKind(page,"Rust integration");
  const rust=await source.inputValue();
  expect(rust).toContain("pub fn mount_exported_preview(");expect(rust).toContain("preview_mode=Signal::derive(move || mode)");
  await expect(page.getByTestId("export-preview-mode")).toContainText("ThemeMode::Light");
  const matching=await download(page,"export-matching-json");expect(JSON.parse(matching.text)).toEqual(candidate);
  expect((await snapshot(page)).document).toEqual(candidate);expect((await snapshot(page)).history).toEqual(applied.history);
  await mkdir(evidence,{recursive:true});
  await writeFile(`${evidence}/studio-v1-review.json`,JSON.stringify({config:candidate.config,lossFields,legacy,full:JSON.parse(full.text)},null,2));
  await outputKind(page,"Compatible JSON v1 · lossy");
  await page.screenshot({path:`${evidence}/studio-v1-review.png`,fullPage:true});
  expect(errors).toEqual([]);
});

test("copy success waits for the actual clipboard promise",async({page})=>{
  await page.context().grantPermissions(["clipboard-read","clipboard-write"]);
  await page.addInitScript(()=>{
    const original=navigator.clipboard.writeText.bind(navigator.clipboard);
    Object.defineProperty(navigator.clipboard,"writeText",{value:(text:string)=>new Promise<void>((resolve,reject)=>{
      (window as any).__finishThemeCopy=()=>original(text).then(resolve,reject);
    })});
  });
  await waitForReady(page);await page.getByTestId("theme-export").click();
  const expected=await page.getByTestId("export-source").inputValue();
  await page.getByTestId("export-copy").click();
  await expect(page.getByTestId("export-feedback")).toHaveText("Copying…");
  await expect(page.getByTestId("export-feedback")).not.toContainText("Copied");
  await page.evaluate(()=>(window as any).__finishThemeCopy());
  await expect(page.getByTestId("export-feedback")).toContainText("Copied to clipboard");
  expect(await page.evaluate(()=>navigator.clipboard.readText())).toBe(expected);
});
