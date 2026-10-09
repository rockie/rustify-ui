# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: utility-runtime.spec.ts >> manifest seed preserves static utility order on first ensure, replacement and retry
- Location: tests/browser/utility-runtime.spec.ts:10:5

# Error details

```
Error: page.evaluate: a prepared result must be committed or discarded first
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
> 28  |     const result = await page.evaluate(async official => {
      |                               ^ Error: page.evaluate: a prepared result must be committed or discarded first
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
  53  |         const replay = await api.retry('scope-b');
  54  |         const retryPadding = getComputedStyle(sample).padding;
  55  |         document.adoptedStyleSheets = document.adoptedStyleSheets.filter(value => value !== sheet); reference.remove();
  56  |         return {seed, digest, expectedDigest:manifest.files_sha256[manifest.static_candidates], first, padding,
  57  |             next, replacedPadding, replay, retryPadding, officialPadding, officialReplacedPadding};
  58  |     }, official);
  59  |     expect(result.seed).toContain('p-4');
  60  |     expect(result.seed).toContain('focus-visible:ring-[3px]');
  61  |     expect(result.seed).not.toContain('group');
  62  |     expect(result.digest).toBe(result.expectedDigest);
  63  |     expect(result.first.admitted).toContain('p-4');
  64  |     expect(result.padding).toBe('16px');
  65  |     expect(result.padding).toBe(result.officialPadding);
  66  |     expect(result.replacedPadding).toBe('32px');
  67  |     expect(result.retryPadding).toBe('32px');
  68  |     expect(result.replacedPadding).toBe(result.officialReplacedPadding);
  69  |     expect(result.next.css.indexOf('.p-2')).toBeLessThan(result.next.css.indexOf('.p-4'));
  70  |     expect(result.replay.admitted).toContain('p-4');
  71  |     expect(result.replay.revision).toBeGreaterThan(result.next.revision);
  72  |     csp.push(...await page.evaluate(() => (window as any).__seedCsp));
  73  |     expect(errors).toEqual([]); expect(failed).toEqual([]); expect(csp).toEqual([]);
  74  |     await info.attach('Static candidate seed and installed cascade', {body: JSON.stringify(result,null,2),contentType:'application/json'});
  75  | });
  76  | 
  77  | test('a modified seed is rejected before installation and explicit retry keeps committed styles', async ({page}) => {
  78  |     await ready(page);
  79  |     await page.evaluate(async () => { await (window as any).__utility_lab.ensure('scope-b',['w-[137px]']); });
  80  |     await page.route('**/utility/static-candidates.json?sha256=*', route => route.fulfill({contentType:'application/json',body:'[]'}));
  81  |     const rejected = await page.evaluate(async () => {
  82  |         const api = (window as any).__utility_lab;
  83  |         const before = api.report('scope-b');
  84  |         let failure;
  85  |         try { await api.retry('scope-b'); } catch (error) { failure = JSON.parse(String(error)); }
  86  |         const sample = document.getElementById('scope-b-sample')!; sample.className='w-[137px]';
  87  |         return {failure,before:before.revision,retained:api.report('scope-b').revision,width:getComputedStyle(sample).width};
  88  |     });
  89  |     expect(rejected.failure.kind).toBe('protocol');
  90  |     expect(rejected.failure.message).toContain('asset digest mismatch: static-candidates.json');
  91  |     expect(rejected.retained).toBe(rejected.before); expect(rejected.width).toBe('137px');
  92  |     await page.unroute('**/utility/static-candidates.json?sha256=*');
  93  |     const replayed = await page.evaluate(async () => (window as any).__utility_lab.retry('scope-b'));
  94  |     expect(replayed.admitted).toContain('w-[137px]');
  95  | });
  96  | 
  97  | test('real DOM components use scope cn on raw fragments and react to caller and size changes', async ({page}, info) => {
  98  |     const errors: string[] = [], failed: string[] = [], csp: string[] = [];
  99  |     page.on('pageerror', error => errors.push(error.message));
  100 |     page.on('console', message => { if (message.type() === 'error') errors.push(message.text()); });
  101 |     page.on('requestfailed', request => failed.push(`${request.url()}: ${request.failure()?.errorText}`));
  102 |     page.on('response', response => { if (response.status() >= 400) failed.push(`${response.status()} ${response.url()}`); });
  103 |     await ready(page);
  104 |     await expect.poll(() => page.evaluate(() => (window as any).__utility_lab.gpuReport('scope-a')?.button.drawn_revision)).toBe(0);
  105 |     const initial = await page.evaluate(async () => {
  106 |         document.addEventListener('securitypolicyviolation', event => (window as any).__cn_csp.push(event.violatedDirective));
  107 |         (window as any).__cn_csp = [];
  108 |         const api = (window as any).__utility_lab.componentBaseline;
  109 |         for (const [id, custom] of [['cn-default', false], ['cn-custom', true]] as const) {
  110 |             const root = document.createElement('div'); root.id = id; document.querySelector('main')!.append(root);
  111 |             api.mount(id, custom);
  112 |             await api.ensure(id, ['p-2', 'p-4', 'p-8', 'bg-[#123456]']);
  113 |         }
  114 |         return ['cn-default', 'cn-custom'].map(id => {
  115 |             const card = document.querySelector(`[data-testid="${id}-card"]`)!;
  116 |             return {classes: card.className, padding: getComputedStyle(card).padding, background: getComputedStyle(card).backgroundColor};
  117 |         });
  118 |     });
  119 |     expect(initial).toEqual([
  120 |         {classes: 'bg-[#123456] p-4', padding: '16px', background: 'rgb(18, 52, 86)'},
  121 |         {classes: 'p-2 bg-[#123456] p-4', padding: '32px', background: 'rgb(18, 52, 86)'},
  122 |     ]);
  123 |     const button = page.getByTestId('cn-custom-button');
  124 |     await expect(button).toHaveClass(/h-9/);
  125 |     await expect(button).toHaveClass(/p-4/);
  126 |     await page.evaluate(() => (window as any).__utility_lab.componentBaseline.update('cn-custom', 'p-8', true));
  127 |     await expect(button).toHaveClass(/h-8/);
  128 |     await expect(button).toHaveClass(/p-8/);
```