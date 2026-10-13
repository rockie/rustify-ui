# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: docs-ssg/dev-components.spec.ts >> Rust saves keep the original interactive document until a complete new pair, and diagnostics recover at its scroll position
- Location: tests/docs-ssg/dev-components.spec.ts:386:5

# Error details

```
Error: expect(locator).toHaveText(expected) failed

Locator: getByTestId('counter-value')
Timeout: 10000ms
- Expected  - 2
+ Received  + 2

  Array [
-   "2",
+   "31",
    "2",
    "2",
-   "11",
+   "7",
  ]

Call log:
  - Expect "toHaveText" getByTestId('counter-value') with timeout 10000ms
  - waiting for getByTestId('counter-value')
    24 × locator resolved to 4 elements

```

# Page snapshot

```yaml
- generic [ref=f1e1]:
  - link "Skip to content" [ref=f1e2] [cursor=pointer]:
    - /url: "#content"
  - banner [ref=f1e3]:
    - generic [ref=f1e4]:
      - generic [ref=f1e5]: Reading laboratory
      - navigation "Site links" [ref=f1e7]:
        - list [ref=f1e8]:
          - listitem [ref=f1e9]:
            - link "Counters" [ref=f1e10] [cursor=pointer]:
              - /url: /docs/counter/
          - listitem [ref=f1e11]:
            - link "Catalog proxy" [ref=f1e12] [cursor=pointer]:
              - /url: /docs/proxy/
      - group "Theme" [ref=f1e13]:
        - button "System" [pressed] [ref=f1e14] [cursor=pointer]
        - button "Light" [ref=f1e20] [cursor=pointer]
        - button "Dark" [ref=f1e26] [cursor=pointer]
  - generic [ref=f1e31]:
    - generic [ref=f1e32]:
      - paragraph [ref=f1e34]: Examples in this guide
      - navigation "Menu" [ref=f1e35]:
        - generic [ref=f1e36]:
          - heading "Custom pages" [level=2] [ref=f1e37]
          - list [ref=f1e38]:
            - listitem [ref=f1e39]:
              - link "Custom page" [ref=f1e40] [cursor=pointer]:
                - /url: /docs/custom/
            - listitem [ref=f1e41]:
              - link "Static custom page" [ref=f1e42] [cursor=pointer]:
                - /url: /docs/plain/
        - generic [ref=f1e43]:
          - heading "Examples" [level=2] [ref=f1e44]
          - list [ref=f1e45]:
            - listitem [ref=f1e46]:
              - link "Registered counters" [ref=f1e47] [cursor=pointer]:
                - /url: /docs/counter/
            - listitem [ref=f1e48]:
              - link "Static built-in components" [ref=f1e49] [cursor=pointer]:
                - /url: /docs/builtins/
            - listitem [ref=f1e50]:
              - link "Browser work after attachment" [ref=f1e51] [cursor=pointer]:
                - /url: /docs/async/
            - listitem [ref=f1e52]:
              - link "Client-only counter" [ref=f1e53] [cursor=pointer]:
                - /url: /docs/client-only/
            - listitem [ref=f1e54]:
              - link "Runtime failure recovery" [ref=f1e55] [cursor=pointer]:
                - /url: /docs/failure/
            - listitem [ref=f1e56]:
              - link "Catalog release proxy" [ref=f1e57] [cursor=pointer]:
                - /url: /docs/proxy/
      - paragraph [ref=f1e59]: Each example keeps its source nearby
    - navigation "On this page" [ref=f1e61]:
      - paragraph [ref=f1e62]: On this page
      - list [ref=f1e63]:
        - listitem [ref=f1e64]:
          - link "Three independent instances" [ref=f1e65] [cursor=pointer]:
            - /url: "#three-independent-instances"
        - listitem [ref=f1e66]:
          - link "Function and full file" [ref=f1e67] [cursor=pointer]:
            - /url: "#function-and-full-file"
    - main [ref=f1e68]:
      - generic [ref=f1e69]:
        - paragraph [ref=f1e72]: Try the example, then read its implementation
        - article [ref=f1e73]:
          - heading "Registered counters" [level=1] [ref=f1e74]
          - heading "Three independent instances" [level=2] [ref=f1e75]
          - paragraph [ref=f1e76]: Each counter starts with the same props and child content. Incrementing one leaves the others at two.
          - generic [ref=f1e77]:
            - group "Interactive example" [ref=f1e78]:
              - generic [ref=f1e81]:
                - paragraph [ref=f1e82]: Shared initial props
                - generic [ref=f1e83]:
                  - button "Increment" [ref=f1e84] [cursor=pointer]
                  - status [ref=f1e85]: "31"
                  - textbox "Counter value" [ref=f1e86]: "31"
                - paragraph [ref=f1e88]:
                  - text: The same
                  - strong [ref=f1e89]: safe child Markdown
                  - text: appears in each instance.
            - status [ref=f1e90]:
              - generic [ref=f1e91]: Interactive example
          - generic [ref=f1e92]:
            - group "Interactive example" [ref=f1e93]:
              - generic [ref=f1e96]:
                - paragraph [ref=f1e97]: Shared initial props
                - generic [ref=f1e98]:
                  - button "Increment" [ref=f1e99] [cursor=pointer]
                  - status [ref=f1e100]: "2"
                  - textbox "Counter value" [ref=f1e101]: "2"
                - paragraph [ref=f1e103]:
                  - text: The same
                  - strong [ref=f1e104]: safe child Markdown
                  - text: appears in each instance.
            - status [ref=f1e105]:
              - generic [ref=f1e106]: Interactive example
          - generic [ref=f1e107]:
            - group "Interactive example" [ref=f1e108]:
              - generic [ref=f1e111]:
                - paragraph [ref=f1e112]: Shared initial props
                - generic [ref=f1e113]:
                  - button "Increment" [ref=f1e114] [cursor=pointer]
                  - status [ref=f1e115]: "2"
                  - textbox "Counter value" [ref=f1e116]: "2"
                - paragraph [ref=f1e118]:
                  - text: The same
                  - strong [ref=f1e119]: safe child Markdown
                  - text: appears in each instance.
            - status [ref=f1e120]:
              - generic [ref=f1e121]: Interactive example
          - heading "Function and full file" [level=2] [ref=f1e122]
          - paragraph [ref=f1e123]: The registered function starts at seven. Its original function and containing file appear below the view.
          - generic [ref=f1e124]:
            - group "Interactive example" [ref=f1e125]:
              - generic [ref=f1e128]:
                - paragraph [ref=f1e129]: Résumé — a real registered counter
                - generic [ref=f1e130]:
                  - button "Increment" [ref=f1e131] [cursor=pointer]
                  - status [ref=f1e132]: "7"
                  - textbox "Counter value" [ref=f1e133]: "7"
                - paragraph [ref=f1e134]: This child is part of the same view on both targets.
            - status [ref=f1e135]:
              - generic [ref=f1e136]: Interactive example
            - generic [ref=f1e137]:
              - paragraph [ref=f1e138]: src/demos.rs
              - generic [ref=f1e139]:
                - generic [ref=f1e140]:
                  - tablist [ref=f1e141]:
                    - tab "Function" [ref=f1e142] [cursor=pointer]
                    - tab "Full file" [active] [selected] [ref=f1e143] [cursor=pointer]
                  - button "Copy" [ref=f1e144] [cursor=pointer]
                - tabpanel "Full file" [ref=f1e149]:
                  - code [ref=f1e151]:
                    - generic [ref=f1e153]:
                      - text: use
                      - generic [ref=f1e154]: "leptos::"
                      - generic [ref=f1e155]: "prelude::"
                      - text: "*;"
                    - generic [ref=f1e157]:
                      - text: use
                      - generic [ref=f1e158]: "rustify_components::"
                      - generic [ref=f1e159]: "{Button, TextField"
                      - generic [ref=f1e160]: "}"
                      - text: ;
                    - generic [ref=f1e162]:
                      - generic [ref=f1e163]:
                        - generic [ref=f1e164]: pub fn
                        - text: counter
                      - generic [ref=f1e165]:
                        - generic [ref=f1e166]: "(initial: i32, label: String, children: AnyView"
                        - generic [ref=f1e167]: )
                      - generic [ref=f1e169]: "-> impl IntoView"
                      - generic [ref=f1e171]: "{"
                    - generic [ref=f1e176]:
                      - text: let value =
                      - generic [ref=f1e177]: "RwSignal::"
                      - text: new
                      - generic [ref=f1e178]: (initial
                      - generic [ref=f1e179]: )
                      - text: ;
                    - generic [ref=f1e183]:
                      - text: view!
                      - generic [ref=f1e184]: "{"
                    - generic [ref=f1e189]:
                      - text: <div data-counter=
                      - generic [ref=f1e190]: "\"\""
                      - text: class=
                      - generic [ref=f1e191]: "\"min-h-24 space-y-3\""
                      - text: ">"
                    - generic [ref=f1e196]:
                      - text: <p>
                      - generic [ref=f1e197]: "{label"
                      - generic [ref=f1e198]: "}"
                      - text: </p>
                    - generic [ref=f1e203]:
                      - text: <div class=
                      - generic [ref=f1e204]: "\"flex flex-wrap items-center gap-3\""
                      - text: ">"
                    - generic [ref=f1e209]:
                      - text: <Button test_id=
                      - generic [ref=f1e210]: "\"counter-increment\""
                      - text: on_click=move || value.update
                      - generic [ref=f1e211]:
                        - text: (
                        - generic [ref=f1e212]: "|"
                        - generic [ref=f1e214]: value|
                        - generic [ref=f1e216]: "*value += 1"
                      - generic [ref=f1e217]: )
                      - text: ">"
                    - generic [ref=f1e218]: "\"Increment\""
                    - generic [ref=f1e224]: </Button>
                    - generic [ref=f1e233]:
                      - text: <output data-testid=
                      - generic [ref=f1e234]: "\"counter-value\""
                      - text: ">"
                      - generic [ref=f1e235]:
                        - text: "{move || value.get"
                        - generic [ref=f1e236]: (
                        - generic [ref=f1e237]: )
                      - generic [ref=f1e238]: "}"
                      - text: </output>
                    - generic [ref=f1e243]:
                      - text: <TextField test_id=
                      - generic [ref=f1e244]: "\"counter-input\""
                      - text: value=
                      - generic [ref=f1e245]: "Signal::"
                      - text: derive
                      - generic [ref=f1e246]:
                        - text: (move || value.get
                        - generic [ref=f1e247]: (
                        - generic [ref=f1e248]: )
                        - text: .to_string
                        - generic [ref=f1e249]: (
                        - generic [ref=f1e250]: )
                      - generic [ref=f1e251]: )
                    - generic [ref=f1e256]:
                      - text: "on_change=move |input: String|"
                      - generic [ref=f1e257]: "{"
                    - generic [ref=f1e263]:
                      - text: if let Ok
                      - generic [ref=f1e264]: (input
                      - generic [ref=f1e265]: )
                      - text: = input.parse
                      - generic [ref=f1e266]: (
                      - generic [ref=f1e267]: )
                      - generic [ref=f1e268]:
                        - text: "{ value.set"
                        - generic [ref=f1e269]: (input
                        - generic [ref=f1e270]: )
                        - text: ;
                      - generic [ref=f1e271]: "}"
                    - generic [ref=f1e272]: "}"
                    - generic [ref=f1e282]:
                      - text: aria_label=
                      - generic [ref=f1e283]: "\"Counter value\""
                    - generic [ref=f1e284]: />
                    - generic [ref=f1e289]: </div>
                    - generic [ref=f1e298]:
                      - generic [ref=f1e299]: "{children"
                      - generic [ref=f1e300]: "}"
                    - generic [ref=f1e301]: </div>
                    - generic [ref=f1e306]: "}"
                    - generic [ref=f1e311]: "}"
                    - generic [ref=f1e315]: /// The function and its containing file are displayed verbatim by the docs.
                    - generic [ref=f1e319]:
                      - generic [ref=f1e320]:
                        - generic [ref=f1e321]: pub fn
                        - text: counter_demo
                      - generic [ref=f1e322]:
                        - generic [ref=f1e323]: (
                        - generic [ref=f1e324]: )
                      - generic [ref=f1e326]: "-> impl IntoView"
                      - generic [ref=f1e328]: "{"
                    - generic [ref=f1e333]:
                      - text: counter
                      - generic [ref=f1e334]: (
                    - generic [ref=f1e335]: 7,
                    - generic [ref=f1e344]:
                      - generic [ref=f1e345]: "\"Résumé — a real registered counter\""
                      - text: .to_string
                      - generic [ref=f1e346]: (
                      - generic [ref=f1e347]: )
                      - text: ","
                    - generic [ref=f1e352]:
                      - text: view!
                      - generic [ref=f1e353]:
                        - text: "{ <p>"
                        - generic [ref=f1e354]: "\"This child is part of the same view on both targets.\""
                        - text: </p>
                      - generic [ref=f1e355]: "}"
                      - text: .into_any
                      - generic [ref=f1e356]: (
                      - generic [ref=f1e357]: )
                      - text: ","
                    - generic [ref=f1e358]: )
                    - generic [ref=f1e363]: "}"
                    - generic [ref=f1e368]:
                      - generic [ref=f1e369]:
                        - generic [ref=f1e370]: pub fn
                        - text: async_after_attach_demo
                      - generic [ref=f1e371]:
                        - generic [ref=f1e372]: (
                        - generic [ref=f1e373]: )
                      - generic [ref=f1e375]: "-> impl IntoView"
                      - generic [ref=f1e377]: "{"
                    - generic [ref=f1e379]: "// No asynchronous initial data: the action starts only after a reader clicks."
                    - generic [ref=f1e387]:
                      - text: let load =
                      - generic [ref=f1e388]: "Action::"
                      - text: new
                      - generic [ref=f1e389]:
                        - text: (
                        - generic [ref=f1e390]: "|"
                        - generic [ref=f1e393]:
                          - text: "_: &"
                          - generic [ref=f1e394]: (
                          - generic [ref=f1e395]: )
                          - text: "|"
                        - generic [ref=f1e396]:
                          - text: async
                          - generic [ref=f1e397]:
                            - text: "{"
                            - generic [ref=f1e398]: "\"Loaded after attachment\""
                            - text: .to_string
                            - generic [ref=f1e399]: (
                            - generic [ref=f1e400]: )
                          - generic [ref=f1e401]: "}"
                      - generic [ref=f1e402]: )
                      - text: ;
                    - generic [ref=f1e406]:
                      - text: view!
                      - generic [ref=f1e407]: "{"
                    - generic [ref=f1e412]:
                      - text: <div data-async-after-attach=
                      - generic [ref=f1e413]: "\"\""
                      - text: class=
                      - generic [ref=f1e414]: "\"space-y-3\""
                      - text: ">"
                    - generic [ref=f1e419]:
                      - text: <Button test_id=
                      - generic [ref=f1e420]: "\"async-start\""
                      - text: on_click=move ||
                      - generic [ref=f1e421]:
                        - text: "{ load.dispatch"
                        - generic [ref=f1e422]:
                          - text: (
                          - generic [ref=f1e423]: (
                          - generic [ref=f1e424]: )
                        - generic [ref=f1e425]: )
                        - text: ;
                      - generic [ref=f1e426]: "}"
                      - text: ">"
                    - generic [ref=f1e427]: "\"Load browser data\""
                    - generic [ref=f1e433]: </Button>
                    - generic [ref=f1e442]:
                      - text: <output data-testid=
                      - generic [ref=f1e443]: "\"async-result\""
                      - text: ">"
                    - generic [ref=f1e448]:
                      - generic [ref=f1e449]:
                        - text: "{move || load.value"
                        - generic [ref=f1e450]: (
                        - generic [ref=f1e451]: )
                        - text: .get
                        - generic [ref=f1e452]: (
                        - generic [ref=f1e453]: )
                        - text: .unwrap_or_else
                        - generic [ref=f1e454]:
                          - text: (
                          - generic [ref=f1e455]: "|"
                          - generic [ref=f1e457]: "|"
                          - generic [ref=f1e459]:
                            - generic [ref=f1e460]: "\"Not loaded\""
                            - text: .to_string
                            - generic [ref=f1e461]: (
                            - generic [ref=f1e462]: )
                        - generic [ref=f1e463]: )
                      - generic [ref=f1e464]: "}"
                    - generic [ref=f1e465]: </output>
                    - generic [ref=f1e470]: </div>
                    - generic [ref=f1e475]: "}"
                    - generic [ref=f1e480]: "}"
                    - generic [ref=f1e485]:
                      - generic [ref=f1e486]:
                        - generic [ref=f1e487]: pub fn
                        - text: failure_demo
                      - generic [ref=f1e488]:
                        - generic [ref=f1e489]: (
                        - generic [ref=f1e490]: )
                      - generic [ref=f1e492]: "-> impl IntoView"
                      - generic [ref=f1e494]: "{"
                    - generic [ref=f1e499]:
                      - text: let value =
                      - generic [ref=f1e500]: "RwSignal::"
                      - text: new
                      - generic [ref=f1e501]: (0
                      - generic [ref=f1e502]: )
                      - text: ;
                    - generic [ref=f1e506]:
                      - text: view!
                      - generic [ref=f1e507]: "{"
                    - generic [ref=f1e512]:
                      - text: <div data-failure-demo=
                      - generic [ref=f1e513]: "\"\""
                      - text: class=
                      - generic [ref=f1e514]: "\"space-y-3\""
                      - text: ">"
                    - generic [ref=f1e519]:
                      - text: <Button test_id=
                      - generic [ref=f1e520]: "\"failure-increment\""
                      - text: on_click=move || value.update
                      - generic [ref=f1e521]:
                        - text: (
                        - generic [ref=f1e522]: "|"
                        - generic [ref=f1e524]: value|
                        - generic [ref=f1e526]: "*value += 1"
                      - generic [ref=f1e527]: )
                      - text: ">"
                    - generic [ref=f1e528]: "\"Change before failure\""
                    - generic [ref=f1e534]: </Button>
                    - generic [ref=f1e543]:
                      - text: <output data-testid=
                      - generic [ref=f1e544]: "\"failure-value\""
                      - text: ">"
                      - generic [ref=f1e545]:
                        - text: "{move || value.get"
                        - generic [ref=f1e546]: (
                        - generic [ref=f1e547]: )
                      - generic [ref=f1e548]: "}"
                      - text: </output>
                    - generic [ref=f1e553]:
                      - text: <Button test_id=
                      - generic [ref=f1e554]: "\"failure-crash\""
                      - text: on_click=
                      - generic [ref=f1e555]: "|"
                      - generic [ref=f1e557]: "|"
                      - generic [ref=f1e559]:
                        - text: panic!
                        - generic [ref=f1e560]:
                          - text: (
                          - generic [ref=f1e561]: "\"docs fixture callback failure\""
                        - generic [ref=f1e562]: )
                        - text: ">"
                    - generic [ref=f1e563]: "\"Crash this example\""
                    - generic [ref=f1e569]: </Button>
                    - generic [ref=f1e574]: </div>
                    - generic [ref=f1e579]: "}"
                    - generic [ref=f1e584]: "}"
                    - generic [ref=f1e589]:
                      - generic [ref=f1e590]:
                        - generic [ref=f1e591]: pub fn
                        - text: catalog_proxy
                      - generic [ref=f1e592]:
                        - generic [ref=f1e593]: (
                        - generic [ref=f1e594]: )
                      - generic [ref=f1e596]: "-> impl IntoView"
                      - generic [ref=f1e598]: "{"
                    - generic [ref=f1e603]:
                      - text: let scope =
                      - generic [ref=f1e604]: "rustify_ui::"
                      - generic [ref=f1e605]: "id::"
                      - text: next
                      - generic [ref=f1e606]:
                        - text: (
                        - generic [ref=f1e607]: "\"catalog\""
                      - generic [ref=f1e608]: )
                      - text: .unwrap_or_else
                      - generic [ref=f1e609]:
                        - text: (
                        - generic [ref=f1e610]: "|"
                        - generic [ref=f1e612]: "|"
                        - generic [ref=f1e614]:
                          - generic [ref=f1e615]: "\"catalog\""
                          - text: .to_string
                          - generic [ref=f1e616]: (
                          - generic [ref=f1e617]: )
                      - generic [ref=f1e618]: )
                      - text: ;
                    - generic [ref=f1e622]:
                      - text: view!
                      - generic [ref=f1e623]:
                        - text: "{ <crate"
                        - generic [ref=f1e624]: "::"
                        - generic [ref=f1e625]: "catalog::"
                        - text: CatalogView scope=scope />
                      - generic [ref=f1e626]: "}"
                    - generic [ref=f1e627]: "}"
        - paragraph [ref=f1e632]: Continue with the next page
        - navigation "Previous / Next" [ref=f1e633]:
          - link "Previous Static custom page" [ref=f1e634] [cursor=pointer]:
            - /url: /docs/plain/
            - generic [ref=f1e635]: Previous
            - generic [ref=f1e636]: Static custom page
          - link "Next Static built-in components" [ref=f1e637] [cursor=pointer]:
            - /url: /docs/builtins/
            - generic [ref=f1e638]: Next
            - generic [ref=f1e639]: Static built-in components
  - contentinfo [ref=f1e640]:
    - paragraph [ref=f1e642]:
      - text: Keep reading ·
      - link "Home" [ref=f1e643] [cursor=pointer]:
        - /url: /docs/
  - paragraph [ref=f1e644]
  - status [ref=f1e645]: Building · Compiling native renderer · 4 warnings · showing the last good version
  - button "Building · Compiling native renderer · 4 warnings · showing the last good version" [ref=f1e647] [cursor=pointer]
```

