# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: utility-runtime.spec.ts >> external variable inputs install live theme dependencies without utility selectors
- Location: tests/browser/utility-runtime.spec.ts:363:5

# Error details

```
Error: expect(received).toEqual(expected) // deep equality

- Expected  - 1
+ Received  + 3

- Array []
+ Array [
+   "http://127.0.0.1:4186/makepad_widgets/resources/NotoColorEmoji.ttf",
+ ]
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
      - generic [ref=e30]:
        - button "Live utility sample" [ref=e31]
        - generic [ref=e32]: Utility text
    - region [ref=e34]:
      - heading "GPU preview" [level=2] [ref=e35]
      - paragraph [ref=e36]: Supported properties update after compilation. Unsupported properties keep the previous appearance.
      - generic [ref=e38]:
        - status [ref=e40]: GPU ready
        - paragraph [ref=e41]: "Button: applied · applied 0 · drawn 0 · 0 clicks"
        - paragraph [ref=e42]: "Container: applied · Text: applied · Color block: applied"
        - group [ref=e43]:
          - generic "GPU values and diagnostics" [ref=e44] [cursor=pointer]
  - region [ref=e45]:
    - heading "Independent scope" [level=2] [ref=e46]
    - paragraph [ref=e47]: Uses its own compiler and stylesheet.
    - generic [ref=e50]:
      - button "Live utility sample" [ref=e51]
      - generic [ref=e52]: Utility text
  - group [ref=e54]:
    - generic "Generated CSS and diagnostics" [ref=e55] [cursor=pointer]
  - generic [ref=e56]: Outside every utility scope
```

# Test source

