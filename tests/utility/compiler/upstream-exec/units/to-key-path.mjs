import { facade, viaRust } from '../facade.mjs'

export const toKeyPath = viaRust(path => facade.unitToKeyPath(path))
