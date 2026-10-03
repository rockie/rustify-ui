import {test,expect} from "./support";
test.use({fresh:true});
test("latest static product starts without a trap or CSP violation",async({page})=> {
  const errors:string[]=[];
  page.on("pageerror",error=>errors.push(error.message));
  await page.addInitScript(()=> { (window as any).__themeCsp=[];document.addEventListener("securitypolicyviolation",event=>(window as any).__themeCsp.push(event.violatedDirective)); });
  await page.goto("./");
  await expect(page.getByTestId("status")).toHaveAttribute("data-status",/ready|failed|fatal/);
  expect(await page.getByTestId("status").getAttribute("data-status"),await page.getByTestId("status").textContent()).toBe("ready");
  expect(errors).toEqual([]);expect(await page.evaluate(()=>(window as any).__themeCsp)).toEqual([]);
  await page.locator(".studio-isolation > summary").click();
  await page.getByTestId("apply-patch").click();
  await page.getByTestId("studio-preview").getByTestId("open-preview-dialog").click();
  expect(await page.evaluate(()=>(window as any).__themeCsp)).toEqual([]);
});
