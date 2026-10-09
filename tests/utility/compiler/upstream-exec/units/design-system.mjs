// A design system over a recorded `Theme`. With registries a test injected
// through `Utilities`/`Variants`, parsing runs the Rust candidate parser
// against them; otherwise the full Rust design system serves every call and
// is rebuilt whenever the recorded theme changes.
import { configuration, crossed, facade } from '../facade.mjs'

export * from 'rustify-foreign:../../../../../ref/tailwindcss/packages/tailwindcss/src/design-system.ts'

export function buildDesignSystem(theme) {
  let system = null
  let built = null
  let rust = () => {
    let css = theme.css
    if (built !== css) {
      system = new facade.DesignSystem(css, configuration())
      built = css
    }
    return system
  }
  let registry = design => ({
    prefix: design.theme.prefix ?? null,
    utilities: design.utilities?.roots ?? { static: [], functional: [] },
    variants: design.variants?.entries ?? [],
  })
  let injected = design => design.utilities !== undefined || design.variants !== undefined
  let design = {
    theme,
    utilities: undefined,
    variants: undefined,
    parseCandidate(raw) {
      crossed()
      return injected(this) ? facade.unitParseCandidate(raw, registry(this)) : rust().parseCandidate(raw)
    },
    parseVariant(raw) {
      crossed()
      return injected(this) ? facade.unitParseVariant(raw, registry(this)) : rust().parseVariant(raw)
    },
  }
  // Every other design-system call reaches the Rust design system.
  return new Proxy(design, {
    get(target, key, receiver) {
      if (key in target || typeof key === 'symbol') return Reflect.get(target, key, receiver)
      let value = rust()[key]
      if (typeof value !== 'function') return value
      return (...args) => {
        crossed()
        return value.apply(rust(), args)
      }
    },
    set(target, key, value, receiver) {
      if (key in target) return Reflect.set(target, key, value, receiver)
      crossed()
      rust()[key] = value
      return true
    },
  })
}
