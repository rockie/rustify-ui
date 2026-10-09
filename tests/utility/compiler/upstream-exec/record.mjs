// Appends, per finished test, how many times it crossed into the Rust facade
// and a bounded excerpt of each failure's actual and expected values.
import { appendFileSync } from 'node:fs'
import { beforeEach, onTestFinished } from 'vitest'

const sink = process.env.RUSTIFY_UPSTREAM_CROSSINGS
const excerpt = value => (value === undefined ? undefined : String(value).slice(0, 2000))

beforeEach(context => {
  if (!sink) return
  let start = globalThis.__rustifyFacadeCalls ?? 0
  let foreignStart = globalThis.__rustifyForeignCalls ?? 0
  onTestFinished(result => {
    let task = context.task
    let suites = []
    for (let suite = task.suite; suite && suite.type === 'suite' && suite.name; suite = suite.suite) suites.unshift(suite.name)
    let errors = (result.errors ?? task.result?.errors ?? []).map(error => ({
      message: excerpt(error.message), actual: excerpt(error.actual), expected: excerpt(error.expected),
    }))
    let calls = (globalThis.__rustifyFacadeCalls ?? 0) - start
    let foreign = (globalThis.__rustifyForeignCalls ?? 0) - foreignStart
    appendFileSync(sink, JSON.stringify({ file: task.file.filepath, suites, name: task.name, line: task.location?.line ?? null, calls, foreign, errors }) + '\n')
  })
})
