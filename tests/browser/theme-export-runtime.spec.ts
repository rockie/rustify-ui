import {test,expect} from "@playwright/test";
import {mkdir,writeFile} from "node:fs/promises";

const evidence = process.env.RUSTIFY_THEME_EXPORT_OUT || "docs/validation/dom-components-p1/export";

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
        expect(got.radius).toBeCloseTo(theme.radii_px[index],5);expect(got.padding).toBeCloseTo(theme.spacing_px*4,5);
        if(kind==="shadow") {
          const geometry=[...got.shadow.matchAll(/(-?\d*\.?\d+)px/g)].map(match=>Number(match[1])).slice(-theme.shadow_lg.length*4);
          const wanted=theme.shadow_lg.flatMap((layer:any)=>[layer.offset_x,layer.offset_y,layer.blur,layer.spread]);
          expect(geometry.length).toBe(wanted.length);geometry.forEach((value,index)=>expect(value).toBeCloseTo(wanted[index],5));
        }
      }
      const primary=await page.getByTestId(`${prefix}-radius-sm`).evaluate(el=> {
        const color=getComputedStyle(el).color,canvas=document.createElement("canvas");canvas.width=canvas.height=1;
        const ctx=canvas.getContext("2d")!;ctx.fillStyle=color;ctx.fillRect(0,0,1,1);return Array.from(ctx.getImageData(0,0,1,1).data);
      });
      theme.primary_rgba.forEach((channel:number,i:number)=>expect(Math.abs(primary[i]-channel*255)).toBeLessThanOrEqual(2));
      for(const [slot,index] of [["sans",0],["serif",1],["mono",2]] as const) {
        const sample=page.getByTestId(`${prefix}-${slot}`);const style=await sample.evaluate(el=>{const s=getComputedStyle(el);return {font:s.fontFamily,size:parseFloat(s.fontSize),spacing:parseFloat(s.letterSpacing)||0};});
        expect(style.font).toContain(theme.fonts[index].family);expect(style.size).toBeCloseTo(theme.font_size_px,5);expect(style.spacing).toBeCloseTo(theme.letter_spacing_px,4);
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
    await mkdir(evidence,{recursive:true});await writeFile(`${evidence}/${info.project.name}-${profile}-${notation}.json`,JSON.stringify(results,null,2));
    if(profile==="scoped"&&notation==="rgb")await page.screenshot({path:`${evidence}/${info.project.name}-css.png`,fullPage:true});
  });
}

test("compiled original Rust snippet mounts JSON in independent light and dark scopes",async({page},info)=> {
  const errors:string[]=[];page.on("pageerror",e=>errors.push(e.message));const failed:string[]=[];page.on("requestfailed",r=>failed.push(r.url()));
  await page.goto("rust.html");await expect(page.getByTestId("runtime-status")).toHaveAttribute("data-status","ready");
  const expected=await (await page.request.get("expected.json")).json();
  // Ordinary providers obey authored appearance=dark, including the snippet's initial light input.
  await expect(page.getByTestId("runtime-light")).toHaveAttribute("data-theme","dark");await expect(page.getByTestId("runtime-dark")).toHaveAttribute("data-theme","dark");
  const light=page.getByTestId("runtime-light").locator("main"),dark=page.getByTestId("runtime-dark-main");
  expect(await light.evaluate(el=>parseFloat(getComputedStyle(el).borderRadius))).toBeCloseTo(expected.themes[1].radii_px[2],5);
  expect(await dark.evaluate(el=>parseFloat(getComputedStyle(el).borderRadius))).toBeCloseTo(expected.themes[1].radii_px[2],5);
  expect(await light.evaluate(el=>parseFloat(getComputedStyle(el).paddingLeft))).toBeCloseTo(expected.themes[1].spacing_px*4,5);
  await page.evaluate(async()=>{await document.fonts.ready;});
  expect(await dark.getByTestId("runtime-sans").evaluate(el=>getComputedStyle(el).fontFamily)).toContain("IBM Plex Sans");
  expect(await dark.getByTestId("runtime-serif").evaluate(el=>getComputedStyle(el).fontFamily)).toContain("Noto Serif");
  expect(await dark.getByTestId("runtime-mono").evaluate(el=>getComputedStyle(el).fontFamily)).toContain("JetBrains Mono");
  expect(errors).toEqual([]);expect(failed).toEqual([]);
  await mkdir(evidence,{recursive:true});await page.screenshot({path:`${evidence}/${info.project.name}-rust.png`,fullPage:true});
  await page.evaluate(()=>(window as any).__theme_export_consumer.theme_export_dispose());
  await expect(page.getByTestId("runtime-light")).not.toHaveAttribute("data-theme");await expect(page.getByTestId("runtime-dark")).not.toHaveAttribute("data-theme");
});

