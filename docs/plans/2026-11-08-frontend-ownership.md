# Central frontend ownership plan

Approved requirement: rust-htmx-ui owns all external frontend resources and calls;
consuming apps do not import, bootstrap, configure or build third-party frameworks.

- [ ] Library v0.2.0: pin official current stable Tailwind/CLI, HTMX, Alpine CSP and
  bundler; keep locked graph, current-version checks, npm audit, upstream licenses
  and generated asset provenance wholly in this public library.
- [ ] Ship prebuilt embedded frontend CSS/core JS. Head supplies each asset once.
  Core owns safe framework configuration/startup, component registration, shared
  navigation lifecycle events and target-owned replacement. Domain factories
  register through RustHTMXUI; no framework globals are exposed.
- [ ] Add an escaped reusable view_host. Retain Maud components, themes, optional
  charts, CSP, host/origin/session and SPA/SSE behavior.
- [ ] Consumer: remove npm manifests, Tailwind imports/source directives and
  frontend build/check scripts. Keep plain domain JS/CSS and Cargo-only embedding.
  Consume only the library facade, not raw framework APIs/events.
- [ ] Qualify regression tests, complete real-browser SPA/auth/config/theme flows,
  facade boundaries, no third-party globals and a clean offline Cargo build with
  Node/npm removed from PATH and no generated frontend artifact directory.
- [ ] Independent review, publish immutable library release on both public forges,
  pin exact released dependency in private app, reviewed private integration,
  canonical synchronization, preserve/upgrade the owned manual-test instance,
  then exact issue worktree/admin cleanup and issue closure.

Public library never receives private app source, inputs or identities. Generated
assets/caches/scratch/evidence stay in the owning V: namespaces. No dependency
patching, runtime CDN/floating versions, app release/counter or protocol changes.
Rust/Clippy1.99.1 x86_64 Windows MSVC, default features; preserve Rustup managed home.
