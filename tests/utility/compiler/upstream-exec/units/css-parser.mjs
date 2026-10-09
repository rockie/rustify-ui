import { facade, viaRust } from '../facade.mjs'

export const parse = viaRust((input, opts) => facade.unitCssParse(input, opts?.from))
