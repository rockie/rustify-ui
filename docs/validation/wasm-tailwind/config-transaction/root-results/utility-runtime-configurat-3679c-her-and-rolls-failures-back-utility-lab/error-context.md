# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: utility-runtime.spec.ts >> configuration replacement commits raw component classes and CSS together and rolls failures back
- Location: tests/browser/utility-runtime.spec.ts:468:5

# Error details

```
Error: expect(received).toBe(expected) // Object.is equality

Expected: "0px"
Received: "0px 16px"
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
  - generic [ref=e58]:
    - generic [ref=e59]: Scoped raw classes
    - button "Scoped button" [ref=e60] [cursor=pointer]
```

# Test source

```ts
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
  483 |         const css = '@theme { --spacing: 0.5rem; } @tailwind utilities;';
  484 |         await api.replace(id, css, true);
  485 |         const installed = read();
  486 |         const original = CSSStyleSheet.prototype.replaceSync;
  487 |         CSSStyleSheet.prototype.replaceSync = function () { throw new Error('injected configuration install failure'); };
  488 |         let installation;
  489 |         try { await api.replace(id, '@theme { --spacing: 1rem; } @tailwind utilities;', false); }
  490 |         catch (error) { installation = JSON.parse(String(error)); }
  491 |         finally { CSSStyleSheet.prototype.replaceSync = original; }
  492 |         const afterInstallationFailure = read();
  493 |         let compiler, callback;
  494 |         try { await api.replace(id, '@theme { .nested { color: red; } } @tailwind utilities;', false); }
  495 |         catch (error) { compiler = JSON.parse(String(error)); }
  496 |         try { await api.replace(id, css, false, true); }
  497 |         catch (error) { callback = JSON.parse(String(error)); }
  498 |         const afterFailures = read();
  499 |         await api.replace(id, '@theme { --spacing: 0.25rem; } @tailwind utilities;', false);
  500 |         const restored = read();
  501 |         return {before, installed, installation, afterInstallationFailure, compiler, callback, afterFailures, restored,
  502 |             otherPadding: getComputedStyle(document.getElementById('scope-b-sample')!).padding};
  503 |     });
  504 |     expect(result.before.classes).toBe('bg-[#123456] p-4');
  505 |     expect(result.before.padding).toBe('16px');
  506 |     expect(result.installed.classes).toBe('p-2 bg-[#123456] p-4');
  507 |     expect(result.installed.padding).toBe('32px');
  508 |     expect(result.installed.snapshot.epoch).toBe(result.installed.snapshot.cn_epoch);
  509 |     expect(result.installed.snapshot.report.revision).toBeGreaterThan(result.before.snapshot.report.revision);
  510 |     expect(result.installed.snapshot.report.admitted).toContain('p-2');
  511 |     expect(result.installation.kind).toBe('install');
  512 |     expect(result.afterInstallationFailure).toEqual(result.installed);
  513 |     expect(result.compiler.kind).toBe('compile');
  514 |     expect(result.callback.message).toContain('fixture callback failed');
  515 |     expect(result.afterFailures).toEqual(result.installed);
  516 |     expect(result.restored.classes).toBe('bg-[#123456] p-4');
  517 |     expect(result.restored.padding).toBe('16px');
  518 |     expect(result.restored.snapshot.epoch).toBe(result.restored.snapshot.cn_epoch);
> 519 |     expect(result.otherPadding).toBe('0px');
      |                                 ^ Error: expect(received).toBe(expected) // Object.is equality
  520 |     await info.attach('configuration transaction', {body: JSON.stringify(result, null, 2), contentType: 'application/json'});
  521 |     await page.evaluate(() => (window as any).__utility_lab.componentBaseline.dispose('cn-transaction'));
  522 |     expect(errors).toEqual([]); expect(failed).toEqual([]);
  523 | });
  524 | 
  525 | test('configuration ack loss replays the installed cn and compiler configuration', async ({page}, info) => {
  526 |     await ready(page);
  527 |     const result = await page.evaluate(async () => {
  528 |         const api = (window as any).__utility_lab.componentBaseline;
  529 |         const id = 'cn-ack';
  530 |         const root = document.createElement('div'); root.id = id; document.querySelector('main')!.append(root);
  531 |         api.mount(id);
  532 |         await api.ensure(id, ['p-4', 'bg-[#123456]']);
  533 |         const owner = root.dataset.rustifyStyleOwner;
  534 |         const original = Worker.prototype.postMessage;
  535 |         Worker.prototype.postMessage = function (message: any, ...rest: any[]) {
  536 |             if (message.runtime_id === owner && message.operation === 'ack') return;
  537 |             return original.call(this, message, ...rest);
  538 |         };
  539 |         const css = '@theme { --spacing: 0.5rem; } @tailwind utilities;';
  540 |         let failure;
  541 |         try { await api.replace(id, css, true); }
  542 |         catch (error) { failure = JSON.parse(String(error)); }
  543 |         finally { Worker.prototype.postMessage = original; }
  544 |         const installed = api.report(id);
  545 |         const replayed = await api.retry(id);
  546 |         const card = document.querySelector(`[data-testid="${id}-card"]`)!;
  547 |         return {failure, installed, replayed, classes: card.className, padding: getComputedStyle(card).padding};
  548 |     });
  549 |     expect(result.failure.kind).toBe('timeout');
  550 |     expect(result.installed.config.input.css).toContain('0.5rem');
  551 |     expect(result.replayed.config).toEqual(result.installed.config);
  552 |     expect(result.replayed.epoch).toBeGreaterThan(result.installed.epoch);
  553 |     expect(result.replayed.cn_epoch).toBe(result.replayed.epoch);
  554 |     expect(result.classes).toBe('p-2 bg-[#123456] p-4');
  555 |     expect(result.padding).toBe('32px');
  556 |     await info.attach('configuration ack recovery', {body: JSON.stringify(result, null, 2), contentType: 'application/json'});
  557 |     await page.evaluate(() => (window as any).__utility_lab.componentBaseline.dispose('cn-ack'));
  558 | });
  559 | 
  560 | test('rapid configuration edits supersede old work and retain same epoch additions', async ({page}) => {
  561 |     await ready(page);
  562 |     const result = await page.evaluate(async () => {
  563 |         const api = (window as any).__utility_lab.componentBaseline;
  564 |         const id = 'cn-rapid';
  565 |         const root = document.createElement('div'); root.id = id; document.querySelector('main')!.append(root);
  566 |         api.mount(id);
  567 |         await api.ensure(id, ['p-4', 'bg-[#123456]']);
  568 |         const first = api.replace(id, '@theme { --spacing: 1rem; } @tailwind utilities;', false)
  569 |             .then(() => 'unexpected success', (error: any) => JSON.parse(String(error)).kind);
  570 |         const second = api.replace(id, '@theme { --spacing: 0.5rem; } @tailwind utilities;', true);
  571 |         const sameEpoch = api.ensure(id, ['w-[183px]']);
  572 |         const stale = await first;
  573 |         const latest = await second;
  574 |         await sameEpoch;
  575 |         const card = document.querySelector(`[data-testid="${id}-card"]`)!;
  576 |         const probe = document.createElement('div'); probe.className = 'w-[183px]'; root.append(probe);
  577 |         return {stale, latest, final: api.report(id), width: getComputedStyle(probe).width, padding: getComputedStyle(card).padding};
  578 |     });
  579 |     expect(result.stale).toBe('superseded');
  580 |     expect(result.latest.epoch).toBe(result.latest.cn_epoch);
  581 |     expect(result.final.report.admitted).toContain('w-[183px]');
  582 |     expect(result.width).toBe('183px');
  583 |     expect(result.padding).toBe('32px');
  584 |     await page.evaluate(() => (window as any).__utility_lab.componentBaseline.dispose('cn-rapid'));
  585 | });
  586 | 
```