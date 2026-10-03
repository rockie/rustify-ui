import { test, expect, waitForReady } from "./support";
import type { Locator,Page } from "@playwright/test";

export const computed = (node:Locator) => node.evaluate((element)=> {
  const style=getComputedStyle(element);
  return {background:style.backgroundColor,color:style.color,radius:style.borderRadius,padding:style.padding,shadow:style.boxShadow,font:style.fontFamily,fontSize:style.fontSize,spacing:style.letterSpacing};
});

async function controls(page:Page){await page.getByTestId("editor-controls").click();}
async function isolation(page:Page){await page.locator(".studio-isolation > summary").click();}

test.describe("resolved DOM themes",()=> {
  test.beforeEach(async({page})=> {await waitForReady(page);});
  test("scopes stay independent and the host keeps its own styles",async({page})=> {
    const preview=page.getByTestId("studio-preview");
    const compare=page.getByTestId("studio-compare");
    const shell=await computed(page.getByTestId("editor-shell"));
    const host=await computed(page.getByTestId("host-probe"));
    const dark=await computed(compare.getByTestId("preview-surface"));
    await page.getByTestId("color-value").fill("rgb(240 0 0 / .4)");
    await page.getByTestId("color-value").press("Tab");
    await expect.poll(async()=> (await computed(preview.getByTestId("preview-primary"))).background).toBe("rgba(240, 0, 0, 0.4)");
    expect(await computed(page.getByTestId("editor-shell"))).toEqual(shell);
    expect(await computed(page.getByTestId("host-probe"))).toEqual(host);
    expect(await computed(compare.getByTestId("preview-surface"))).toEqual(dark);
    await page.getByTestId("mode-dark").click();
    await expect(preview).toHaveAttribute("data-theme","dark");
    await expect(compare).toHaveAttribute("data-theme","dark");
    expect(await computed(page.getByTestId("editor-shell"))).toEqual(shell);
    expect(await page.locator("body").getAttribute("data-theme")).toBeNull();
  });
  test("utility unit and layout gap remain separate and caller classes override",async({page})=> {
    const preview=page.getByTestId("studio-preview");
    await controls(page);
    expect((await computed(preview.getByTestId("radius-md"))).padding).toBe("16px");
    await page.getByTestId("edit-spacing").fill("6px");await page.getByTestId("edit-spacing").press("Tab");
    await expect.poll(async()=> (await computed(preview.getByTestId("radius-md"))).padding).toBe("24px");
    expect(await preview.evaluate((el)=>getComputedStyle(el).getPropertyValue("--spacing"))).toBe("8px");
    expect((await computed(preview.getByTestId("class-override"))).radius).toBe("0px");
    expect((await computed(preview.getByTestId("class-override"))).background).toBe("rgb(247, 144, 9)");
  });
  test("boundary projection and open overlay follow updates then restore inheritance",async({page})=> {
    const preview=page.getByTestId("studio-preview");
    const local=preview.getByTestId("local-card");
    await controls(page);await isolation(page);
    await page.getByTestId("apply-patch").click();
    await expect.poll(async()=> (await computed(local)).radius).toBe("0px");
    await preview.getByTestId("open-preview-dialog").click();
    const layer=preview.getByTestId("preview-dialog");
    const panel=layer.locator('[data-name="Dialog"]');
    await expect.poll(async()=> (await computed(panel)).radius).toBe("0px");
    expect((await computed(panel)).background).toBe("rgb(255, 244, 221)");
    expect((await computed(panel)).shadow).toContain("0.4");
    await page.evaluate(()=>window.__theme_studio.edit("shadow-blur","12px"));
    await expect.poll(async()=> (await computed(panel)).shadow).toContain("12px");
    await page.getByTestId("clear-patch").evaluate((node:HTMLButtonElement)=>node.click());
    await expect.poll(async()=> (await computed(panel)).radius).toBe("6px");
    expect((await computed(panel)).background).toBe("rgb(255, 255, 255)");
    expect((await computed(panel)).shadow).not.toContain("0.4");
    expect(await preview.getByTestId("local-boundary").getAttribute("style")).toBe("");
    await preview.getByTestId("preview-dialog-close").click();
    await preview.getByTestId("open-preview-menu").click();
    const menu=preview.getByTestId("preview-menu");
    await expect(menu).toBeVisible();
    await page.getByTestId("apply-patch").click();
    await expect.poll(async()=> (await computed(menu)).radius).toBe("0px");
    await menu.getByRole("menuitem").first().focus();
    await page.keyboard.press("Escape");await expect(menu).toHaveCount(0);
  });
  test("host rem is resolved and owner disposal leaves no runtime variables",async({page})=> {
    await controls(page);
    await page.evaluate(()=>document.documentElement.style.setProperty("font-size","20px"));
    const preview=page.getByTestId("studio-preview");
    await page.getByTestId("edit-radius").fill(".5rem");await page.getByTestId("edit-radius").press("Tab");
    await expect.poll(async()=> (await computed(preview.getByTestId("radius-lg"))).radius).toBe("10px");
    expect((await computed(preview.getByTestId("radius-xl"))).radius).toBe("14px");
    await page.evaluate(()=>window.__theme_studio.dispose());
    expect(await preview.getAttribute("data-rustify-theme-runtime")).toBeNull();
    expect(await preview.getAttribute("data-theme")).toBeNull();
    expect(await preview.evaluate((el)=>(el as HTMLElement).style.getPropertyValue("--rustify-radius-lg"))).toBe("");
    await page.evaluate(()=> {document.documentElement.style.removeProperty("font-size");window.__theme_studio.mount();});
    await expect(preview).toHaveAttribute("data-theme","light");
  });
  test("invalid edit preserves last valid document and a no-shadow focus ring still renders",async({page})=> {
    const preview=page.getByTestId("studio-preview");
    await page.getByTestId("color-value").fill("var(--unknown)");await page.getByTestId("color-value").press("Tab");
    await expect(page.getByTestId("color-value")).toHaveAttribute("aria-invalid","true");
    await expect(page.locator("#color-value-help")).not.toBeEmpty();
    expect((await computed(preview.getByTestId("preview-primary"))).background).toBe("rgb(21, 112, 239)");
    const before=preview.getByTestId("preview-primary");
    await page.getByTestId("color-value").focus();
    await before.focus();await page.keyboard.press("Tab");await page.keyboard.press("Shift+Tab");
    expect((await computed(before)).shadow).not.toBe("none");
  });
});

