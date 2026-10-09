# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: utility-runtime.spec.ts >> manifest seed preserves static utility order on first ensure, replacement and retry
- Location: tests/browser/utility-runtime.spec.ts:10:5

# Error details

```
Error: expect(received).toBe(expected) // Object.is equality

Expected: "0px"
Received: "16px"
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
  1   | import { test, expect } from '@playwright/test';
  2   | import { createRequire } from 'node:module';
  3   | import { resolve } from 'node:path';
  4   | 
  5   | async function ready(page: any) {
  6   |     await page.goto('./');
  7   |     await expect(page.locator('#status')).toHaveAttribute('data-status', 'ready');
  8   | }
  9   | 
  10  | test('manifest seed preserves static utility order on first ensure, replacement and retry', async ({page}, info) => {
  11  |     const errors: string[] = [], failed: string[] = [], csp: string[] = [];
  12  |     page.on('pageerror', error => errors.push(error.message));
  13  |     page.on('console', message => { if (message.type() === 'error') errors.push(message.text()); });
  14  |     page.on('requestfailed', request => failed.push(`${request.url()}: ${request.failure()?.errorText}`));
  15  |     await page.addInitScript(() => {
  16  |         (window as any).__seedCsp = [];
  17  |         document.addEventListener('securitypolicyviolation', event => (window as any).__seedCsp.push(event.violatedDirective));
  18  |     });
  19  |     const requireOracle = createRequire(resolve('tests/utility/compiler/oracle/package.json'));
  20  |     expect(requireOracle('tailwindcss/package.json').version).toBe('4.1.13');
  21  |     const oracle = requireOracle('tailwindcss');
  22  |     const official: string[] = [];
  23  |     for (const spacing of [4,8]) {
  24  |         const compiler = await oracle.compile(`@theme inline { --spacing: ${spacing}px; } @tailwind utilities;`);
  25  |         official.push(compiler.build(['p-2','p-4']));
  26  |     }
  27  |     await ready(page);
  28  |     const result = await page.evaluate(async official => {
  29  |         const api = (window as any).__utility_lab;
  30  |         const manifest = await fetch('./utility/build-manifest.json').then(response => response.json());
  31  |         const response = await fetch(`./utility/${manifest.static_candidates}`);
  32  |         const bytes = await response.arrayBuffer();
  33  |         const digest = [...new Uint8Array(await crypto.subtle.digest('SHA-256', bytes))].map(byte => byte.toString(16).padStart(2,'0')).join('');
  34  |         const seed = JSON.parse(new TextDecoder().decode(bytes));
  35  |         api.dispose('scope-b');
  36  |         api.mount('scope-b', 10000, '@theme inline { --spacing: 4px; } @tailwind utilities;');
  37  |         const sample = document.getElementById('scope-b-sample')!;
  38  |         const reference = document.createElement('div'); reference.id = 'seed-reference'; reference.className = 'p-2 p-4';
  39  |         document.querySelector('main')!.append(reference);
  40  |         const sheet = new CSSStyleSheet();
  41  |         const installReference = (css: string) => { sheet.replaceSync(`@scope (#seed-reference) { ${css} }`); };
  42  |         installReference(official[0]);
  43  |         document.adoptedStyleSheets = [...document.adoptedStyleSheets, sheet];
  44  |         const first = await api.ensure('scope-b', ['p-2']);
  45  |         // Read synchronously before Observer can supply p-4: its rule must come from the seed.
  46  |         sample.className = 'p-2 p-4';
  47  |         const padding = getComputedStyle(sample).padding;
  48  |         const officialPadding = getComputedStyle(reference).padding;
  49  |         const next = await api.replace('scope-b', '@theme inline { --spacing: 8px; } @tailwind utilities;');
  50  |         const replacedPadding = getComputedStyle(sample).padding;
  51  |         installReference(official[1]);
  52  |         const officialReplacedPadding = getComputedStyle(reference).padding;
  53  |         await api.flush('scope-b');
  54  |         const replay = await api.retry('scope-b');
  55  |         const retryPadding = getComputedStyle(sample).padding;
  56  |         document.adoptedStyleSheets = document.adoptedStyleSheets.filter(value => value !== sheet); reference.remove();
  57  |         return {seed, digest, expectedDigest:manifest.files_sha256[manifest.static_candidates], first, padding,
  58  |             next, replacedPadding, replay, retryPadding, officialPadding, officialReplacedPadding};
  59  |     }, official);
  60  |     expect(result.seed).toContain('p-4');
  61  |     expect(result.seed).toContain('focus-visible:ring-[3px]');
  62  |     expect(result.seed).not.toContain('group');
  63  |     expect(result.digest).toBe(result.expectedDigest);
  64  |     expect(result.first.admitted).toContain('p-4');
  65  |     expect(result.padding).toBe('16px');
