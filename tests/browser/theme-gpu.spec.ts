import {test,expect,waitForReady,waitForQuiet} from "./support";
import {PNG} from "pngjs";
import type {Page} from "@playwright/test";
import {mkdir,writeFile} from "node:fs/promises";

const evidence="docs/validation/theme-studio/M3";
async function record(page:Page,name:string,state:any) {
  await mkdir(evidence,{recursive:true});
  await page.getByTestId("theme-gpu").screenshot({path:`${evidence}/${name}.png`});
  await writeFile(`${evidence}/${name}.json`,JSON.stringify(state,null,2));
}

const snapshot=(page:Page)=>page.evaluate(()=>window.__theme_studio.snapshot());
async function drawn(page:Page) {
  await page.getByTestId("theme-gpu").scrollIntoViewIfNeeded();
  await expect.poll(async()=> (await snapshot(page)).gpu_state).toBe("Ready");
  await expect.poll(async()=> {const s=await snapshot(page);return s.drawn_revision===s.revision && s.samples.length>50;}).toBe(true);
  await waitForQuiet(page);
  return snapshot(page);
}
const sample=(state:any,id:string)=>state.samples.find((value:any)=>value.id===id);
const pixel=(png:PNG,x:number,y:number)=> Array.from(png.data.subarray((Math.floor(y)*png.width+Math.floor(x))*4,(Math.floor(y)*png.width+Math.floor(x))*4+4));

