import {test, expect, waitForReady} from "./support";
import {mkdir, writeFile} from "node:fs/promises";

test.use({fresh:true});

for (const width of [1280,390]) {
  test(`slider parts share their center at ${width}px and changing rem`, async ({page}, info) => {
    await page.setViewportSize({width,height:900});
    await waitForReady(page);
    await page.getByTestId("editor-controls").click();
    const slider=page.getByTestId("hsl-hue-slider");
    await expect(slider).toHaveAttribute("aria-valuenow", "0");
    const root=page.getByTestId("hsl-hue-slider-root");
    const directory=process.env.RUSTIFY_SLIDER_EVIDENCE_OUT || "docs/validation/dom-components-p1/acceptance/slider";
    await mkdir(directory,{recursive:true});
    for (const rem of [16,20]) {
      await page.evaluate(rem => {
        document.documentElement.style.fontSize=`${rem}px`;
        window.__theme_studio.environment();
      },rem);
      await slider.scrollIntoViewIfNeeded();
      const path=`${directory}/slider-${width}-${rem}.png`;
      await root.screenshot({path});
      const track=(await root.locator('[data-rustify-part="slider-track"]').boundingBox())!;
      const fill=(await root.locator('[data-rustify-part="slider-range"]').boundingBox())!;
      const result={width,rem,track,fill,fillCenter:fill.y+fill.height/2,trackCenter:track.y+track.height/2};
      await writeFile(path.replace(/\.png$/,".json"),JSON.stringify(result,null,2));
      await info.attach(`slider ${width}px rem ${rem}`,{path,contentType:"image/png"});
      expect(fill.width,JSON.stringify(result)).toBeGreaterThan(0);
      expect(track.height,JSON.stringify(result)).toBeGreaterThan(0);
      expect(fill.x).toBeGreaterThanOrEqual(track.x-1);
      expect(fill.x+fill.width).toBeLessThanOrEqual(track.x+track.width+1);
      expect(Math.abs(result.fillCenter-result.trackCenter),JSON.stringify(result)).toBeLessThanOrEqual(1);
    }
    await slider.focus();
    await slider.press("ArrowRight");
    await expect(slider).toHaveAttribute("aria-valuenow", "1");
    const bounds=await root.boundingBox();
    expect(bounds).not.toBeNull();
    await page.mouse.move(bounds!.x+bounds!.width/2,bounds!.y+bounds!.height/2);
    await page.mouse.down();
    await page.mouse.move(bounds!.x+bounds!.width*.75,bounds!.y+bounds!.height/2,{steps:5});
    await page.mouse.up();
    await expect.poll(async()=>Number(await slider.getAttribute("aria-valuenow"))).toBeGreaterThan(1);
  });
}
