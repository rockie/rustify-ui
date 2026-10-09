const { chromium } = require(process.cwd() + '/node_modules/@playwright/test');
const { spawn } = require('node:child_process');
const fs = require('node:fs/promises');
const path = require('node:path');
const { once } = require('node:events');
const evidence = path.resolve('docs/validation/leptos-0.8.22');
const examples = ['fusion-basic', 'component-catalog', 'property-workbench', 'data-workbench', 'vellum', 'theme-studio', 'utility-lab'];
(async () => {
  const browser = await chromium.launch({executablePath: '/Applications/Google Chrome.app/Contents/MacOS/Google Chrome', headless: true, args: ['--enable-unsafe-swiftshader', '--use-angle=swiftshader']});
  const results = [];
  try {
    for (const example of examples) {
      const server = spawn(path.resolve('target/debug/xtask'), ['serve', '--example', example, '--release'], {cwd: process.cwd(), stdio: ['ignore', 'pipe', 'pipe']});
      let stdout = '';
      const url = await new Promise((resolve, reject) => {
        const timeout = setTimeout(() => reject(new Error('Preview startup timeout')), 30000);
        server.stdout.on('data', data => {
          stdout += data;
          const match = stdout.match(/at (http:\/\/127\.0\.0\.1:\d+\/)/);
          if (match) {clearTimeout(timeout); resolve(match[1]);}
        });
        server.once('exit', code => {clearTimeout(timeout);reject(new Error('Preview exited: ' + code));});
      });
      const context = await browser.newContext({viewport: {width: 1440, height: 1000}});
      const page = await context.newPage();
      const errors = [], failed = [], badResponses = [], knownConsole = [];
      page.on('pageerror', error => errors.push(error.message));
      page.on('console', message => {
        if(message.type() !== 'error') return;
        const item = {text: message.text(), location: message.location()};
        if(item.location.url.endsWith('/favicon.ico') && item.text.includes('404')) knownConsole.push(item);
        else errors.push(item);
      });
      page.on('requestfailed', request => failed.push({url: request.url(), failure: request.failure()}));
      page.on('response', response => {if(response.status() >= 400) badResponses.push({url: response.url(), status: response.status()});});
      try {
        await page.goto(url);
        await page.waitForFunction(() => document.querySelector('#status')?.getAttribute('data-status') === 'ready', null, {timeout: 120000});
        await page.evaluate(() => new Promise(resolve => requestAnimationFrame(() => requestAnimationFrame(resolve))));
        const manifest = JSON.parse(await fs.readFile(path.resolve('target/makepad-wasm-app/release', example, 'build-manifest.json'), 'utf8'));
        const state = await page.evaluate(() => ({title: document.title, status: document.querySelector('#status')?.textContent, scopes: document.querySelectorAll('[data-rustify-scope]').length, canvases: document.querySelectorAll('canvas').length}));
        await page.screenshot({path: path.join(evidence, example + '.png')});
        const result = {example, url, buildId: manifest.build_id, browser: browser.version(), state, errors, failed, badResponses, knownConsole};
        results.push(result);
        console.log(JSON.stringify(result));
        if(errors.length || failed.length || badResponses.length) throw new Error('Browser errors for ' + example);
      } finally {
        await context.close();
        const ended = once(server, 'exit'); server.kill('SIGTERM'); await ended;
      }
    }
  } finally {
    await fs.writeFile(path.join(evidence, 'browser-smoke.json'), JSON.stringify(results, null, 2) + '\n');
    await browser.close();
  }
})().catch(error => {console.error(error);process.exitCode=1;});
