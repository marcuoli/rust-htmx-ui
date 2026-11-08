//! Embedded, allowlisted assets; no filesystem access or route registration.
use maud::{Markup, html};
/// One embedded asset selected by its exact relative filename.
pub struct Asset {
    /// HTTP media type.
    pub mime: &'static str,
    /// Immutable embedded bytes.
    pub bytes: &'static [u8],
}
/// Find a shared asset. Unknown names and traversal return None.
pub fn asset(name: &str) -> Option<Asset> {
    let (mime, bytes): (&str, &[u8]) = match name {
        "frontend.provenance.json" => (
            "application/json",
            include_bytes!("../assets/frontend.provenance.json"),
        ),
        "frontend.css" => (
            "text/css; charset=utf-8",
            include_bytes!("../assets/frontend.css"),
        ),
        "ui-core.js" => (
            "text/javascript; charset=utf-8",
            include_bytes!("../assets/ui-core.js"),
        ),
        "frontend-versions.json" => (
            "application/json",
            include_bytes!("../assets/frontend-versions.json"),
        ),
        "frontend-LICENSES.txt" => (
            "text/plain; charset=utf-8",
            include_bytes!("../assets/frontend-LICENSES.txt"),
        ),
        "rust-ui.css" => (
            "text/css; charset=utf-8",
            include_bytes!("../assets/rust-ui.css"),
        ),
        "ui.css" => (
            "text/css; charset=utf-8",
            include_bytes!("../assets/ui.css"),
        ),
        "ui-runtime.js" => (
            "text/javascript; charset=utf-8",
            include_bytes!("../assets/ui-runtime.js"),
        ),
        "ui-feedback.js" => (
            "text/javascript; charset=utf-8",
            include_bytes!("../assets/ui-feedback.js"),
        ),
        "echarts.min.js" => (
            "text/javascript; charset=utf-8",
            include_bytes!("../assets/echarts.min.js"),
        ),
        "echarts-format.js" => (
            "text/javascript; charset=utf-8",
            include_bytes!("../assets/echarts-format.js"),
        ),
        "ui-htmx-charts.js" => (
            "text/javascript; charset=utf-8",
            include_bytes!("../assets/ui-htmx-charts.js"),
        ),
        "LICENSE.txt" => ("text/plain; charset=utf-8", include_bytes!("../LICENSE")),
        "echarts.LICENSE.txt" => (
            "text/plain; charset=utf-8",
            include_bytes!("../assets/echarts.LICENSE.txt"),
        ),
        "echarts.NOTICE.txt" => (
            "text/plain; charset=utf-8",
            include_bytes!("../assets/echarts.NOTICE.txt"),
        ),
        "echarts-d3.LICENSE.txt" => (
            "text/plain; charset=utf-8",
            include_bytes!("../assets/echarts-d3.LICENSE.txt"),
        ),
        "zrender.LICENSE.txt" => (
            "text/plain; charset=utf-8",
            include_bytes!("../assets/zrender.LICENSE.txt"),
        ),
        "tslib.LICENSE.txt" => (
            "text/plain; charset=utf-8",
            include_bytes!("../assets/tslib.LICENSE.txt"),
        ),
        "tslib.CopyrightNotice.txt" => (
            "text/plain; charset=utf-8",
            include_bytes!("../assets/tslib.CopyrightNotice.txt"),
        ),
        _ => return None,
    };
    Some(Asset { mime, bytes })
}
/// A validated local absolute URL prefix with one trailing slash.
#[derive(Clone, Debug)]
pub struct AssetPrefix(String);
/// The asset prefix contains unsafe URL syntax or dot segments.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InvalidAssetPrefix;
impl std::fmt::Display for InvalidAssetPrefix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("asset prefix must be a local absolute path without dot segments")
    }
}
impl std::error::Error for InvalidAssetPrefix {}
impl Default for AssetPrefix {
    fn default() -> Self {
        Self("/assets/ui/".into())
    }
}
impl AssetPrefix {
    /// Validate a route prefix. Remote URLs, encoded paths and traversal are rejected.
    pub fn new(value: &str) -> Result<Self, InvalidAssetPrefix> {
        if value.len() > 1024
            || !value.starts_with('/')
            || value.starts_with("//")
            || !value
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b"/-_.~".contains(&b))
            || value.split('/').any(|part| part == "." || part == "..")
        {
            return Err(InvalidAssetPrefix);
        }
        Ok(Self(format!("{}/", value.trim_end_matches('/'))))
    }
    /// Return the normalized local path.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
/// Render shared CSS and deferred runtimes once per full document.
/// Charts are opt-in. Applications supply their own CSP and optional nonce.
pub fn head(prefix: &AssetPrefix, charts: bool, nonce: Option<&str>) -> Markup {
    html! {
        link rel="stylesheet" href=(format!("{}frontend.css",prefix.0));
        link rel="stylesheet" href=(format!("{}ui.css",prefix.0));
        link rel="stylesheet" href=(format!("{}rust-ui.css",prefix.0));
        script defer src=(format!("{}ui-core.js",prefix.0)) nonce=[nonce] {}
        script defer src=(format!("{}ui-runtime.js",prefix.0)) nonce=[nonce] {}
        script defer src=(format!("{}ui-feedback.js",prefix.0)) nonce=[nonce] {}
        @if charts {
            @for name in ["echarts.min.js","echarts-format.js","ui-htmx-charts.js"] {
                script defer src=(format!("{}{name}",prefix.0)) nonce=[nonce] {}
            }
        }
    }
}
