# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: docs-ssg/dev-components.spec.ts >> gaps-2 code spans: frontmatter language preserves author text resembling a YAML location
- Location: tests/docs-ssg/dev-components.spec.ts:1050:5

# Error details

```
Error: expect(received).toMatchObject(expected)

- Expected  - 1
+ Received  + 1

  Object {
    "column": 7,
    "file": "content/index.md",
    "line": 3,
-   "message": "lang: `` a` at line `b `` is not a language tag",
+   "message": "lang: `` a`",
  }
```

# Page snapshot

```yaml
- generic [active] [ref=f1e1]:
  - link "Skip to content" [ref=f1e2] [cursor=pointer]:
    - /url: "#content"
  - banner [ref=f1e3]:
    - generic [ref=f1e4]:
      - link "Plain" [ref=f1e5] [cursor=pointer]:
        - /url: /
      - group "Theme" [ref=f1e7]:
        - button "System" [pressed] [ref=f1e8] [cursor=pointer]
        - button "Light" [ref=f1e14] [cursor=pointer]
        - button "Dark" [ref=f1e20] [cursor=pointer]
  - generic [ref=f1e25]:
    - navigation "Menu" [ref=f1e27]:
      - generic [ref=f1e28]:
        - heading "Plain" [level=2] [ref=f1e29]
        - list [ref=f1e30]:
          - listitem [ref=f1e31]:
            - link "Home" [ref=f1e32] [cursor=pointer]:
              - /url: /
    - main [ref=f1e33]:
      - article [ref=f1e35]:
        - heading "Home" [level=1] [ref=f1e36]
  - contentinfo [ref=f1e37]:
    - navigation "Footer links" [ref=f1e38]:
      - link "Back to the home page" [ref=f1e39] [cursor=pointer]:
        - /url: /
  - paragraph [ref=f1e40]
  - status [ref=f1e41]: 1 error · showing the last good version · revision 2
  - dialog "1 error · showing the last good version" [ref=f1e43]:
    - generic [ref=f1e44]:
      - paragraph [ref=f1e45]:
        - text: 1 error
        - generic [ref=f1e46]:
          - generic [ref=f1e47]: ·
          - text: showing the last good version
      - generic [ref=f1e48]: revision 2
      - button "Hide problems" [ref=f1e49] [cursor=pointer]
    - region "Problems" [ref=f1e53]:
      - generic [ref=f1e54]:
        - heading "content/index.md" [level=2] [ref=f1e55]
        - list [ref=f1e56]:
          - listitem [ref=f1e57]:
            - generic [ref=f1e58]:
              - generic [ref=f1e59]: 3:7
              - generic [ref=f1e60]:
                - generic [ref=f1e61]: "Error:"
                - text: lang
            - paragraph [ref=f1e62]: "lang: `` a`"
            - paragraph [ref=f1e63]:
              - text: use a language tag such as
              - code [ref=f1e64]: en
              - text: ","
              - code [ref=f1e65]: zh-CN
              - text: or
              - code [ref=f1e66]: fr