```ts
  289 |         return { before, after, isolated, padding: getComputedStyle(root).padding,
  290 |             spanWidth: getComputedStyle(span).width, restoredHeight: getComputedStyle(child).height,
  291 |             unsupported: report.unsupported, css: report.css };
  292 |     });
  293 |     expect(result.before).toEqual({ color: 'rgba(0, 0, 0, 0)', height: '11px' });
  294 |     expect(result.after).toEqual({ color: 'rgb(18, 52, 86)', height: '37px' });
  295 |     expect(result.isolated).toEqual(['rgba(0, 0, 0, 0)', 'rgba(0, 0, 0, 0)']);
  296 |     expect(result.padding).toBe('7px');
  297 |     expect(result.spanWidth).toBe('63px');
  298 |     expect(result.restoredHeight).toBe('11px');
  299 |     expect(result.unsupported).toEqual([]);
  300 |     await testInfo.attach('scoped relation variant results', { body: JSON.stringify(result, null, 2), contentType: 'application/json' });
  301 | });
  302 | 
  303 | test('arbitrary responsive conditions follow the browser viewport at both sizes', async ({ page }, testInfo) => {
  304 |     await ready(page);
  305 |     await page.setViewportSize({ width: 1000, height: 844 });
  306 |     const report = await page.evaluate(async () => {
  307 |         const api = (window as any).__utility_lab;
  308 |         const classes = ['w-[89px]', 'h-[11px]', 'min-[600px]:w-[149px]', 'max-[600px]:h-[47px]'];
  309 |         const report = await api.ensure('scope-a', classes);
  310 |         document.getElementById('scope-a-sample')!.className = classes.join(' ');
  311 |         return report;
  312 |     });
  313 |     await expect(page.locator('#scope-a-sample')).toHaveCSS('width', '149px');
  314 |     await expect(page.locator('#scope-a-sample')).toHaveCSS('height', '11px');
  315 |     await page.setViewportSize({ width: 390, height: 844 });
  316 |     await expect(page.locator('#scope-a-sample')).toHaveCSS('width', '89px');
  317 |     await expect(page.locator('#scope-a-sample')).toHaveCSS('height', '47px');
  318 |     expect(report.unsupported).toEqual([]);
  319 |     await testInfo.attach('responsive prepared report', { body: JSON.stringify(report, null, 2), contentType: 'application/json' });
  320 | });
  321 | 
  322 | test('color alpha and scoped placeholder selectors retain their browser semantics', async ({ page }, testInfo) => {
  323 |     await ready(page);
  324 |     const result = await page.evaluate(async () => {
  325 |         const api = (window as any).__utility_lab;
  326 |         const classes = ['bg-[#123456]/50', 'text-[#abcdef]/[.35]', 'border-[#fedcba]/25', 'placeholder-[#123456]/50'];
  327 |         const report = await api.ensure('scope-a', classes);
  328 |         const sample = document.getElementById('scope-a-sample')!;
  329 |         sample.className = classes.slice(0, 3).join(' ');
  330 |         const input = document.createElement('input');
  331 |         input.placeholder = 'Color sample';
  332 |         input.className = classes[3];
  333 |         sample.append(input);
  334 |         const outside = document.createElement('input');
  335 |         outside.placeholder = 'Outside scope';
  336 |         outside.className = classes[3];
  337 |         document.getElementById('outside')!.append(outside);
  338 |         const canvas = document.createElement('canvas');
  339 |         canvas.width = canvas.height = 1;
  340 |         const context = canvas.getContext('2d')!;
  341 |         function pixel(color: string) {
  342 |             context.clearRect(0, 0, 1, 1);
  343 |             context.fillStyle = color;
  344 |             context.fillRect(0, 0, 1, 1);
  345 |             return Array.from(context.getImageData(0, 0, 1, 1).data);
  346 |         }
  347 |         const style = getComputedStyle(sample);
  348 |         return { background: pixel(style.backgroundColor), text: pixel(style.color), border: pixel(style.borderTopColor),
  349 |             placeholder: pixel(getComputedStyle(input, '::placeholder').color),
  350 |             outside: pixel(getComputedStyle(outside, '::placeholder').color),
  351 |             unsupported: report.unsupported, css: report.css };
  352 |     });
  353 |     for (const [actual, expected] of [[result.background, [18, 52, 86, 128]],
  354 |         [result.text, [171, 205, 239, 89]], [result.border, [254, 220, 186, 64]],
  355 |         [result.placeholder, [18, 52, 86, 128]]] as const) {
  356 |         actual.forEach((value, index) => expect(Math.abs(value - expected[index])).toBeLessThanOrEqual(2));
  357 |     }
  358 |     expect(result.outside).not.toEqual(result.placeholder);
  359 |     expect(result.unsupported).toEqual([]);
  360 |     await testInfo.attach('color alpha and placeholder results', { body: JSON.stringify(result, null, 2), contentType: 'application/json' });
  361 | });
  362 | 
  363 | test('external variable inputs install live theme dependencies without utility selectors', async ({page}, info) => {
  364 |     const errors: string[] = [], failed: string[] = [];
  365 |     page.on('pageerror', error => errors.push(error.message));
  366 |     page.on('console', message => { if (message.type() === 'error') errors.push(message.text()); });
  367 |     page.on('requestfailed', request => failed.push(request.url()));
  368 |     await ready(page);
  369 |     const result = await page.evaluate(async () => {
  370 |         const api = (window as any).__utility_lab;
  371 |         api.dispose('scope-a');
  372 |         api.mount('scope-a', 10000, '@theme { --color-brand: #123456; --color-alias: var(--color-brand); } @tailwind utilities;');
  373 |         const value = (scope: string) => getComputedStyle(document.getElementById(`${scope}-sample`)!).getPropertyValue('--color-alias').trim();
  374 |         await api.ensure('scope-a', []);
  375 |         const before = value('scope-a');
  376 |         const first = await api.ensure('scope-a', ['--color-alias']);
  377 |         const installed = value('scope-a');
  378 |         const repeated = await api.ensure('scope-a', ['--color-alias']);
  379 |         return {before,installed,other:value('scope-b'),first,repeated};
  380 |     });
  381 |     expect(result.before).toBe('');
  382 |     expect(result.installed).toBe('#123456');
  383 |     expect(result.other).toBe('');
  384 |     expect(result.first.admitted).toEqual([]);
  385 |     expect(result.first.rejected.filter((item: any) => item.candidate.startsWith('--'))).toEqual([]);
  386 |     expect(result.first.unsupported).toEqual([]);
  387 |     expect(result.first.css).toContain('--color-alias: var(--color-brand)');
  388 |     expect(result.repeated.revision).toBe(result.first.revision);
> 389 |     expect(errors).toEqual([]); expect(failed).toEqual([]);
      |                                                ^ Error: expect(received).toEqual(expected) // deep equality
  390 |     await info.attach('External variable browser values', {body:JSON.stringify(result,null,2),contentType:'application/json'});
  391 | });
  392 | 
  393 | test('compiler-only Worker diagnoses disabled scanning and keeps its session usable', async ({page}, info) => {
  394 |     await ready(page);
  395 |     await expect(page.locator('#scan')).toBeDisabled();
  396 |     const sdk = await page.evaluate(async () => {
  397 |         const api = (window as any).__utility_lab;
  398 |         const before = await api.ensure('scope-a', ['w-[137px]']);
  399 |         const failures = [];
  400 |         for (const scan of [
  401 |             () => api.scanContent('scope-a', [{content:'w-[159px]',extension:'html'}]),
  402 |             () => api.scanSources('scope-a', {root:'/',base:'/',entries:[]}),
  403 |         ]) {
  404 |             try {await scan();} catch (error) {failures.push(JSON.parse(String(error)));}
  405 |         }
  406 |         const retained = api.report('scope-a');
  407 |         const next = await api.ensure('scope-a',['w-[179px]']);
  408 |         return {before,failures,retained,next};
  409 |     });
  410 |     expect(sdk.failures).toEqual(Array(2).fill({kind:'compile',error:{kind:'unsupported_feature',feature:'source scanning'}}));
  411 |     expect(sdk.retained.revision).toBe(sdk.before.revision);
  412 |     expect(sdk.next.admitted).toContain('w-[179px]');
  413 |     const result = await page.evaluate(async () => {
  414 |         const manifestUrl = new URL('./utility/build-manifest.json', location.href);
  415 |         const manifest = await (await fetch(manifestUrl)).json();
  416 |         const worker = new Worker(new URL('./utility-worker.js', manifestUrl), {type:'module'});
  417 |         let request = 0;
  418 |         const send = (operation: string, payload: unknown) => new Promise<any>((resolve,reject) => {
  419 |             const requestId = ++request;
  420 |             const timer = setTimeout(() => reject(new Error('utility probe timeout')),10000);
  421 |             worker.onmessage = ({data}) => { clearTimeout(timer); resolve(data); };
  422 |             worker.onerror = event => { clearTimeout(timer); reject(new Error(event.message)); };
  423 |             worker.postMessage({protocol_version:1,runtime_id:'compiler-only-probe',epoch:1,request_id:requestId,operation,payload});
  424 |         });
  425 |         const limits = {input_bytes:65536,asset_bytes:65536,candidate_count:1024,ast_depth:32,css_bytes:262144,queued_bytes:65536,cache_bytes:1048576};
  426 |         try {
  427 |             const ready = await send('init',{build_id:manifest.build_id,config:JSON.stringify({input:{css:'@tailwind utilities;',entry:'/app/input.css'},assets:[],limits,owner:null})});
  428 |             const content = await send('scan_content',JSON.stringify([{content:'w-[137px]',extension:'html'}]));
  429 |             const sources = await send('scan_sources',JSON.stringify({root:'/',base:'/app',entries:[]}));
  430 |             const prepared = await send('add_candidates',JSON.stringify(['w-[137px]']));
  431 |             const committed = await send('ack',{token:prepared.payload.token});
  432 |             return {ready,content,sources,prepared,committed};
  433 |         } finally {worker.terminate();}
  434 |     });
  435 |     expect(result.ready.operation).toBe('ready');
  436 |     expect(result.ready.payload.scanner_enabled).toBe(false);
  437 |     for (const response of [result.content,result.sources]) {
  438 |         expect(response.operation).toBe('error');
  439 |         expect(response.payload).toEqual({kind:'compile',error:{kind:'unsupported_feature',feature:'source scanning'}});
  440 |     }
  441 |     expect(result.prepared.operation).toBe('prepared');
  442 |     expect(result.committed.operation).toBe('committed');
  443 |     expect(result.committed.payload.css).toContain('width: 137px');
  444 |     await info.attach('Compiler-only Worker boundary', {body:JSON.stringify(result,null,2),contentType:'application/json'});
  445 | });
  446 | 
  447 | test('non-scalar compiler output keeps its UTF-16 units across Worker and SDK installation', async ({page}, info) => {
  448 |     await ready(page);
  449 |     const result = await page.evaluate(async () => {
  450 |         const api = (window as any).__utility_lab;
  451 |         api.dispose('scope-a');
  452 |         api.mount('scope-a',10000,'@theme static { --custom-\\d800: 101px; } @tailwind utilities;');
  453 |         const report = await api.ensure('scope-a',[]);
  454 |         const units = Array.from({length:report.css.length},(_,index) => report.css.charCodeAt(index));
  455 |         const repeated = await api.ensure('scope-a',[]);
  456 |         const replayed = await api.retry('scope-a');
  457 |         return {units, report, repeated, replayed, sheets:document.adoptedStyleSheets.length};
  458 |     });
  459 |     expect(result.units).toContain(0xd800);
  460 |     expect(result.units).not.toContain(0xfffd);
  461 |     expect(result.report.css).toContain(':scope');
  462 |     expect(result.repeated.revision).toBe(result.report.revision);
  463 |     expect(result.replayed.css).toBe(result.report.css);
  464 |     expect(result.sheets).toBeLessThanOrEqual(2);
  465 |     await info.attach('UTF-16 installed report',{body:JSON.stringify(result,null,2),contentType:'application/json'});
  466 | });
  467 | 
  468 | test('configuration replacement commits raw component classes and CSS together and rolls failures back', async ({page}, info) => {
  469 |     const errors: string[] = [], failed: string[] = [];
  470 |     page.on('pageerror', error => errors.push(error.message));
  471 |     page.on('console', message => { if (message.type() === 'error') errors.push(message.text()); });
  472 |     page.on('requestfailed', request => failed.push(`${request.url()}: ${request.failure()?.errorText}`));
  473 |     await ready(page);
  474 |     const result = await page.evaluate(async () => {
  475 |         const api = (window as any).__utility_lab.componentBaseline;
  476 |         const id = 'cn-transaction';
  477 |         const root = document.createElement('div'); root.id = id; document.querySelector('main')!.append(root);
  478 |         api.mount(id);
  479 |         await api.ensure(id, ['p-4', 'bg-[#123456]']);
  480 |         const card = document.querySelector(`[data-testid="${id}-card"]`)!;
  481 |         const read = () => ({classes: card.className, padding: getComputedStyle(card).padding, snapshot: api.report(id)});
  482 |         const before = read();
  483 |         const beforeOtherPadding = getComputedStyle(document.getElementById('scope-b-sample')!).padding;
  484 |         const css = '@theme { --spacing: 0.5rem; } @tailwind utilities;';
  485 |         await api.replace(id, css, true);
  486 |         const installed = read();
  487 |         const original = CSSStyleSheet.prototype.replaceSync;
  488 |         CSSStyleSheet.prototype.replaceSync = function () { throw new Error('injected configuration install failure'); };
  489 |         let installation;
```