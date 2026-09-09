# M5 report: native text editing and accessible journeys

Date: 2026-09-09. Machine: macOS (Darwin 25.6.0), Apple Silicon, rustc 1.97.0-nightly (2026-05-19), Node 26.1.0, Playwright 1.63.0 (bundled Chromium, SwiftShader WebGL). Tests: `npx playwright test --project=property-workbench` (`m3-workbench`, `m5-text`, `m5-semantics`), `--project=fusion-basic` as regression.

## Status

M5 is **not closed**. Everything that can be verified without a person is delivered and passing; two of its exit conditions cannot be met by this machine and are listed under "What still needs a person". Per §9.4 no synthetic substitute is recorded as a pass.

## What is delivered

### The browser's own text control edits the region's text

`crates/rustify-ui/src/text.rs`. A region draws its text and, asked to edit, hands over the rectangle it drew into; a real `input` (or `textarea`) takes exactly that rectangle for the life of the session, so the caret, the selection, the system's undo and the input method are the browser's rather than a redrawing of them.

The session is controlled: it opens with the value the application holds and proposes a new one. Every way it ends is a callback - commit, cancel, or the value moving underneath it - so the region and the application never hold two ideas of the value at once.

| Rule | Evidence |
| --- | --- |
| The control covers what the region drew, opens with the current value, focused and selected | `the control takes the rectangle the region drew the name into` |
| Enter commits once; the region draws the result | `enter commits once and the region draws the new value` |
| Escape abandons the edit and the value is untouched | `escape abandons the edit and the value is untouched` |
| Enter and Escape during composition belong to the composition: nothing commits, nothing closes | `keys pressed while composing belong to the composition` |
| A composition proposes one value, not one per event | `a composition produces one value, not one per event` |
| A value changed from elsewhere ends the session instead of being overwritten | `a value that moves underneath the session ends it instead of overwriting` |
| Leaving the control commits exactly once | `losing the control commits exactly once` |

The key order of §6.2 is enforced in the scope's own capture-phase listener: composition and native editing first, then the top layer, then the application's commands.

### One semantic entry per meaningful control

| Rule | Evidence |
| --- | --- |
| Twenty controls found by role and name, none of them twice | `twenty controls are found by role and name` |
| The region's pixels are decoration, not a mute second copy of the panel | `the region's pixels are not a second, mute copy of the panel` - the canvas carries `aria-hidden` |
| The pointer path and the DOM path reach the same object | `the DOM path and the pointer path select the same object` - a real click on a grid cell, then the same object reached by name through "go to object" |
| A lookup answers, or says it could not, within five seconds | `looking for an object answers, or says it could not, within five seconds` - `found` and `disposed` are immediate and final, a deleted id is distinguished from one that never existed, and a wait for an absent id ends at its deadline as `timeout` |
| Five journeys with the keyboard alone | `select, edit, run a command, read the result, and undo the selection` |

`UiError` gained `NotFound`, `Disposed` and `Timeout` for those three answers.

## What still needs a person

These are exit conditions of M5 that this machine cannot produce, and they are recorded as gaps rather than approximated:

1. **Real macOS Pinyin, twenty Chinese phrases** (V5). The composition *rules* are covered by synthetic events above - what is not covered is a real IME's own sequence of events, candidate window and commit timing. §9.4 forbids substituting one for the other.
2. **VoiceOver with Chrome, five journeys** (V6): announced name, role, value and state, no duplicate announcements, no focus trap. The keyboard journeys are automated; what a screen reader says about them is not.

A record for either needs the device, OS, browser and assistive-technology versions, the build id, the steps, the expectation and the result.

## Also outstanding for M5

- Multi-line editing exists in `TextEdit` (`multiline`) but no example uses it yet, so it has no browser coverage.
- Unicode selection, deletion and undo inside a session rely on the native control and are not separately asserted.
- Chinese font provenance is recorded in `sources.lock.json` and the font licences, but M5's "font source is explicit" is not yet written up as a section of the documentation.
