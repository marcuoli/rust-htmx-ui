# rust-htmx-ui

Composable Maud UI components and library-owned frontend resources for Rust apps.

Version0.2.0 owns the complete frontend dependency boundary: official Tailwind,
HTMX, Alpine CSP, optional ECharts, exact npm pins/lockfile, audits, reproducible
builds, embedded assets, licenses and framework initialization. Consumers need
Cargo only; no Node/npm, frontend dependency manifest or frontend build step.

## Consumer integration

Use page/head once per document and serve asset(filename) at the matching
application-owned local prefix (default /assets/ui/). The shared head loads the
library CSS/runtime before your extra head's deferred domain script.

~~~rust
use maud::html;
use rust_htmx_ui as ui;
let document=ui::page("Monitor",html! {
    script defer src="/assets/domain.js" {}
},html!{
    div x-data="monitor" {
        (ui::view_host("content",ui::widget("Status","",html!{},html!{})))
    }
});
~~~

Your plain domain script registers its controller through the library:

~~~js
RustHTMXUI.register('monitor', () => ({
  message: 'Ready',
  async showDetails() {
    await RustHTMXUI.navigate('/views/details', {target:'#content'});
  }
}));
~~~

Do not import/load/configure/start Alpine, HTMX, Tailwind or other frontend
framework resources in a consumer. They are private to the shared runtime.
Deferred domain scripts register before DOMContentLoaded; the library then starts
Alpine once. The facade exposes register, navigate and immutable versions.
There are no raw Alpine/HTMX globals. Domain API calls, application state,
credentials, server routes, native SSE and security policy remain app-owned.

navigate performs same-origin GETs into a shared view_host. Link nav_item and
programmatic navigation share target-owned request replacement. The library emits
bubbling lifecycle events: rusthtmxui:before-navigation (cancelable, target/source),
rusthtmxui:navigation (target) and rusthtmxui:navigation-error (target/status).
Consumers handle these events through the library contract rather than framework
events. HTMX eval, script injection, indicator style injection, external requests
and history snapshots are disabled centrally.

Plain domain CSS may style application-specific layout using shared tokens.
It must not import/build Tailwind or carry framework setup. Shared frontend.css
supplies current Tailwind base/utilities; reusable component styles belong here.
Local assets only: no runtime CDN or floating package version resolution.

## Components

page, head, AssetPrefix, asset, app_shell, theme_switcher, view_host, stack, grid,
widget, metric, Button/Field/FieldKind, nav_item, Status/Tone, alert, modal and
native ECharts chart hosts compose as escaped Maud Markup. The optional chart
engine is loaded only by head(..., true, ...). Standard/Ocean/Forest support
light/dark/system appearance and persistent preferences. theme_switcher combines a
theme selector with appearance_toggle, a keyboard-accessible sun/moon button.
It shows the next action, follows the OS until clicked, then persists light/dark.
System mode remains available through the scoped appearance API/root default.

Status labels use neutral text; only the dot/ring carries the semantic tone.
For a stable label such as Live updates, supply description/description_binding
with the full state for the tooltip and accessible name.

The API is not source-compatible with Go templ props. Go numeric compact/unit
formatting APIs are not ported; tables remain a separate concern. The reviewed
first-party presentation runtime keeps its existing GoHTMXUI namespace/data-ui-*
contract for themes, feedback and optional charts; consumers use RustHTMXUI for
frontend dependency/runtime operations.

## Trust boundaries

Maud escapes text, values and chart JSON attributes. Markup, Alpine expressions,
URLs and extra head content are trusted developer code. Never use raw
PreEscaped markup for user values. The asset API selects exact embedded filenames;
AssetPrefix rejects remote/encoded paths and dot segments. The library registers
no routes or auth policy. Apply your app's middleware to all local asset/view
routes and preserve CSP/session/origin controls.

## Go/Rust UI parity

Keep shared features, UI patterns, defaults and accessibility synchronized with
[go-htmx-ui](https://github.com/marcuoli/go-htmx-ui). Every shared change checks
counterpart impact and records a matching implementation or linked owner gap.
See [the parity contract](docs/ui-parity.md) for defaults, appearance/status
patterns, public API mapping, behavior checks and exact release evidence.

## Maintenance

Only this repository updates/builds frontend dependencies. Run
scripts/Build-Assets.ps1 with a scoped ProjectStorage/TaskName. It checks every
direct pin against official stable registry tags, installs with npm ci,
audits the locked graph, builds styles/runtime and verifies/copies official chart
assets/licenses. All temporary/generated state is staged on V: before snapshots
are deliberately promoted. Third-party source and caches remain unmodified.

The CSP package omits its license in npm; licenses/ retains the exact official
Alpine release license with version/hash provenance. Asset metadata/notices are
served as frontend-versions.json, frontend.provenance.json and frontend-LICENSES.txt.
See NOTICE.md for the preserved chart/presentation provenance.

Rust/Clippy1.99.1, edition2024, Windows x86_64 MSVC, default features:

~~~text
cargo fmt --all -- --check
cargo clippy --all-targets --locked -- -D warnings
cargo test --locked
cargo package --locked
~~~

The synthetic showcase exercises the same embedded runtime and components.
