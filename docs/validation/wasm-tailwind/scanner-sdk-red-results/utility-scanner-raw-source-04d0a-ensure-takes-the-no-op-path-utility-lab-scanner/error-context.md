# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: utility-scanner.spec.ts >> raw source scanning installs CSS and variables, then direct ensure takes the no-op path
- Location: tests/browser/utility-scanner.spec.ts:10:5

# Error details

```
Error: expect(received).toBe(expected) // Object.is equality

Expected: true
Received: undefined
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
  - generic [ref=e21]:
    - region [ref=e22]:
      - heading "DOM preview" [level=2] [ref=e23]
      - paragraph [ref=e24]: Updates after the generated stylesheet is installed.
      - generic [ref=e28]:
        - button "Live utility sample" [ref=e29]
        - generic [ref=e30]: Utility text
    - region [ref=e32]:
      - heading "GPU preview" [level=2] [ref=e33]
      - paragraph [ref=e34]: Supported properties update after compilation. Unsupported properties keep the previous appearance.
      - generic [ref=e36]:
        - status [ref=e38]: GPU ready
        - paragraph [ref=e39]: "Button: applied · applied 0 · drawn 0 · 0 clicks"
        - paragraph [ref=e40]: "Container: applied · Text: applied · Color block: applied"
        - group [ref=e41]:
          - generic "GPU values and diagnostics" [ref=e42] [cursor=pointer]
  - region [ref=e43]:
    - heading "Independent scope" [level=2] [ref=e44]
    - paragraph [ref=e45]: Uses its own compiler and stylesheet.
    - generic [ref=e48]:
      - button "Live utility sample" [ref=e49]
      - generic [ref=e50]: Utility text
  - group [ref=e52]:
    - generic "Generated CSS and diagnostics" [ref=e53] [cursor=pointer]
  - generic [ref=e54]: Outside every utility scope
```

# Test source

