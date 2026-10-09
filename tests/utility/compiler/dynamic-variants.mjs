import assert from 'node:assert/strict'
import { readFile, writeFile } from 'node:fs/promises'
import { createRequire } from 'node:module'
import { fileURLToPath } from 'node:url'
import { dirname, resolve } from 'node:path'

const directory = dirname(fileURLToPath(import.meta.url))
const oracle = createRequire(resolve(directory, 'oracle/package.json'))
const tailwind = oracle('tailwindcss')
assert.equal(oracle('tailwindcss/package.json').version, '4.1.13')
const base = '@tailwind utilities;'
const themed = '@theme { --breakpoint-tiny: 20rem; --breakpoint-sm: 40rem; --breakpoint-md: 48rem; --breakpoint-lg: 64rem; --breakpoint-px: 800px; --breakpoint-fn: calc(100rem - 1px); --breakpoint-variable: var(--screen); --container-sm: 24rem; --container-lg: 32rem; }\n@tailwind utilities;'
const stripBanner = css => css.replace(/^\/\*! tailwindcss[^\n]*\n/, '')

export async function dynamicVariantFixtures() {
  const cases = []
  async function add(input, candidates, id = candidates.join(' ')) {
    const compiler = await tailwind.compile(input)
    cases.push({ id: `dynamic-variant:${id}`, input, candidates, expectedCss: stripBanner(compiler.build(candidates)) })
  }
  const design = await tailwind.__unstable__loadDesignSystem(base)
  const staticRoots = Array.from(design.variants.entries()).filter(([,item]) => item.kind === 'static').map(([root]) => root)
  const children = Array.from(new Set([...staticRoots, 'aria-checked', 'aria-[sort=ascending]', 'data-[state=open]', 'nth-2', 'nth-[2n+1]', '[&>p]', '[p]', '[>p]', 'has-[>p]', 'not-hover', 'group-hover', 'peer-focus']))
  for (const compound of ['not', 'group', 'peer', 'in', 'has']) {
    const candidates = []
    for (const child of children) {
      const candidate = `${compound}-${child}:flex`
      candidates.push(candidate)
      await add(base, [candidate])
    }
    if (compound === 'group' || compound === 'peer') {
      for (const child of ['hover', 'open', 'data-[state=open]', '[&:is(:hover,:focus)]']) {
        const candidate = `${compound}-${child}/card:flex`
        candidates.push(candidate)
        await add(base, [candidate])
      }
    } else await add(base, [`${compound}-hover/card:flex`])
    await add(base, candidates, `${compound}:union`)
  }
  const functional = ['aria-checked', 'aria-anything', 'aria-[sort=ascending]', 'aria-[label=Hello_world]', 'aria-[label="Hello_world"]', 'aria-[label=Hello_i]', 'data-state', 'data-[state=open]', 'data-[value~=Hello_world_s]', 'data-[value="hello"]', 'nth-0', 'nth-2', 'nth-02', 'nth-2.5', 'nth--1', 'nth-[2n+1]', 'nth-last-3', 'nth-of-type-4', 'nth-last-of-type-[odd]', 'supports-grid', 'supports-[display]', 'supports-[display:grid]', 'supports-[(display:grid)_and_(color:red)]', 'supports-[selector(:has(*))]', 'supports-[not(display:grid)]', 'supports-[font-tech(color-COLRv1)]', 'supports-[--custom]', 'aria', 'data', 'nth', 'supports', 'aria-checked/foo', 'data-[state=open]/foo', 'nth-2/foo', 'supports-grid/foo']
  for (const variant of functional) await add(base, [`${variant}:flex`])
  await add(base, functional.map(variant => `${variant}:flex`), 'functional:union')
  const boundaries = ['aria-(--state)', 'data-(--state)', 'nth-(--count)', 'supports-(--condition)', 'group-hover/[card_name]', 'peer-hover/[card_name]', 'group-hover/(--name)', 'group-hover/[]', 'group-hover/[;]', 'group-hover/(bad)', 'group-hover/[😀]', 'group-hover/[]', 'data-[😀]', 'data-[]', '[&:is(:hover,:focus)]', '[@media_(width>500px)]', 'group-[@supports_(display:grid)]', 'not-[@media_screen,print]', 'not-not-focus', 'group-not-focus', 'not-not-dark', 'group-has-[&_p]', 'has-group-[&_p]', '@sm/[side_bar]', '@sm/(--name)', '@sm/[]']
  for (const variant of boundaries) await add(themed, [`${variant}:flex`])
  for (const variant of ['[@media(width>500px)]', 'not-[@media(width>500px)]', '[@supports(display:grid)]', 'not-[@supports(display:grid)]']) {
    await add(themed, [`${variant}:flex`])
  }
  await add(themed, boundaries.map(variant => `${variant}:flex`), 'boundary:union')
  const responsive = ['tiny', 'sm', 'md', 'lg', 'px', 'fn', 'variable', 'min-sm', 'min-md', 'max-lg', 'max-sm', 'min-[600px]', 'max-[1200px]', 'min-[40.9rem]', 'min-[40.1rem]', 'max-[40.9rem]', 'max-[40.1rem]', 'min-[calc(100%-1rem)]', 'min-[var(--screen)]', 'max-missing', 'min', 'max', 'sm/foo', 'min-sm/foo', '@sm', '@lg', '@min-sm', '@max-lg', '@[500px]', '@min-[32rem]/card', '@max-[40rem]/panel', '@missing', '@', '@min', '@max', 'sm:hover', 'hover:sm', 'md:max-lg', 'group-sm', 'not-sm', 'not-@sm']
  for (const variant of responsive) await add(themed, [`${variant}:flex`])
  await add(themed, responsive.map(variant => `${variant}:flex`), 'responsive:union')
  await add(base, ['sm:flex', 'max-sm:flex', '@sm:flex'], 'no-theme')
  await add(themed, [...functional, ...children.map(child => `group-${child}`), ...responsive].map(variant => `${variant}:flex`), 'all:union')
  return { schemaVersion: 1, sourceCommit: '1334c99db8fd26c8ea065375dd9259800863f072', oracleVersion: '4.1.13', cases }
}

export async function checkDynamicVariantFixtures() {
  const fixture = await dynamicVariantFixtures()
  assert.deepEqual(fixture, JSON.parse(await readFile(resolve(directory, 'dynamic-variant-fixtures.json'), 'utf8')))
  return { cases: fixture.cases.length }
}
if (process.argv[1] && resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  if (process.argv.includes('--write')) {
    const fixture = await dynamicVariantFixtures()
    await writeFile(resolve(directory, 'dynamic-variant-fixtures.json'), JSON.stringify(fixture, null, 2) + '\n')
    console.log(JSON.stringify({ cases: fixture.cases.length }))
  } else console.log(JSON.stringify(await checkDynamicVariantFixtures()))
}
