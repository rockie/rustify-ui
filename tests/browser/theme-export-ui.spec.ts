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
  await expect(page.getByTestId("import-version")).toHaveText("Theme JSON v2");
  const scaling=page.getByTestId("import-config").locator("tr.tools-diff-row").filter({has:page.getByText("Scaling",{exact:true})});
  await expect(scaling.locator("td")).toHaveText(["Changed","100%","110%"]);
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

test("default export supplies portable setup notes for every format and follows preview without changing author data",async({page})=>{
  const errors:string[]=[],failed:string[]=[];
  page.on("pageerror",error=>errors.push(error.message));
  page.on("console",message=>{if(message.type()==="error")errors.push(message.text());});
  page.on("requestfailed",request=>failed.push(request.url()));
  page.on("response",response=>{if(response.status()>=400)failed.push(response.url());});
  await page.addInitScript(()=>{(window as any).__setupCsp=[];document.addEventListener("securitypolicyviolation",event=>(window as any).__setupCsp.push(event.violatedDirective));});
  await waitForReady(page);
  await page.emulateMedia({reducedMotion:"reduce"});
  await page.evaluate(()=>{document.documentElement.style.fontSize="20px";window.__theme_studio.environment();});
  const before=await snapshot(page);
  await page.getByTestId("theme-export").click();
  const source=page.getByTestId("export-source"),summary=page.getByTestId("export-setup-summary");
  const original=await source.inputValue();
  await expect(summary).toContainText("0.2.1");
  const revision=(await summary.locator("code").textContent())!;
  expect(revision).toMatch(/^[a-f0-9]{40}$/);
  const light=await download(page,"export-setup-download");
  expect(light.name).toBe("theme-setup.txt");expect(light.text).toContain(revision);
  expect(light.text).toContain("mount_exported_preview(container, config, ThemeMode::Light, &font_context(20., true))");
  for(const kind of ["Compatible JSON v1 · lossy","CSS · resolved values","Rust integration"]){
    await outputKind(page,kind);await expect(summary).toBeVisible();
    await expect(page.getByTestId("export-setup-download")).toBeVisible();
    if(kind.startsWith("CSS"))await expect(summary).toContainText("Static CSS needs no wasm or CLI");
    expect((await download(page,"export-setup-download")).text).toBe(light.text);
  }
  await page.keyboard.press("Escape");await expect(page.getByTestId("theme-export")).toBeFocused();
  await page.getByTestId("mode-dark").click();await page.getByTestId("theme-export").click();
  await expect(page.getByTestId("export-preview-mode")).toContainText("mount_exported_preview(container, config, ThemeMode::Dark, &font_context(20., true))");
  const dark=await download(page,"export-setup-download");
  expect(dark.text).toContain("Preview / 当前预览: Dark");
  await outputKind(page,"Theme JSON v2");expect(await source.inputValue()).toBe(original);
  expect((await snapshot(page)).document).toEqual(before.document);
  expect((await snapshot(page)).history).toEqual(before.history);
  await page.getByTestId("export-setup-details").locator("summary").click();
  await expect(page.getByTestId("export-setup-notes")).toHaveText(dark.text);
  const paths=[...new Set([...dark.text.matchAll(/^makepad_widgets\/resources\/[^\n]+/gm)].map(match=>match[0]))];
  expect(paths.length).toBeGreaterThan(0);
  for(const path of paths)expect((await page.request.get(path)).ok(),path).toBe(true);
  expect((await page.request.get("makepad_widgets/resources/FONT-LICENSES.md")).ok()).toBe(true);
  await mkdir(evidence,{recursive:true});
  await writeFile(`${evidence}/theme-setup-light.txt`,light.text);await writeFile(`${evidence}/theme-setup-dark.txt`,dark.text);
  await writeFile(`${evidence}/theme.json`,original);
  await outputKind(page,"Rust integration");
  await writeFile(`${evidence}/theme.rs`,(await download(page)).text);
  await outputKind(page,"Theme JSON v2");
  await page.getByTestId("export-setup-details").locator("summary").click();
  for(const width of [1440,768,390]){
    await page.setViewportSize({width,height:900});
    // The overlay repositions through ResizeObserver after the viewport changes.
    // Wait for that geometry before scrolling inside its content.
    await expect.poll(async()=>{
      const box=await page.getByTestId("data-tools-dialog").locator('[data-name="Dialog"]').boundingBox();
      return !!box&&box.x>=8&&box.y>=8&&box.x+box.width<=width-8&&box.y+box.height<=892;
    }).toBe(true);
    await expect(summary).toBeVisible();
    await page.getByTestId("export-setup-download").scrollIntoViewIfNeeded();
    await expect(page.getByTestId("export-setup-download")).toBeInViewport();
    expect(await page.evaluate(()=>document.documentElement.scrollWidth<=innerWidth)).toBe(true);
    await page.screenshot({path:`${evidence}/setup-${width}-en.png`,fullPage:true});
  }
  await page.keyboard.press("Escape");await page.getByTestId("locale-zh").click();
  await page.getByTestId("theme-export").click();
  await expect(summary).toContainText("源码修订");await expect(page.getByTestId("export-setup-download")).toHaveText("下载接入说明");
  await page.screenshot({path:`${evidence}/setup-390-zh.png`,fullPage:true});
  await page.setViewportSize({width:640,height:450});
  await page.evaluate(()=>{document.documentElement.style.fontSize="200%";window.__theme_studio.environment();});
  await page.getByTestId("export-setup-download").scrollIntoViewIfNeeded();
  await expect(page.getByTestId("export-setup-download")).toBeInViewport();
  // Keyboard focus remains inside the existing dialog, including at doubled text size.
  await page.getByTestId("export-setup-download").focus();await page.keyboard.press("Tab");
  expect(await page.evaluate(()=>!!document.activeElement?.closest('[role="dialog"]'))).toBe(true);
  await page.screenshot({path:`${evidence}/setup-200-percent-zh.png`,fullPage:true});
  await page.keyboard.press("Escape");await expect(page.getByTestId("theme-export")).toBeFocused();
  expect(errors).toEqual([]);expect(failed).toEqual([]);expect(await page.evaluate(()=>(window as any).__setupCsp)).toEqual([]);
});
