// The dev problem panel for one refused page whose repair quotes a default
// literal containing a backtick: record what the panel shows next to the
// CLI's repair text. node tick-browser.cjs <dev url> <cli repair> <output prefix>
const { chromium, expect } = require(process.cwd() + '/node_modules/@playwright/test');
const fs = require('node:fs');
const [url, repair, output] = process.argv.slice(2);
(async () => {
  const browser = await chromium.launch({ executablePath: process.env.RUSTIFY_CHROMIUM || undefined });
  const context = await browser.newContext({ viewport: { width: 1280, height: 900 } });
  const page = await context.newPage();
  const audit = { url, browser: browser.version(), cli_repair: repair, console: [], pageErrors: [] };
  page.on('console', msg => { if (['error', 'warning'].includes(msg.type())) audit.console.push(msg.text()); });
  page.on('pageerror', error => audit.pageErrors.push(String(error)));
  try {
    await page.goto(url);
    const panel = page.locator('.rd-dev-panel');
    await expect(panel).toBeVisible({ timeout: 60000 });
    const group = panel.locator('.rd-dev-list > section').filter({ has: page.getByRole('heading', { name: 'content/gaps.md', exact: true }) });
    const shown = group.locator('.rd-dev-suggestion').first();
    await expect(shown).toBeVisible();
    audit.panel_text = await shown.textContent();
    audit.panel_code = await shown.locator('code').allTextContents();
    // What a faithful rendering shows: every code-span delimiter dropped,
    // the literal's own backtick kept.
    audit.faithful_text = repair.replace('"a`b"', '"a\u0000b"').replaceAll('`', '').replace('\u0000', '`');
    audit.panel_keeps_literal = audit.panel_text.includes('"a`b".to_string()');
    audit.panel_equals_faithful = audit.panel_text === audit.faithful_text;
    await page.screenshot({ path: output + '.png', fullPage: false });
  } finally {
    fs.writeFileSync(output + '.json', JSON.stringify(audit, null, 2) + '\n');
    await context.close();
    await browser.close();
  }
})().catch(error => { console.error(error); process.exitCode = 1; });
