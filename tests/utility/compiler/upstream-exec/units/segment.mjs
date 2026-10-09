import { facade, viaRust } from '../facade.mjs'

export const segment = viaRust((input, separator) => facade.unitSegment(input, separator))
