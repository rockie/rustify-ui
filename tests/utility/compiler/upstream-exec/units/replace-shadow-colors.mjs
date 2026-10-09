import { facade, viaRust } from '../facade.mjs'

export const replaceShadowColors = viaRust((input, replacement) => facade.unitReplaceShadowColors(input, replacement))
