// Headed Chrome, real GPU: the utility runtime as an application sees it, timed
// inside the page. Needs a compiler-only release utility-lab build and a built
// xtask (`target/debug/xtask`); measures, applies no threshold.
// Usage: node tests/utility/bench-utility-lab.mjs --out <file.json>
import assert from 'node:assert/strict'
import { spawn } from 'node:child_process'
import { mkdir, readFile, writeFile } from 'node:fs/promises'
import { dirname, resolve } from 'node:path'
import { chromium } from '@playwright/test'

const output = process.argv.includes('--out') ? process.argv[process.argv.indexOf('--out') + 1] : 'target/utility-lab-benchmark.json'
const child = spawn('target/debug/xtask', ['serve', '--example', 'utility-lab', '--release', '--port', '0'], { stdio: ['ignore', 'pipe', 'pipe'] })
let browser
const url = await new Promise((resolveUrl, reject) => {
  const timer = setTimeout(() => reject(new Error('utility-lab server startup deadline')), 10000)
  child.stdout.on('data', chunk => { const match = String(chunk).match(/http:\/\/\S+/); if (match) { clearTimeout(timer); resolveUrl(match[0]) } })
  child.once('exit', code => { clearTimeout(timer); reject(new Error(`utility-lab server exited ${code}`)) })
})
const quantiles = values => {
  const ordered = [...values].sort((a, b) => a - b)
  const at = p => ordered[Math.ceil(p * ordered.length) - 1]
  return { samples: values.length, p50Ms: at(0.5), p95Ms: at(0.95), maxMs: ordered.at(-1) }
}
try {
  browser = await chromium.launch({ channel: 'chrome', headless: false })
  const page = await browser.newPage({ viewport: { width: 1280, height: 1000 } })
  const errors = []
  page.on('pageerror', error => errors.push(error.message))
  await page.goto(url)
  await page.waitForFunction(() => document.getElementById('status')?.dataset.status === 'ready', null, { timeout: 120000 })
  await page.waitForFunction(() => window.__utility_lab.gpuReport('scope-a')?.button.drawn_revision === 0, null, { timeout: 120000 })
  const manifest = await page.evaluate(() => fetch('./utility/build-manifest.json').then(response => response.json()))
  assert.equal(manifest.scanner_enabled, false, 'benchmark the default compiler-only product')
  const measured = await page.evaluate(async () => {
    const api = window.__utility_lab
    const canvas = document.querySelector('[data-testid="utility-gpu"]')
    const gl = canvas.getContext('webgl2')
    const info = gl.getExtension('WEBGL_debug_renderer_info')
    const renderer = info ? gl.getParameter(info.UNMASKED_RENDERER_WEBGL) : gl.getParameter(gl.RENDERER)
    if (/swiftshader|llvmpipe|software/i.test(renderer)) throw new Error(`Real GPU required: ${renderer}`)
    // Main-thread CSSOM work, timed where the SDK host performs it.
    const cssom = []
    const replaceSync = CSSStyleSheet.prototype.replaceSync
    const adopted = Object.getOwnPropertyDescriptor(Document.prototype, 'adoptedStyleSheets')
    CSSStyleSheet.prototype.replaceSync = function (text) { const start = performance.now(); try { return replaceSync.call(this, text) } finally { cssom.push({ phase: 'replaceSync', ms: performance.now() - start, bytes: text.length }) } }
    Object.defineProperty(document, 'adoptedStyleSheets', { configurable: true, get() { return adopted.get.call(this) }, set(value) { const start = performance.now(); adopted.set.call(this, value); cssom.push({ phase: 'adopt', ms: performance.now() - start }) } })
    const fresh = [], repeat = []
    for (let i = 0; i < 60; i++) {
      const start = performance.now()
      const report = await api.ensure('scope-a', [`w-[${300 + i}px]`])
      fresh.push(performance.now() - start)
      if (!report.css.includes(`${300 + i}px`)) throw new Error('ensure did not install the new rule')
    }
    const before = api.report('scope-a').revision
    for (let i = 0; i < 60; i++) {
      const start = performance.now()
      await api.ensure('scope-a', [`w-[${300 + i}px]`])
      repeat.push(performance.now() - start)
    }
    if (api.report('scope-a').revision !== before) throw new Error('repeated ensure changed the revision')
    CSSStyleSheet.prototype.replaceSync = replaceSync
    delete document.adoptedStyleSheets
    // GPU: from a new utility revision to the frame that drew it.
    const draws = [], frames = []
    let previous = null
    const frame = async () => {
      const now = await new Promise(resolve => requestAnimationFrame(resolve))
      if (previous !== null) frames.push(now - previous)
      previous = now
    }
    for (let i = 0; i < 400 && (i < 40 || frames.length < 180); i++) {
      const start = performance.now()
      const { revision } = await api.gpuApply('scope-a', [[], [`w-[${140 + (i % 2) * 20}px]`, 'h-[48px]', 'bg-[#123456]', 'ring-[3px]', 'ring-[#abcdef]', 'shadow-[0_4px_0_0_#234567]'], [], []])
      for (let wait = 0; api.gpuReport('scope-a')?.button.drawn_revision !== revision; wait++) {
        if (wait > 600) throw new Error(`revision ${revision} not drawn; frames ${frames.length}`)
        await frame()
      }
      draws.push(performance.now() - start)
      // One frame per apply, outside the timed span: the frame cadence while
      // revisions keep arriving.
      await frame()
    }
    return { renderer, applies: draws.length, fresh, repeat, cssom, draws, frames: frames.slice(30), heap: performance.memory ? { usedJSHeapSize: performance.memory.usedJSHeapSize, totalJSHeapSize: performance.memory.totalJSHeapSize } : null, userAgent: navigator.userAgent }
  })
  assert.deepEqual(errors, [])
  const product = 'target/makepad-wasm-app/release/utility-lab'
  const report = {
    recordedAt: new Date().toISOString(), browser: browser.version(), headed: true, renderer: measured.renderer, userAgent: measured.userAgent,
    build: { main: JSON.parse(await readFile(resolve(product, 'build-manifest.json'))).build_id, worker: manifest.build_id, scannerEnabled: manifest.scanner_enabled },
    ensureNewClass: { ...quantiles(measured.fresh), timing: 'api.ensure of one new arbitrary width: queue, Worker prepare, transfer, CSSOM staging/adoption and ack, measured in the page' },
    ensureRepeat: { ...quantiles(measured.repeat), timing: 'api.ensure of an already installed class: the runtime no-op path' },
    cssomReplaceSync: { ...quantiles(measured.cssom.filter(entry => entry.phase === 'replaceSync').map(entry => entry.ms)), maxBytes: Math.max(...measured.cssom.filter(entry => entry.phase === 'replaceSync').map(entry => entry.bytes)) },
    cssomAdopt: quantiles(measured.cssom.filter(entry => entry.phase === 'adopt').map(entry => entry.ms)),
    gpuApplyToDrawn: { ...quantiles(measured.draws), timing: 'gpuApply of a new revision (ensure + resolve + props) until the region reports that revision drawn' },
    frameIntervals: { ...quantiles(measured.frames), applies: measured.applies, timing: 'rAF intervals while the GPU applies run, after 30 warm-up frames' },
    heap: measured.heap,
  }
  await mkdir(dirname(resolve(output)), { recursive: true })
  await writeFile(output, JSON.stringify(report, null, 2) + '\n')
  console.log(JSON.stringify({ ensureNewClass: report.ensureNewClass.p50Ms, ensureRepeat: report.ensureRepeat.p50Ms, replaceSync: report.cssomReplaceSync.p50Ms, adopt: report.cssomAdopt.p50Ms, gpuApplyToDrawn: report.gpuApplyToDrawn.p50Ms, frames: [report.frameIntervals.p50Ms, report.frameIntervals.p95Ms] }))
} finally {
  await browser?.close()
  child.kill()
}