```

# Test source

```ts
  969  |     expect(audit.csp).toEqual([]);
  970  |     expect(audit.failures).toEqual([]);
  971  |     expect(audit.console).toEqual([]);
  972  |     expect(audit.pageErrors).toEqual([]);
  973  |     expect(server.streamErrors).toEqual([]);
  974  | });
  975  | 
  976  | // Independent display expectations: never derive these from the prose reader.
  977  | async function exactProse(page: Page, state: State, rule: string, expected: { message: string; messageCodes: string[]; suggestion?: string; suggestionCodes?: string[] }) {
  978  |     const item = page.locator('.rd-dev-item').filter({ has: page.locator('.rd-dev-rule', { hasText: rule }) });
  979  |     await expect(item).toHaveCount(1);
  980  |     await expect.poll(() => item.locator('.rd-dev-message').textContent()).toBe(expected.message);
  981  |     expect(await item.locator('.rd-dev-message code').allTextContents()).toEqual(expected.messageCodes);
  982  |     if (expected.suggestion !== undefined) {
  983  |         expect(await item.locator('.rd-dev-suggestion').textContent()).toBe(expected.suggestion);
  984  |         expect(await item.locator('.rd-dev-suggestion code').allTextContents()).toEqual(expected.suggestionCodes);
  985  |     }
  986  |     await test.info().attach(`${rule}-prose.json`, { body: JSON.stringify({ diagnostic: state.diagnostics.find(d => d.rule === rule), expected, actual: await item.evaluate(node => ({ message: node.querySelector('.rd-dev-message')?.textContent, messageCodes: [...node.querySelectorAll('.rd-dev-message code')].map(code => code.textContent), suggestion: node.querySelector('.rd-dev-suggestion')?.textContent, suggestionCodes: [...node.querySelectorAll('.rd-dev-suggestion code')].map(code => code.textContent) })) }, null, 2), contentType: 'application/json' });
  987  | }
  988  | 
  989  | async function proseLayouts(page: Page, label: string) {
  990  |     for (const width of [1280, 375]) {
  991  |         for (const theme of ['light', 'dark']) {
  992  |             await page.setViewportSize({ width, height: 900 });
  993  |             await page.locator(`[data-rd-mode="${theme}"]`).click();
  994  |             await expect(page.locator('html')).toHaveAttribute('data-theme', theme);
  995  |             expect(await page.evaluate(() => document.documentElement.scrollWidth - innerWidth)).toBeLessThanOrEqual(0);
  996  |             const actual = await page.locator('.rd-dev-item').evaluateAll(items => items.map(item => ({ message: item.querySelector('.rd-dev-message')?.textContent, suggestion: item.querySelector('.rd-dev-suggestion')?.textContent, codes: [...item.querySelectorAll('code')].map(code => ({ text: code.textContent, font: getComputedStyle(code).fontFamily })), overflow: [...item.querySelectorAll<HTMLElement>('.rd-dev-message, .rd-dev-suggestion')].map(node => node.scrollWidth - node.clientWidth) })));
  997  |             expect(actual.every(item => item.overflow.every(extra => extra <= 0))).toBe(true);
  998  |             await test.info().attach(`${label}-${width}-${theme}.json`, { body: JSON.stringify({ width, theme, actual }, null, 2), contentType: 'application/json' });
  999  |             await test.info().attach(`${label}-${width}-${theme}.png`, { body: await page.screenshot(), contentType: 'image/png' });
  1000 |         }
  1001 |     }
  1002 | }
  1003 | 
  1004 | test('gaps-2 code spans: plain author paths, prefixed anchors, YAML keys and empty language remain exact', async ({ page }) => {
  1005 |     const plain = path.join(site.dir, 'plain');
  1006 |     mkdirSync(path.join(plain, 'content'), { recursive: true });
  1007 |     const config = "schema_version=1\n[site]\nname='Plain'\nurl='https://example.com/'\n";
  1008 |     const good = '---\ndescription: Home\n---\n# Home\n';
  1009 |     writeFileSync(path.join(plain, 'rustify-docs.toml'), config);
  1010 |     writeFileSync(path.join(plain, 'content/index.md'), good);
  1011 |     writeFileSync(path.join(plain, 'content/a`b`c.md'), '---\ndescription: Target\n---\n# Target\n');
  1012 |     server = await Dev.start(site, plain);
  1013 |     await page.goto(server.url);
  1014 |     let state = await settled();
  1015 |     writeFileSync(path.join(plain, 'content/index.md'), `${good}\n[Target](a%60b%60c.md#a%60b)\n`);
  1016 |     state = await settled(state.revision, false);
  1017 |     expect(state.diagnostics.find(d => d.rule === 'broken-anchor')?.message).toBe('`` content/a`b`c.md `` has no heading `` #a`b ``');
  1018 |     await exactProse(page, state, 'broken-anchor', { message: 'content/a`b`c.md has no heading #a`b', messageCodes: ['content/a`b`c.md', '#a`b'] });
  1019 |     await proseLayouts(page, 'gaps2-anchor');
  1020 |     const hide = page.getByRole('button', { name: 'Hide problems' });
  1021 |     const marker = page.locator('.rd-dev-marker');
  1022 |     await hide.click();
  1023 |     await expect(marker).toBeFocused();
  1024 |     await page.keyboard.press('Enter');
  1025 |     await expect(hide).toBeFocused();
  1026 |     await page.keyboard.press('Escape');
  1027 |     await expect(marker).toBeFocused();
  1028 |     await marker.click();
  1029 | 
  1030 |     writeFileSync(path.join(plain, 'content/index.md'), `${good}\n\`\`\`rustify-component\nname: Steps\nun\`known: 1\n\`\`\`\n`);
  1031 |     state = await settled(state.revision, false);
  1032 |     await exactProse(page, state, 'embed-payload', { message: 'unknown field un`known; a demo takes only name, a component takes name, props and children', messageCodes: ['un`known', 'name', 'name', 'props', 'children'] });
  1033 |     await proseLayouts(page, 'gaps2-yaml');
  1034 | 
  1035 |     writeFileSync(path.join(plain, 'rustify-docs.toml'), `${config}lang=''\n`);
  1036 |     state = await settled(state.revision, false);
  1037 |     await exactProse(page, state, 'config-lang', { message: ' is not a language tag', messageCodes: [''], suggestion: 'use a tag like en or zh-CN', suggestionCodes: ['en', 'zh-CN'] });
  1038 |     await proseLayouts(page, 'gaps2-empty');
  1039 |     writeFileSync(path.join(plain, 'content/index.md'), good);
  1040 |     writeFileSync(path.join(plain, 'rustify-docs.toml'), config);
  1041 |     state = await settled(state.revision);
  1042 |     await cleared(page, state);
  1043 |     expect(audit.csp).toEqual([]);
  1044 |     expect(audit.failures).toEqual([]);
  1045 |     expect(audit.console).toEqual([]);
  1046 |     expect(audit.pageErrors).toEqual([]);
  1047 |     expect(server.streamErrors).toEqual([]);
  1048 | });
  1049 | 
  1050 | test('gaps-2 code spans: frontmatter language preserves author text resembling a YAML location', async ({ page }) => {
  1051 |     const plain = path.join(site.dir, 'plain');
  1052 |     mkdirSync(path.join(plain, 'content'), { recursive: true });
  1053 |     const good = '---\ndescription: Home\n---\n# Home\n';
  1054 |     writeFileSync(path.join(plain, 'rustify-docs.toml'), "schema_version=1\n[site]\nname='Plain'\nurl='https://example.com/'\n");
  1055 |     writeFileSync(path.join(plain, 'content/index.md'), good);
  1056 |     server = await Dev.start(site, plain);
  1057 |     await page.goto(server.url);
  1058 |     let state = await settled();
  1059 |     const first = state;
  1060 |     const lastGood = await (await page.request.get(server.url)).body();
  1061 |     await page.evaluate(() => window.__dev_component_document = 'last good language page');
  1062 |     for (const [label, value, encoded] of [
  1063 |         ['ticks', 'a` at line `b', '`` a` at line `b ``'],
  1064 |         ['plain', 'a at line b', '`a at line b`'],
  1065 |     ]) {
  1066 |         writeFileSync(path.join(plain, 'content/index.md'), `---\ndescription: Home\nlang: "${value}"\n---\n# Home\n`);
  1067 |         state = await settled(state.revision, false);
  1068 |         const diagnostic = state.diagnostics.find(d => d.rule === 'lang')!;
> 1069 |         expect(diagnostic).toMatchObject({ file: 'content/index.md', line: 3, column: 7, message: `lang: ${encoded} is not a language tag` });
       |                            ^ Error: expect(received).toMatchObject(expected)
  1070 |         await exactProse(page, state, 'lang', { message: `lang: ${value} is not a language tag`, messageCodes: [value], suggestion: 'use a language tag such as en, zh-CN or fr', suggestionCodes: ['en', 'zh-CN', 'fr'] });
  1071 |         expect(state.good).toBe(first.good);
  1072 |         expect(await (await page.request.get(server.url)).body()).toEqual(lastGood);
  1073 |         expect(await page.evaluate(() => window.__dev_component_document)).toBe('last good language page');
  1074 |         await expect.poll(() => server!.log).toContain(diagnostic.message);
  1075 |         await expect.poll(() => server!.log).toContain('content/index.md:3:7');
  1076 |         await proseLayouts(page, `gaps2-frontmatter-${label}`);
  1077 |         const hide = page.getByRole('button', { name: 'Hide problems' });
  1078 |         const marker = page.locator('.rd-dev-marker');
  1079 |         await hide.click();
  1080 |         await expect(marker).toBeFocused();
  1081 |         await page.keyboard.press('Enter');
  1082 |         await expect(hide).toBeFocused();
  1083 |         await page.keyboard.press('Escape');
  1084 |         await expect(marker).toBeFocused();
  1085 |         await marker.click();
  1086 |     }
  1087 |     writeFileSync(path.join(plain, 'content/index.md'), good);
  1088 |     state = await settled(state.revision);
  1089 |     await cleared(page, state);
  1090 |     expect(audit.csp).toEqual([]);
  1091 |     expect(audit.failures).toEqual([]);
  1092 |     expect(audit.responses.filter(response => response.status >= 400)).toEqual([]);
  1093 |     expect(audit.console).toEqual([]);
  1094 |     expect(audit.pageErrors).toEqual([]);
  1095 |     expect(server.streamErrors).toEqual([]);
  1096 | });
  1097 | 
  1098 | test('gaps-2 code spans: defaults with literal backticks survive unknown props, type errors and external unmatched ticks', async ({ page }) => {
  1099 |     test.setTimeout(900_000);
  1100 |     site.edit('src/lib.rs', 'label: String = "Counter".to_string()', 'label: String = "a`b".to_string()');
  1101 |     server = await Dev.start(site);
  1102 |     await page.goto(new URL('counter/', server.url).href);
  1103 |     site.release();
  1104 |     await ready(page);
  1105 |     let state = await settled();
  1106 |     const original = site.read('content/counter.md');
  1107 |     await captureTrees(page);
  1108 |     site.write('content/counter.md', `${original}\n\`\`\`rustify-component\nname: Counter\nprops:\n  un\`known: 1\n\`\`\`\n`);
  1109 |     state = await settled(state.revision, false);
  1110 |     await exactProse(page, state, 'unknown-property', { message: 'component Counter has no property un`known', messageCodes: ['Counter', 'un`known'], suggestion: 'Use one of initial (i32; default expression 2), label (String; default expression "a`b".to_string()) for Counter; rename or remove un`known.', suggestionCodes: ['initial', 'i32', '2', 'label', 'String', '"a`b".to_string()', 'Counter', 'un`known'] });
  1111 |     await retained(page);
  1112 |     await proseLayouts(page, 'gaps2-default-unknown');
  1113 |     site.write('content/counter.md', `${original}\n\`\`\`rustify-component\nname: Counter\nprops:\n  label: [wrong]\n\`\`\`\n`);
  1114 |     state = await settled(state.revision, false);
  1115 |     const typed = state.diagnostics.find(d => d.rule === 'invalid_props')!;
  1116 |     expect(typed.suggestion).toBe('Write `label: "text"` for declared type `String` (default expression `` "a`b".to_string() ``). Reported props path: `label`.');
  1117 |     await exactProse(page, state, 'invalid_props', { message: 'invalid type: sequence, expected a string', messageCodes: [], suggestion: 'Write label: "text" for declared type String (default expression "a`b".to_string()). Reported props path: label.', suggestionCodes: ['label: "text"', 'String', '"a`b".to_string()', 'label'] });
  1118 |     await retained(page);
  1119 |     await proseLayouts(page, 'gaps2-default-type');
  1120 |     site.write('content/counter.md', `${original}\n\`\`\`rustify-component\nname: Counter\nprops:\n  initial: a\`b\n\`\`\`\n`);
  1121 |     state = await settled(state.revision, false);
  1122 |     const external = state.diagnostics.find(d => d.rule === 'invalid_props')!;
  1123 |     expect(external.message).toContain('a`b');
  1124 |     await exactProse(page, state, 'invalid_props', { message: 'invalid type: string "a`b", expected i32', messageCodes: [] });
  1125 |     await proseLayouts(page, 'gaps2-external');
  1126 |     await page.getByRole('button', { name: 'Hide problems' }).click();
  1127 |     await expect(page.locator('.rd-dev-marker')).toBeFocused();
  1128 |     site.write('content/counter.md', original);
  1129 |     state = await settled(state.revision);
  1130 |     await cleared(page, state);
  1131 |     await healthy();
  1132 | });
  1133 | 
  1134 | test('gaps-2 code spans: the real DOM reader preserves the handwritten whitespace and delimiter table', async ({ page }) => {
  1135 |     const plain = path.join(site.dir, 'plain');
  1136 |     mkdirSync(path.join(plain, 'content'), { recursive: true });
  1137 |     writeFileSync(path.join(plain, 'rustify-docs.toml'), "schema_version=1\n[site]\nname='Plain'\nurl='https://example.com/'\n");
  1138 |     writeFileSync(path.join(plain, 'content/index.md'), '---\ndescription: Home\n---\n# Home\n');
  1139 |     server = await Dev.start(site, plain);
  1140 |     const state = await settled();
  1141 |     const cases: { encoded: string; text: string; codes: string[] }[] = [
  1142 |         { encoded: '``  ``', text: '', codes: [''] },
  1143 |         { encoded: '` `', text: ' ', codes: [' '] },
  1144 |         { encoded: '`  `', text: '  ', codes: ['  '] },
  1145 |         { encoded: '` a `', text: ' a ', codes: [' a '] },
  1146 |         { encoded: '`a\nb`', text: 'a\nb', codes: ['a\nb'] },
  1147 |         { encoded: '`a\r\nb`', text: 'a\r\nb', codes: ['a\r\nb'] },
  1148 |         { encoded: '`a\\b`', text: 'a\\b', codes: ['a\\b'] },
  1149 |         { encoded: '`Counter`', text: 'Counter', codes: ['Counter'] },
  1150 |         { encoded: '`` a`b ``', text: 'a`b', codes: ['a`b'] },
  1151 |         { encoded: '``` a``b ```', text: 'a``b', codes: ['a``b'] },
  1152 |         { encoded: '`` `a` ``', text: '`a`', codes: ['`a`'] },
  1153 |         { encoded: '`` ` ``', text: '`', codes: ['`'] },
  1154 |         { encoded: '``` `` ```', text: '``', codes: ['``'] },
  1155 |         { encoded: '``  `\r\n  ``', text: ' `\r\n ', codes: [' `\r\n '] },
  1156 |         { encoded: '``    ``', text: '  ', codes: ['  '] },
  1157 |         { encoded: '``a b``', text: 'a b', codes: ['a b'] },
  1158 |         { encoded: '``\na\r\n``', text: '\na\r\n', codes: ['\na\r\n'] },
  1159 |         { encoded: '``  `` then `` file/a`b `` and `en` / `zh-CN`', text: ' then file/a`b and en / zh-CN', codes: ['', 'file/a`b', 'en', 'zh-CN'] },
  1160 |         { encoded: 'plain `unmatched and `` value ``', text: 'plain `unmatched and value', codes: ['value'] },
  1161 |         { encoded: 'unknown field `a`b`', text: 'unknown field ab`', codes: ['a'] },
  1162 |         { encoded: 'single a`b', text: 'single a`b', codes: [] },
  1163 |         { encoded: '`<img src=x onerror=alert(1)>`', text: '<img src=x onerror=alert(1)>', codes: ['<img src=x onerror=alert(1)>'] },
  1164 |     ];
  1165 |     cases.forEach(({ encoded, text, codes }) => expect(diagnosticProse(encoded)).toEqual({ text, codes }));
  1166 |     // This table isolates the DOM reader using a controlled SSE response.
  1167 |     // The other gaps-2 tests exercise actual author input through the service.
  1168 |     await page.route('**/__rustify-docs/events?**', route => route.fulfill({ contentType: 'text/event-stream', body: `event: state\ndata: ${JSON.stringify({ ...state, ok: false, errors: cases.length, warnings: 0, reload: false, diagnostics: cases.map((entry, i) => ({ severity: 'error', rule: `span-${i}`, file: 'table', line: i + 1, column: 1, message: entry.encoded, suggestion: entry.encoded })) })}\n\n` }));
  1169 |     await page.goto(server.url);
```