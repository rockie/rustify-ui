# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: docs-ssg/dev-components.spec.ts >> Rust saves keep the original interactive document until a complete new pair, and diagnostics recover at its scroll position
- Location: tests/docs-ssg/dev-components.spec.ts:447:5

# Error details

```
Error: expect(locator).toContainText(expected) failed

Locator: locator('.rd-dev-panel').locator('.rd-dev-file')
Expected substring: "src/demos.rs"
Error: strict mode violation: locator('.rd-dev-panel').locator('.rd-dev-file') resolved to 5 elements:
    1) <h2 class="rd-dev-file">…</h2> aka getByRole('heading', { name: 'src/demos.rs' })
    2) <h2 class="rd-dev-file">…</h2> aka getByRole('heading', { name: '/Users/rockie/Documents/gh-rockie/rustify-ui/crates/rustify-ui/src/overlay.rs' })
    3) <h2 class="rd-dev-file">…</h2> aka getByRole('heading', { name: '/Users/rockie/Documents/gh-rockie/rustify-ui/crates/rustify-ui/src/router.rs' })
    4) <h2 class="rd-dev-file">…</h2> aka getByRole('heading', { name: 'content/index.md' })
    5) <h2 class="rd-dev-file">theme.json</h2> aka getByRole('heading', { name: 'theme.json' })

Call log:
  - Expect "toContainText" locator('.rd-dev-panel').locator('.rd-dev-file') with timeout 10000ms
  - waiting for locator('.rd-dev-panel').locator('.rd-dev-file')

```

# Page snapshot

