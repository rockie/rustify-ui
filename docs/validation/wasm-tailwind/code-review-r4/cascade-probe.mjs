import { createRequire } from 'node:module';
import { resolve } from 'node:path';
import { readFileSync, writeFileSync } from 'node:fs';

const require = createRequire(resolve('package.json'));
const { chromium } = require('playwright');
const input = JSON.parse(readFileSync(process.argv[2], 'utf8'));
const output = process.argv[3];
const url = process.argv[4];
const browser = await chromium.launch({ channel: 'chrome', headless: true });
try {
  const page = await browser.newPage();
  const errors = [], requests = [];
  page.on('pageerror', error => errors.push(error.message));
  page.on('console', message => { if (message.type() === 'error') errors.push(message.text()); });
  page.on('requestfailed', request => requests.push(request.url()));
  await page.addInitScript(() => {
    window.__r4Csp = [];
    document.addEventListener('securitypolicyviolation', event => window.__r4Csp.push(event.violatedDirective));
  });
  if (url) {
    await page.goto(url);
    await page.locator('#status[data-status="ready"]').waitFor();
  } else await page.setContent('<main></main>');
  const rows = await page.evaluate(async ({ input, withSdk }) => {
    const read = element => {
      const style = getComputedStyle(element);
      const animations = element.getAnimations();
      return {
        variable: style.getPropertyValue('--v').trim(),
        name: style.animationName,
        timing: style.animationTimingFunction,
        duration: style.animationDuration,
        iteration: style.animationIterationCount,
        count: animations.length,
        lastOpacity: animations[0]?.effect.getKeyframes().at(-1)?.opacity,
        transition: style.transitionTimingFunction,
        font: style.fontFamily,
      };
    };
    const results = [];
    for (const item of input) {
      const row = { id: item.id, input: item.input, versions: {} };
      for (const version of ['raw', 'r3', 'r4', ...(withSdk ? ['sdk'] : [])]) {
        if (version !== 'raw' && version !== 'sdk' && item[version].error) {
          row.versions[version] = item[version];
          continue;
        }
        const root = document.createElement('div');
        root.id = `probe-${item.id}-${version}`;
        document.querySelector('main').append(root);
        let container = root, sheet, css, owner;
        if (version === 'sdk') {
          window.__utility_lab.mount(root.id, 10000, item.input);
          await window.__utility_lab.flush(root.id);
          container = document.getElementById(root.id + '-frame');
          const report = await window.__utility_lab.ensure(root.id, []);
          css = report.css;
          owner = root.dataset.rustifyStyleOwner;
        } else {
          css = version === 'raw' ? item.input.replaceAll(':root', '#' + root.id) : item[version].css;
          if (version !== 'raw') root.dataset.rustifyStyleOwner = owner = 'review-r4';
          sheet = new CSSStyleSheet();
          sheet.replaceSync(css);
          document.adoptedStyleSheets = [...document.adoptedStyleSheets, sheet];
        }
        const elements = {};
        for (const className of ['motion', 'plain', 'other']) {
          const child = document.createElement('div');
          child.className = className;
          container.append(child);
          elements[className] = child;
        }
        if (item.id === 'public-inline-consumer') elements.plain.style.transitionTimingFunction = 'var(--v)';
        const before = Object.fromEntries(Object.entries(elements).map(([name, element]) => [name, read(element)]));
        if (item.id === 'outside-override') elements.plain.style.setProperty('--name', 'serif');
        if (item.id === 'multi-inherit' || item.id === 'alias-mixed') elements.other.style.setProperty('--v', 'ease-in');
        if (item.id === 'external-disable-name') elements.other.style.setProperty('--v', 'none');
        if (item.id === 'external-disable-cyclic') root.style.setProperty('--v', 'none');
        const after = Object.fromEntries(Object.entries(elements).map(([name, element]) => [name, read(element)]));
        row.versions[version] = { owner, css, before, after };
        if (version === 'sdk') window.__utility_lab.dispose(root.id);
        else document.adoptedStyleSheets = document.adoptedStyleSheets.filter(value => value !== sheet);
        root.remove();
      }
      results.push(row);
    }
    return results;
  }, { input, withSdk: Boolean(url) });
  const manifests = url ? await page.evaluate(async () => ({
    main: await (await fetch('build-manifest.json')).json(),
    worker: await (await fetch('utility/build-manifest.json')).json(),
  })) : null;
  const csp = await page.evaluate(() => window.__r4Csp ?? []);
  writeFileSync(output, JSON.stringify({ url, browser: browser.version(), cases: rows, manifests, errors, requests, csp }, null, 2) + '\n');
  console.log(JSON.stringify({ browser: browser.version(), cases: rows.map(row => ({
    id: row.id,
    versions: Object.fromEntries(Object.entries(row.versions).map(([name, value]) => [name, value.error ? value : {
      motion: value.after.motion, plain: value.after.plain, other: value.after.other,
    }])),
  })), errors, requests }, null, 2));
} finally { await browser.close(); }
