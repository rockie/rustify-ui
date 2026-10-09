import { facade, viaRust } from '../facade.mjs'

export const escape = viaRust(value => facade.unitEscape(value))
export const unescape = viaRust(escaped => facade.unitUnescape(escaped))
