# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: utility-runtime.spec.ts >> SDK ensure installs scanner-free CSS, coalesces requests and isolates independent scopes
- Location: tests/browser/utility-runtime.spec.ts:262:5

# Error details

```
Error: expect(received).toEqual(expected) // deep equality

- Expected  - 2
+ Received  + 2

  Array [
-   1,
-   1,
+   2,
+   2,
  ]
```

# Page snapshot

```yaml
- main [ref=e2]:
  - generic [ref=e3]:
    - heading "Utility Lab" [level=1] [ref=e4]
    - paragraph [ref=e5]: Apply complete class names to a live, isolated preview.
  - generic [ref=e6]:
    - generic [ref=e7]: Classes
    - textbox "Classes" [ref=e8]: w-[137px] bg-[#123456] h-[72px] text-[#ffffff] rounded-[8px]
    - generic [ref=e9]:
      - button "Apply classes" [ref=e10] [cursor=pointer]
      - button "Restart compiler" [ref=e11] [cursor=pointer]
      - button "Copy CSS" [ref=e12] [cursor=pointer]
    - generic [ref=e13]:
      - generic [ref=e14]:
        - checkbox "Apply supported GPU properties only" [ref=e15]
        - text: Apply supported GPU properties only
      - generic [ref=e16]:
        - checkbox "Disable sample button" [ref=e17]
        - text: Disable sample button
    - group [ref=e18]:
      - generic "Container, text and color block classes" [ref=e19] [cursor=pointer]
    - status [ref=e20]: Ready. Apply classes to update the preview.
  - group [ref=e21]:
    - generic "Scan source text" [ref=e22] [cursor=pointer]
  - generic [ref=e23]:
    - region [ref=e24]:
      - heading "DOM preview" [level=2] [ref=e25]
      - paragraph [ref=e26]: Updates after the generated stylesheet is installed.
    - region [ref=e28]:
      - heading "GPU preview" [level=2] [ref=e29]
      - paragraph [ref=e30]: Supported properties update after compilation. Unsupported properties keep the previous appearance.
  - region [ref=e32]:
    - heading "Independent scope" [level=2] [ref=e33]
    - paragraph [ref=e34]: Uses its own compiler and stylesheet.
    - generic [ref=e37]:
      - button "Live utility sample" [ref=e38]
      - generic [ref=e39]: Utility text
  - group [ref=e41]:
    - generic "Generated CSS and diagnostics" [ref=e42] [cursor=pointer]
  - generic [ref=e43]: Outside every utility scope
```

# Test source

