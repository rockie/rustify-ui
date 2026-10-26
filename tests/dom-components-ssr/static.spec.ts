import { test, expect } from "@playwright/test";

test("static theme components render both modes and responsive external geometry without scripts", async ({page,context},info)=>{
  const baseline=await context.newPage();
  await baseline.setContent('<button type="button">Host sentinel</button>');
  const hostBefore=await baseline.locator('button').evaluate(el=>{const s=getComputedStyle(el);return [s.color,s.backgroundColor,s.fontSize,s.borderRadius,s.padding];});
  await baseline.close();
  const requests:string[]=[],failures:string[]=[];
  page.on('request',request=>requests.push(request.url()));
  page.on('pageerror',error=>failures.push(error.message));
  page.on('console',message=>{if(message.type()==='error')failures.push(message.text());});
  page.on('requestfailed',request=>failures.push(request.url()));
  page.on('response',response=>{if(response.status()>=400)failures.push(`${response.status()} ${response.url()}`);});
  await page.setViewportSize({width:1280,height:900});
  await page.goto('/');
  await expect(page.locator('script,style,[style]')).toHaveCount(0);
  const path='docs/validation/dom-components-p1/components-ssr.png';
  await page.screenshot({path});
  await info.attach('static components',{path,contentType:'image/png'});
  for(const mode of ['light','dark']){
    const seed=`ssr-${mode}`,root=page.getByTestId(seed);
    await expect(root).toHaveAttribute('data-theme',mode);
    await expect(root.getByTestId(`${seed}-button`)).toHaveCSS('background-color',mode==='light'?'rgb(124, 58, 237)':'rgb(167, 139, 250)');
    await expect(root.getByTestId(`${seed}-icon-button`)).toHaveAccessibleName(/.+/);
    await expect(root.getByTestId(`${seed}-loading`)).toBeDisabled();
    await expect(root.getByTestId(`${seed}-disabled-link`)).not.toHaveAttribute('href');
    await expect(root.getByTestId(`${seed}-box`)).toHaveCSS('padding-left','12px');
    await expect(root.getByTestId(`${seed}-panel`)).toBeVisible();
    for(const field of ['appearance','accent_color','gray_color','panel_background','radius','scaling']){
      await expect(root.getByTestId(`${seed}-panel-${field}`)).toBeVisible();
    }
    await expect(root.getByTestId(`${seed}-truncate`)).toHaveCSS('text-overflow','ellipsis');
    for(const [width,columns] of [[390,1],[767,1],[768,2],[1279,2],[1280,3]]){
      await page.setViewportSize({width,height:900});
      expect(await root.getByTestId(`${seed}-grid`).evaluate(el=>getComputedStyle(el).gridTemplateColumns.split(' ').length)).toBe(columns);
    }
  }
  expect(await page.getByTestId('static-host').evaluate(el=>{const s=getComputedStyle(el);return [s.color,s.backgroundColor,s.fontSize,s.borderRadius,s.padding];})).toEqual(hostBefore);
  expect(requests.filter(url=>/\.(?:js|wasm|ttf|woff2?)(?:\?|$)/.test(url))).toEqual([]);
  expect(failures).toEqual([]);
});
