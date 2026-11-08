# rust-htmx-ui

Composable, escaped Rust UI components with embedded styles and optional charts,
following the public go-htmx-ui design. MIT licensed.

The library owns presentation and shared theme/feedback behavior. Applications
own routes, authentication, authorization, state, Alpine/HTMX, SSE and CSP.
Ordinary consumers need no Node runtime or template generation step.

## Render components

Use Maud 0.27 with the crate. Components return Maud Markup and compose as Rust functions:

~~~rust
use maud::html;
use rust_htmx_ui as ui;
let markup = ui::page("Monitor", html! {}, ui::widget(
    "Requests", "Live service information", html! {},
    ui::button(&ui::Button { label: "Refresh", ..Default::default() }),
));
let html = markup.into_string();
~~~

Return the rendered string using your web framework's HTML response type.
Maud also provides optional framework integrations. Register your own
/assets/ui/{filename} route using asset(filename), returning its mime and bytes.
Keep that route behind your application's existing request/security middleware.

page supplies one copy of shared assets with Standard/System appearance.
app_shell adds a keyboard-accessible skip link, responsive navigation and theme
switcher. Standard, Ocean and Forest have paired light/dark palettes.
Local appearance preferences use the rust-htmx-ui:appearance storage key.

## Components and compatibility

| Go UI area | Rust API / status |
|---|---|
| Page, Head, assets | page, head, AssetPrefix, asset |
| AppShell, ThemeSwitcher, Stack, Grid | app_shell, theme_switcher, stack, grid |
| Widget, WidgetHeader | widget (heading, actions and body) |
| WidgetMetric | metric; exact values, optional application-owned live binding |
| Buttons, fields, inputs, textarea, checkbox | Button, Field, FieldKind |
| Tabs/navigation | nav_item with caller-owned local route and fragment target |
| StatusIndicator | Status and Tone; label, tone and pulse bindings |
| Inline feedback, dialog shell | alert, modal |
| ECharts native host | chart; head(..., true, ...) loads optional official engine |
| Numeric compact-format/unit conversion APIs | Not ported; no full Go API parity claim |
| Table implementation | Separate concern; not copied from go-htmx-table |
| Go template-specific props/aliases | Not source-compatible with Rust |

This initial API covers reusable application controls and layout. The shared
web runtime retains its existing GoHTMXUI JavaScript namespace and data-ui-*
contract so the original runtimes remain byte-for-byte identifiable. The stylesheet is
rebuilt from those first-party sources with pinned Tailwind4.3.5.
CSS is in Tailwind's components layer; consumer utilities may override defaults.

## Trust boundaries

Maud escapes text, values and chart JSON in HTML attributes. Markup represents
trusted HTML composition. Alpine expressions, URLs, extra head markup and HTMX
routes are developer-defined code, never untrusted user values. HTML escaping
does not sanitize JavaScript expressions or URL schemes. Do not use raw
PreEscaped content for user input. AssetPrefix rejects remote/encoded URLs,
controls and dot segments; asset only selects exact embedded filenames.

The library does not bundle Alpine or HTMX. Consumers select their current stable
official releases and test them with the shared runtime. Serve local embedded
assets rather than runtime CDN dependencies. Preserve all third-party notices
when enabling charts; see NOTICE.md and assets/provenance.json.

## Development

Supported validation: Rust/Clippy 1.99.1, edition2024, Windows x86_64 MSVC,
default features. Scope Cargo outputs/cache/temp to the owning V: namespace.

~~~text
cargo fmt --all -- --check
cargo clippy --all-targets --locked -- -D warnings
cargo test --locked
cargo package --locked
~~~

Run scripts/Build-Styles.ps1 to rebuild shared CSS with current stable Tailwind
and an advisory-checked locked npm graph; generated scratch stays on V:.
examples/showcase.rs emits a synthetic HTML page for browser qualification.
Asset serving and transport remain outside the library.
