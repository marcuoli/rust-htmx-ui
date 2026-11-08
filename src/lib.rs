//! Escaped Rust components and embedded assets for server-rendered applications.
//! Consumers own authentication, routes, domain state and SSE; this crate owns frontend runtimes.
#![deny(missing_docs)]
pub use maud;
mod assets;
mod controls;
mod feedback;
mod layout;
mod widgets;
pub use assets::*;
pub use controls::*;
pub use feedback::*;
pub use layout::*;
pub use widgets::*;