test.describe("GPU consumes the resolved snapshot",()=> {
  test.beforeEach(async({page})=> {await page.setViewportSize({width:1440,height:3600});await waitForReady(page);await drawn(page);});
  test("actual solid pixels include all semantics and alpha blends on the checker",async({page})=> {
    await page.evaluate(()=> {window.__theme_studio.edit("primary","rgb(200 40 80 / .5)");window.__theme_studio.edit("chart-3","#18a4e0");window.__theme_studio.edit("sidebar-primary","#844acc");});
    const state=await drawn(page);
    expect(state.region_count).toBe(1);
    expect(state.samples.filter((s:any)=>s.id.startsWith("color.")).length).toBe(35);
    const png=PNG.sync.read(await page.getByTestId("theme-gpu").screenshot());
    for(const s of state.samples.filter((s:any)=>s.id.startsWith("color.")&&s.rgba[3]===1)) {
      const [x,y,w,h]=s.rect,got=pixel(png,x+w/2,y+h/2);
      expect(got).toHaveLength(4);
      s.rgba.slice(0,3).forEach((channel:number,index:number)=>expect(Math.abs(got[index]-channel*255),s.id).toBeLessThanOrEqual(2));
    }
    for (const [token,want] of [["chart-3",[24,164,224]],["sidebar-primary",[132,74,204]]] as const) {
      const s=sample(state,`color.${token}`), [x,y,w,h]=s.rect;
      const got=pixel(png,x+w/2,y+h/2);
      want.forEach((channel,index)=>expect(Math.abs(got[index]-channel),`${token}: ${got}`).toBeLessThanOrEqual(2));
    }
    const s=sample(state,"color.primary"),[x,y,w,h]=s.rect;
    const px=Math.floor(w/2),py=Math.floor(h/2);
    const checker=((Math.floor(px/8)+Math.floor(py/8))%2 ? .82:.98)*255;
    const got=pixel(png,x+px,y+py),want=[200,40,80].map(channel=>channel*.5+checker*.5);
    want.forEach((channel,index)=>expect(Math.abs(got[index]-channel),`alpha blend: ${got}, ${want}`).toBeLessThanOrEqual(3));
    expect(got[3]).toBe(255);
    await record(page,"gpu-alpha",state);
  });
  test("host rem, actual font resources and letter spacing change measured layout",async({page})=> {
    const before=await drawn(page),initial=sample(before,"font.sans.latin");
    await page.evaluate(()=> {document.documentElement.style.setProperty("font-size","20px");window.__theme_studio.environment();window.__theme_studio.edit("radius",".5rem");window.__theme_studio.edit("font-sans","Noto Serif");window.__theme_studio.edit("letter-spacing","0.1em");});
    const next=await drawn(page),changed=sample(next,"font.sans.latin");
    expect(sample(next,"radius.lg").radius_px).toBe(10);
    expect(sample(next,"radius.xl").radius_px).toBe(14);
    expect(changed.font).toBe("Noto Serif");expect(changed.letter_spacing_em).toBe(.1);
    expect(changed.layout_size_px[0]).not.toBe(initial.layout_size_px[0]);
    const dom=await page.getByTestId("studio-preview").getByTestId("preview-surface").evaluate(el=>({font:getComputedStyle(el).fontFamily,spacing:getComputedStyle(el).letterSpacing}));
    expect(dom.font).toContain('"Noto Serif"');expect(dom.spacing).toBe("1.5px");
    await record(page,"gpu-font-spacing",next);
    const beforeSpacing=changed.layout_size_px[0];
    await page.evaluate(()=>window.__theme_studio.edit("letter-spacing","0em"));
    expect(sample(await drawn(page),"font.sans.latin").layout_size_px[0]).toBeLessThan(beforeSpacing);
    await page.evaluate(()=>window.__theme_studio.edit("font-sans","Unknown External Font, sans-serif"));
    const fallback=sample(await drawn(page),"font.sans.latin");expect(fallback.font).toBe("IBM Plex Sans");
    expect((await snapshot(page)).document.styles.light["font-sans"]).toBe("Unknown External Font, sans-serif");
    await page.evaluate(()=>document.documentElement.style.removeProperty("font-size"));
  });
  test("soft shadows extend visual bounds without extending hit bounds",async({page})=> {
    await page.evaluate(()=> {for(const [token,value] of Object.entries({"shadow-opacity":".5","shadow-blur":"10px","shadow-spread":"3px","shadow-offset-x":"8px","shadow-offset-y":"10px"})) window.__theme_studio.edit(token,value);});
    const state=await drawn(page),s=sample(state,"shadow.shadow-lg");
    expect(s.shadow_layers.length).toBe(2);
    expect(s.visual_rect[2]).toBeGreaterThan(s.hit_rect[2]);expect(s.visual_rect[3]).toBeGreaterThan(s.hit_rect[3]);
    const png=PNG.sync.read(await page.getByTestId("theme-gpu").screenshot());
    const [x,y,w,h]=s.hit_rect;
    const edge=pixel(png,x+w/2,y+h+5),away=pixel(png,x+w/2,y+h+28);
    expect(edge.slice(0,3).reduce((a,b)=>a+b)).toBeLessThan(away.slice(0,3).reduce((a,b)=>a+b));
    await record(page,"gpu-shadow",state);
    await page.getByTestId("theme-gpu").click({position:{x:x+w/2,y:y+h+4}});
    expect((await snapshot(page)).controls.clicks).toBe(0);
    await page.evaluate(()=> {window.__theme_studio.edit("shadow-blur","0px");window.__theme_studio.edit("shadow-spread","-2px");});
    expect(sample(await drawn(page),"shadow.shadow-lg").shadow_layers[0].blur).toBe(0);
  });
  test("controlled inputs update through actions and context restoration uses the latest revision",async({page})=> {
    let state=await drawn(page);const button=sample(state,"control.button");
    await page.getByTestId("theme-gpu").scrollIntoViewIfNeeded();
    const [x,y,w,h]=button.rect;
    await page.getByTestId("theme-gpu").click({position:{x:x+w/2,y:y+h/2}});
    await expect.poll(async()=> (await snapshot(page)).controls.clicks).toBe(1);
    await page.getByTestId("theme-gpu").evaluate((canvas:HTMLCanvasElement)=> {
      const gl=canvas.getContext("webgl2")!;const extension=gl.getExtension("WEBGL_lose_context")!;(window as any).__theme_gpu_loss=extension;extension.loseContext();
    });
    await expect.poll(async()=> (await snapshot(page)).gpu_state).toBe("Lost");
    await page.evaluate(()=> {window.__theme_studio.edit("primary","#00a88a");(window as any).__theme_gpu_loss.restoreContext();delete (window as any).__theme_gpu_loss;});
    state=await drawn(page);expect(state.region_count).toBe(1);expect(state.drawn_revision).toBe(state.revision);
    expect(sample(state,"color.primary").rgba.slice(0,3)).toEqual([0,168/255,138/255]);
    await record(page,"gpu-restored",state);
  });
  test("all six packaged families paint actual Latin, CJK and emoji glyphs",async({page})=> {
    const layouts=[];
    for(const family of ["IBM Plex Sans","Noto Sans","Rustify WenKai","Noto Serif","Liberation Mono","JetBrains Mono"]) {
      await page.evaluate(async family=>{window.__theme_studio.edit("font-sans",family);await document.fonts.load(`15px "${family}"`);},family);
      const state=await drawn(page),png=PNG.sync.read(await page.getByTestId("theme-gpu").screenshot());
      for(const kind of ["latin","multilingual"]) {
        const s=sample(state,`font.sans.${kind}`);expect(s.font).toBe(family);
        const [x,y,w,h]=s.rect;
        let ink=0;
        for(let py=Math.ceil(y);py<Math.min(png.height,y+h);py++)for(let px=Math.ceil(x);px<Math.min(png.width,x+w);px++) {
          const got=pixel(png,px,py);if(got[0]+got[1]+got[2]<600)ink++;
        }
        expect(ink,`${family} ${kind} must paint glyphs`).toBeGreaterThan(10);
      }
      expect(await page.evaluate(family=>document.fonts.check(`15px "${family}"`),family)).toBe(true);
      layouts.push({family,latin:sample(state,"font.sans.latin"),multilingual:sample(state,"font.sans.multilingual")});
    }
    await mkdir(evidence,{recursive:true});await writeFile(`${evidence}/fonts.json`,JSON.stringify(layouts,null,2));
  });
});
