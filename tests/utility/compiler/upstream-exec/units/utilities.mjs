// `Utilities` records registrations for the Rust candidate parser; the
// compile functions a test registers are never called while parsing.
import { facade, viaRust } from '../facade.mjs'

export * from 'rustify-foreign:../../../../../ref/tailwindcss/packages/tailwindcss/src/utilities.ts'
export const replaceAlpha = viaRust((value, alpha) => facade.unitReplaceAlpha(value, alpha))
export class Utilities {
  roots = { static: [], functional: [] }
  static(name) {
    this.roots.static.push(name)
  }
  functional(name) {
    this.roots.functional.push(name)
  }
}
