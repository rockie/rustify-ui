import {test,expect,waitForReady} from "./support";

const radii=async(root:any)=>Promise.all(["sm","md","lg","xl"].map(size=>root.getByTestId(`fixture-${size}`).evaluate((el:Element)=>getComputedStyle(el).borderRadius)));
const shadow=async(node:any)=>node.evaluate((el:Element)=>getComputedStyle(el).boxShadow);

test("one Tailwind product retains legacy geometry and elevation beside v4",async({page})=> {
  await waitForReady(page);
  await page.evaluate(()=>window.__component_catalog.mount_theme_fixtures());
  const old=page.getByTestId("catalog-theme-legacy"),modern=page.getByTestId("catalog-theme-v4");
  await expect.poll(()=>radii(old)).toEqual(["4px","6px","8px","12px"]);
  expect(await radii(modern)).toEqual(["2px","4px","6px","10px"]);
  const oldShadow=await shadow(old.getByTestId("fixture-lg"));
  expect(oldShadow).toContain("0px 10px 15px -3px");
  expect(oldShadow).toContain("0px 4px 6px -4px");
  await old.getByTestId("fixture-dialog-open").click();
  const oldDialog=old.getByTestId("fixture-dialog").locator('[data-name="Dialog"]');
  expect(await shadow(oldDialog)).toBe(oldShadow);
  await modern.getByTestId("fixture-dialog-open").click();
  const newDialog=modern.getByTestId("fixture-dialog").locator('[data-name="Dialog"]');
  await modern.getByTestId("fixture-change").click();
  await expect.poll(()=>radii(modern)).toEqual(["6px","8px","10px","14px"]);
  expect(await radii(old)).toEqual(["4px","6px","8px","12px"]);
  expect(await shadow(oldDialog)).toBe(oldShadow);
  await modern.getByTestId("fixture-patch").click();
  await expect.poll(()=>radii(modern)).toEqual(["0px","0px","0px","4px"]);
  await expect.poll(()=>shadow(newDialog)).toContain("0.3");
  await modern.getByTestId("fixture-unpatch").click();
  await expect.poll(()=>radii(modern)).toEqual(["6px","8px","10px","14px"]);
  await expect.poll(()=>shadow(newDialog)).not.toContain("0.3");
  expect(await page.locator('link[href$="tailwind.css"]').count()).toBe(1);
  expect(await page.locator('link[href$="rustify.css"]').count()).toBe(0);
  await page.evaluate(()=>window.__component_catalog.dispose_theme_fixtures());
  expect(await modern.getAttribute("data-theme")).toBeNull();
});
