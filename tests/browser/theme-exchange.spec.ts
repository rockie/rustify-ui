import {test,expect,waitForReady} from "./support";
import type {Page} from "@playwright/test";
import {readFile} from "node:fs/promises";

test.use({fresh:true});
const state=(page:Page)=>page.evaluate(()=>window.__theme_studio.snapshot());
async function select(page:Page,id:string,name:string) {
  await page.getByTestId(id).click();await page.getByRole("option",{name,exact:true}).click();
}
async function importSource(page:Page,source:string,kind="Theme JSON") {
  await page.getByTestId("theme-import").click();await select(page,"import-format",kind);
  await page.getByTestId("import-source").fill(source);await page.getByTestId("import-review").click();
}

test("JSON review is atomic and cancel, invalid input and rejected code preserve the draft",async({page})=> {
  await waitForReady(page);const before=await state(page),candidate=structuredClone(before.document);
  candidate.name="Imported 中文";candidate.config.accent_color="tomato";candidate.config.scaling=105;candidate.base.dark["future-base"]="keep base text";candidate.overrides.light.primary="oklch(0.72 0.25 35 / 0.7)";candidate.overrides.dark.primary="#223344";candidate.base.light["future-token"]="keep author text";
  await importSource(page,JSON.stringify(candidate));await expect(page.getByTestId("import-preview")).toContainText("future-token");
  await expect(page.getByTestId("import-version")).toContainText("2");
  await expect(page.getByTestId("import-config")).toContainText("accent_color");
  expect((await state(page)).document).toEqual(before.document);
  await page.getByTestId("import-cancel").click();expect((await state(page)).history).toEqual(before.history);
  await importSource(page,JSON.stringify(candidate));await page.getByTestId("import-apply").click();
  await expect.poll(async()=> (await state(page)).document).toEqual(candidate);
  await page.getByTestId("theme-undo").click();expect((await state(page)).document).toEqual(before.document);
  await page.getByTestId("theme-redo").click();expect((await state(page)).document).toEqual(candidate);
  await importSource(page,'{"schema_version":999}');await expect(page.getByTestId("import-error")).toBeVisible();expect((await state(page)).document).toEqual(candidate);
  await page.keyboard.press("Escape");
  const requests:string[]=[];page.on("request",request=> {if(request.url().includes("attacker.invalid"))requests.push(request.url());});
  await importSource(page,'@import url("https://attacker.invalid/a.css"); :root{--primary:#123456;}',"CSS");await expect(page.getByTestId("import-diagnostics")).toBeVisible();await page.getByTestId("import-cancel").click();
  for(const css of [':root{--primary:url("https://attacker.invalid/evil");}',':root{--font-sans:"bad; background:url(https://attacker.invalid/x)";}',':root{--primary:var(--anything);}',':root{--radius:calc(1rem + 2px);}']) {
    await importSource(page,css,"CSS");await expect(page.getByTestId("import-error")).toBeVisible();expect((await state(page)).document).toEqual(candidate);await page.keyboard.press("Escape");
  }
  expect(requests).toEqual([]);
  await importSource(page,':root{--primary:rgb(12 34 56 / .5);--radius:0px;} .dark{--primary:#334455;}',"CSS");
  await expect(page.getByTestId("import-preview")).toContainText("radius");await page.getByTestId("import-apply").click();
  expect((await state(page)).values.light.primary).toBe("rgb(12 34 56 / .5)");expect((await state(page)).values.dark.primary).toBe("#334455");
  await test.info().attach("atomic-import", {body:JSON.stringify({before:before.document,json:candidate,afterCss:(await state(page)).document,maliciousRequests:requests},null,2),contentType:"application/json"});
});

test("file import validates before apply and wrong type or size cannot change the theme",async({page})=> {
  await waitForReady(page);const before=(await state(page)).document,candidate=structuredClone(before);candidate.name="From file";candidate.overrides.light.primary="#56789a";
  await page.getByTestId("theme-import").click();
  async function file(name:string,buffer:Buffer) {
    const chooser=page.waitForEvent("filechooser");await page.getByTestId("import-file").click();await (await chooser).setFiles({name,mimeType:"application/json",buffer});
  }
  await file("theme.txt",Buffer.from(JSON.stringify(candidate)));await expect(page.getByTestId("import-error")).toBeVisible();expect((await state(page)).document).toEqual(before);
  await file("too-large.json",Buffer.alloc(262145,32));await expect(page.getByTestId("import-error")).toContainText("256 KiB");expect((await state(page)).document).toEqual(before);
  await file("theme.json",Buffer.from(JSON.stringify(candidate)));await expect(page.getByTestId("import-preview")).toBeVisible();expect((await state(page)).document).toEqual(before);
  await page.getByTestId("import-apply").click();expect((await state(page)).document).toEqual(candidate);
});