```yaml
- generic [ref=f2e1]:
  - link "Skip to content" [ref=f2e2] [cursor=pointer]:
    - /url: "#content"
  - banner [ref=f2e3]:
    - generic [ref=f2e4]:
      - generic [ref=f2e5]: Reading laboratory
      - navigation "Site links" [ref=f2e7]:
        - list [ref=f2e8]:
          - listitem [ref=f2e9]:
            - link "Counters" [ref=f2e10] [cursor=pointer]:
              - /url: /docs/counter/
          - listitem [ref=f2e11]:
            - link "Catalog proxy" [ref=f2e12] [cursor=pointer]:
              - /url: /docs/proxy/
      - group "Theme" [ref=f2e13]:
        - button "System" [pressed] [ref=f2e14] [cursor=pointer]
        - button "Light" [ref=f2e20] [cursor=pointer]
        - button "Dark" [ref=f2e26] [cursor=pointer]
  - generic [ref=f2e31]:
    - generic [ref=f2e32]:
      - paragraph [ref=f2e34]: Examples in this guide
      - navigation "Menu" [ref=f2e35]:
        - generic [ref=f2e36]:
          - heading "Custom pages" [level=2] [ref=f2e37]
          - list [ref=f2e38]:
            - listitem [ref=f2e39]:
              - link "Custom page" [ref=f2e40] [cursor=pointer]:
                - /url: /docs/custom/
            - listitem [ref=f2e41]:
              - link "Static custom page" [ref=f2e42] [cursor=pointer]:
                - /url: /docs/plain/
        - generic [ref=f2e43]:
          - heading "Examples" [level=2] [ref=f2e44]
          - list [ref=f2e45]:
            - listitem [ref=f2e46]:
              - link "Registered counters" [ref=f2e47] [cursor=pointer]:
                - /url: /docs/counter/
            - listitem [ref=f2e48]:
              - link "Static built-in components" [ref=f2e49] [cursor=pointer]:
                - /url: /docs/builtins/
            - listitem [ref=f2e50]:
              - link "Browser work after attachment" [ref=f2e51] [cursor=pointer]:
                - /url: /docs/async/
            - listitem [ref=f2e52]:
              - link "Client-only counter" [ref=f2e53] [cursor=pointer]:
                - /url: /docs/client-only/
            - listitem [ref=f2e54]:
              - link "Runtime failure recovery" [ref=f2e55] [cursor=pointer]:
                - /url: /docs/failure/
            - listitem [ref=f2e56]:
              - link "Catalog release proxy" [ref=f2e57] [cursor=pointer]:
                - /url: /docs/proxy/
      - paragraph [ref=f2e59]: Each example keeps its source nearby
    - navigation "On this page" [ref=f2e61]:
      - paragraph [ref=f2e62]: On this page
      - list [ref=f2e63]:
        - listitem [ref=f2e64]:
          - link "Three independent instances" [ref=f2e65] [cursor=pointer]:
            - /url: "#three-independent-instances"
        - listitem [ref=f2e66]:
          - link "Function and full file" [ref=f2e67] [cursor=pointer]:
            - /url: "#function-and-full-file"
    - main [ref=f2e68]:
      - generic [ref=f2e69]:
        - paragraph [ref=f2e72]: Try the example, then read its implementation
        - article [ref=f2e73]:
          - heading "Registered counters" [level=1] [ref=f2e74]
          - heading "Three independent instances" [level=2] [ref=f2e75]
          - paragraph [ref=f2e76]: Each counter starts with the same props and child content. Incrementing one leaves the others at two.
          - generic [ref=f2e77]:
            - group "Interactive example" [ref=f2e78]:
              - generic [ref=f2e81]:
                - paragraph [ref=f2e82]: Shared initial props
                - generic [ref=f2e83]:
                  - button "Increment" [active] [ref=f2e84] [cursor=pointer]
                  - status [ref=f2e85]: "5"
                  - textbox "Counter value" [ref=f2e86]: "5"
                - paragraph [ref=f2e88]:
                  - text: The same
                  - strong [ref=f2e89]: safe child Markdown
                  - text: appears in each instance.
            - status [ref=f2e90]:
              - generic [ref=f2e91]: Interactive example
          - generic [ref=f2e92]:
            - group "Interactive example" [ref=f2e93]:
              - generic [ref=f2e96]:
                - paragraph [ref=f2e97]: Shared initial props
                - generic [ref=f2e98]:
                  - button "Increment" [ref=f2e99] [cursor=pointer]
                  - status [ref=f2e100]: "2"
                  - textbox "Counter value" [ref=f2e101]: "2"
                - paragraph [ref=f2e103]:
                  - text: The same
                  - strong [ref=f2e104]: safe child Markdown
                  - text: appears in each instance.
            - status [ref=f2e105]:
              - generic [ref=f2e106]: Interactive example
          - generic [ref=f2e107]:
            - group "Interactive example" [ref=f2e108]:
              - generic [ref=f2e111]:
                - paragraph [ref=f2e112]: Shared initial props
                - generic [ref=f2e113]:
                  - button "Increment" [ref=f2e114] [cursor=pointer]
                  - status [ref=f2e115]: "2"
                  - textbox "Counter value" [ref=f2e116]: "2"
                - paragraph [ref=f2e118]:
                  - text: The same
                  - strong [ref=f2e119]: safe child Markdown
                  - text: appears in each instance.
            - status [ref=f2e120]:
              - generic [ref=f2e121]: Interactive example
          - heading "Function and full file" [level=2] [ref=f2e122]
          - paragraph [ref=f2e123]: The registered function starts at seven. Its original function and containing file appear below the view.
          - generic [ref=f2e124]:
            - group "Interactive example" [ref=f2e125]:
              - generic [ref=f2e128]:
                - paragraph [ref=f2e129]: Résumé — a real registered counter
                - generic [ref=f2e130]:
                  - button "Increment" [ref=f2e131] [cursor=pointer]
                  - status [ref=f2e132]: "11"
                  - textbox "Counter value" [ref=f2e133]: "11"
                - paragraph [ref=f2e134]: This child is part of the same view on both targets.
            - status [ref=f2e135]:
              - generic [ref=f2e136]: Interactive example
            - generic [ref=f2e137]:
              - paragraph [ref=f2e138]: src/demos.rs
              - generic [ref=f2e139]:
                - generic [ref=f2e140]:
                  - tablist [ref=f2e141]:
                    - tab "Function" [ref=f2e142] [cursor=pointer]
                    - tab "Full file" [selected] [ref=f2e143] [cursor=pointer]
                  - button "Copy" [ref=f2e144] [cursor=pointer]
                - tabpanel "Full file" [ref=f2e149]:
                  - code [ref=f2e151]:
                    - generic [ref=f2e153]:
                      - text: use
                      - generic [ref=f2e154]: "leptos::"
                      - generic [ref=f2e155]: "prelude::"
                      - text: "*;"
                    - generic [ref=f2e157]:
                      - text: use
                      - generic [ref=f2e158]: "rustify_components::"
                      - generic [ref=f2e159]: "{Button, TextField"
                      - generic [ref=f2e160]: "}"
                      - text: ;
                    - generic [ref=f2e162]:
                      - generic [ref=f2e163]:
                        - generic [ref=f2e164]: pub fn
                        - text: counter
                      - generic [ref=f2e165]:
                        - generic [ref=f2e166]: "(initial: i32, label: String, children: AnyView"
                        - generic [ref=f2e167]: )
                      - generic [ref=f2e169]: "-> impl IntoView"
                      - generic [ref=f2e171]: "{"
                    - generic [ref=f2e176]:
                      - text: let value =
                      - generic [ref=f2e177]: "RwSignal::"
                      - text: new
                      - generic [ref=f2e178]: (initial
                      - generic [ref=f2e179]: )
                      - text: ;
                    - generic [ref=f2e183]:
                      - text: view!
                      - generic [ref=f2e184]: "{"
                    - generic [ref=f2e189]:
                      - text: <div data-counter=
                      - generic [ref=f2e190]: "\"\""
                      - text: class=
                      - generic [ref=f2e191]: "\"min-h-24 space-y-3\""
                      - text: ">"
                    - generic [ref=f2e196]:
                      - text: <p>
                      - generic [ref=f2e197]: "{label"
                      - generic [ref=f2e198]: "}"
                      - text: </p>
                    - generic [ref=f2e203]:
                      - text: <div class=
                      - generic [ref=f2e204]: "\"flex flex-wrap items-center gap-3\""
                      - text: ">"
                    - generic [ref=f2e209]:
                      - text: <Button test_id=
                      - generic [ref=f2e210]: "\"counter-increment\""
                      - text: on_click=move || value.update
                      - generic [ref=f2e211]:
                        - text: (
                        - generic [ref=f2e212]: "|"
                        - generic [ref=f2e214]: value|
                        - generic [ref=f2e216]: "*value += 3"
                      - generic [ref=f2e217]: )
                      - text: ">"
                    - generic [ref=f2e218]: "\"Increment\""
                    - generic [ref=f2e224]: </Button>
                    - generic [ref=f2e233]:
                      - text: <output data-testid=
                      - generic [ref=f2e234]: "\"counter-value\""
                      - text: ">"
                      - generic [ref=f2e235]:
                        - text: "{move || value.get"
                        - generic [ref=f2e236]: (
                        - generic [ref=f2e237]: )
                      - generic [ref=f2e238]: "}"
                      - text: </output>
                    - generic [ref=f2e243]:
                      - text: <TextField test_id=
                      - generic [ref=f2e244]: "\"counter-input\""
                      - text: value=
                      - generic [ref=f2e245]: "Signal::"
                      - text: derive
                      - generic [ref=f2e246]:
                        - text: (move || value.get
                        - generic [ref=f2e247]: (
                        - generic [ref=f2e248]: )
                        - text: .to_string
                        - generic [ref=f2e249]: (
                        - generic [ref=f2e250]: )
                      - generic [ref=f2e251]: )
                    - generic [ref=f2e256]:
                      - text: "on_change=move |input: String|"
                      - generic [ref=f2e257]: "{"
                    - generic [ref=f2e263]:
                      - text: if let Ok
                      - generic [ref=f2e264]: (input
                      - generic [ref=f2e265]: )
                      - text: = input.parse
                      - generic [ref=f2e266]: (
                      - generic [ref=f2e267]: )
                      - generic [ref=f2e268]:
                        - text: "{ value.set"
                        - generic [ref=f2e269]: (input
                        - generic [ref=f2e270]: )
                        - text: ;
                      - generic [ref=f2e271]: "}"
                    - generic [ref=f2e272]: "}"
                    - generic [ref=f2e282]:
                      - text: aria_label=
                      - generic [ref=f2e283]: "\"Counter value\""
                    - generic [ref=f2e284]: />
                    - generic [ref=f2e289]: </div>
                    - generic [ref=f2e298]:
                      - generic [ref=f2e299]: "{children"
                      - generic [ref=f2e300]: "}"
                    - generic [ref=f2e301]: </div>
                    - generic [ref=f2e306]: "}"
                    - generic [ref=f2e311]: "}"
                    - generic [ref=f2e315]: /// The function and its containing file are displayed verbatim by the docs.
                    - generic [ref=f2e319]:
                      - generic [ref=f2e320]:
                        - generic [ref=f2e321]: pub fn
                        - text: counter_demo
                      - generic [ref=f2e322]:
                        - generic [ref=f2e323]: (
                        - generic [ref=f2e324]: )
                      - generic [ref=f2e326]: "-> impl IntoView"
                      - generic [ref=f2e328]: "{"
                    - generic [ref=f2e333]:
                      - text: counter
                      - generic [ref=f2e334]: (
                    - generic [ref=f2e335]: 11,
                    - generic [ref=f2e344]:
                      - generic [ref=f2e345]: "\"Résumé — a real registered counter\""
                      - text: .to_string
                      - generic [ref=f2e346]: (
                      - generic [ref=f2e347]: )
                      - text: ","
                    - generic [ref=f2e352]:
                      - text: view!
                      - generic [ref=f2e353]:
                        - text: "{ <p>"
                        - generic [ref=f2e354]: "\"This child is part of the same view on both targets.\""
                        - text: </p>
                      - generic [ref=f2e355]: "}"
                      - text: .into_any
                      - generic [ref=f2e356]: (
                      - generic [ref=f2e357]: )
                      - text: ","
                    - generic [ref=f2e358]: )
                    - generic [ref=f2e363]: "}"
                    - generic [ref=f2e368]:
                      - generic [ref=f2e369]:
                        - generic [ref=f2e370]: pub fn
                        - text: async_after_attach_demo
                      - generic [ref=f2e371]:
                        - generic [ref=f2e372]: (
                        - generic [ref=f2e373]: )
                      - generic [ref=f2e375]: "-> impl IntoView"
                      - generic [ref=f2e377]: "{"
                    - generic [ref=f2e379]: "// No asynchronous initial data: the action starts only after a reader clicks."
                    - generic [ref=f2e387]:
                      - text: let load =
                      - generic [ref=f2e388]: "Action::"
                      - text: new
                      - generic [ref=f2e389]:
                        - text: (
                        - generic [ref=f2e390]: "|"
                        - generic [ref=f2e393]:
                          - text: "_: &"
                          - generic [ref=f2e394]: (
                          - generic [ref=f2e395]: )
                          - text: "|"
                        - generic [ref=f2e396]:
                          - text: async
                          - generic [ref=f2e397]:
                            - text: "{"
                            - generic [ref=f2e398]: "\"Loaded after attachment\""
                            - text: .to_string
                            - generic [ref=f2e399]: (
                            - generic [ref=f2e400]: )
                          - generic [ref=f2e401]: "}"
                      - generic [ref=f2e402]: )
                      - text: ;
                    - generic [ref=f2e406]:
                      - text: view!
                      - generic [ref=f2e407]: "{"
                    - generic [ref=f2e412]:
                      - text: <div data-async-after-attach=
                      - generic [ref=f2e413]: "\"\""
                      - text: class=
                      - generic [ref=f2e414]: "\"space-y-3\""
                      - text: ">"
                    - generic [ref=f2e419]:
                      - text: <Button test_id=
                      - generic [ref=f2e420]: "\"async-start\""
                      - text: on_click=move ||
                      - generic [ref=f2e421]:
                        - text: "{ load.dispatch"
                        - generic [ref=f2e422]:
                          - text: (
                          - generic [ref=f2e423]: (
                          - generic [ref=f2e424]: )
                        - generic [ref=f2e425]: )
                        - text: ;
                      - generic [ref=f2e426]: "}"
                      - text: ">"
                    - generic [ref=f2e427]: "\"Load browser data\""
                    - generic [ref=f2e433]: </Button>
                    - generic [ref=f2e442]:
                      - text: <output data-testid=
                      - generic [ref=f2e443]: "\"async-result\""
                      - text: ">"
                    - generic [ref=f2e448]:
                      - generic [ref=f2e449]:
                        - text: "{move || load.value"
                        - generic [ref=f2e450]: (
                        - generic [ref=f2e451]: )
                        - text: .get
                        - generic [ref=f2e452]: (
                        - generic [ref=f2e453]: )
                        - text: .unwrap_or_else
                        - generic [ref=f2e454]:
                          - text: (
                          - generic [ref=f2e455]: "|"
                          - generic [ref=f2e457]: "|"
                          - generic [ref=f2e459]:
                            - generic [ref=f2e460]: "\"Not loaded\""
                            - text: .to_string
                            - generic [ref=f2e461]: (
                            - generic [ref=f2e462]: )
                        - generic [ref=f2e463]: )
                      - generic [ref=f2e464]: "}"
                    - generic [ref=f2e465]: </output>
                    - generic [ref=f2e470]: </div>
                    - generic [ref=f2e475]: "}"
                    - generic [ref=f2e480]: "}"
                    - generic [ref=f2e485]:
                      - generic [ref=f2e486]:
                        - generic [ref=f2e487]: pub fn
                        - text: failure_demo
                      - generic [ref=f2e488]:
                        - generic [ref=f2e489]: (
                        - generic [ref=f2e490]: )
                      - generic [ref=f2e492]: "-> impl IntoView"
                      - generic [ref=f2e494]: "{"
                    - generic [ref=f2e499]:
                      - text: let value =
                      - generic [ref=f2e500]: "RwSignal::"
                      - text: new
                      - generic [ref=f2e501]: (0
                      - generic [ref=f2e502]: )
                      - text: ;
                    - generic [ref=f2e506]:
                      - text: view!
                      - generic [ref=f2e507]: "{"
                    - generic [ref=f2e512]:
                      - text: <div data-failure-demo=
                      - generic [ref=f2e513]: "\"\""
                      - text: class=
                      - generic [ref=f2e514]: "\"space-y-3\""
                      - text: ">"
                    - generic [ref=f2e519]:
                      - text: <Button test_id=
                      - generic [ref=f2e520]: "\"failure-increment\""
                      - text: on_click=move || value.update
                      - generic [ref=f2e521]:
                        - text: (
                        - generic [ref=f2e522]: "|"
                        - generic [ref=f2e524]: value|
                        - generic [ref=f2e526]: "*value += 1"
                      - generic [ref=f2e527]: )
                      - text: ">"
                    - generic [ref=f2e528]: "\"Change before failure\""
                    - generic [ref=f2e534]: </Button>
                    - generic [ref=f2e543]:
                      - text: <output data-testid=
                      - generic [ref=f2e544]: "\"failure-value\""
                      - text: ">"
                      - generic [ref=f2e545]:
                        - text: "{move || value.get"
                        - generic [ref=f2e546]: (
                        - generic [ref=f2e547]: )
                      - generic [ref=f2e548]: "}"
                      - text: </output>
                    - generic [ref=f2e553]:
                      - text: <Button test_id=
                      - generic [ref=f2e554]: "\"failure-crash\""
                      - text: on_click=
                      - generic [ref=f2e555]: "|"
                      - generic [ref=f2e557]: "|"
                      - generic [ref=f2e559]:
                        - text: panic!
                        - generic [ref=f2e560]:
                          - text: (
                          - generic [ref=f2e561]: "\"docs fixture callback failure\""
                        - generic [ref=f2e562]: )
                        - text: ">"
                    - generic [ref=f2e563]: "\"Crash this example\""
                    - generic [ref=f2e569]: </Button>
                    - generic [ref=f2e574]: </div>
                    - generic [ref=f2e579]: "}"
                    - generic [ref=f2e584]: "}"
                    - generic [ref=f2e589]:
                      - generic [ref=f2e590]:
                        - generic [ref=f2e591]: pub fn
                        - text: catalog_proxy
                      - generic [ref=f2e592]:
                        - generic [ref=f2e593]: (
                        - generic [ref=f2e594]: )
                      - generic [ref=f2e596]: "-> impl IntoView"
                      - generic [ref=f2e598]: "{"
                    - generic [ref=f2e603]:
                      - text: let scope =
                      - generic [ref=f2e604]: "rustify_ui::"
                      - generic [ref=f2e605]: "id::"
                      - text: next
                      - generic [ref=f2e606]:
                        - text: (
                        - generic [ref=f2e607]: "\"catalog\""
                      - generic [ref=f2e608]: )
                      - text: .unwrap_or_else
                      - generic [ref=f2e609]:
                        - text: (
                        - generic [ref=f2e610]: "|"
                        - generic [ref=f2e612]: "|"
                        - generic [ref=f2e614]:
                          - generic [ref=f2e615]: "\"catalog\""
                          - text: .to_string
                          - generic [ref=f2e616]: (
                          - generic [ref=f2e617]: )
                      - generic [ref=f2e618]: )
                      - text: ;
                    - generic [ref=f2e622]:
                      - text: view!
                      - generic [ref=f2e623]:
                        - text: "{ <crate"
                        - generic [ref=f2e624]: "::"
                        - generic [ref=f2e625]: "catalog::"
                        - text: CatalogView scope=scope />
                      - generic [ref=f2e626]: "}"
                    - generic [ref=f2e627]: "}"
        - paragraph [ref=f2e632]: Continue with the next page
        - navigation "Previous / Next" [ref=f2e633]:
          - link "Previous Static custom page" [ref=f2e634] [cursor=pointer]:
            - /url: /docs/plain/
            - generic [ref=f2e635]: Previous
            - generic [ref=f2e636]: Static custom page
          - link "Next Static built-in components" [ref=f2e637] [cursor=pointer]:
            - /url: /docs/builtins/
            - generic [ref=f2e638]: Next
            - generic [ref=f2e639]: Static built-in components
  - contentinfo [ref=f2e640]:
    - paragraph [ref=f2e642]:
      - text: Keep reading ·
      - link "Home" [ref=f2e643] [cursor=pointer]:
        - /url: /docs/
  - paragraph [ref=f2e644]
  - status [ref=f2e645]: 2 errors · 4 warnings · showing the last good version · revision 3
  - dialog "2 errors · 4 warnings · showing the last good version" [ref=f2e647]:
    - generic [ref=f2e648]:
      - paragraph [ref=f2e649]:
        - text: 2 errors · 4 warnings
        - generic [ref=f2e650]:
          - generic [ref=f2e651]: ·
          - text: showing the last good version
      - generic [ref=f2e652]: revision 3
      - button "Hide problems" [ref=f2e653] [cursor=pointer]
    - region "Problems" [ref=f2e657]:
      - list [ref=f2e659]:
        - listitem [ref=f2e660]:
          - generic [ref=f2e662]:
            - generic [ref=f2e663]: "Error:"
            - text: rust-compile
          - paragraph [ref=f2e664]:
            - text: For more information about this error, try
            - code [ref=f2e665]: rustc --explain E0425
            - text: .
      - generic [ref=f2e666]:
        - heading "src/demos.rs" [level=2] [ref=f2e667]
        - list [ref=f2e668]:
          - listitem [ref=f2e669]:
            - generic [ref=f2e670]:
              - generic [ref=f2e671]: 5:41
              - generic [ref=f2e672]:
                - generic [ref=f2e673]: "Error:"
                - text: rust-compile
            - paragraph [ref=f2e674]:
              - text: "error[E0425]: cannot find value"
              - code [ref=f2e675]: missing_counter_step
              - text: in this scope --> src/demos.rs:5:41 | 5 | let value = RwSignal::new(initial + missing_counter_step); | ^^^^^^^^^^^^^^^^^^^^ not found in this scope
      - generic [ref=f2e676]:
        - heading "/Users/rockie/Documents/gh-rockie/rustify-ui/crates/rustify-ui/src/overlay.rs" [level=2] [ref=f2e677]
        - list [ref=f2e678]:
          - listitem [ref=f2e679]:
            - generic [ref=f2e680]:
              - generic [ref=f2e681]: 349:23
              - generic [ref=f2e682]:
                - generic [ref=f2e683]: "Warning:"
                - text: rust-compile
            - paragraph [ref=f2e684]:
              - text: "warning: method"
              - code [ref=f2e685]: fallback_element
              - text: "is never used --> /Users/rockie/Documents/gh-rockie/rustify-ui/crates/rustify-ui/src/overlay.rs:349:23 | 284 | impl OverlayStack { | ----------------- method in this implementation ... 349 | pub(crate) fn fallback_element(&self) -> Element { | ^^^^^^^^^^^^^^^^ | = note:"
              - code [ref=f2e686]: "#[warn(dead_code)]"
              - text: (part of
              - code [ref=f2e687]: "#[warn(unused)]"
              - text: ) on by default
            - paragraph [ref=f2e688]:
              - code [ref=f2e689]: "#[warn(dead_code)]"
              - text: (part of
              - code [ref=f2e690]: "#[warn(unused)]"
              - text: ) on by default
      - generic [ref=f2e691]:
        - heading "/Users/rockie/Documents/gh-rockie/rustify-ui/crates/rustify-ui/src/router.rs" [level=2] [ref=f2e692]
        - list [ref=f2e693]:
          - listitem [ref=f2e694]:
            - generic [ref=f2e695]:
              - generic [ref=f2e696]: 556:9
              - generic [ref=f2e697]:
                - generic [ref=f2e698]: "Warning:"
                - text: rust-compile
            - paragraph [ref=f2e699]:
              - text: "warning: field"
              - code [ref=f2e700]: guest_click
              - text: "is never read --> /Users/rockie/Documents/gh-rockie/rustify-ui/crates/rustify-ui/src/router.rs:556:9 | 549 | struct Router { | ------ field in this struct ... 556 | guest_click: Option<SendWrapper<Rc<GuestClick>>>, | ^^^^^^^^^^^ | = note:"
              - code [ref=f2e701]: Router
              - text: has a derived impl for the trait
              - code [ref=f2e702]: Clone
              - text: ", but this is intentionally ignored during dead code analysis"
            - paragraph [ref=f2e703]:
              - code [ref=f2e704]: Router
              - text: has a derived impl for the trait
              - code [ref=f2e705]: Clone
              - text: ", but this is intentionally ignored during dead code analysis"
      - generic [ref=f2e706]:
        - heading "content/index.md" [level=2] [ref=f2e707]
        - list [ref=f2e708]:
          - listitem [ref=f2e709]:
            - generic [ref=f2e711]:
              - generic [ref=f2e712]: "Warning:"
              - text: not-in-nav
            - paragraph [ref=f2e713]: this page is in no navigation group, so readers can only reach it by a link
      - generic [ref=f2e714]:
        - heading "theme.json" [level=2] [ref=f2e715]
        - list [ref=f2e716]:
          - listitem [ref=f2e717]:
            - generic [ref=f2e719]:
              - generic [ref=f2e720]: "Warning:"
              - text: theme-token
            - paragraph [ref=f2e721]: "radius: Light and dark have different author values; editing this common field applies to both"
```

