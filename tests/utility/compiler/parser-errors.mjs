import assert from 'node:assert/strict'
import { createHash } from 'node:crypto'
import { readFile, writeFile } from 'node:fs/promises'
import { createRequire } from 'node:module'
import { dirname, resolve } from 'node:path'
import { fileURLToPath } from 'node:url'

const directory = dirname(fileURLToPath(import.meta.url))
const oracle = createRequire(resolve(directory, 'oracle/package.json'))
const { compile } = oracle('tailwindcss')
const { createJiti } = oracle('jiti')
assert.equal(oracle('tailwindcss/package.json').version, '4.1.13')
const sha256 = value => createHash('sha256').update(value).digest('hex')
const upstream = resolve(directory, '../../../ref/tailwindcss/packages/tailwindcss')
const baseline = ['w-[13px]', 'bg-[#123456]', 'hover:w-[17px]!']
const signature = (input, candidates) => JSON.stringify([input, candidates])
const css = String.raw

export async function parserErrorFixtures() {
  const parser = await createJiti(import.meta.url, { moduleCache:false, fsCache:false }).import(resolve(upstream, 'src/css-parser.ts'))
  const cases = [], reusedCases = [], signatures = new Set(), referenced = new Set(), existing = new Map()
  const fixtureFiles = ['functions-fixtures.json', 'arbitrary-properties-fixtures.json', 'typography-fixtures.json', 'effects-complete-fixtures.json', 'custom-utilities-fixtures.json', 'custom-variants-fixtures.json', 'theme-fixtures.json', 'imports-fixtures.json']
  for (const fixture of fixtureFiles) {
    const text = await readFile(resolve(directory, fixture), 'utf8')
    const fixtureSha256 = sha256(text)
    for (const item of JSON.parse(text).cases) if (!item.assets && !item.buildSteps) {
      const key = signature(item.input, item.candidates)
      if (!existing.has(key)) existing.set(key, { fixture, fixtureSha256, item })
    }
  }
  function scalar(value) {
    for (const character of value) assert.ok(character.codePointAt(0) < 0xd800 || character.codePointAt(0) > 0xdfff, 'lone UTF-16 belongs to the separate text migration')
  }
  async function result(input, candidates) {
    scalar(input); candidates.forEach(scalar)
    assert.ok(input.includes('@tailwind utilities'), 'each parser fixture must include utilities')
    let phase = 'compile'
    try {
      const compiled = await compile(input)
      phase = 'build'
      const expectedCss = compiled.build(candidates).replace(/^\/\*! tailwindcss[^\n]*\n/, '')
      return { expectedCss, expectedCssSha256:sha256(expectedCss) }
    } catch (error) {
      let expectedErrorSourcePhase = phase
      if (phase === 'compile') {
        try { parser.parse(input) } catch (parseError) {
          assert.equal(parseError.name, error.name); assert.equal(parseError.message, error.message)
          expectedErrorSourcePhase = 'css-parser'
        }
      }
      return { expectedError:error.message, expectedErrorName:error.name, expectedErrorPhase:phase, expectedErrorSourcePhase }
    }
  }
  async function reuse(reference, reason) {
    const key = `${reference.fixture}:${reference.item.id}`
    if (referenced.has(key)) return
    referenced.add(key)
    const actual = await result(reference.item.input, reference.item.candidates)
    if (Object.hasOwn(reference.item, 'expectedCss')) assert.equal(actual.expectedCss, reference.item.expectedCss, key)
    else assert.equal(actual.expectedError, reference.item.expectedError, key)
    reusedCases.push({ fixture:reference.fixture, id:reference.item.id, fixtureSha256:reference.fixtureSha256, reason })
  }
  async function add(id, input, candidates = baseline) {
    const key = signature(input, candidates)
    if (signatures.has(key)) return
    signatures.add(key)
    const reference = existing.get(key)
    if (reference) { await reuse(reference, `exact input/candidate duplicate of parser-errors:${id}`); return }
    cases.push({ id:`parser-errors:${id}`, input, candidates, ...await result(input, candidates) })
  }
  const authored = {
    plain:'.probe { color:red; width:13px; }', rootDeclarations:'color:red; --root:13px;', emptyRules:'.probe {} .outer { .inner {} &:hover {} } @media screen {}', emptyAtRules:'@unknown; @unknown {} @media screen { @unknown {} }', nested:'.probe { color:red; &:hover { width:17px; } @media (width > 13px) { & > p { opacity:.5; } } }',
    normalComments:'/* root */ .probe/* selector */.child { color:/* before */red/* after */; /* end */ }', commentWhitespace:'.one/**/.two { color:red; } .three/**/ /**/.four { color:blue; }', escapedComment:css`/* escaped \*\/ ending */ .probe { color:red; }`, licenseHoist:'/*! first */ .probe { color:red; /*! nested */ } /*! last */', licenseLikeTailwind:'/*! tailwindcss authored-comment */ .probe { color:red; }', customComments:'.probe { --raw: a/* inside */b; --license: a/*! retained */b; color:a/* removed */b; }',
    doubleQuote:css`.probe { content:"a;:{}[]()'b"; }`, singleQuote:css`.probe { content:'a;:{}[]()"b'; }`, escapedQuote:css`.probe { content:"a\"b"; --raw:'a\'b'; }`, escapedDelimiters:css`.probe { value:a\;b\}c\(d\)e; }`, escapedSelector:css`.a\:b\[c\]\{d\} { color:red; }`, escapedHex:css`.\31 23 { content:"\1f642 "; --raw:\0000e9 ; }`, escapedNewline:'.probe { content:"first\\\nsecond"; value:first\\\nsecond; }',
    customBlocks:'.probe { --empty:{}; --object:{"x":{"y":";"}}; --array:[a;b]; --function:foo(a;b); --tokens: [a {b:c}] ; }', customWhitespace:'.probe { --raw: first  second\n\tthird; color:first  second\n\tthird; }', customNoSemicolon:'.probe { --raw:13px }', propertyCase:'.probe { COLOR:RED; color:Blue; --Case:VALUE; --case:value; }', duplicateDeclaration:'.probe { color:red; color:blue; --raw:one; --raw:two; }',
    emptyDeclarations:'.probe { :; :red; color:; --raw:; --:; }', repeatedSemicolons:'.probe { ;;;color:red;;;;width:13px;;; }', colonValues:'.probe { value:a:b:c; --raw:a:b:c; }', lastNoSemicolon:'.probe { color:red }', rootLastNoSemicolon:'color:red',
    important:'.probe { color:red!important; width:13px !important /* tail */; --raw:17px !important; }', importantCase:'.probe { color:red !IMPORTANT; width:13px !Important; opacity:.5 ! important; }', importantSuffix:'.probe { color:red !important-more; width:13px !important ignored; opacity:.5!important!important; }', importantQuoted:'.probe { content:"a!important b"; --raw:"a!important b"; value:foo(!important); }', importantComments:'.probe { color:red !/**/important; --raw:red !/**/important; }',
    unknownAtRule:'@unknown thing; @unknown thing { .probe { value:13px; } }', shortAtRule:'@x a; @x (a:b) { .probe { value:13px; } } @ab cd ef;', atRuleCase:'@MEDIA screen { .probe { color:red; } } @media screen { .probe { color:blue; } }', mediaFunction:'@media (width > calc(1px + 2px)) { .probe { color:red; } }', selectorQuotes:'.probe[data-text="};{()"][data-other=\'x;y\'] { color:red; }', selectorComma:'.one:is(.a,.b), .two:not([a="x,y"]) { color:red; }', bareAmp:'.probe { & { color:red; } & > p { width:13px; } }',
    missingOpeningBrace:'.probe color:red; }', extraClosingBrace:'.probe { color:red; }}', missingClosingBrace:'.probe { color:red;', nestedMissingClosingBrace:'.outer { .inner { color:red; }', atRuleMissingClosingBrace:'@media screen { .probe { color:red; }', missingOpeningParen:'.probe { value:foo); }', missingClosingParen:'.probe { value:foo(13px; }', rootMissingClosingParen:'value:foo(13px;', squareUnbalanced:'.probe[a { color:red; }', rootMissingClosingSquare:'[data-foo', missingColon:'.probe { red; }', noSemicolonMissingColon:'.probe { red }', customMissingColon:'.probe { --raw; }', customNoValueNoColon:'.probe { --raw }', rootCustomMissingColon:'--raw;',
    quoteEof:'.probe { content:"unfinished', quoteBeforeBrace:'.probe { content:"unfinished }', quoteNewline:'.probe { content:"unfinished\nnext"; }', quoteSemicolonNewline:'.probe { content:"unfinished;\nnext"; }', singleQuoteNewline:'.probe { content:\'unfinished\nnext\'; }', quoteCarriageReturn:'.probe { content:"first\rsecond"; }', quoteEscapedEof:'.probe { content:"unfinished\\',
    unclosedComment:'.probe { color:red; /* unfinished', rootUnclosedComment:'/* unfinished', commentEndsAtEof:'/* finished */', unclosedLicense:'/*! unfinished', customUnclosedComment:'.probe { --raw:first/* unfinished', customUnclosedParen:'.probe { --raw:foo(13px; }', customMismatchedBrackets:'.probe { --raw:[foo); }', customBraces:'--raw: { color:red; };',
    rootTrailingEscape:'trailing\\', declarationTrailingEscape:'.probe { value:a\\', escapedClose:'.probe { value:a\\}', leadingBom:'\ufeff.probe { color:red; }', interiorBom:'.probe { value:a\ufeffb; }', scalarUnicode:'.雪🙂e\u0301 { content:"é雪🙂𝄞"; --雪:🙂; --raw:e\u0301; }', unicodeWhitespace:'.probe\u00a0.child { value:a\u00a0b\u2003c; }', nul:'.probe { value:a\u0000b; --raw:a\u0000b; }', separators:'.probe { value:foo(a:b,c=d > e < f / g); --raw:foo(   a  b  ); }', customEmbeddedLanguage:'.probe { --raw:if(x > 5) this.width = 10; }',
  }
  for (const [name, text] of Object.entries(authored)) for (const endings of ['LF','CRLF']) for (const built of [false,true]) {
    const input = `@tailwind utilities;\n${text}`.replaceAll('\r\n', '\n').replaceAll('\n', endings === 'CRLF' ? '\r\n' : '\n')
    await add(`authored:${name}:${endings}:${built ? 'candidates' : 'empty-build'}`, input, built ? baseline : [])
  }
  await add('file-leading-bom', '\ufeff@tailwind utilities;\n.probe { color:red; }')
  for (const text of ['.probe { --raw:13px', '--raw:13px', '.probe { --raw:"unfinished\nnext"; }', '.probe { --raw:"unfinished;\nnext"; }']) await add(`custom-eof:${JSON.stringify(text)}`, `@tailwind utilities;\n${text}`)
  // Probe the parser's exact declaration splitting, not browser CSS validation.
  const properties = ['value', '--raw', '', '-', '--', 'COLOR', 'a b', 'é', '雪', css`foo\:bar`]
  const values = ['', ' ', 'red', ':', 'a:b', 'a;b', '(a;b)', '[a;b]', '{a;b}', 'foo(13px)', 'foo(13px', 'foo(13px))', '"a!important b"', 'red !IMPORTANT', 'red !important ignored', 'a/* comment */b', 'a\\;b', '🙂']
  for (const property of properties) for (const value of values) for (const end of [';', '']) await add(`declaration:${property || '<empty>'}:${JSON.stringify(value)}:${end || 'no-semicolon'}`, `@tailwind utilities;\n.probe { ${property}:${value}${end} }`)
  const fragments = ['', '_', '__', '13px', '13px;', '13px}', '{color:red}', '[13px]', '[13px', '13px]', '(13px)', '(13px', '13px)', 'calc(1px+2px)', 'calc(1px+2px', 'calc((1px+2px)', 'calc(1px+2px))', 'var(--probe)', 'var(--probe', 'foo(', 'foo(bar', 'foo(bar)', 'foo(bar))', 'foo([a)', 'foo([a]', 'foo(a;b)', 'foo(a;', 'foo({a;b})', 'foo(a\\', 'foo("unfinished', "foo('unfinished", 'url(a_b.svg)', 'url("a_b.svg")', '"unterminated', "'unterminated", '"[({;}"', '"x\\"y"', 'a\\;b', 'a\\}b', 'a\\', 'length:13px', 'length:', 'Color:red', 'color:red', ':red', 'nonsense:foo', 'a_b', 'a\\_b', '🙂', '雪', 'e\u0301', '\u0000']
  for (const fragment of fragments) for (const candidate of [`[value:${fragment}]`, `w-[${fragment}]`, `bg-[${fragment}]`, `content-[${fragment}]`]) for (const decorated of [candidate, `${candidate}!`, `hover:${candidate}`]) await add(`candidate:${decorated}`, '@tailwind utilities;', [decorated])
  const shorthand = ['', '--probe', '--my_color', 'probe', '_', 'length:--probe', 'color:--probe', 'length:', ':--probe', '--probe;', '--probe}', '--probe(', '--probe)', '--probe\\', '--雪', '--🙂']
  for (const value of shorthand) for (const root of ['w','bg','content']) for (const close of [')','',']]']) await add(`shorthand:${root}:${value}:${close || 'unclosed'}`, '@tailwind utilities;', [`${root}-(${value}${close}`])
  const candidates = ['', ' ', '\n', '\t', '\u0000', '!', '!!block', 'block!!', '!block!', 'hover:', ':block', 'hover::block', 'block/', 'block//', 'bg-[red]/50/foo', '[value:red]/[]', '[value:red]/()', '[value:red]/[;]', '[value:red]/(bad)', '[value:red]/[foo(', '[value:red]/(--probe', '[value:red', '[value:red]]', '[value:]', '[:red]', '[1value:red]', '[VALUE:red]', '[value:red;value:blue]', '[value:red}html{value:blue]', '[&{color:red}]:block', '[&]:', '[]:block', '[_]:block', '[@media(width>13px)]:block', '[&:is(.a,.b)]:block', '[&:is(.a,.b]:block', 'data-[]:block', 'data-():block', 'group-data-[]:block', 'data-foo[bar]:block', 'data-[x="};{()"]:block', 'unknown:block', 'tw:block', 'hover:tw:block']
  for (const candidate of candidates) await add(`syntax:${JSON.stringify(candidate)}`, '@tailwind utilities;', [candidate])
  // All ASCII code points at syntax-sensitive boundaries, plus scalar Unicode.
  const characters = [...Array.from({length:128}, (_, index) => String.fromCharCode(index)), 'é','雪','🙂','𝄞','\u0301','\u00a0','\u2003','\u2028','\u2029','\ufeff']
  for (const character of characters) for (const candidate of [`[${character}value:red]`, `[value:a${character}b]`, `w-[a${character}b]`, `[&${character}p]:block`, `data-[x${character}y]:block`]) await add(`character:U+${character.codePointAt(0).toString(16)}:${candidate}`, '@tailwind utilities;', [candidate])
  for (const candidate of ['[value:foo(13px]', '[value:foo(13px))]', 'w-[length:var(--probe]', 'bg-(color:--probe)', '[&:is(.a,.b]:block', '[value:"unfinished]']) for (const mode of ['base','inline','reference','prefix']) {
    const theme = `@theme ${mode === 'base' ? '' : mode === 'prefix' ? 'prefix(tw)' : mode} { --probe:13px; --color-brand:#123456; }`
    await add(`theme:${mode}:${candidate}`, `${theme} @tailwind utilities;\n.probe { color:var(--color-brand); value:foo(13px); }`, [mode === 'prefix' ? `tw:${candidate}` : candidate])
  }
  await add('union-forward', '@tailwind utilities;\n.probe { --raw: a/*keep*/b; color:red !IMPORTANT; }', ['[value:foo(13px]', 'w-[13px]', 'hover:w-[17px]!', '[value:foo(13px))]', 'w-[13px]', '[value:red;value:blue]'])
  await add('union-reverse', '@tailwind utilities;\n.probe { --raw: a/*keep*/b; color:red !IMPORTANT; }', ['[value:red;value:blue]', 'w-[13px]', '[value:foo(13px))]', 'hover:w-[17px]!', 'w-[13px]', '[value:foo(13px]'])
  for (const [fixture, id, reason] of [
    ['functions-fixtures.json','functions:quoted-function-text','quoted Tailwind function text is already frozen'],
    ['functions-fixtures.json','functions:invalid-candidate:--alpha()','candidate Tailwind function argument failure is already frozen'],
    ['theme-fixtures.json','theme:inline-license','theme inline license comment preservation is already frozen'],
    ['theme-fixtures.json','theme:root-license','root license comment is already frozen with theme output'],
    ['custom-utilities-fixtures.json','custom:behavior:emptyValue:empty:2','empty custom declaration inside a utility template is already frozen'],
  ]) {
    const reference = [...existing.values()].find(value => value.fixture === fixture && value.item.id === id)
    assert.ok(reference, `missing existing coverage ${fixture}:${id}`); await reuse(reference, reason)
  }
  const ids = new Set(); for (const item of cases) { assert.ok(!ids.has(item.id), `duplicate ${item.id}`); ids.add(item.id) }
  const paths = ['src/css-parser.ts','src/css-parser.test.ts','src/value-parser.ts','src/value-parser.test.ts','src/index.ts','src/index.test.ts','src/ast.ts','src/ast.test.ts','src/candidate.ts','src/candidate.test.ts','src/compile.ts','src/utils/segment.ts','src/utils/decode-arbitrary-value.ts','src/utils/is-valid-arbitrary.ts']
  const sources = await Promise.all(paths.map(async path => ({ path:`packages/tailwindcss/${path}`, sha256:sha256(await readFile(resolve(upstream, path))) })))
  return { schemaVersion:1, sourceCommit:'1334c99db8fd26c8ea065375dd9259800863f072', oracleVersion:'4.1.13', fullFamilyConformance:'pending', errorPhases:['compile','build'], errorSourcePhases:['css-parser','compile','build'], excludedBoundaries:['lone UTF-16 code units: separate text migration','JavaScript plugin/config execution','@apply: separate directive family'], sources, reusedCases, cases }
}
function counts(actual) {
  return { cases:actual.cases.length, cssCases:actual.cases.filter(item => Object.hasOwn(item,'expectedCss')).length, errorCases:actual.cases.filter(item => Object.hasOwn(item,'expectedError')).length, reusedCases:actual.reusedCases.length, errorSourcePhases:Object.fromEntries(actual.errorSourcePhases.map(phase => [phase, actual.cases.filter(item => item.expectedErrorSourcePhase === phase).length])) }
}
export async function checkParserErrorFixtures() {
  const actual = await parserErrorFixtures()
  assert.deepEqual(actual, JSON.parse(await readFile(resolve(directory, 'parser-errors-fixtures.json'), 'utf8')))
  return counts(actual)
}
if (process.argv[1] && resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  if (process.argv.includes('--write')) { const actual=await parserErrorFixtures(); await writeFile(resolve(directory, 'parser-errors-fixtures.json'), JSON.stringify(actual,null,2)+'\n'); console.log(JSON.stringify(counts(actual))) }
  else console.log(JSON.stringify(await checkParserErrorFixtures()))
}
