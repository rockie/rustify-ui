import {test, expect, waitForReady} from "./support";
import {PNG} from "pngjs";
import {mkdir, writeFile} from "node:fs/promises";

test.use({fresh:true});

for (const width of [1280,390]) {
  test(`slider fill and empty track share their center at ${width}px and changing rem`, async ({page}, info) => {
    await page.setViewportSize({width,height:900});
    await waitForReady(page);
    await page.getByTestId("editor-controls").click();
    const slider=page.getByTestId("hsl-hue-slider");
    await expect(slider).toHaveValue("0");
    const directory=process.env.RUSTIFY_SLIDER_EVIDENCE_OUT || "docs/validation/dom-components-p1/acceptance/slider";
    await mkdir(directory,{recursive:true});
    for (const rem of [16,20]) {
      await page.evaluate(rem => {
        document.documentElement.style.fontSize=`${rem}px`;
        window.__theme_studio.environment();
      },rem);
      await slider.scrollIntoViewIfNeeded();
      const colors=await slider.evaluate(node => {
        const canvas=document.createElement("canvas");canvas.width=canvas.height=1;
        const context=canvas.getContext("2d")!;
        return ["--primary","--border"].map(name => {
          context.fillStyle=getComputedStyle(node).getPropertyValue(name);
          context.fillRect(0,0,1,1);
          return Array.from(context.getImageData(0,0,1,1).data).slice(0,3);
        });
      });
      const path=`${directory}/slider-${width}-${rem}.png`;
      const image=PNG.sync.read(await slider.screenshot({path}));
      const rows=(fraction:number,color:number[]) => Array.from({length:image.height},(_,y)=>y).filter(y => {
        const offset=(y*image.width+Math.floor(image.width*fraction))*4;
        return color.every((value,index)=>Math.abs(image.data[offset+index]-value)<=3);
      });
      const fill=rows(.25,colors[0]),track=rows(.75,colors[1]);
      const center=(values:number[]) => values.reduce((sum,value)=>sum+value,0)/values.length;
      const result={width,rem,fill,track,fillCenter:center(fill),trackCenter:center(track)};
      await writeFile(path.replace(/\.png$/,".json"),JSON.stringify(result,null,2));
      await info.attach(`slider ${width}px rem ${rem}`,{path,contentType:"image/png"});
      expect(fill.length,JSON.stringify(result)).toBeGreaterThan(0);
      expect(track.length,JSON.stringify(result)).toBeGreaterThan(0);
      expect(Math.abs(result.fillCenter-result.trackCenter),JSON.stringify(result)).toBeLessThanOrEqual(1);
    }
    await slider.focus();
    await slider.press("ArrowRight");
    await expect(slider).toHaveValue("1");
    const bounds=await slider.boundingBox();
    expect(bounds).not.toBeNull();
    await page.mouse.move(bounds!.x+bounds!.width/2,bounds!.y+bounds!.height/2);
    await page.mouse.down();
    await page.mouse.move(bounds!.x+bounds!.width*.75,bounds!.y+bounds!.height/2,{steps:5});
    await page.mouse.up();
    await expect.poll(async()=>Number(await slider.inputValue())).toBeGreaterThan(1);
  });
}
