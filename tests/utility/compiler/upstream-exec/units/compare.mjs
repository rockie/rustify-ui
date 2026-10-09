import { facade, viaRust } from '../facade.mjs'

export const compare = viaRust((a, z) => facade.unitCompare(a, z))
