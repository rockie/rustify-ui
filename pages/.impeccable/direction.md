# Rustify UI landing surface

Mode: Persuade. Scope: pages/ only. Audience: Chinese-speaking Rust and web developers evaluating the SDK. The user selected Chinese and explicitly delegated visual choices, requesting phoenix-ui with ui-loop. The source prototype carries the layout decisions; no image comp was approved. Phoenix engine context failed because its pinned binary is absent; project originals, design references and actual browser evidence are the fallback sources.

## Direction contract

THESIS: 用 Rust 构建复杂界面。 Make the DOM/GPU responsibility split immediately readable, then show the real repository example.

OWN-WORLD: Cool daylight ground #f5f7fb; ink #16243b; cobalt #2348e5; mint #c9f3dd. Sora for Latin and locally hosted Noto Sans SC for Chinese, with large, direct, upright headings. Large quiet fields and fine dividers; no decorative 3D or invented metrics.

STORY: Understand the SDK, see the native DOM and optional GPU roles, inspect Theme Studio, follow the GitHub Pages docs or inspect the repository.

FIRST VIEWPORT: Brand-only header; large left-aligned H1 and concise explanatory copy; visible documentation and source actions. A cobalt DOM/GPU construction panel follows, with both renderer names visible together on phones. The primary action is 阅读中文文档。

FORM: Technical construction kit, grounded candidate 3; concept seed 0a92a3b2. Code-led source prototype; no approved raster comp. The comparative reference is the actual Lusion v3 SOTD image, with product facts and interface proof drawn from this repository.

FINISH: unreviewed and undocumented is unfinished; this build ends with the finish review, the verdict, DESIGN.md, and every shipping raster carrying its provenance

## Comparative evidence

- https://www.awwwards.com/sites/lusion-v3 — read the actual award entry (SOTD, 2023-10-02) and viewed its original desktop capture: https://assets.awwwards.com/awards/sites_of_the_day/2023/09/lusion-1.jpg. Borrowed its decisive cobalt/neutral contrast, direct typography and singular focal region; its 3D visuals are not copied. The live lusion.co animation did not finish loading in the background browser, so its motion was not used as verified evidence.
- Source product proof: docs/validation/leptos-0.8.22/theme-studio.png at 032756c041615dca075a49685fbb458ce7cc833f; copied with origin metadata, unchanged visually.

## Iteration

First production screenshots showed the mobile DOM/GPU construction panel stacking the two roles too far apart. The second source version places them side by side and reduces the mobile headline. A 320px follow-up exposed DOM lettering wrapping inside its column; the renderer labels now use a smaller mobile minimum and stay on one line. Actual browser checks and screenshots live in review/.

## Build context

The local source CLI is rustify-docs 0.2.1, source 032756c041615dca075a49685fbb458ce7cc833f, panic unwind. No site Rust or wasm is compiled. New landing deployment URL is unknown; missing-site-url is retained rather than inventing a canonical address. Existing documentation destination is https://rockie.github.io/rustify-ui/zh/.

## Finish-review corrections

Added a local Noto Sans SC subset for the current Chinese copy and 404. DOM/GPU blocks now top-align independently of description length, with the connector centered separately. The collaboration panel coordinates a mint connector response and a 4px action-arrow movement on hover/focus; reduced motion disables the movement, and keyboard focus has a contrasting mint outline.