# Test source

```ts
  100 |     static async start(site: Site): Promise<Dev> {
  101 |         const child = spawn(bin, ["dev", "--port", "0"], {
  102 |             cwd: site.root, detached: true, env: { ...process.env, RUSTIFY_DOCS_CARGO: site.compiler },
  103 |         });
  104 |         let log = "";
  105 |         child.stderr!.on("data", chunk => { log += chunk; });
  106 |         const url = await new Promise<string>((resolve, reject) => {
  107 |             let out = "";
  108 |             child.stdout!.on("data", chunk => {
  109 |                 out += chunk;
  110 |                 const match = /^dev: (\S+)\n/m.exec(out);
  111 |                 if (match) resolve(match[1]);
  112 |             });
  113 |             child.on("error", reject);
  114 |             child.on("exit", code => reject(new Error(`dev exited ${code}: ${out}${log}`)));
  115 |         });
  116 |         const server = new Dev(child, url);
  117 |         server.log = log;
  118 |         child.stderr!.on("data", chunk => { server.log += chunk; });
  119 |         const eventUrl = new URL("/__rustify-docs/events", url);
  120 |         eventUrl.searchParams.set("path", new URL("counter/", url).pathname);
  121 |         eventUrl.searchParams.set("good", "0");
  122 |         server.events = get(eventUrl, response => {
  123 |             if (response.statusCode !== 200) server.streamErrors.push(`SSE response ${response.statusCode}`);
  124 |             response.setEncoding("utf8");
  125 |             let pending = "";
  126 |             response.on("data", chunk => {
  127 |                 pending += chunk;
  128 |                 let end: number;
  129 |                 while ((end = pending.indexOf("\n\n")) !== -1) {
  130 |                     const message = pending.slice(0, end);
  131 |                     pending = pending.slice(end + 2);
  132 |                     if (message.startsWith("event: state\n")) server.states.push(JSON.parse(message.slice("event: state\ndata: ".length)));
  133 |                 }
  134 |             });
  135 |             response.on("error", error => { if (!server.stopping) server.streamErrors.push(String(error)); });
  136 |         });
  137 |         server.events.on("error", error => { if (!server.stopping) server.streamErrors.push(String(error)); });
  138 |         return server;
  139 |     }
  140 | 
  141 |     state(): State | undefined { return this.states.at(-1); }
  142 | 
  143 |     async stop() {
  144 |         this.stopping = true;
  145 |         this.events?.destroy();
  146 |         if (this.child.exitCode !== null || this.child.signalCode !== null) return;
  147 |         const exited = new Promise(resolve => this.child.once("exit", resolve));
  148 |         // The compiler wrapper and its actual compiler share this new process
  149 |         // group; a failed assertion must not leave a build running afterwards.
  150 |         process.kill(-this.child.pid!, "SIGTERM");
  151 |         await exited;
  152 |     }
  153 | }
  154 | 
  155 | let site: Site;
  156 | let server: Dev | undefined;
  157 | let audit: Audit;
  158 | 
  159 | test.beforeEach(async ({ page }) => {
  160 |     site = new Site();
  161 |     server = undefined;
  162 |     audit = { requests: [], failures: [], expectedStreamClosures: [], responses: [], console: [], pageErrors: [], csp: [] };
  163 |     await page.exposeBinding("__dev_component_csp", (_, value: string) => audit.csp.push(value));
  164 |     await page.addInitScript(() => {
  165 |         document.addEventListener("securitypolicyviolation", event => window.__dev_component_csp(`${event.violatedDirective} ${event.blockedURI}`));
  166 |     });
  167 |     page.on("request", request => audit.requests.push(request.url()));
  168 |     page.on("requestfailed", request => {
  169 |         const message = `${request.url()} ${request.failure()?.errorText}`;
  170 |         if (new URL(request.url()).pathname === "/__rustify-docs/events" && request.failure()?.errorText === "net::ERR_ABORTED") audit.expectedStreamClosures.push(message);
  171 |         else audit.failures.push(message);
  172 |     });
  173 |     page.on("response", response => audit.responses.push({ url: response.url(), status: response.status(), mime: response.headers()["content-type"] || "" }));
  174 |     page.on("console", message => { if (message.type() === "error") audit.console.push(message.text()); });
  175 |     page.on("pageerror", error => audit.pageErrors.push(String(error)));
  176 | });
  177 | 
  178 | test.afterEach(async ({}, info) => {
  179 |     try {
  180 |         await info.attach("dev-component-browser-audit.json", { body: JSON.stringify(audit, null, 2), contentType: "application/json" });
  181 |         await info.attach("dev-component-states.json", { body: JSON.stringify(server?.states || [], null, 2), contentType: "application/json" });
  182 |         await info.attach("dev-component-compiler-commands.json", { body: JSON.stringify(site.commands(), null, 2), contentType: "application/json" });
  183 |         await info.attach("dev-component-cli.log", { body: server?.log || "", contentType: "text/plain" });
  184 |         const history = path.join(site.root, ".rustify-docs/logs/history");
  185 |         if (existsSync(history)) {
  186 |             const timings = readdirSync(history).map(directory => JSON.parse(readFileSync(path.join(history, directory, "timing.json"), "utf8")));
  187 |             await info.attach("dev-component-compile-history.json", { body: JSON.stringify(timings, null, 2), contentType: "application/json" });
  188 |         }
  189 |     } finally {
  190 |         try { await server?.stop(); } finally { site.remove(); }
  191 |     }
  192 | });
  193 | 
  194 | const islands = (page: Page) => page.locator("[data-rustify-island]");
  195 | const source = (page: Page) => page.locator(".rd-demo-source");
  196 | const values = (page: Page) => page.getByTestId("counter-value");
  197 | 
  198 | async function ready(page: Page, demoValue = "7") {
  199 |     await expect(page.locator('[data-rustify-island][data-rd-island-state="ready"]')).toHaveCount(4, { timeout: buildTimeout });
> 200 |     await expect(values(page)).toHaveText(["2", "2", "2", demoValue]);
      |                                ^ Error: expect(locator).toHaveText(expected) failed
  201 |     for (const island of await islands(page).all()) {
  202 |         const metadata = JSON.parse((await island.getAttribute("data-rustify-island-metadata"))!);
  203 |         const field = island.locator("fieldset[data-rd-island-controls]");
  204 |         await expect(field).toHaveJSProperty("disabled", false);
  205 |         await expect(field).toHaveJSProperty("inert", false);
  206 |         await expect(field.locator(":scope > [data-rustify-scope]")).toHaveAttribute("data-rustify-mounted", metadata.seed);
  207 |         await expect(field.locator("[style]")).toHaveCount(0);
  208 |     }
  209 |     expect(await page.evaluate(() => {
  210 |         const ids = [...document.querySelectorAll("[id]")].map(node => node.id);
  211 |         return new Set(ids).size === ids.length;
  212 |     })).toBe(true);
  213 |     await page.evaluate(() => document.fonts.ready);
  214 | }
  215 | 
  216 | async function sourceText(page: Page, original: string) {
  217 |     const fn = original.match(/(?:\/\/\/[^\r\n]*\r?\n)*pub fn counter_demo\([^)]*\)[\s\S]*?^\}(?:\r?\n|$)/m)?.[0];
  218 |     expect(fn).toBeTruthy();
  219 |     expect(await source(page).locator("template.rd-code-raw").evaluateAll(nodes => nodes.map(node => (node as HTMLTemplateElement).content.textContent))).toEqual([fn, original]);
  220 | }
  221 | 
  222 | async function captureTrees(page: Page) {
  223 |     await page.evaluate(() => {
  224 |         const nodes = (root: Node) => {
  225 |             const items: Node[] = [root];
  226 |             const walker = document.createTreeWalker(root, NodeFilter.SHOW_ALL);
  227 |             while (walker.nextNode()) items.push(walker.currentNode);
  228 |             return items;
  229 |         };
  230 |         const source = document.querySelector(".rd-demo-source")!;
  231 |         window.__dev_component_trees = [...document.querySelectorAll("[data-rustify-island]")].map(block => {
  232 |             const root = block.querySelector("fieldset[data-rd-island-controls]")!.firstElementChild!;
  233 |             return { root, nodes: nodes(root), ids: [...root.querySelectorAll("[id]")].map(node => [node, node.id] as [Element, string]), source, sourceNodes: nodes(source), values: [...root.querySelectorAll("input")].map(node => node.value) };
  234 |         });
  235 |         window.__dev_component_document = "the actual last good document";
  236 |     });
  237 | }
  238 | 
  239 | async function retained(page: Page) {
  240 |     const records = await page.evaluate(() => window.__dev_component_trees!.map(tree => ({
  241 |         nodes: tree.nodes.length, retained: tree.nodes.filter(node => node.isConnected && tree.root.contains(node)).length,
  242 |         root: [...document.querySelectorAll("fieldset[data-rd-island-controls]")].some(field => field.firstElementChild === tree.root),
  243 |         ids: tree.ids.every(([node, id]) => node.id === id),
  244 |         source: document.querySelector(".rd-demo-source") === tree.source,
  245 |         sourceNodes: tree.sourceNodes.every(node => node.isConnected && tree.source.contains(node)),
  246 |         values: [...tree.root.querySelectorAll("input")].map(node => node.value), originalValues: tree.values,
  247 |     })));
  248 |     expect(records).toHaveLength(4);
  249 |     for (const record of records) {
  250 |         expect(record.retained).toBe(record.nodes);
  251 |         expect(record.root && record.ids && record.source && record.sourceNodes).toBe(true);
  252 |         expect(record.values).toEqual(record.originalValues);
  253 |     }
  254 |     expect(await page.evaluate(() => window.__dev_component_document)).toBe("the actual last good document");
  255 | }
  256 | 
  257 | async function settled(after = -1, ok = true): Promise<State> {
  258 |     await expect.poll(() => {
  259 |         const state = server!.state();
  260 |         return !!state && state.revision > after && !state.building && state.ok === ok;
  261 |     }, { timeout: buildTimeout }).toBe(true);
  262 |     return server!.state()!;
  263 | }
  264 | 
  265 | function compilation(state: State) {
  266 |     expect(state.compile).not.toBeNull();
  267 |     const compiled = state.compile!;
  268 |     expect(compiled.completed_at_ms).toBe(Math.max(compiled.host.completed_at_ms, compiled.wasm.completed_at_ms));
  269 |     expect(compiled.host.profile).toBe("docs-host");
  270 |     expect(compiled.wasm.profile).toBe("docs-wasm");
  271 |     expect(compiled.host.target).toBeNull();
  272 |     expect(compiled.wasm.target).toBe("wasm32-unknown-unknown");
  273 |     for (const timing of [compiled.host, compiled.wasm]) {
  274 |         expect(timing.success).toBe(true);
  275 |         expect(timing.flags).toContain("--locked");
  276 |         expect(timing.elapsed_ms).toBeGreaterThanOrEqual(0);
  277 |         expect(timing.fresh_artifacts + timing.compiled_artifacts).toBeGreaterThan(0);
  278 |     }
  279 |     return compiled;
  280 | }
  281 | 
  282 | async function layouts(page: Page, state: string) {
  283 |     const viewport = page.viewportSize()!;
  284 |     const position = await page.evaluate(() => scrollY);
  285 |     const preference = await page.locator("html").getAttribute("data-theme") || "system";
  286 |     for (const variant of [
  287 |         { name: "desktop-light", mode: "light", width: 1280, height: 900 },
  288 |         { name: "desktop-dark", mode: "dark", width: 1280, height: 900 },
  289 |         { name: "narrow-light", mode: "light", width: 375, height: 812 },
  290 |     ]) {
  291 |         await page.setViewportSize({ width: variant.width, height: variant.height });
  292 |         await page.locator(`[data-rd-mode="${variant.mode}"]`).click();
  293 |         await page.evaluate(async () => {
  294 |             await document.fonts.ready;
  295 |             await Promise.allSettled(document.getAnimations().filter(animation => animation instanceof CSSTransition).map(animation => animation.finished));
  296 |             await new Promise<void>(resolve => requestAnimationFrame(() => requestAnimationFrame(() => resolve())));
  297 |         });
  298 |         await expect(page.locator("html")).toHaveAttribute("data-theme", variant.mode);
  299 |         expect(await page.evaluate(() => document.documentElement.scrollWidth)).toBeLessThanOrEqual(variant.width);
  300 |         await test.info().attach(`${state}-${variant.name}.png`, { body: await page.screenshot(), contentType: "image/png" });
```