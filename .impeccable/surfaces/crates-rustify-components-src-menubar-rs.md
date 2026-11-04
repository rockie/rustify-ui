---
version: 1
slug: "crates-rustify-components-src-menubar-rs"
primary_target: "crates/rustify-components/src/menubar.rs"
related_targets: ["crates/rustify-components/src/toolbar.rs", "crates/rustify-components/css/menus.css"]
---

Mode: Operate. Extend the existing SDK component language for document commands and editing tools. Scope and behavior are fixed by DOM-COMPONENTS-P4 §§5.3–5.4; authored themes and caller utilities retain control.

## Direction contract

THESIS: A compact command strip and a grouped editing toolbar make available actions and keyboard focus easy to scan.

OWN-WORLD: Inherit DESIGN.md: semantic background/foreground, restrained border and muted hover, theme control metrics, blue/default ring. Menu panels reuse the shared menu skin. No new size or variant API.

STORY: Choose a command menu or an editing action, move within the group using arrows, and continue through native fields without losing their editing keys.

FIRST VIEWPORT: Menubar is one horizontal row of short text triggers. Toolbar groups actions with perpendicular separators and keeps an inline field intact. Selected/open and focused controls remain distinguishable; long rows can scroll within available width.

FORM: Local extension of the established SDK, without a concept tournament or seed. No shell redesign, imagery, decorative motion, or GPU counterpart.

FINISH: The current plan controls acceptance: M3 static gates, M4 shared browser/visual verification, M7 cross-component verification and documentation. Independent code review remains user-initiated after delivery.
