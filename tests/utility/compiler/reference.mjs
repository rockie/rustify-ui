import assert from 'node:assert/strict'
import { createHash } from 'node:crypto'
import { readFile, writeFile } from 'node:fs/promises'
import { createRequire } from 'node:module'
import { dirname, resolve } from 'node:path'
import { fileURLToPath } from 'node:url'

const directory = dirname(fileURLToPath(import.meta.url))
const requireOracle = createRequire(resolve(directory, 'oracle/package.json'))
const tailwind = requireOracle('tailwindcss')
const packageJson = requireOracle('tailwindcss/package.json')
assert.equal(packageJson.version, '4.1.13')
const digest = value => createHash('sha256').update(value).digest('hex')
const withoutBanner = css => css.replace(/^\/\*! tailwindcss[^\n]*\n/, '')

export async function compilerFixtures() {
  const inputs = [
    { name: 'compiler-smoke', css: '@tailwind utilities;', candidates: ['w-[137px]', 'bg-[#123456]'], invalidCandidates: ['not-a-tailwind-utility'], computed: { width: '137px', backgroundColor: 'rgb(18, 52, 86)' } },
    { name: 'css-scope-property-nesting', css: '@scope ([data-utility-probe]) { @tailwind utilities; }', candidates: ['w-[137px]', 'bg-[#123456]', 'ring-[3px]', 'before:content-["probe"]'], invalidCandidates: [], computed: { width: '137px', backgroundColor: 'rgb(18, 52, 86)', beforeContent: '"probe"', boxShadowIncludes: '3px' } },
  ]
  const cases = []
  for (const input of inputs) {
    const compiler = await tailwind.compile(input.css)
    const expectedCss = withoutBanner(compiler.build([...input.candidates, ...input.invalidCandidates]))
    const stable = withoutBanner(compiler.build([...input.candidates].reverse()))
    assert.equal(expectedCss, stable, `${input.name}: repeated candidates changed CSS`)
    const fresh = await tailwind.compile(input.css)
    assert.equal(withoutBanner(fresh.build([...input.candidates].reverse())), expectedCss, `${input.name}: input order changed CSS`)
    const design = await tailwind.__unstable__loadDesignSystem(input.css)
    const candidateCss = design.candidatesToCss(input.candidates)
    for (const value of candidateCss) assert.notEqual(value, null)
    assert.deepEqual(design.candidatesToCss(input.invalidCandidates), input.invalidCandidates.map(() => null))
    cases.push({ ...input, expectedCss, expectedCssSha256: digest(expectedCss), candidateCss, sources: compiler.sources, root: compiler.root, features: compiler.features, implementationStatus: 'pending', browserStatus: 'pending' })
  }
  return { schemaVersion: 1, oracle: { package: 'tailwindcss', version: '4.1.13', sourceCommit: '1334c99db8fd26c8ea065375dd9259800863f072', options: {}, normalization: 'remove generated license banner only; preserve every declaration, rule, duplicate and order' }, cases }
}

export async function checkCompilerFixtures() {
  const actual = await compilerFixtures()
  const frozen = JSON.parse(await readFile(resolve(directory, 'fixtures.json'), 'utf8'))
  assert.deepEqual(actual, frozen)
  for (const entry of actual.cases) {
    assert.equal(await readFile(resolve(directory, `${entry.name}.css`), 'utf8'), entry.expectedCss)
  }
  return { cases: actual.cases.length, oracleVersion: packageJson.version }
}

if (process.argv[1] && resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  if (process.argv.includes('--write')) {
    const fixtures = await compilerFixtures()
    await writeFile(resolve(directory, 'fixtures.json'), `${JSON.stringify(fixtures, null, 2)}\n`)
    for (const entry of fixtures.cases) await writeFile(resolve(directory, `${entry.name}.css`), entry.expectedCss)
    console.log(`Wrote ${fixtures.cases.length} official compiler fixtures`)
  } else {
    console.log(JSON.stringify(await checkCompilerFixtures()))
  }
}
