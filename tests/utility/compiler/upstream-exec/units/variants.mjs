// `Variants` records registrations; Rust applies upstream's defaults for
// omitted `compounds` and `compoundsWith`.
import { facade, viaRust } from '../facade.mjs'

export * from 'rustify-foreign:../../../../../ref/tailwindcss/packages/tailwindcss/src/variants.ts'
export const compoundsForSelectors = viaRust(selectors => facade.unitCompoundsForSelectors(selectors))
export class Variants {
  entries = []
  static(name, _applyFn, { compounds } = {}) {
    this.entries.push({ name, kind: 'static', compounds })
  }
  functional(name, _applyFn, { compounds } = {}) {
    this.entries.push({ name, kind: 'functional', compounds })
  }
  compound(name, compoundsWith, _applyFn, { compounds } = {}) {
    this.entries.push({ name, kind: 'compound', compounds, compoundsWith })
  }
}
