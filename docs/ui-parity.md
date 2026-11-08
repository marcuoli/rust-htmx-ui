# Go/Rust UI parity contract

go-htmx-ui and rust-htmx-ui share a product contract for reusable UI features,
patterns, defaults and accessibility. A shared change must assess its counterpart
and update both libraries or record the pending port in a linked owner issue.
Keep Go/templ and Rust/Maud APIs idiomatic; matching behavior does not require
identical types, names or implementation files.

Public counterparts:
- [go-htmx-ui](https://github.com/marcuoli/go-htmx-ui)
- [rust-htmx-ui](https://github.com/marcuoli/rust-htmx-ui)

## Shared defaults and ownership

| Concern | Shared contract |
| --- | --- |
| Palette | Standard initially; Standard, Ocean and Forest remain selectable. |
| Appearance | System initially, following the OS until an explicit choice. |
| Density | Comfortable initially; Compact remains supported. |
| Appearance control | Sun/moon action button; palette selection is independent. |
| Status text | Neutral text; semantic color belongs to the decorative dot/ring. |
| Status animation | Pulse is opt-in, stops when inactive and respects reduced motion. |
| Resources | Local assets owned, built, versioned and initialized by the shared library. |
| Consumer responsibilities | Domain state/data, authentication, routes, native SSE and application security policy. |

Library ownership includes frontend packages, locks, stable-version/advisory
checks, notices, assets, framework initialization and reusable interactions.
Consumers compose shared components and domain behavior. They must not acquire
or initialize their own copies of shared frontend frameworks.

## Appearance and status pattern

The sun/moon button describes its next action. Light mode shows a moon with
"Switch to dark mode"; dark mode shows a sun with "Switch to light mode".
A click resolves the current appearance and persists an explicit light/dark
choice. System mode remains available through the appearance API/root default.
Controls are disabled before runtime enhancement. Use native keyboard activation,
decorative icons and an accessible action name/title.

Multiple controls within one root synchronize; nested roots remain independent.
New DOM/SPA fragments, OS changes and cross-tab storage updates reconcile controls.
Unavailable storage must not prevent in-page changes. Glyphs remain square and
legible in both densities; verify computed layout against shared button padding.
Within shell headers, keep the visible theme label inline with its selector.
Action buttons, selector and appearance button share control height and vertical
center within each wrapped row; status text aligns to that same center. Check
computed bounds in both densities and narrow screens. Header alignment is tracked
in [rust-htmx-ui #9](https://github.com/marcuoli/rust-htmx-ui/issues/9); the Go
counterpart belongs to [go-htmx-ui #34](https://github.com/marcuoli/go-htmx-ui/issues/34).

For a live-update indicator, use a stable visible label such as "Live updates".
Expose full state, including the visible label, in its tooltip and accessible
name. Keep the text neutral as the dot/ring changes semantic tone. The consumer
decides connection freshness and pulse state; the shared component presents it.

## Public API mapping and verified baseline

| Pattern | Rust API | Go counterpart |
| --- | --- | --- |
| Theme controls | theme_switcher, appearance_toggle | ThemeSwitcher and its shared appearance button |
| Status | Status / status, description and description_binding | StatusIndicator; Attributes/Alpine bindings supply full state |
| Defaults | page and scoped data-ui-* root | App / theme.Config and scoped data-ui-* root |

Rust behavior is qualified in [v0.3.0](https://github.com/marcuoli/rust-htmx-ui/releases/tag/v0.3.0),
commit 7c8786cabd7727b24638912421559fdf8501b346,
[PR #6](https://github.com/marcuoli/rust-htmx-ui/pull/6).
The Go appearance/status/default alignment and counterpart documentation are
tracked in [go-htmx-ui #34](https://github.com/marcuoli/go-htmx-ui/issues/34).
That link records pending work until the Go owner provides tested release proof;
a sent request or source change alone does not establish released parity.

The Rust README records existing API limitations, including unported Go numeric
compact/unit formatting. This contract does not establish full historical API
parity. Record affected gaps when assessing each feature; unrelated historical
ports are not silently treated as implemented.

## Required change and release record

For every shared feature, pattern or default change:

1. Compare the counterpart's current public API, implementation and exact release.
   Record the intended semantics, defaults, affected consumer behavior and
   counterpart issue/PR. Keep pending work explicit.
2. Implement the equivalent behavior in each owner's normal workflow. Explain
   compatibility/platform differences in both documents, including their reason,
   tested scope and next action. A pending difference must have an owner issue.
3. Verify observable behavior in both libraries: keyboard/focus and accessible
   names, persistence/OS/storage changes, nested roots, inserted SPA controls,
   supported palettes/densities, status tones/pulse and reduced motion. Include
   narrow screens and failed/unavailable storage where relevant. Native checks
   alone do not establish browser parity.
4. Update both docs/ui-parity.md files and user-facing component guidance.
   Record exact tested commits/releases, required checks and unresolved gaps.
   Keep private consumer data and first-party domain code out of public docs.
5. Qualify and publish each library independently when a release is needed.
   Preserve immutable tags/assets and upstream-only dependency rules. Consumers
   adopt the qualified release through their own workflow. State separately
   whether each side is requested, implemented, tested, merged or published.

Documentation-only policy changes do not require an otherwise unchanged runtime
release. Existing releases retain their original contents and qualification.