test("default DOM SDK consumes original Rust and JSON, respects appearance and explicit preview",async({page},info)=> {
  const errors:string[]=[],failed:string[]=[],requests:string[]=[];
  page.on("pageerror",error=>errors.push(error.message));
  page.on("request",request=>requests.push(request.url()));
  page.on("requestfailed",request=>failed.push(request.url()));
  page.on("response",response=>{if(response.status()>=400)failed.push(response.url());});
  await page.addInitScript(()=>{
    (window as any).__csp=[];(window as any).__webgl=[];
    document.addEventListener("securitypolicyviolation",event=>(window as any).__csp.push(event.violatedDirective));
    const getContext=HTMLCanvasElement.prototype.getContext;
    HTMLCanvasElement.prototype.getContext=function(kind:any,...args:any[]){
      if(String(kind).startsWith("webgl"))(window as any).__webgl.push(kind);
      return (getContext as any).call(this,kind,...args);
    } as any;
  });
  await page.goto("dom.html");
  await expect(page.locator("#dom-status")).toHaveAttribute("data-status","ready");
  const expected=await (await page.request.get("expected.json")).json();
  const json=await (await page.request.get("theme.json")).json();
  expect(json.schema_version).toBe(2);expect(json.config.scaling).toBe(110);
  expect(json.config.appearance).toBe("dark");expect(json.config.gray_color).toBe("auto");
  expect(json.base.light["consumer-note"]).toBe("保留原始作者字符串 / not projected");
  const results=[];
  for(const [id,index] of [["dom-snippet",1],["dom-json",1],["dom-preview-light",0],["dom-preview-dark",1]] as const){
    const node=page.getByTestId(id),theme=expected.themes[index];
    await expect(node).toHaveAttribute("data-theme",theme.mode);
    await expect(node).toHaveAttribute("data-rustify-theme-runtime","v4");
    const properties=await node.evaluate((element,names:string[])=>{
      const style=getComputedStyle(element);
      return Object.fromEntries(names.map(name=>[name,style.getPropertyValue(name).trim()]));
    },theme.properties.map(([name]:string[])=>name));
    // CSS-wide initial is the guaranteed-invalid computed custom-property value.
    expect(properties).toEqual(Object.fromEntries(theme.properties.map(([name,value]:string[])=>[name,value==="initial"?"":value])));
    const declared=await node.evaluate((element,names:string[])=>(Object.fromEntries(names.map(name=>[name,(element as HTMLElement).style.getPropertyValue(name).trim()]))),theme.properties.filter(([,value]:string[])=>value==="initial").map(([name]:string[])=>name));
    expect(Object.values(declared).every(value=>value==="initial")).toBe(true);
    expect(await node.evaluate(element=>getComputedStyle(element).getPropertyValue("--consumer-note"))).toBe("");
    const main=node.locator("main");await expect(main).toBeVisible();
    const size=await main.evaluate(element=>{const style=getComputedStyle(element);return {radius:parseFloat(style.borderRadius),padding:parseFloat(style.paddingLeft)};});
    expect(size.radius).toBeCloseTo(theme.radii_px[2],5);
    expect(size.padding).toBeCloseTo(theme.spacing_px*4,5);
    if(id==="dom-json"){
      const button=node.getByRole("button",{name:"Continue"});await expect(button).toBeVisible();
      const height=await button.evaluate(element=>parseFloat(getComputedStyle(element).height));
      // Layout dimensions are quantized to Chromium's 1/64 CSS-pixel units.
      expect(Math.abs(height-parseFloat(properties["--rustify-control-height-2"]))).toBeLessThan(.016);
    }
    results.push({id,mode:theme.mode,size,properties});
  }
  await page.evaluate(async()=>{await document.fonts.ready;});
  expect(await page.evaluate(()=>(window as any).__theme_export_dom.hostStyle())).toEqual(await page.evaluate(()=>(window as any).__theme_export_dom.before));
  expect(await page.locator("canvas").count()).toBe(0);
  expect(await page.evaluate(()=>(window as any).__webgl)).toEqual([]);
  expect(requests.filter(url=>url.endsWith(".wasm")).every(url=>url.endsWith("/pkg/app_bg.wasm"))).toBe(true);
  expect(requests.some(url=>/message_bridge|rustify_makepad|embedded\.js/.test(url))).toBe(false);
  expect(requests.some(url=>url.includes("/makepad_widgets/resources/")&&url.endsWith(".ttf"))).toBe(true);
  expect(errors).toEqual([]);expect(failed).toEqual([]);expect(await page.evaluate(()=>(window as any).__csp)).toEqual([]);
  await mkdir(evidence,{recursive:true});
  await writeFile(`${evidence}/${info.project.name}-dom.json`,JSON.stringify({base:info.project.use.baseURL,config:json.config,results,requests},null,2));
  await page.screenshot({path:`${evidence}/${info.project.name}-dom.png`,fullPage:true});
  await page.setViewportSize({width:390,height:844});
  expect(await page.evaluate(()=>document.documentElement.scrollWidth<=innerWidth+1)).toBe(true);
  await page.screenshot({path:`${evidence}/${info.project.name}-dom-mobile.png`,fullPage:true});
  await page.evaluate(()=>(window as any).__theme_export_dom.app.dom_export_dispose());
  for(const id of ["dom-snippet","dom-json","dom-preview-light","dom-preview-dark"]){
    await expect(page.getByTestId(id)).not.toHaveAttribute("data-theme");
  }
});
