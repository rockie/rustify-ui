import { facade, viaRust } from '../facade.mjs'

export const decodeArbitraryValue = viaRust(input => facade.unitDecodeArbitraryValue(input))