# Test source

```ts
  417 |     expect((await settled(previous)).compile).toEqual(compiled);
  418 |     expect(site.commands()).toEqual(commands);
  419 |     await ready(page);
  420 | 
  421 |     previous = server.state()!.revision;
  422 |     site.edit("rustify-docs.toml", 'name = "Component fixture"', 'name = "Component handbook"');
  423 |     await expect(page).toHaveTitle("Registered counters | Component handbook");
  424 |     expect((await settled(previous)).compile).toEqual(compiled);
  425 |     expect(site.commands()).toEqual(commands);
  426 |     await expect(page.locator('meta[property="og:site_name"]')).toHaveAttribute("content", "Component handbook");
  427 |     await ready(page);
  428 |     await sourceText(page, site.read("src/demos.rs"));
  429 | 
  430 |     previous = server.state()!.revision;
  431 |     const theme = JSON.parse(site.read("theme.json"));
  432 |     theme.styles.light.background = "#f2f7ff";
  433 |     site.write("theme.json", JSON.stringify(theme, null, 2) + "\n");
  434 |     await page.locator('[data-rd-mode="light"]').click();
  435 |     await expect(page.locator("body")).toHaveCSS("background-color", "rgb(242, 247, 255)");
  436 |     expect((await settled(previous)).compile).toEqual(compiled);
  437 |     expect(site.commands()).toEqual(commands);
  438 |     await ready(page);
  439 |     await sourceText(page, site.read("src/demos.rs"));
  440 |     const wasm = audit.responses.filter(response => response.url.endsWith("/app_bg.wasm"));
  441 |     expect(wasm.length).toBeGreaterThan(0);
  442 |     expect(new Set(wasm.map(response => response.url)).size).toBe(1);
  443 |     expect(wasm.every(response => response.status === 200 && response.mime.split(";")[0] === "application/wasm")).toBe(true);
  444 |     await healthy();
  445 | });
  446 | 
  447 | test("Rust saves keep the original interactive document until a complete new pair, and diagnostics recover at its scroll position", async ({ page }) => {
  448 |     test.setTimeout(900_000);
  449 |     server = await Dev.start(site);
  450 |     await page.goto(new URL("counter/", server.url).href);
  451 |     site.release();
  452 |     await ready(page);
  453 |     const firstState = await settled();
  454 |     const first = compilation(firstState);
  455 |     const original = site.read("src/demos.rs");
  456 |     const firstWasm = audit.requests.filter(url => url.endsWith("/app_bg.wasm")).at(-1)!;
  457 |     expect(firstWasm).toBeTruthy();
  458 |     await sourceText(page, original);
  459 |     await source(page).getByRole("tab", { name: "Full file", exact: true }).click();
  460 |     await page.getByTestId("counter-input").first().fill("30");
  461 |     await expect(values(page)).toHaveText(["30", "2", "2", "7"]);
  462 |     await captureTrees(page);
  463 | 
  464 |     const changed = original.replace("*value += 1", "*value += 3").replace('        7,\n        "Résumé', '        11,\n        "Résumé');
  465 |     expect(changed).not.toBe(original);
  466 |     site.hold();
  467 |     site.write("src/demos.rs", changed);
  468 |     await expect.poll(() => existsSync(site.entered)).toBe(true);
  469 |     await expect(page.locator(".rd-dev-marker")).toContainText("Building");
  470 |     await expect(page.locator(".rd-dev-marker")).toContainText("showing the last good version");
  471 |     await retained(page);
  472 |     await sourceText(page, original);
  473 |     await layouts(page, "rust-building");
  474 |     await page.getByTestId("counter-increment").first().click();
  475 |     await expect(values(page)).toHaveText(["31", "2", "2", "7"]);
  476 |     await source(page).getByRole("tab", { name: "Full file", exact: true }).click();
  477 |     const scroll = await page.evaluate(() => {
  478 |         const maximum = document.documentElement.scrollHeight - innerHeight;
  479 |         if (maximum < 300) throw new Error("the real source page must have room for a scroll-restoration check");
  480 |         const position = Math.min(500, maximum - 50);
  481 |         window.scrollTo(0, position);
  482 |         return position;
  483 |     });
  484 |     await expect.poll(() => page.evaluate(() => scrollY)).toBe(scroll);
  485 |     site.release();
  486 |     const secondState = await settled(firstState.revision);
  487 |     await newDocument(page, changed);
  488 |     await sourceText(page, changed);
  489 |     await ready(page, "11");
  490 |     await expect(source(page).getByRole("tab", { name: "Full file", exact: true })).toHaveAttribute("aria-selected", "true");
  491 |     await expect.poll(() => page.evaluate(() => scrollY)).toBe(scroll);
  492 |     expect(await page.evaluate(() => window.__dev_component_document)).toBeUndefined();
  493 |     const second = compilation(secondState);
  494 |     expect(second.completed_at_ms).toBeGreaterThan(first.completed_at_ms);
  495 |     expect(second.generation).not.toBe(first.generation);
  496 |     const secondWasm = audit.requests.filter(url => url.endsWith("/app_bg.wasm")).at(-1)!;
  497 |     expect(secondWasm).not.toBe(firstWasm);
  498 |     const group = secondWasm.slice(0, secondWasm.lastIndexOf("/") + 1);
  499 |     expect(audit.responses.some(response => response.url === `${group}app.js` && response.status === 200)).toBe(true);
  500 |     expect(audit.responses.some(response => response.url === secondWasm && response.status === 200 && response.mime.split(";")[0] === "application/wasm")).toBe(true);
  501 |     await page.getByTestId("counter-increment").first().click();
  502 |     await expect(values(page)).toHaveText(["5", "2", "2", "11"]);
  503 | 
  504 |     const bad = changed.replace("RwSignal::new(initial)", "RwSignal::new(initial + missing_counter_step)");
  505 |     const line = bad.slice(0, bad.indexOf("missing_counter_step")).split("\n").length;
  506 |     const column = bad.split("\n")[line - 1].indexOf("missing_counter_step") + 1;
  507 |     const previous = server.state()!.revision;
  508 |     await captureTrees(page);
  509 |     site.write("src/demos.rs", bad);
  510 |     const failed = await settled(previous, false);
  511 |     expect(failed.compile?.host.success).toBe(false);
  512 |     expect(failed.compile?.wasm.success).toBe(false);
  513 |     const errors = failed.diagnostics.filter(diagnostic => diagnostic.rule === "rust-compile" && diagnostic.file === "src/demos.rs" && diagnostic.line === line && diagnostic.column === column);
  514 |     expect(errors.length).toBeGreaterThan(0);
  515 |     const panel = page.locator(".rd-dev-panel");
  516 |     await expect(panel).toBeVisible();
> 517 |     await expect(panel.locator(".rd-dev-file")).toContainText("src/demos.rs");
      |                                                 ^ Error: expect(locator).toContainText(expected) failed
  518 |     await expect(panel.locator(".rd-dev-location").first()).toHaveText(`${line}:${column}`);
  519 |     await expect(panel).toContainText("missing_counter_step");
  520 |     await expect(panel).toContainText("showing the last good version");
  521 |     await retained(page);
  522 |     await sourceText(page, changed);
  523 |     await layouts(page, "rust-error");
  524 |     await panel.getByRole("button", { name: "Hide problems" }).click();
  525 |     await expect(panel).toBeHidden();
  526 |     await expect(page.locator(".rd-dev-marker")).toBeVisible();
  527 |     await page.getByTestId("counter-increment").first().click();
  528 |     await expect(values(page)).toHaveText(["8", "2", "2", "11"]);
  529 |     await page.evaluate(y => window.scrollTo(0, y), scroll);
  530 | 
  531 |     const repaired = changed.replace("*value += 3", "*value += 5").replace('        11,\n        "Résumé', '        13,\n        "Résumé');
  532 |     site.write("src/demos.rs", repaired);
  533 |     const fixedState = await settled(failed.revision);
  534 |     await newDocument(page, repaired);
  535 |     await sourceText(page, repaired);
  536 |     await ready(page, "13");
  537 |     await cleared(page, fixedState);
  538 |     await expect.poll(() => page.evaluate(() => scrollY)).toBe(scroll);
  539 |     await expect(source(page).getByRole("tab", { name: "Full file", exact: true })).toHaveAttribute("aria-selected", "true");
  540 |     const fixed = compilation(fixedState);
  541 |     expect(fixed.completed_at_ms).toBeGreaterThan(second.completed_at_ms);
  542 |     await page.getByTestId("counter-increment").first().click();
  543 |     await expect(values(page)).toHaveText(["7", "2", "2", "13"]);
  544 |     await healthy();
  545 | });
  546 | 
```