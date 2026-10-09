# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: utility-runtime.spec.ts >> rapid configuration edits supersede old work and retain same epoch additions
- Location: tests/browser/utility-runtime.spec.ts:561:5

# Error details

```
Error: expect(received).toEqual(expected) // deep equality

- Expected  - 3
+ Received  + 3

@@ -42,11 +42,11 @@
        "disabled:pointer-events-none",
        "disabled:opacity-50",
        "aria-disabled:cursor-not-allowed",
        "aria-disabled:opacity-50",
      ],
-     "changed": false,
+     "changed": true,
      "css": "@scope ([data-rustify-style-owner=\"53cc9ebd-3f43-4ac5-870d-73d9b2edb3db\"]) to ([data-rustify-style-owner]) {
    @layer properties;
    :scope {
      --spacing: 0.5rem;
    }
@@ -643,13 +643,13 @@
        "--tw-shadow-alpha": "100%",
      },
      "metrics": Object {
        "cache_bytes": 10223,
        "cache_entries": 31,
-       "generated_candidates": 0,
+       "generated_candidates": 1,
        "output_bytes": 5671,
-       "parsed_candidates": 0,
+       "parsed_candidates": 1,
      },
      "rejected": Array [
        Object {
          "candidate": "aria-invalid:border-destructive",
          "family": "border",
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
  486 |         const installed = read();
  487 |         const original = CSSStyleSheet.prototype.replaceSync;
  488 |         CSSStyleSheet.prototype.replaceSync = function () { throw new Error('injected configuration install failure'); };
  489 |         let installation;
  490 |         try { await api.replace(id, '@theme { --spacing: 1rem; } @tailwind utilities;', false); }
  491 |         catch (error) { installation = JSON.parse(String(error)); }
  492 |         finally { CSSStyleSheet.prototype.replaceSync = original; }
  493 |         const afterInstallationFailure = read();
  494 |         let compiler, callback;
  495 |         try { await api.replace(id, '@theme { .nested { color: red; } } @tailwind utilities;', false); }
  496 |         catch (error) { compiler = JSON.parse(String(error)); }
  497 |         try { await api.replace(id, css, false, true); }
  498 |         catch (error) { callback = JSON.parse(String(error)); }
  499 |         const afterFailures = read();
  500 |         await api.replace(id, '@theme { --spacing: 0.25rem; } @tailwind utilities;', false);
  501 |         const restored = read();
  502 |         return {before, installed, installation, afterInstallationFailure, compiler, callback, afterFailures, restored, beforeOtherPadding,
  503 |             otherPadding: getComputedStyle(document.getElementById('scope-b-sample')!).padding};
  504 |     });
  505 |     expect(result.before.classes).toBe('bg-[#123456] p-4');
  506 |     expect(result.before.padding).toBe('16px');
  507 |     expect(result.installed.classes).toBe('p-2 bg-[#123456] p-4');
  508 |     expect(result.installed.padding).toBe('32px');
  509 |     expect(result.installed.snapshot.epoch).toBe(result.installed.snapshot.cn_epoch);
  510 |     expect(result.installed.snapshot.report.revision).toBeGreaterThan(result.before.snapshot.report.revision);
  511 |     expect(result.installed.snapshot.report.admitted).toContain('p-2');
  512 |     expect(result.installation.kind).toBe('install');
  513 |     expect(result.afterInstallationFailure).toEqual(result.installed);
  514 |     expect(result.compiler.kind).toBe('compile');
  515 |     expect(result.callback.message).toContain('fixture callback failed');
  516 |     expect(result.afterFailures).toEqual(result.installed);
  517 |     expect(result.restored.classes).toBe('bg-[#123456] p-4');
  518 |     expect(result.restored.padding).toBe('16px');
  519 |     expect(result.restored.snapshot.epoch).toBe(result.restored.snapshot.cn_epoch);
  520 |     expect(result.otherPadding).toBe(result.beforeOtherPadding);
  521 |     await info.attach('configuration transaction', {body: JSON.stringify(result, null, 2), contentType: 'application/json'});
  522 |     await page.evaluate(() => (window as any).__utility_lab.componentBaseline.dispose('cn-transaction'));
  523 |     expect(errors).toEqual([]); expect(failed).toEqual([]);
  524 | });
  525 | 
  526 | test('configuration ack loss replays the installed cn and compiler configuration', async ({page}, info) => {
  527 |     await ready(page);
  528 |     const result = await page.evaluate(async () => {
  529 |         const api = (window as any).__utility_lab.componentBaseline;
  530 |         const id = 'cn-ack';
  531 |         const root = document.createElement('div'); root.id = id; document.querySelector('main')!.append(root);
  532 |         api.mount(id);
  533 |         await api.ensure(id, ['p-4', 'bg-[#123456]']);
  534 |         const owner = root.dataset.rustifyStyleOwner;
  535 |         const original = Worker.prototype.postMessage;
  536 |         Worker.prototype.postMessage = function (message: any, ...rest: any[]) {
  537 |             if (message.runtime_id === owner && message.operation === 'ack') return;
  538 |             return original.call(this, message, ...rest);
  539 |         };
  540 |         const css = '@theme { --spacing: 0.5rem; } @tailwind utilities;';
  541 |         let failure;
  542 |         try { await api.replace(id, css, true); }
  543 |         catch (error) { failure = JSON.parse(String(error)); }
  544 |         finally { Worker.prototype.postMessage = original; }
  545 |         const installed = api.report(id);
  546 |         const replayed = await api.retry(id);
  547 |         const card = document.querySelector(`[data-testid="${id}-card"]`)!;
  548 |         return {failure, installed, replayed, classes: card.className, padding: getComputedStyle(card).padding};
  549 |     });
  550 |     expect(result.failure.kind).toBe('timeout');
  551 |     expect(result.installed.config.input.css).toContain('0.5rem');
  552 |     expect(result.replayed.config).toEqual(result.installed.config);
  553 |     expect(result.replayed.epoch).toBeGreaterThan(result.installed.epoch);
  554 |     expect(result.replayed.cn_epoch).toBe(result.replayed.epoch);
  555 |     expect(result.classes).toBe('p-2 bg-[#123456] p-4');
  556 |     expect(result.padding).toBe('32px');
  557 |     await info.attach('configuration ack recovery', {body: JSON.stringify(result, null, 2), contentType: 'application/json'});
  558 |     await page.evaluate(() => (window as any).__utility_lab.componentBaseline.dispose('cn-ack'));
  559 | });
  560 | 
  561 | test('rapid configuration edits supersede old work and retain same epoch additions', async ({page}) => {
  562 |     await ready(page);
  563 |     const result = await page.evaluate(async () => {
  564 |         const api = (window as any).__utility_lab.componentBaseline;
  565 |         const id = 'cn-rapid';
  566 |         const root = document.createElement('div'); root.id = id; document.querySelector('main')!.append(root);
  567 |         api.mount(id);
  568 |         await api.ensure(id, ['p-4', 'bg-[#123456]']);
  569 |         const first = api.replace(id, '@theme { --spacing: 1rem; } @tailwind utilities;', false)
  570 |             .then(() => 'unexpected success', (error: any) => JSON.parse(String(error)).kind);
  571 |         const second = api.replace(id, '@theme { --spacing: 0.5rem; } @tailwind utilities;', true);
  572 |         const sameEpoch = api.ensure(id, ['w-[183px]']);
  573 |         const flushed = api.flush(id);
  574 |         const stale = await first;
  575 |         const latest = await second;
  576 |         await sameEpoch;
  577 |         const barrier = await flushed;
  578 |         const unchanged = await api.flush(id);
  579 |         const card = document.querySelector(`[data-testid="${id}-card"]`)!;
  580 |         const probe = document.createElement('div'); probe.className = 'w-[183px]'; root.append(probe);
  581 |         return {stale, latest, barrier, unchanged, final: api.report(id), width: getComputedStyle(probe).width, padding: getComputedStyle(card).padding};
  582 |     });
  583 |     expect(result.stale).toBe('superseded');
  584 |     expect(result.latest.epoch).toBe(result.latest.cn_epoch);
  585 |     expect(result.final.report.admitted).toContain('w-[183px]');
> 586 |     expect(result.barrier).toEqual(result.final);
      |                            ^ Error: expect(received).toEqual(expected) // deep equality
  587 |     expect(result.unchanged).toEqual(result.barrier);
  588 |     expect(result.width).toBe('183px');
  589 |     expect(result.padding).toBe('32px');
  590 |     await page.evaluate(() => (window as any).__utility_lab.componentBaseline.dispose('cn-rapid'));
  591 | });
  592 | 
```