---
version: 1
slug: "crates-rustify-components-src-navigation-menu-rs"
primary_target: "crates/rustify-components/src/navigation_menu.rs"
related_targets: ["crates/rustify-components/css/menus.css", "tests/dom-only/consumer/src/menus.rs"]
---

Mode: Operate. Extend the SDK navigation language within DOM-COMPONENTS-P4 §5.5 and DESIGN.md.

## Direction contract

THESIS: A compact row of destinations reveals grouped native links without obscuring the current destination.

OWN-WORLD: Existing semantic background, foreground, border, muted, accent and ring tokens; shared control metrics. No added visual parameter family.

STORY: Scan destinations, reveal a group with pointer or keyboard, then follow an ordinary browser link. Current page, expanded trigger and keyboard focus are distinct.

FIRST VIEWPORT: Horizontal destination list, a narrow active indicator, and a bounded content surface below. Content remains in its list item; Viewport supplies the measured backdrop. A 390px surface scrolls its list locally and keeps content within the viewport.

FORM: Existing SDK extension with no replacement identity, illustration, animation concept or GPU counterpart. Sub-navigation follows the same controls inside content.

FINISH: M4 static and shared real-browser gates, then M7 cross-component acceptance. User initiates independent code review after delivery; no autonomous review delegation.
