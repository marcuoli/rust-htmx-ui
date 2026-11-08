# Notices and provenance

Rust component implementation: copyright 2026 marcuoli, MIT; see LICENSE.

Shared first-party presentation sources and runtimes originate from
go-htmx-ui v0.11.0+20261106.31, commit
c60517495bac85c28d23e4ffb0b5276eeecf5cb5, MIT.
Source: https://github.com/marcuoli/go-htmx-ui/tree/c60517495bac85c28d23e4ffb0b5276eeecf5cb5/web/assets
Original SHA-256 hashes are retained in assets/provenance.json. ui.css is rebuilt
from the original first-party styles/theme generator using pinned Tailwind4.3.5;
its derived hash and build version are recorded in that manifest. Other copied
runtimes and official chart assets retain their original bytes.
rust-ui.css contains the Rust adapter's small additional component rules.

The official ECharts 6.1.0 engine remains unmodified. Its original provenance
and all Apache/ECharts/zrender/d3/tslib licenses/notices are retained beside it.
A consumer enabling charts must distribute those notices. Maud and serde_json
are official released dependencies recorded in Cargo.lock and their upstream
licenses apply separately.

Since0.2.0 the library also bundles official HTMX, Alpine CSP and their dependency notices, builds shared Tailwind frontend CSS and owns initialization. frontend-LICENSES.txt, frontend-versions.json and frontend.provenance.json record the exact distribution; licenses/ retains the official Alpine release license omitted from its npm CSP package.
