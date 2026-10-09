import { crossed, facade } from '../facade.mjs'

export * from 'rustify-foreign:../../../../../ref/tailwindcss/packages/tailwindcss/src/source-maps/source-map.ts'
export function createTranslationMap({ original, generated }) {
  crossed()
  let map = new facade.UnitTranslationMap(original, generated)
  return node => {
    crossed()
    let span = location => (location ? [location[1], location[2]] : undefined)
    return map.translate(span(node.src), span(node.dst))
  }
}
