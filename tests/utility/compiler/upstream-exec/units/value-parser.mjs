// `walk` stays upstream's JavaScript traversal; calling it marks a test mixed.
import { facade, viaRust } from '../facade.mjs'

export * from 'rustify-foreign:../../../../../ref/tailwindcss/packages/tailwindcss/src/value-parser.ts'
export const parse = viaRust(input => facade.unitValueParse(input))
export const toCss = viaRust(ast => facade.unitValueToCss(ast))