test('new runtime shadows honor caller color while empty layers keep rings transparent',async({page})=>{
 await waitForReady(page);const node=page.getByTestId('studio-preview').getByTestId('radius-lg');
 await page.evaluate(()=>{for(const [token,value]of Object.entries({'shadow-opacity':'.4','shadow-color':'rgb(30 10 20 / .6)','shadow-blur':'8px','shadow-offset-x':'2px','shadow-offset-y':'4px'}))window.__theme_studio.edit(token,value);});
 await expect.poll(async()=>(await computed(node)).shadow).toContain('2px 4px 8px');const original=(await computed(node)).shadow;
 const classes=await node.getAttribute('class');
 try{
  await node.evaluate(el=>el.classList.add('shadow-red-500'));
  const colored=(await computed(node)).shadow;expect(colored).not.toBe(original);expect(colored).toContain('2px 4px 8px');
  await page.evaluate(()=>window.__theme_studio.edit('shadow-opacity','0'));
  await expect.poll(async()=>(await computed(node)).shadow).not.toContain('2px 4px 8px');
  const empty=(await computed(node)).shadow;await node.evaluate(el=>el.classList.remove('shadow-red-500'));expect((await computed(node)).shadow).toBe(empty);
  await node.evaluate(el=>el.classList.add('ring-2','shadow-red-500'));const ring=(await computed(node)).shadow;expect(ring).not.toBe(empty);
  await node.evaluate(el=>el.classList.remove('shadow-red-500'));expect((await computed(node)).shadow).toBe(ring);
 }finally{await node.evaluate((el,classes)=>el.setAttribute('class',classes!),classes);}
});