> 66  |     expect(result.padding).toBe(result.officialPadding);
      |                            ^ Error: expect(received).toBe(expected) // Object.is equality
  67  |     expect(result.replacedPadding).toBe('32px');
  68  |     expect(result.retryPadding).toBe('32px');
  69  |     expect(result.replacedPadding).toBe(result.officialReplacedPadding);
  70  |     expect(result.next.css.indexOf('.p-2')).toBeLessThan(result.next.css.indexOf('.p-4'));
  71  |     expect(result.replay.admitted).toContain('p-4');
  72  |     expect(result.replay.revision).toBeGreaterThan(result.next.revision);
  73  |     csp.push(...await page.evaluate(() => (window as any).__seedCsp));
  74  |     expect(errors).toEqual([]); expect(failed).toEqual([]); expect(csp).toEqual([]);
  75  |     await info.attach('Static candidate seed and installed cascade', {body: JSON.stringify(result,null,2),contentType:'application/json'});
  76  | });
  77  | 
  78  | test('a modified seed is rejected before installation and explicit retry keeps committed styles', async ({page}) => {
  79  |     await ready(page);
  80  |     await page.evaluate(async () => { await (window as any).__utility_lab.ensure('scope-b',['w-[137px]']); });
  81  |     await page.route('**/utility/static-candidates.json?sha256=*', route => route.fulfill({contentType:'application/json',body:'[]'}));
  82  |     const rejected = await page.evaluate(async () => {
  83  |         const api = (window as any).__utility_lab;
  84  |         const before = api.report('scope-b');
  85  |         let failure;
  86  |         try { await api.retry('scope-b'); } catch (error) { failure = JSON.parse(String(error)); }
  87  |         const sample = document.getElementById('scope-b-sample')!; sample.className='w-[137px]';
  88  |         return {failure,before:before.revision,retained:api.report('scope-b').revision,width:getComputedStyle(sample).width};
  89  |     });
  90  |     expect(rejected.failure.kind).toBe('protocol');
  91  |     expect(rejected.failure.message).toContain('asset digest mismatch: static-candidates.json');
  92  |     expect(rejected.retained).toBe(rejected.before); expect(rejected.width).toBe('137px');
  93  |     await page.unroute('**/utility/static-candidates.json?sha256=*');
  94  |     const replayed = await page.evaluate(async () => (window as any).__utility_lab.retry('scope-b'));
  95  |     expect(replayed.admitted).toContain('w-[137px]');
  96  | });
  97  | 
  98  | test('seed count and transfer budgets reject init without retaining a half initialized session', async ({page}, info) => {
  99  |     await ready(page);
  100 |     const result = await page.evaluate(async () => {
  101 |         const manifest = await fetch('./utility/build-manifest.json').then(response => response.json());
  102 |         const worker = new Worker(new URL('./utility/utility-worker.js', document.baseURI), {type:'module'});
  103 |         let request = 0;
  104 |         const limits = {input_bytes:65536,asset_bytes:65536,candidate_count:1024,ast_depth:32,css_bytes:262144,queued_bytes:65536,cache_bytes:1048576};
  105 |         const init = (override: object) => new Promise<any>((resolve, reject) => {
  106 |             worker.onmessage = ({data}) => resolve(data);
  107 |             worker.onerror = event => reject(new Error(event.message));
  108 |             worker.postMessage({protocol_version:1,runtime_id:'seed-budget',epoch:1,request_id:++request,operation:'init',payload:{build_id:manifest.build_id,
  109 |                 config:JSON.stringify({input:{css:'@tailwind utilities;',entry:'memory:input.css'},assets:[],limits:{...limits,...override}})}});
  110 |         });
  111 |         try {
  112 |             const count = await init({candidate_count:1});
  113 |             const transfer = await init({queued_bytes:1024});
  114 |             const recovered = await init({});
  115 |             return {count,transfer,recovered};
  116 |         } finally {worker.terminate();}
  117 |     });
  118 |     expect(result.count.operation).toBe('error');
  119 |     expect(result.count.payload).toEqual({kind:'compile',error:{kind:'limit',resource:'candidate_count',limit:1}});
  120 |     expect(result.transfer.operation).toBe('error');
  121 |     expect(result.transfer.payload).toEqual({kind:'compile',error:{kind:'limit',resource:'queued_bytes',limit:1024}});
  122 |     expect(result.recovered.operation).toBe('ready');
  123 |     expect(result.recovered.payload.static_candidates).toContain('p-4');
  124 |     await info.attach('Seed init budget boundaries and recovery', {body:JSON.stringify(result,null,2),contentType:'application/json'});
  125 | });
  126 | 
  127 | test('real DOM components use scope cn on raw fragments and react to caller and size changes', async ({page}, info) => {
  128 |     const errors: string[] = [], failed: string[] = [], csp: string[] = [];
  129 |     page.on('pageerror', error => errors.push(error.message));
  130 |     page.on('console', message => { if (message.type() === 'error') errors.push(message.text()); });
  131 |     page.on('requestfailed', request => failed.push(`${request.url()}: ${request.failure()?.errorText}`));
  132 |     page.on('response', response => { if (response.status() >= 400) failed.push(`${response.status()} ${response.url()}`); });
  133 |     await ready(page);
  134 |     await expect.poll(() => page.evaluate(() => (window as any).__utility_lab.gpuReport('scope-a')?.button.drawn_revision)).toBe(0);
  135 |     const initial = await page.evaluate(async () => {
  136 |         document.addEventListener('securitypolicyviolation', event => (window as any).__cn_csp.push(event.violatedDirective));
  137 |         (window as any).__cn_csp = [];
  138 |         const api = (window as any).__utility_lab.componentBaseline;
  139 |         for (const [id, custom] of [['cn-default', false], ['cn-custom', true]] as const) {
  140 |             const root = document.createElement('div'); root.id = id; document.querySelector('main')!.append(root);
  141 |             api.mount(id, custom);
  142 |             await api.ensure(id, ['p-2', 'p-4', 'p-8', 'bg-[#123456]']);
  143 |         }
  144 |         return ['cn-default', 'cn-custom'].map(id => {
  145 |             const card = document.querySelector(`[data-testid="${id}-card"]`)!;
  146 |             return {classes: card.className, padding: getComputedStyle(card).padding, background: getComputedStyle(card).backgroundColor};
  147 |         });
  148 |     });
  149 |     expect(initial).toEqual([
  150 |         {classes: 'bg-[#123456] p-4', padding: '16px', background: 'rgb(18, 52, 86)'},
  151 |         {classes: 'p-2 bg-[#123456] p-4', padding: '32px', background: 'rgb(18, 52, 86)'},
  152 |     ]);
  153 |     const button = page.getByTestId('cn-custom-button');
  154 |     await expect(button).toHaveClass(/h-9/);
  155 |     await expect(button).toHaveClass(/p-4/);
  156 |     await page.evaluate(() => (window as any).__utility_lab.componentBaseline.update('cn-custom', 'p-8', true));
  157 |     await expect(button).toHaveClass(/h-8/);
  158 |     await expect(button).toHaveClass(/p-8/);
  159 |     await expect(button).not.toHaveClass(/h-9|p-4/);
  160 |     await page.evaluate(() => (window as any).__utility_lab.componentBaseline.update('cn-custom', '', false));
  161 |     await expect(button).toHaveClass(/h-9/);
  162 |     await expect(button).toHaveClass(/px-4/);
  163 |     await expect(button).toHaveClass(/py-2/);
  164 |     await expect(button).not.toHaveClass(/p-8/);
  165 |     await info.attach('Scope cn classes and computed styles', {body: JSON.stringify(initial, null, 2), contentType: 'application/json'});
  166 |     await page.getByTestId('cn-custom-card').screenshot({path: info.outputPath('cn-component.png')});
```