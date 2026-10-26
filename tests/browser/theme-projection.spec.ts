import {test, expect, waitForReady} from "./support";

test.use({fresh:true});

test("a masked parent edit reaches the preview without rewriting unchanged toolbar properties", async ({page}) => {
  await waitForReady(page);
  await page.getByTestId("renderer-dom").click();
  const toolbar=page.getByTestId("preview-toolbar");
  const before=await toolbar.evaluate(node => {
    const boundary=node.closest("[data-rustify-theme-boundary]") as HTMLElement;
    if (!boundary) throw new Error("Missing local toolbar boundary");
    const style=boundary.style;
    const original=CSSStyleDeclaration.prototype.setProperty;
    const rootStyle=(document.querySelector('[data-testid="studio-preview"]') as HTMLElement).style;
    const counter={calls:0,rootProperties:[] as string[]};
    (window as any).__toolbarProjection=counter;
    CSSStyleDeclaration.prototype.setProperty=function(name,value,priority) {
      if (this===style) counter.calls++;
      if (this===rootStyle) counter.rootProperties.push(name);
      return original.call(this,name,value,priority);
    };
    return style.cssText;
  });
  await page.getByTestId("color-token-primary").click();
  await page.getByTestId("color-value").fill("#3467ab");
  await page.getByTestId("color-value").press("Enter");
  await expect(page.getByTestId("studio-preview")).toHaveCSS("--primary","rgb(52 103 171)");
  await page.evaluate(() => new Promise<void>(resolve => requestAnimationFrame(() => requestAnimationFrame(() => resolve()))));
  expect(await toolbar.evaluate(node => (node.closest("[data-rustify-theme-boundary]") as HTMLElement).style.cssText)).toBe(before);
  expect(await page.evaluate(() => (window as any).__toolbarProjection.calls)).toBe(0);
  expect(await page.evaluate(() => (window as any).__toolbarProjection.rootProperties)).toEqual(["--primary"]);
});
