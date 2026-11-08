use maud::{Markup, html};
/// Semantic status/message tone.
#[derive(Default, Clone, Copy)]
pub enum Tone {
    /// Unknown or inactive.
    #[default]
    Neutral,
    /// Successful/healthy.
    Success,
    /// Needs attention.
    Warning,
    /// Failed/destructive.
    Danger,
    /// Informational.
    Info,
}
impl Tone {
    /// Stable DOM token used by the shared styles.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Neutral => "neutral",
            Self::Success => "success",
            Self::Warning => "warning",
            Self::Danger => "danger",
            Self::Info => "info",
        }
    }
}
/// Accessible status dot and label; application state and freshness remain external.
#[derive(Default)]
pub struct Status<'a> {
    /// Visible status text.
    pub label: &'a str,
    /// Static tone.
    pub tone: Tone,
    /// Optional trusted Alpine label expression.
    pub text: Option<&'a str>,
    /// Optional trusted Alpine tone expression.
    pub tone_binding: Option<&'a str>,
    /// Enable the reduced-motion-aware ring.
    pub pulse: bool,
    /// Optional trusted Alpine pulse expression.
    pub pulse_binding: Option<&'a str>,
}
/// Render status without using color alone to communicate its meaning.
pub fn status(props: &Status<'_>) -> Markup {
    html! {span class="ghtui-status" data-tone=(props.tone.as_str()) data-pulse=(props.pulse) data-pulse-style="ring"
        x-bind:data-tone=[props.tone_binding] x-bind:data-pulse=[props.pulse_binding] {
        span class="ghtui-status-dot" aria-hidden="true" {}
        span class="ghtui-status-label" x-text=[props.text] {(props.label)}
    }}
}
/// Shared panel with a heading, description, optional actions and body.
pub fn widget(title: &str, description: &str, actions: Markup, body: Markup) -> Markup {
    html! {section class="ghtui-widget" {
        div class="ghtui-widget-heading" {div {h2 {(title)} @if !description.is_empty(){p {(description)}}}(actions)}
        div class="ghtui-stack" {(body)}
    }}
}
/// Shared metric card. Values remain exact; the binding is trusted application code.
pub fn metric(label: &str, value: &str, note: &str, text: Option<&str>) -> Markup {
    html! {article class="ghtui-metric-card" {p class="ghtui-metric-label" {(label)}
        p class="ghtui-metric-value" title=(value) x-bind:title=[text] x-text=[text] {(value)}
        @if !note.is_empty(){p class="ghtui-metric-note" {(note)}}
    }}
}
/// Native ECharts host. JSON is HTML-attribute escaped by Maud and read by the
/// unmodified shared controller; no application data is evaluated as JavaScript.
pub fn chart(
    id: &str,
    label: &str,
    option: &serde_json::Value,
    config: &serde_json::Value,
) -> Markup {
    html! {
        div class="ghtui-chart-frame" {div id=(id) data-ui-echarts=(option.to_string())
            data-ui-echarts-config=(config.to_string()) role="img" aria-label=(label) {}}
        p class="ghtui-chart-empty" data-ui-empty-message="No data available" {"No data available"}
    }
}
