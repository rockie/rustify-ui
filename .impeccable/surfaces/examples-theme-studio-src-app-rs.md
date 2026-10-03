---
version: 1
slug: "examples-theme-studio-src-app-rs"
primary_target: "examples/theme-studio/src/app.rs"
related_targets: ["examples/theme-studio/app.css","examples/theme-studio/index.html"]
---

Mode: Operate. Target: examples/theme-studio. User task: select a preset, edit shared theme tokens, compare real DOM/GPU controls, save locally and export. The approved scope is docs/plan/THEME-STUDIO.md, including offline use, stable editing tools and separate preview scopes. All preset colors remain authored; user contrast warnings stay visible.

## Direction contract

THESIS: A theme workbench where one resolved snapshot drives two renderers, and controls stay readable throughout editing. Follow the approved tweakcn two-column editor structure.

OWN-WORLD: Rustify neutral light shell, IBM Plex Sans, restrained dividers and blue focus/selection. Preview surfaces own the chosen palette, font, spacing and shadow. Existing Rustify controls supply the visual language.

STORY: Choose a preset, edit a token, inspect its DOM and GPU use, then save or export. Alpha and font fallback remain visible facts.

FIRST VIEWPORT: Compact top action bar; left 320px editing column with preset search and primary visible immediately; right expansive preview with scene and renderer controls. At 390px, Edit/Preview tabs retain the action bar and focus path. The signature interaction is synchronized DOM/GPU comparison without remounting the canvas.

FORM: Approved two-column tool layout from plan §6.1. Seed: user-pinned-theme-studio; no concept tournament because structure and purpose were explicitly approved.

FINISH: unreviewed and undocumented is unfinished; this build ends with the finish review, the verdict, DESIGN.md, and every shipping raster carrying its provenance
