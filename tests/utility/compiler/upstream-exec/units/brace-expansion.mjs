import { facade, viaRust } from '../facade.mjs'

export const expand = viaRust(pattern => facade.unitExpand(pattern))