test("a reviewed import cannot apply after another controller change until explicitly reviewed again",async({page})=> {
  await waitForReady(page);
  const initial=await state(page),candidate=structuredClone(initial.document);
  candidate.name="Reviewed candidate";candidate.overrides.dark.primary="#789abc";
  await importSource(page,JSON.stringify(candidate));
  // The modal excludes background clicks. This existing bridge injects a
  // controller change while the user is reviewing, without applying imports
  // through a private test shortcut.
  await page.evaluate(()=>window.__theme_studio.edit("primary","#654321"));
  const changed=await state(page);
  await page.getByTestId("import-apply").click();
  await expect(page.getByTestId("import-error")).toContainText("editor changed after review");
  expect((await state(page)).document).toEqual(changed.document);
  expect((await state(page)).history).toEqual(changed.history);
  await page.getByTestId("import-review").click();
  await page.getByTestId("import-apply").click();
  expect((await state(page)).document).toEqual(candidate);
  await page.getByTestId("theme-undo").click();
  expect((await state(page)).document).toEqual(changed.document);
});

test("all CSS profiles and notations export without editing; JSON and Rust downloads match their previews",async({page})=> {
  await waitForReady(page);const before=await state(page);await page.getByTestId("theme-export").click();
  const source=page.getByTestId("export-source");expect(JSON.parse(await source.inputValue())).toMatchObject({schema_version:2,id:before.document.id,name:before.document.name});
  async function downloadText() {
    const pending=page.waitForEvent("download",{timeout:5000});await page.getByTestId("export-download").click();const download=await pending;return {name:download.suggestedFilename(),text:await readFile((await download.path())!,"utf8")};
  }
  const json=await downloadText();expect(json.text).toBe(await source.inputValue());expect(json.name).toMatch(/\.json$/);
  await select(page,"export-kind","Tailwind v4 CSS");const outputs=[];
  for(const profile of ["Standard Tailwind v4","Rustify scoped"]) {
    await select(page,"export-profile",profile);
    for(const format of ["HEX","RGB","HSL","OKLCH"]) {
      await select(page,"export-color-format",format);const css=await source.inputValue();
      expect(css).toContain("@theme inline");expect(css).toContain("--color-primary");expect(css).toContain("Rustify WenKai");
      expect(css).toContain(profile==="Rustify scoped"?"data-rustify-theme":".dark");
      outputs.push({profile,format,bytes:Buffer.byteLength(css)});
    }
  }
  await select(page,"export-kind","Rust integration");const rust=await source.inputValue();expect(rust).toContain('include_str!("theme.json")');expect(rust).toContain("ThemeScope");
  // Each artifact download is exercised in a fresh browser context below.


  const after=await state(page);expect(after.document).toEqual(before.document);expect(after.history).toEqual(before.history);
  await test.info().attach("exports", {body:JSON.stringify(outputs,null,2),contentType:"application/json"});
});

test("clipboard refusal offers selection and download without a false success message",async({page})=> {
  await page.addInitScript(()=>Object.defineProperty(navigator,"clipboard",{value:{writeText:()=>Promise.reject(new DOMException("Test denied","NotAllowedError"))}}));
  await waitForReady(page);await page.getByTestId("theme-export").click();await page.getByTestId("export-copy").click();
  await expect(page.getByTestId("export-feedback")).toContainText("Clipboard unavailable or refused");
  await page.getByTestId("export-select").click();const source=page.getByTestId("export-source");
  expect(await source.evaluate((node:HTMLTextAreaElement)=>node.selectionEnd-node.selectionStart)).toBe((await source.inputValue()).length);
  const pending=page.waitForEvent("download",{timeout:5000});await page.getByTestId("export-download").click();await pending;await expect(page.getByTestId("export-feedback")).toContainText("Download started");
});

test("granted clipboard contains the actual exported bytes",async({page})=> {
  await page.context().grantPermissions(["clipboard-read","clipboard-write"]);await waitForReady(page);
  await page.getByTestId("theme-export").click();await page.getByTestId("export-copy").click();await expect(page.getByTestId("export-feedback")).toContainText("Copied to clipboard");
  expect(await page.evaluate(()=>navigator.clipboard.readText())).toBe(await page.getByTestId("export-source").inputValue());
});

for(const kind of ["Tailwind v4 CSS","Rust integration"]) test(`fresh download contains exact ${kind} source`,async({page})=>{
  await waitForReady(page);await page.getByTestId("theme-export").click();await select(page,"export-kind",kind);
  const expected=await page.getByTestId("export-source").inputValue();const pending=page.waitForEvent("download",{timeout:5000});
  await page.getByTestId("export-download").click();const download=await pending;expect(await readFile((await download.path())!,"utf8")).toBe(expected);
});

test('export dialogs recenter as contents and viewport change, with reachable actions',async({page})=>{
  await waitForReady(page);await page.getByTestId('theme-export').click();
  await select(page,'export-kind','Tailwind v4 CSS');
  const panel=page.getByTestId('data-tools-dialog').locator('[data-name="Dialog"]');
  for(const viewport of [{width:1280,height:720},{width:768,height:1024},{width:390,height:844}]){
    await page.setViewportSize(viewport);
    await expect.poll(async()=>{
      const box=await panel.boundingBox();return !!box&&box.x>=8&&box.y>=8&&box.x+box.width<=viewport.width-8&&box.y+box.height<=viewport.height-8;
    }).toBe(true);
    await page.getByTestId('export-download').scrollIntoViewIfNeeded();
    const button=await page.getByTestId('export-download').boundingBox();
    expect(button!.y).toBeGreaterThanOrEqual(0);expect(button!.y+button!.height).toBeLessThanOrEqual(viewport.height);
  }
});