```ts
  1   | import { test, expect } from '@playwright/test';
  2   | 
  3   | async function ready(page: any) {
  4   |     await page.goto('./');
  5   |     await expect(page.locator('#status')).toHaveAttribute('data-status', 'ready');
  6   |     const manifest = await page.request.get('utility/build-manifest.json');
> 7   |     expect((await manifest.json()).scanner_enabled).toBe(true);
      |                                                     ^ Error: expect(received).toBe(expected) // Object.is equality
  8   | }
  9   | 
  10  | test('raw source scanning installs CSS and variables, then direct ensure takes the no-op path', async ({page}, info) => {
  11  |     const errors: string[] = [], failed: string[] = [];
  12  |     page.on('pageerror', error => errors.push(error.message));
  13  |     page.on('console', message => { if (message.type() === 'error') errors.push(message.text()); });
  14  |     page.on('requestfailed', request => failed.push(request.url()));
  15  |     await ready(page);
  16  |     const result = await page.evaluate(async () => {
  17  |         const api = (window as any).__utility_lab;
  18  |         const violations: string[] = [];
  19  |         document.addEventListener('securitypolicyviolation', event => violations.push(event.violatedDirective));
  20  |         api.dispose('scope-a');
  21  |         api.mount('scope-a', 10000, '@theme { --color-probe:#123456; } @tailwind utilities;');
  22  |         const first = await api.scanContent('scope-a', [{content:'<div class="w-[147px] bg-[#abcdef]">var(--color-probe)</div>', extension:'html'}]);
  23  |         const sample = document.getElementById('scope-a-sample')!;
  24  |         sample.className = 'sample w-[147px] bg-[#abcdef]';
  25  |         const style = getComputedStyle(sample);
  26  |         const width = style.width, color = style.backgroundColor;
  27  |         const variable = getComputedStyle(document.getElementById('scope-a')!).getPropertyValue('--color-probe').trim();
  28  |         const repeated = await api.ensure('scope-a', ['w-[147px]', 'bg-[#abcdef]', '--color-probe']);
  29  |         const scannedAgain = await api.scanContent('scope-a', [{content:'w-[147px] bg-[#abcdef] --color-probe', extension:'html'}]);
  30  |         return {first, repeated, scannedAgain, width, color, variable, violations,
  31  |             other: getComputedStyle(document.getElementById('scope-b-sample')!).width};
  32  |     });
  33  |     expect(result.width).toBe('147px'); expect(result.color).toBe('rgb(171, 205, 239)');
  34  |     expect(result.variable).toBe('#123456'); expect(result.other).toBe('170px');
  35  |     expect(result.first.admitted).toContain('w-[147px]');
  36  |     expect(result.repeated.revision).toBe(result.first.revision);
  37  |     expect(result.scannedAgain.revision).toBe(result.first.revision);
  38  |     expect(result.scannedAgain.metrics.parsed_candidates).toBe(0);
  39  |     expect(result.violations).toEqual([]); expect(errors).toEqual([]); expect(failed).toEqual([]);
  40  |     await info.attach('Source scanner results', {body: JSON.stringify(result, null, 2), contentType:'application/json'});
  41  | });
  42  | 
  43  | test('virtual sources follow CSS paths and exclusions, retaining compiled rules after file removal', async ({page}, info) => {
  44  |     await ready(page);
  45  |     const result = await page.evaluate(async () => {
  46  |         const api = (window as any).__utility_lab;
  47  |         api.dispose('scope-a');
  48  |         api.mount('scope-a', 10000, '@tailwind utilities source(none); @source "./src/**/*.html"; @source not "./src/excluded.html";');
  49  |         const source = {root:'/', base:'/', entries:[
  50  |             {kind:'file', path:'/src/probe.html', content:'w-[153px] bg-[#123456]', revision:1},
  51  |             {kind:'file', path:'/src/excluded.html', content:'w-[199px]', revision:1},
  52  |             {kind:'file', path:'/other/probe.html', content:'w-[201px]', revision:1},
  53  |         ]};
  54  |         const first = await api.scanSources('scope-a', source);
  55  |         const sample = document.getElementById('scope-a-sample')!;
  56  |         sample.className = 'sample w-[153px] bg-[#123456]';
  57  |         const width = getComputedStyle(sample).width;
  58  |         const removed = await api.scanSources('scope-a', {...source, entries:[]});
  59  |         return {first, removed, width, after: getComputedStyle(sample).width};
  60  |     });
  61  |     expect(result.first.admitted).toContain('w-[153px]');
  62  |     expect(result.first.admitted).not.toContain('w-[199px]');
  63  |     expect(result.first.admitted).not.toContain('w-[201px]');
  64  |     expect(result.width).toBe('153px'); expect(result.after).toBe('153px');
  65  |     expect(result.removed.revision).toBe(result.first.revision);
  66  |     await info.attach('Virtual source results', {body:JSON.stringify(result,null,2),contentType:'application/json'});
  67  | });
  68  | 
  69  | test('scan install failures and input quotas preserve the snapshot; lost ack replays scanned variables', async ({page}) => {
  70  |     await ready(page);
  71  |     const result = await page.evaluate(async () => {
  72  |         const api = (window as any).__utility_lab;
  73  |         api.dispose('scope-a');
  74  |         api.mount('scope-a', 1000, '@theme { --color-probe:#123456; } @tailwind utilities;');
  75  |         const first = await api.scanContent('scope-a', [{content:'w-[147px] --color-probe',extension:'html'}]);
  76  |         const replace = CSSStyleSheet.prototype.replaceSync;
  77  |         CSSStyleSheet.prototype.replaceSync = function() {throw new DOMException('scan install failure','SyntaxError');};
  78  |         let install, quota, ack;
  79  |         try {await api.scanContent('scope-a', [{content:'w-[179px]',extension:'html'}]);}
  80  |         catch (error) {install = JSON.parse(String(error));}
  81  |         finally {CSSStyleSheet.prototype.replaceSync = replace;}
  82  |         const retained = api.report('scope-a');
  83  |         try {await api.scanContent('scope-a', [{content:'a'.repeat(300000),extension:'html'}]);}
  84  |         catch (error) {quota = JSON.parse(String(error));}
  85  |         const afterQuota = api.report('scope-a');
  86  |         const post = Worker.prototype.postMessage;
  87  |         Worker.prototype.postMessage = function(message:any,...args:any[]) {
  88  |             if (message.operation === 'ack') return;
  89  |             return post.call(this,message,...args as [any]);
  90  |         };
  91  |         try {await api.scanContent('scope-a', [{content:'w-[181px]',extension:'html'}]);}
  92  |         catch (error) {ack = JSON.parse(String(error));}
  93  |         finally {Worker.prototype.postMessage = post;}
  94  |         const installed = api.report('scope-a');
  95  |         const replay = await api.retry('scope-a');
  96  |         const sample = document.getElementById('scope-a-sample')!;
  97  |         sample.className = 'sample w-[181px]';
  98  |         return {first,install,quota,ack,retained,afterQuota,installed,replay,
  99  |             width:getComputedStyle(sample).width,
  100 |             variable:getComputedStyle(document.getElementById('scope-a')!).getPropertyValue('--color-probe').trim()};
  101 |     });
  102 |     expect(result.install.kind).toBe('install');
  103 |     expect(result.retained.revision).toBe(result.first.revision);
  104 |     expect(result.retained.admitted).not.toContain('w-[179px]');
  105 |     expect(result.quota.kind).toBe('compile'); expect(result.quota.error.kind).toBe('limit');
  106 |     expect(result.afterQuota.revision).toBe(result.first.revision);
  107 |     expect(result.ack.kind).toBe('timeout');
```