```ts
  199 |         const closed = getComputedStyle(sample).width;
  200 |         frame.setAttribute('data-state', 'open');
  201 |         const open = getComputedStyle(sample).width;
  202 |         frame.setAttribute('data-state', 'closed');
  203 |         return {report, closed, open, restored: getComputedStyle(sample).width,
  204 |             background: getComputedStyle(sample).backgroundColor,
  205 |             otherWidth: getComputedStyle(document.getElementById('scope-b-sample')!).width};
  206 |     });
  207 |     expect(result.report.unsupported).toEqual([]);
  208 |     expect(result.report.rejected.filter((item: any) => ['[_]:block', 'data-foo[bar]:block'].includes(item.candidate))
  209 |         .map((item: any) => item.candidate).sort()).toEqual(['[_]:block', 'data-foo[bar]:block']);
  210 |     expect(result.report.admitted.filter((candidate: string) => ['[_]:block', 'data-foo[bar]:block', 'group-data-[state=open]:w-[159px]!'].includes(candidate))).toEqual(['group-data-[state=open]:w-[159px]!']);
  211 |     expect(result.report.css).not.toContain('ignored');
  212 |     expect(result.closed).toBe('137px');
  213 |     expect(result.open).toBe('159px');
  214 |     expect(result.restored).toBe('137px');
  215 |     expect(result.background).toBe('rgb(18, 52, 86)');
  216 |     expect(result.otherWidth).toBe('170px');
  217 |     expect(errors).toEqual([]); expect(failed).toEqual([]);
  218 |     await info.attach('Parser browser values', {body: JSON.stringify(result, null, 2), contentType: 'application/json'});
  219 | });
  220 | 
  221 | test('structural spacing, container and line clamp keep their actual browser semantics', async ({page}, info) => {
  222 |     await ready(page);
  223 |     const result = await page.evaluate(async () => {
  224 |         const api = (window as any).__utility_lab;
  225 |         const classes = ['border-separate', 'border-spacing-[12px]', 'flex', 'space-x-[9px]',
  226 |             'space-x-reverse', 'line-clamp-2', '@container/probe'];
  227 |         const report = await api.ensure('scope-a', classes);
  228 |         const root = document.getElementById('scope-a')!;
  229 |         const table = document.createElement('table');
  230 |         table.className = 'border-separate border-spacing-[12px]';
  231 |         table.innerHTML = '<tbody><tr><td>one</td><td>two</td></tr></tbody>';
  232 |         const siblings = document.createElement('div');
  233 |         siblings.className = 'flex space-x-[9px] space-x-reverse';
  234 |         siblings.innerHTML = '<span>one</span><span>two</span><span>three</span>';
  235 |         const container = document.createElement('div');
  236 |         container.className = '@container/probe';
  237 |         const clamp = document.createElement('p');
  238 |         clamp.className = 'line-clamp-2';
  239 |         clamp.textContent = 'Browser utility checks';
  240 |         container.append(clamp);
  241 |         root.append(table, siblings, container);
  242 |         const margins = Array.from(siblings.children).map(child => {
  243 |             const style = getComputedStyle(child);
  244 |             return [style.marginInlineStart, style.marginInlineEnd];
  245 |         });
  246 |         const containerStyle = getComputedStyle(container), clampStyle = getComputedStyle(clamp);
  247 |         return {revision: report.revision, pending: report.unsupported, spacing: getComputedStyle(table).borderSpacing,
  248 |             margins, containerType: containerStyle.containerType, containerName: containerStyle.containerName,
  249 |             lineClamp: clampStyle.webkitLineClamp, overflow: clampStyle.overflow, css: report.css};
  250 |     });
  251 |     expect(result.pending).toEqual([]);
  252 |     expect(result.spacing).toBe('12px');
  253 |     expect(result.margins).toEqual([['9px', '0px'], ['9px', '0px'], ['0px', '0px']]);
  254 |     expect(result.containerType).toBe('inline-size');
  255 |     expect(result.containerName).toBe('probe');
  256 |     expect(result.lineClamp).toBe('2');
  257 |     expect(result.overflow).toBe('hidden');
  258 |     expect(result.css).not.toContain('--tw-sort');
  259 |     await info.attach('Structural browser values', {body: JSON.stringify(result, null, 2), contentType: 'application/json'});
  260 | });
  261 | 
  262 | test('SDK ensure installs scanner-free CSS, coalesces requests and isolates independent scopes', async ({ page }, testInfo) => {
  263 |     const errors: string[] = [];
  264 |     const failed: string[] = [];
  265 |     const paths: string[] = [];
  266 |     page.on('pageerror', error => errors.push(error.message));
  267 |     page.on('requestfailed', request => failed.push(request.url()));
  268 |     page.on('request', request => paths.push(request.url()));
  269 |     await ready(page);
  270 |     const result = await page.evaluate(async () => {
  271 |         const api = (window as any).__utility_lab;
  272 |         const violations: string[] = [];
  273 |         document.addEventListener('securitypolicyviolation', event => violations.push(event.violatedDirective));
  274 |         const first = await Promise.all([
  275 |             api.ensure('scope-a', ['w-[137px]']),
  276 |             api.ensure('scope-a', ['bg-[#123456]', 'w-[137px]']),
  277 |         ]);
  278 |         const sample = document.getElementById('scope-a-sample')!;
  279 |         sample.className = 'sample w-[137px] bg-[#123456]';
  280 |         const style = getComputedStyle(sample);
  281 |         const width = style.width;
  282 |         const color = style.backgroundColor;
  283 |         const initial = api.report('scope-a');
  284 |         const repeated = await api.ensure('scope-a', ['w-[137px]']);
  285 |         const other = await api.ensure('scope-b', ['w-[93px]', 'bg-[#abcdef]']);
  286 |         const second = document.getElementById('scope-b-sample')!;
  287 |         second.className = 'sample w-[93px] bg-[#abcdef]';
  288 |         const outside = document.getElementById('outside')!;
  289 |         const outsideColor = getComputedStyle(outside).backgroundColor;
  290 |         const otherColor = getComputedStyle(second).backgroundColor;
  291 |         const sheetCount = document.adoptedStyleSheets.length;
  292 |         const css = api.report('scope-a').css;
  293 |         api.dispose('scope-a');
  294 |         return { first: first.map((report: any) => report.revision), revision: initial.revision,
  295 |             repeatedRevision: repeated.revision, width, color, otherColor, outsideColor,
  296 |             otherRevision: other.revision, sheetCount, remainingSheets: document.adoptedStyleSheets.length,
  297 |             remainingColor: getComputedStyle(second).backgroundColor, violations, css };
  298 |     });
> 299 |     expect(result.first).toEqual([1, 1]);
      |                          ^ Error: expect(received).toEqual(expected) // deep equality
  300 |     expect(result.width).toBe('137px');
  301 |     expect(result.color).toBe('rgb(18, 52, 86)');
  302 |     expect(result.otherColor).toBe('rgb(171, 205, 239)');
  303 |     expect(result.outsideColor).toBe('rgba(0, 0, 0, 0)');
  304 |     expect(result.repeatedRevision).toBe(result.revision);
  305 |     expect(result.remainingSheets).toBe(result.sheetCount - 1);
  306 |     expect(result.remainingColor).toBe(result.otherColor);
  307 |     expect(result.violations).toEqual([]);
  308 |     expect(errors).toEqual([]);
  309 |     expect(failed).toEqual([]);
  310 |     expect(paths.some(path => path.includes('/extractor/'))).toBe(false);
  311 |     await testInfo.attach('SDK committed stylesheet', { body: JSON.stringify(result, null, 2), contentType: 'application/json' });
  312 | });
  313 | 
  314 | test('controls apply classes and fit desktop and mobile', async ({ page }, testInfo) => {
  315 |     await ready(page);
  316 |     await page.locator('#classes').fill('w-[137px] bg-[#123456] h-[72px] text-[#ffffff] rounded-[8px]');
  317 |     await page.locator('#apply').click();
  318 |     await expect(page.locator('#status')).toContainText('DOM styles installed. Revision 1.');
  319 |     await expect(page.locator('#scope-a-sample')).toHaveCSS('width', '137px');
  320 |     await expect(page.locator('#scope-a-sample')).toHaveCSS('background-color', 'rgb(18, 52, 86)');
  321 |     await expect(page.locator('#scope-a-sample')).toHaveCSS('color', 'rgb(255, 255, 255)');
  322 |     await page.screenshot({ path: testInfo.outputPath('desktop.png'), fullPage: true });
  323 |     await page.setViewportSize({ width: 390, height: 844 });
  324 |     await page.evaluate(async () => {
  325 |         await document.fonts.ready;
  326 |         await new Promise(resolve => requestAnimationFrame(() => requestAnimationFrame(resolve)));
  327 |     });
  328 |     expect(await page.evaluate(() => document.documentElement.scrollWidth <= innerWidth)).toBe(true);
  329 |     await page.screenshot({ path: testInfo.outputPath('mobile.png'), fullPage: true });
  330 | });
  331 | 
  332 | test('nested owners stop outer rules and disposal removes the owned stylesheet', async ({ page }) => {
  333 |     await ready(page);
  334 |     const result = await page.evaluate(async () => {
  335 |         const api = (window as any).__utility_lab;
  336 |         api.mount('scope-a-nested');
  337 |         const nested = document.getElementById('scope-a-nested-sample')!;
  338 |         const baseline = getComputedStyle(nested).backgroundColor;
  339 |         await api.ensure('scope-a', ['w-[137px]', 'bg-[#123456]']);
  340 |         nested.className = 'sample w-[137px] bg-[#123456]';
  341 |         const unstyled = getComputedStyle(nested).backgroundColor;
  342 |         await api.ensure('scope-a-nested', ['bg-[#abcdef]']);
  343 |         nested.className = 'sample bg-[#abcdef]';
  344 |         const styled = getComputedStyle(nested).backgroundColor;
  345 |         api.dispose('scope-a-nested');
  346 |         return { baseline, unstyled, styled, owner: document.getElementById('scope-a-nested')!.hasAttribute('data-rustify-style-owner') };
  347 |     });
  348 |     expect(result.unstyled).toBe(result.baseline);
  349 |     expect(result.unstyled).not.toBe('rgb(18, 52, 86)');
  350 |     expect(result.styled).toBe('rgb(171, 205, 239)');
  351 |     expect(result.owner).toBe(false);
  352 | });
  353 | 
  354 | test('classList changes and newly added subtrees compile through the bounded observer without scanner', async ({ page }) => {
  355 |     await ready(page);
  356 |     await page.evaluate(async () => {
  357 |         const api = (window as any).__utility_lab;
  358 |         await api.ensure('scope-a', []);
  359 |         document.getElementById('scope-a-sample')!.classList.add('w-[151px]');
  360 |     });
  361 |     await expect(page.locator('#scope-a-sample')).toHaveCSS('width', '151px');
  362 |     await page.evaluate(() => {
  363 |         const child = document.createElement('div');
  364 |         child.id = 'observer-child';
  365 |         child.className = 'bg-[#abcdef] h-[37px]';
  366 |         document.getElementById('scope-a-sample')!.append(child);
  367 |     });
  368 |     await expect(page.locator('#observer-child')).toHaveCSS('background-color', 'rgb(171, 205, 239)');
  369 |     await expect(page.locator('#observer-child')).toHaveCSS('height', '37px');
  370 |     const unchanged = await page.evaluate(async () => {
  371 |         const api = (window as any).__utility_lab;
  372 |         const before = api.report('scope-a').revision;
  373 |         const sample = document.getElementById('scope-a-sample')!;
  374 |         sample.classList.remove('w-[151px]'); sample.classList.add('w-[151px]');
  375 |         await new Promise(resolve => setTimeout(resolve, 50));
  376 |         return { before, after: api.report('scope-a').revision };
  377 |     });
  378 |     expect(unchanged.after).toBe(unchanged.before);
  379 | });
  380 | 
  381 | test('scoped relation variants and arbitrary selectors retain conditional matching', async ({ page }, testInfo) => {
  382 |     await ready(page);
  383 |     const result = await page.evaluate(async () => {
  384 |         const api = (window as any).__utility_lab;
  385 |         const classes = ['w-[89px]', 'h-[11px]', 'group-data-[state=open]:bg-[#123456]',
  386 |             'peer-data-[state=open]:h-[37px]', '[&>span]:w-[63px]', 'p-[7px]'];
  387 |         const report = await api.ensure('scope-a', classes);
  388 |         const root = document.getElementById('scope-a-sample')!;
  389 |         root.className = 'group p-[7px]';
  390 |         const flag = document.createElement('div');
  391 |         flag.className = 'peer';
  392 |         const child = document.createElement('div');
  393 |         child.className = 'w-[89px] h-[11px] group-data-[state=open]:bg-[#123456] peer-data-[state=open]:h-[37px] [&>span]:w-[63px]';
  394 |         const span = document.createElement('span');
  395 |         span.style.display = 'block';
  396 |         child.append(span);
  397 |         root.append(flag, child);
  398 |         const before = { color: getComputedStyle(child).backgroundColor, height: getComputedStyle(child).height };
  399 |         root.dataset.state = 'open';
```