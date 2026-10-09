import assert from 'node:assert/strict'
import {createHash} from 'node:crypto'
import {readFile,writeFile} from 'node:fs/promises'
import {createRequire} from 'node:module'
import {resolve} from 'node:path'

// Generate the production current-walk fixtures from the fixed upstream walk.
const oracle = createRequire(resolve('tests/utility/compiler/oracle/package.json'))
assert.equal(oracle('tailwindcss/package.json').version, '4.1.13')
const inventory = JSON.parse(await readFile('tests/utility/compiler/inventory.json', 'utf8'))
assert.equal(inventory.baseline.commit, '1334c99db8fd26c8ea065375dd9259800863f072')
const sourcePath = 'packages/tailwindcss/src/ast.ts'
const source = await readFile(resolve('ref/tailwindcss', sourcePath))
assert.equal(createHash('sha256').update(source).digest('hex'),
  inventory.sources.find(entry => entry.path === sourcePath)?.sha256)
const {createJiti} = oracle('jiti')
const {walk} = await createJiti(import.meta.url, {moduleCache:false, fsCache:false})
  .import(resolve('ref/tailwindcss', sourcePath))
const actions = {continue:0, skip:1, stop:2}
const cases = []
function fixture(name, build) {
  const nodes = [], arrays = []
  const list = (ids = []) => arrays.push(ids) - 1
  const node = (tag, {kind='at-rule', children=null, context={}, operation=null}={}) =>
    nodes.push({kind, tag, children, context, operation}) - 1
  const root = build({node, list, nodes, arrays})
  cases.push({name, input:{nodes, arrays, root}})
}
fixture('shared-array-removal-visible-to-second-context', ({node,list}) => {
  const gone = node('source', {operation:{type:'replace', nodes:[], action:'continue'}})
  const kept = node('kept'), shared = list([gone,kept])
  return list([node('one',{kind:'context',children:shared}), node('two',{kind:'context',children:shared})])
})
fixture('legacy-rename-and-removal-before-error', ({node,list}) => list([
  node('legacy',{operation:{type:'rename-error',name:'custom-variant',error:'invalid name'}}), node('unvisited'),
]))
for (const count of [0,1,2]) for (const action of ['continue','skip','stop']) {
  fixture(`replace-${count}-${action}`, ({node,list}) => {
    const children = list([node('replacement-child')])
    const replacements = Array.from({length:count}, (_,index) => node(`replacement-${index}`,{children}))
    return list([node('replace',{operation:{type:'replace',nodes:replacements,action}}), node('following')])
  })
}
fixture('nested-stop-propagates', ({node,list}) => list([
  node('parent',{children:list([node('stop',{operation:{type:'action',action:'stop'}}),node('unvisited-child')])}),
  node('unvisited-sibling'),
]))
fixture('context-overrides-false-and-empty-preserves-parent', ({node,list}) => {
  const leaf = node('leaf',{kind:'declaration'})
  const inner = node('inner',{kind:'context',children:list([leaf]),context:{reference:false,base:''}})
  const parent = node('parent',{kind:'rule',children:list([inner])})
  return list([node('outer',{kind:'context',children:list([parent]),context:{reference:true,base:'outer',sourceBase:'source'}})])
})
fixture('deferred-body-reads-later-shared-array-removal', ({node,list}) => {
  const shared = list([node('body',{kind:'declaration'})])
  return list([node('utility',{children:shared,operation:{type:'capture'}}),
    node('later',{operation:{type:'edit-array',array:shared,index:0,remove:1,nodes:[]}})])
})
fixture('visitor-child-reassignment-is-current', ({node,list}) => {
  const old = list([node('old')]), next = list([node('new')])
  return list([node('switch',{children:old,operation:{type:'children',array:next}})])
})
fixture('shared-array-edit-visible-after-earlier-occurrence', ({node,list}) => {
  const shared = node('shared'), left = list([shared]), right = list([shared])
  return list([node('left',{kind:'context',children:left}),
    node('edit',{operation:{type:'edit-array',array:right,index:0,remove:1,nodes:[]}}),
    node('right',{kind:'context',children:right})])
})
fixture('ordinary-at-root-establishes-parent', ({node,list}) => list([
  node('at-root',{kind:'at-root',children:list([node('leaf')])}),
]))
fixture('semantic-error-stops-before-later-array-write', ({node,list}) => {
  const body = list([node('kept')])
  return list([node('bad',{operation:{type:'error',message:'first error'}}),
    node('later',{operation:{type:'edit-array',array:body,index:0,remove:1,nodes:[]}})])
})

