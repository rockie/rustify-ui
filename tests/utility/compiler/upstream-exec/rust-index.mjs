// The upstream package entry, backed by the Rust/WASM facade. Only argument
// adaptation lives here: CSS parsing, compilation and printing run in Rust.
// The facade takes a virtual stylesheet map, so a caller's async
// `loadStylesheet` is driven from the `MissingAsset` error it reports.
import { configuration, crossed, facade } from './facade.mjs'

export const Polyfills = { None: 0, AtProperty: 1, ColorMix: 2, All: 3 }
export const Features = { None: 0, AtApply: 1, AtImport: 2, JsPluginCompat: 4, ThemeFunction: 8, Utilities: 16, Variants: 32 }

function missing(error) {
  let cause = error?.cause?.error
  return cause?.kind === 'missing_asset' ? cause : null
}

// Rebuilds with each resolved stylesheet until no import is missing.
async function construct(create, opts) {
  let assets = []
  let loader = typeof opts.loadStylesheet === 'function'
  for (;;) {
    crossed()
    try {
      return create(configuration(opts, assets, loader))
    } catch (error) {
      let request = loader && missing(error)
      if (!request) throw error
      let loaded = await opts.loadStylesheet(request.id, request.base)
      assets.push({ id: request.id, path: loaded.path, base: loaded.base, content: loaded.content })
    }
  }
}

export async function compile(css, opts = {}) {
  let api = await construct(config => new facade.CssCompiler(css, config), opts)
  // Upstream reports these through console.warn while registering utilities.
  for (let warning of api.warnings) console.warn(warning)
  return {
    sources: api.sources,
    root: api.root,
    features: api.features,
    build: candidates => api.build(candidates),
    buildSourceMap: () => api.buildSourceMap(),
  }
}

export async function compileAst(input, opts = {}) {
  let assets = []
  let loader = typeof opts.loadStylesheet === 'function'
  for (;;) {
    crossed()
    try {
      let api = await facade.compileAst(input, configuration(opts, assets, loader))
      return {
        sources: api.sources,
        root: api.root,
        features: api.features,
        build: candidates => api.build(candidates),
      }
    } catch (error) {
      let request = loader && missing(error)
      if (!request) throw error
      let loaded = await opts.loadStylesheet(request.id, request.base)
      assets.push({ id: request.id, path: loaded.path, base: loaded.base, content: loaded.content })
    }
  }
}

export async function __unstable__loadDesignSystem(css, opts = {}) {
  return construct(config => new facade.DesignSystem(css, config), opts)
}

export default function postcssPluginWarning() {
  throw new Error(
    `It looks like you're trying to use \`tailwindcss\` directly as a PostCSS plugin. The PostCSS plugin has moved to a separate package, so to continue using Tailwind CSS with PostCSS you'll need to install \`@tailwindcss/postcss\` and update your PostCSS configuration.`,
  )
}
