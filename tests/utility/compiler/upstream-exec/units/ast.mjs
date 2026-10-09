// Printing and optimization run in Rust; node constructors and `walk` stay
// upstream JavaScript and mark a test that calls them as mixed.
import { crossed, facade, viaRust } from '../facade.mjs'

export * from 'rustify-foreign:../../../../../ref/tailwindcss/packages/tailwindcss/src/ast.ts'
export const toCss = viaRust((ast, track) => facade.unitToCss(ast, track))
export function optimizeAst(ast, designSystem, polyfills) {
  // The Rust passes take no theme; upstream's tests pass an empty one.
  if (designSystem?.theme?.css) throw new Error('optimizeAst shim requires an empty theme')
  crossed()
  return facade.unitOptimizeAst(ast, polyfills)
}
