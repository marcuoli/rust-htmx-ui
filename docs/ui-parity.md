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
| Appearance control | Visually plain sun/moon icon by default; boxed button is explicit opt-in. Palette selection is independent. |
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
decorative icons and an accessible action name/title. Plain presentation has no
persistent background, visible border or shadow. Retain the full control hit area
and visible keyboard focus. The explicit Button variant may add the boxed secondary
treatment without changing behavior, glyph dimensions or header alignment.
Standalone, theme-switcher and shell APIs must expose the option and default to Plain.

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
| Theme controls | theme_switcher, appearance_toggle; *_with_style / AppearanceToggleStyle | AppearanceToggle / AppearanceToggleStyle; ThemeSwitcher and AppShell AppearanceStyle |
| Status | Status / status, description and description_binding | StatusIndicator; Description / Alpine.Description supply full state, explicit caller attributes take precedence |
| Defaults | page and scoped data-ui-* root | App / theme.Config and scoped data-ui-* root |

Rust plain/default and explicit Button behavior is qualified for
[v0.4.0](https://github.com/marcuoli/rust-htmx-ui/releases/tag/v0.4.0), commit
e6345bedd4fc3f40059edc44a83e4165f94eb9c5, tracked in
[issue #11](https://github.com/marcuoli/rust-htmx-ui/issues/11). Rust/Clippy1.99.1,
Windows x86_64 MSVC default features: fmt, locked all-target Clippy -D warnings,
9 native tests and package verification passed. Edge checks cover both styles,
keyboard focus, OS/storage/scoping behavior and60 header geometry combinations.
The Go counterpart is published as
[v0.13.0+20261108.35](https://github.com/marcuoli/go-htmx-ui/releases/tag/v0.13.0%2B20261108.35),
commit fece9a59746e2c28ac532a5bfc8e7871c546882f,
[PR #35](https://github.com/marcuoli/go-htmx-ui/pull/35). Its public release, tag
and merged commit agree; both hosted verify checks passed. See its
[immutable parity policy](https://github.com/marcuoli/go-htmx-ui/blob/v0.13.0%2B20261108.35/docs/ui-parity.md)
for the owner-recorded Go1.27.3 Windows/amd64 (CGO disabled) qualification:
Go tests/vet/module verification,130 JavaScript tests, advisory checks and Edge
behavior coverage including48 width/palette/light-dark/density geometry
combinations. Separate checks cover both visual styles, keyboard, OS/storage/
scoping and HTMX2/4. Those are Go
release results, not a new cross-library test run for this documentation update.
[Go issue #34](https://github.com/marcuoli/go-htmx-ui/issues/34) retains the
delivery/cleanup record separately from the verified runtime publication.

## Documented differences and follow-up

Go uses AppearancePlain for zero/unknown styles and AppearanceButton for the
explicit boxed style. Rust uses the typed AppearanceToggleStyle::Plain default
and Button variant. Standalone, theme-switcher and shell APIs expose the same
visual choice while keeping their language-specific configuration forms.

Status announcement defaults differ. Go described indicators use a named,
non-live role="group"; explicit caller role/attributes win, and live announcements
are opt-in through role="status" or aria-live. This preserves existing repeated
and table indicator behavior. Rust v0.4.0 status() always renders role="status",
including described indicators, and has no equivalent role/announcement option.
[Rust issue #14](https://github.com/marcuoli/rust-htmx-ui/issues/14) tracks aligning
that default and explicit option with native/AX/browser and consumer-compatibility
checks. Until qualified, consumers must not assume announcement behavior matches.

Default preference storage keys remain language-specific
(go-htmx-ui:appearance / rust-htmx-ui:appearance) to isolate application choices.

### Metric values and pending compact/unit formatting

Rust [v0.4.0](https://github.com/marcuoli/rust-htmx-ui/releases/tag/v0.4.0),
commit e6345bedd4fc3f40059edc44a83e4165f94eb9c5, renders the supplied metric
value directly as text. The optional binding updates both text and title; there
is no separate compact visual value/exact accessible value API. See the
[released implementation](https://github.com/marcuoli/rust-htmx-ui/blob/e6345bedd4fc3f40059edc44a83e4165f94eb9c5/src/widgets.rs#L66).
The current direct-text rendering has no matching immediate defect from Go's
paragraph-label issue.

[Rust issue #16](https://github.com/marcuoli/rust-htmx-ui/issues/16) tracks the
unimplemented compact/unit formatter port and its exact accessible-value
contract. Closed [policy issue #7](https://github.com/marcuoli/rust-htmx-ui/issues/7)
does not represent implementation or qualification of this port.

Go's WidgetMetricValue also supports compact visual text with a separate exact
accessible value. Its correction is published in
[v0.13.1+20261117.37](https://github.com/marcuoli/go-htmx-ui/releases/tag/v0.13.1%2B20261117.37),
commit ae4f9881cdf4ca27f8ad7b1e5e546170e6c04b02,
[PR #37](https://github.com/marcuoli/go-htmx-ui/pull/37), with
[full-review issue #36](https://github.com/marcuoli/go-htmx-ui/issues/36) closed.
The [released template](https://github.com/marcuoli/go-htmx-ui/blob/ae4f9881cdf4ca27f8ad7b1e5e546170e6c04b02/components/widget/widget.templ#L66)
overrides the paragraph's implicit role with non-live role="group", supplies the
exact value as its title/aria-label, and uses an aria-hidden="true" compact child.
This permits the exact accessible name without duplicate compact text.

The Go release record reports Go 1.27.4 (templ 0.3.1100), Go checks,
131 JavaScript tests, native CSS compilation, clean source/binary advisory scans,
independent review and green main/tag CI. Native Edge checks cover exact
uint64/duration names and 24 geometry cases each with HTMX 2 and 4.
Screen-reader applications and other browser engines
were not tested. The public release/tag/merged commit and metric source were
checked for this documentation update; the other results are the Go owner's
recorded release qualification, not a new cross-library or Rust test run.

Rust's formatter/API port remains unimplemented under issue #16. This published
Go correction does not change the Rust v0.4.0 behavior or establish released Rust
compact/exact accessible-value parity.

When porting the formatter under issue #16:

- Preserve exact integer and duration values alongside compact presentation;
  avoid precision loss when handling the full unsigned-integer range.
- Use semantics that permit the exact accessible name. A non-live named group
  with its compact visual child hidden from accessibility is the Go reference;
  do not attach aria-label alone to the current paragraph as a substitute.
- Verify the exact accessible value appears once, without a second compact
  announcement. Match the Go non-live default and assess consumer compatibility.
- Qualify idiomatic Rust APIs and static/bound/inserted values with native tests
  and native-browser accessibility-tree checks. Record both owners' exact tested
  and published references before claiming released parity.

The Rust README records other existing API limitations. This contract does not
establish full historical API parity; unrelated ports remain explicit gaps.

### Subsequent Go-only dependency maintenance

The later
[v0.13.2+20261117.39](https://github.com/marcuoli/go-htmx-ui/releases/tag/v0.13.2%2B20261117.39),
commit 60e86ac30357259ce426d03d8f9b2398484894a5,
[PR #39](https://github.com/marcuoli/go-htmx-ui/pull/39), closes
[Go issue #38](https://github.com/marcuoli/go-htmx-ui/issues/38). Its dependency
refresh adopts templ 0.3.1118, parse 0.0.42 and the refreshed table dependency.
The Go release records regenerated bodies, public APIs and shared assets as
unchanged, with generator-version comments updated. See its
[dependency review](https://github.com/marcuoli/go-htmx-ui/blob/60e86ac30357259ce426d03d8f9b2398484894a5/docs/go-dependencies.md)
for exact coordinates, compatibility constraints and qualification.

Go's declared module/build requirement remains 1.27.4. Separately labeled README
source-compatibility evidence does not lower that requirement. This Go-only
maintenance release requests no new Rust feature/default port and does not
implement the existing Rust issue #16 compact/exact metric gap. Rust runtime,
frontend dependency pins and qualification remain unchanged.

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