function reference(input) {
  const arrays = input.arrays.map(() => [])
  const nodes = input.nodes.map(node => ({kind:node.kind, tag:node.tag,
    ...(node.children === null ? {} : {nodes:arrays[node.children]}), context:{...node.context}}))
  input.arrays.forEach((list,index) => arrays[index].push(...list.map(id => nodes[id])))
  const nodeIds = new Map(nodes.map((node,id) => [node,id]))
  const arrayIds = new Map(arrays.map((array,id) => [array,id]))
  const visits = [], effects = [], captured = []
  let error = null, stopped = false
  function occurrence(id) {
    // Operations used in fixtures have a unique current position when replacing.
    for (let array=0; array<arrays.length; array++) {
      const index = arrays[array].indexOf(nodes[id])
      if (index >= 0) return {array,index}
    }
    throw new Error(`fixture position missing: ${id}`)
  }
  try {
    stopped = walk(arrays[input.root], (node, utils) => {
      const id = nodeIds.get(node), operation = input.nodes[id].operation
      visits.push({node:id,parent:utils.parent === null ? null : nodeIds.get(utils.parent),
        path:utils.path.map(node => nodeIds.get(node)),context:{...utils.context}})
      if (!operation) return
      switch (operation.type) {
        case 'replace': {
          const position = occurrence(id)
          utils.replaceWith(operation.nodes.map(id => nodes[id]))
          effects.push({type:'replace',...position,nodes:operation.nodes})
          // A second replaceWith is ignored by the fixed reference cursor.
          utils.replaceWith([node])
          return actions[operation.action]
        }
        case 'rename-error': {
          node.tag = operation.name
          effects.push({type:'rename',node:id,name:operation.name})
          const position = occurrence(id)
          utils.replaceWith([])
          effects.push({type:'replace',...position,nodes:[]})
          throw new Error(operation.error)
        }
        case 'error': throw new Error(operation.message)
        case 'capture': captured.push(node.nodes); return
        case 'edit-array': {
          const {array,index,remove,nodes:replacement} = operation
          arrays[array].splice(index,remove,...replacement.map(id => nodes[id]))
          effects.push({type:'edit-array',array,index,remove,nodes:replacement}); return
        }
        case 'children': node.nodes=arrays[operation.array];effects.push({type:'children',node:id,array:operation.array});return
        case 'action': return actions[operation.action]
        default: throw new Error(operation.type)
      }
    }) === 2
  } catch (caught) { error=caught.message }
  return {error,stopped,visits,effects,arrays:arrays.map(list => list.map(node => nodeIds.get(node))),
    nodes:nodes.map((node,id) => ({...input.nodes[id],tag:node.tag,
      children:node.nodes === undefined ? null : arrayIds.get(node.nodes)})),
    deferred:captured.map(list => list.map(node => nodeIds.get(node)))}
}

const result={baseline:inventory.baseline.commit,sourceSha256:createHash('sha256').update(source).digest('hex'),cases:cases.map(entry=>({...entry,expected:reference(entry.input)}))}
const output=resolve('tests/utility/compiler/current-walk-fixtures.json')
const serialized=JSON.stringify(result,null,2)+'\n'
if(process.argv.includes('--check'))assert.equal(await readFile(output,'utf8'),serialized,'fixed walk fixtures must match the source oracle')
else await writeFile(output,serialized)
console.log(JSON.stringify({comparisons:result.cases.length,sourceSha256:result.sourceSha256,generatedFrom:'fixed upstream ast.ts walk',checked:process.argv.includes('--check')}))
