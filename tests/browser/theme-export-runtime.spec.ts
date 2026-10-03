import {test,expect} from "@playwright/test";
import {mkdir,writeFile} from "node:fs/promises";

for(const profile of ["standard","scoped"])for(const notation of ["hex","rgb","hsl","oklch"]) {
  test(`independent ${profile} ${notation} CSS renders modes, utilities and fonts`,async({page},info)=> {
    const errors:string[]=[];page.on("pageerror",e=>errors.push(e.message));
    await page.addInitScript(()=>{(window as any).__csp=[];document.addEventListener("securitypolicyviolation",e=>(window as any).__csp.push(e.violatedDirective));});
    const failed:string[]=[];page.on("requestfailed",r=>failed.push(r.url()));page.on("response",r=>{if(r.status()>=400)failed.push(r.url());});
    await page.goto(`${profile}-${notation}.html`);
    const expected=await (await page.request.get("expected.json")).json();const results=[];
    for(const theme of expected.themes.slice(0,profile==="scoped"?4:2)) {
      const prefix=`${theme.document_id}-${theme.mode}`,node=page.getByTestId(prefix);
      await expect(node).toBeVisible();
      for(const [kind,index] of [["radius-sm",0],["radius-md",1],["shadow",2],["radius-xl",3]] as const) {
        const got=await page.getByTestId(`${prefix}-${kind}`).evaluate(el=>{const s=getComputedStyle(el);return {radius:parseFloat(s.borderTopLeftRadius),padding:parseFloat(s.paddingLeft),shadow:s.boxShadow};});
        expect(got.radius).toBe(theme.radii_px[index]);expect(got.padding).toBe(theme.spacing_px*4);
        if(kind==="shadow") {
          for(const layer of theme.shadow_lg)expect(got.shadow).toContain(`${layer.offset_x}px ${layer.offset_y}px ${layer.blur}px ${layer.spread}px`);
        }
      }
      const primary=await page.getByTestId(`${prefix}-radius-sm`).evaluate(el=> {
        const color=getComputedStyle(el).color,canvas=document.createElement("canvas");canvas.width=canvas.height=1;
        const ctx=canvas.getContext("2d")!;ctx.fillStyle=color;ctx.fillRect(0,0,1,1);return Array.from(ctx.getImageData(0,0,1,1).data);
      });
      theme.primary_rgba.forEach((channel:number,i:number)=>expect(Math.abs(primary[i]-channel*255)).toBeLessThanOrEqual(2));
      for(const [slot,index] of [["sans",0],["serif",1],["mono",2]] as const) {
        const sample=page.getByTestId(`${prefix}-${slot}`);const style=await sample.evaluate(el=>{const s=getComputedStyle(el);return {font:s.fontFamily,size:parseFloat(s.fontSize),spacing:parseFloat(s.letterSpacing)||0};});
        expect(style.font).toContain(theme.fonts[index].family);expect(style.size).toBe(theme.font_size_px);expect(style.spacing).toBeCloseTo(theme.letter_spacing_px,4);
        await page.evaluate(async family=>{await document.fonts.load(`16px "${family}"`);},theme.fonts[index].family);
        expect(await page.evaluate(family=>document.fonts.check(`16px "${family}"`),theme.fonts[index].family)).toBe(true);
      }
      const ring=await page.getByTestId(`${prefix}-ring`).evaluate(el=>getComputedStyle(el).boxShadow);expect(ring).not.toBe("none");
      expect(await page.getByTestId(`${prefix}-shadow-color`).evaluate(el=>getComputedStyle(el).boxShadow)).not.toBe(await page.getByTestId(`${prefix}-shadow`).evaluate(el=>getComputedStyle(el).boxShadow));
      results.push({prefix,primary});
    }
    if(profile==="scoped") {
      const host=await page.getByTestId("host-sentinel").evaluate(el=>{const s=getComputedStyle(el);return {radius:s.borderRadius,padding:s.paddingLeft,font:s.fontFamily,shadow:s.boxShadow};});
      expect(host.radius).toBe("4px");expect(host.padding).toBe("16px");expect(host.font).toContain("ui-sans-serif");expect(host.font).not.toContain("IBM Plex Sans");
      expect(host.shadow).toBe([...Array(4).fill("rgba(0, 0, 0, 0) 0px 0px 0px 0px"),"rgba(0, 0, 0, 0.1) 0px 10px 15px -3px","rgba(0, 0, 0, 0.1) 0px 4px 6px -4px"].join(", "));
    }
    expect(errors).toEqual([]);expect(failed).toEqual([]);expect(await page.evaluate(()=>(window as any).__csp)).toEqual([]);
    await mkdir("docs/validation/theme-studio/M5",{recursive:true});await writeFile(`docs/validation/theme-studio/M5/${info.project.name}-${profile}-${notation}.json`,JSON.stringify(results,null,2));
    if(profile==="scoped"&&notation==="rgb")await page.screenshot({path:`docs/validation/theme-studio/M5/${info.project.name}-css.png`,fullPage:true});
  });
}

test("compiled original Rust snippet mounts JSON in independent light and dark scopes",async({page},info)=> {
  const errors:string[]=[];page.on("pageerror",e=>errors.push(e.message));const failed:string[]=[];page.on("requestfailed",r=>failed.push(r.url()));
  await page.goto("rust.html");await expect(page.getByTestId("runtime-status")).toHaveAttribute("data-status","ready");
  const expected=await (await page.request.get("expected.json")).json();
  await expect(page.getByTestId("runtime-light")).toHaveAttribute("data-theme","light");await expect(page.getByTestId("runtime-dark")).toHaveAttribute("data-theme","dark");
  const light=page.getByTestId("runtime-light").locator("main"),dark=page.getByTestId("runtime-dark-main");
  expect(await light.evaluate(el=>getComputedStyle(el).borderRadius)).toBe(`${expected.themes[0].radii_px[2]}px`);
  expect(await dark.evaluate(el=>getComputedStyle(el).borderRadius)).toBe(`${expected.themes[1].radii_px[2]}px`);
  expect(await light.evaluate(el=>getComputedStyle(el).paddingLeft)).toBe(`${expected.themes[0].spacing_px*4}px`);
  await page.evaluate(async()=>{await document.fonts.ready;});
  expect(await dark.getByTestId("runtime-sans").evaluate(el=>getComputedStyle(el).fontFamily)).toContain("IBM Plex Sans");
  expect(await dark.getByTestId("runtime-serif").evaluate(el=>getComputedStyle(el).fontFamily)).toContain("Noto Serif");
  expect(await dark.getByTestId("runtime-mono").evaluate(el=>getComputedStyle(el).fontFamily)).toContain("JetBrains Mono");
  expect(errors).toEqual([]);expect(failed).toEqual([]);
  await mkdir("docs/validation/theme-studio/M5",{recursive:true});await page.screenshot({path:`docs/validation/theme-studio/M5/${info.project.name}-rust.png`,fullPage:true});
  await page.evaluate(()=>(window as any).__theme_export_consumer.theme_export_dispose());
  await expect(page.getByTestId("runtime-light")).not.toHaveAttribute("data-theme");await expect(page.getByTestId("runtime-dark")).not.toHaveAttribute("data-theme");
});
