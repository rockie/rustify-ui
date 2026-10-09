# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: utility-runtime.spec.ts >> parser recovery preserves authored important values and valid compound variants
- Location: tests/browser/utility-runtime.spec.ts:8:5

# Error details

```
Error: expect(received).toEqual(expected) // deep equality

- Expected  - 0
+ Received  + 6

  Array [
    "[_]:block",
    "data-foo[bar]:block",
+   "nested-slot",
+   "preview",
+   "preview-content",
+   "sample",
+   "sample-swatch",
+   "sample-text",
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
  6   | }
  7   | 
  8   | test('parser recovery preserves authored important values and valid compound variants', async ({page}, info) => {
  9   |     const errors: string[] = [], failed: string[] = [];
  10  |     page.on('pageerror', error => errors.push(error.message));
  11  |     page.on('console', message => { if (message.type() === 'error') errors.push(message.text()); });
  12  |     page.on('requestfailed', request => failed.push(request.url()));
  13  |     await ready(page);
  14  |     const result = await page.evaluate(async () => {
  15  |         const api = (window as any).__utility_lab;
  16  |         api.dispose('scope-a');
  17  |         api.mount('scope-a', 10000, '@tailwind utilities; .sample { width:137px!important ignored; background-color:#123456; }');
  18  |         const classes = ['[_]:block', 'data-foo[bar]:block', 'group-data-[state=open]:w-[159px]!'];
  19  |         const report = await api.ensure('scope-a', classes);
  20  |         const frame = document.getElementById('scope-a-frame')!;
  21  |         frame.classList.add('group');
  22  |         const sample = document.getElementById('scope-a-sample')!;
  23  |         sample.className = `sample ${classes.join(' ')}`;
  24  |         frame.setAttribute('data-state', 'closed');
  25  |         const closed = getComputedStyle(sample).width;
  26  |         frame.setAttribute('data-state', 'open');
  27  |         const open = getComputedStyle(sample).width;
  28  |         frame.setAttribute('data-state', 'closed');
  29  |         return {report, closed, open, restored: getComputedStyle(sample).width,
  30  |             background: getComputedStyle(sample).backgroundColor,
  31  |             otherWidth: getComputedStyle(document.getElementById('scope-b-sample')!).width};
  32  |     });
  33  |     expect(result.report.unsupported).toEqual([]);
> 34  |     expect(result.report.rejected.map((item: any) => item.candidate).sort()).toEqual(['[_]:block', 'data-foo[bar]:block']);
      |                                                                              ^ Error: expect(received).toEqual(expected) // deep equality
  35  |     expect(result.report.admitted).toEqual(['group-data-[state=open]:w-[159px]!']);
  36  |     expect(result.report.css).not.toContain('ignored');
  37  |     expect(result.closed).toBe('137px');
  38  |     expect(result.open).toBe('159px');
  39  |     expect(result.restored).toBe('137px');
  40  |     expect(result.background).toBe('rgb(18, 52, 86)');
  41  |     expect(result.otherWidth).toBe('170px');
  42  |     expect(errors).toEqual([]); expect(failed).toEqual([]);
  43  |     await info.attach('Parser browser values', {body: JSON.stringify(result, null, 2), contentType: 'application/json'});
  44  | });
  45  | 
  46  | test('structural spacing, container and line clamp keep their actual browser semantics', async ({page}, info) => {
  47  |     await ready(page);
  48  |     const result = await page.evaluate(async () => {
  49  |         const api = (window as any).__utility_lab;
  50  |         const classes = ['border-separate', 'border-spacing-[12px]', 'flex', 'space-x-[9px]',
  51  |             'space-x-reverse', 'line-clamp-2', '@container/probe'];
  52  |         const report = await api.ensure('scope-a', classes);
  53  |         const root = document.getElementById('scope-a')!;
  54  |         const table = document.createElement('table');
  55  |         table.className = 'border-separate border-spacing-[12px]';
  56  |         table.innerHTML = '<tbody><tr><td>one</td><td>two</td></tr></tbody>';
  57  |         const siblings = document.createElement('div');
  58  |         siblings.className = 'flex space-x-[9px] space-x-reverse';
  59  |         siblings.innerHTML = '<span>one</span><span>two</span><span>three</span>';
  60  |         const container = document.createElement('div');
  61  |         container.className = '@container/probe';
  62  |         const clamp = document.createElement('p');
  63  |         clamp.className = 'line-clamp-2';
  64  |         clamp.textContent = 'Browser utility checks';
  65  |         container.append(clamp);
  66  |         root.append(table, siblings, container);
  67  |         const margins = Array.from(siblings.children).map(child => {
  68  |             const style = getComputedStyle(child);
  69  |             return [style.marginInlineStart, style.marginInlineEnd];
  70  |         });
  71  |         const containerStyle = getComputedStyle(container), clampStyle = getComputedStyle(clamp);
  72  |         return {revision: report.revision, pending: report.unsupported, spacing: getComputedStyle(table).borderSpacing,
  73  |             margins, containerType: containerStyle.containerType, containerName: containerStyle.containerName,
  74  |             lineClamp: clampStyle.webkitLineClamp, overflow: clampStyle.overflow, css: report.css};
  75  |     });
  76  |     expect(result.pending).toEqual([]);
  77  |     expect(result.spacing).toBe('12px');
  78  |     expect(result.margins).toEqual([['9px', '0px'], ['9px', '0px'], ['0px', '0px']]);
  79  |     expect(result.containerType).toBe('inline-size');
  80  |     expect(result.containerName).toBe('probe');
  81  |     expect(result.lineClamp).toBe('2');
  82  |     expect(result.overflow).toBe('hidden');
  83  |     expect(result.css).not.toContain('--tw-sort');
  84  |     await info.attach('Structural browser values', {body: JSON.stringify(result, null, 2), contentType: 'application/json'});
  85  | });
  86  | 
  87  | test('SDK ensure installs scanner-free CSS, coalesces requests and isolates independent scopes', async ({ page }, testInfo) => {
  88  |     const errors: string[] = [];
  89  |     const failed: string[] = [];
  90  |     const paths: string[] = [];
  91  |     page.on('pageerror', error => errors.push(error.message));
  92  |     page.on('requestfailed', request => failed.push(request.url()));
  93  |     page.on('request', request => paths.push(request.url()));
  94  |     await ready(page);
  95  |     const result = await page.evaluate(async () => {
  96  |         const api = (window as any).__utility_lab;
  97  |         const violations: string[] = [];
  98  |         document.addEventListener('securitypolicyviolation', event => violations.push(event.violatedDirective));
  99  |         const first = await Promise.all([
  100 |             api.ensure('scope-a', ['w-[137px]']),
  101 |             api.ensure('scope-a', ['bg-[#123456]', 'w-[137px]']),
  102 |         ]);
  103 |         const sample = document.getElementById('scope-a-sample')!;
  104 |         sample.className = 'sample w-[137px] bg-[#123456]';
  105 |         const style = getComputedStyle(sample);
  106 |         const width = style.width;
  107 |         const color = style.backgroundColor;
  108 |         const initial = api.report('scope-a');
  109 |         const repeated = await api.ensure('scope-a', ['w-[137px]']);
  110 |         const other = await api.ensure('scope-b', ['w-[93px]', 'bg-[#abcdef]']);
  111 |         const second = document.getElementById('scope-b-sample')!;
  112 |         second.className = 'sample w-[93px] bg-[#abcdef]';
  113 |         const outside = document.getElementById('outside')!;
  114 |         const outsideColor = getComputedStyle(outside).backgroundColor;
  115 |         const otherColor = getComputedStyle(second).backgroundColor;
  116 |         const sheetCount = document.adoptedStyleSheets.length;
  117 |         const css = api.report('scope-a').css;
  118 |         api.dispose('scope-a');
  119 |         return { first: first.map((report: any) => report.revision), revision: initial.revision,
  120 |             repeatedRevision: repeated.revision, width, color, otherColor, outsideColor,
  121 |             otherRevision: other.revision, sheetCount, remainingSheets: document.adoptedStyleSheets.length,
  122 |             remainingColor: getComputedStyle(second).backgroundColor, violations, css };
  123 |     });
  124 |     expect(result.first).toEqual([1, 1]);
  125 |     expect(result.width).toBe('137px');
  126 |     expect(result.color).toBe('rgb(18, 52, 86)');
  127 |     expect(result.otherColor).toBe('rgb(171, 205, 239)');
  128 |     expect(result.outsideColor).toBe('rgba(0, 0, 0, 0)');
  129 |     expect(result.repeatedRevision).toBe(result.revision);
  130 |     expect(result.remainingSheets).toBe(result.sheetCount - 1);
  131 |     expect(result.remainingColor).toBe(result.otherColor);
  132 |     expect(result.violations).toEqual([]);
  133 |     expect(errors).toEqual([]);
  134 |     expect(failed).toEqual([]);
```