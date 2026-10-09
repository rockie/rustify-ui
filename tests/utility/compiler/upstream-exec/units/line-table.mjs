import { facade, viaRust } from '../facade.mjs'

export const createLineTable = viaRust(source => {
  let table = new facade.UnitLineTable(source)
  return {
    find: viaRust(offset => table.find(offset)),
    findOffset: viaRust(({ line, column }) => table.findOffset(line, column)),
  }
})
