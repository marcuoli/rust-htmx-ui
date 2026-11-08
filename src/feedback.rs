use crate::Tone;
use maud::{Markup, html};
/// A shared inline notification with optional trusted Alpine text/visibility bindings.
pub fn alert(tone: Tone, message: &str, text: Option<&str>, show: Option<&str>) -> Markup {
    html! {div class="ghtui-message ghtui-alert" role=(if matches!(tone,Tone::Danger){"alert"}else{"status"})
        aria-live="polite" data-ui-alert data-ui-tone=(tone.as_str()) x-show=[show] {
        span class="ghtui-message-icon" aria-hidden="true" {(match tone{Tone::Danger=>"!",Tone::Success=>"✓",_=>"i"})}
        div class="ghtui-message-content" {div data-ui-message-text {
            p class="ghtui-message-text" x-text=[text] {(message)}
        }}
    }}
}
/// Native dialog shell using the shared external focus/close runtime.
pub fn modal(id: &str, title: &str, body: Markup, footer: Markup) -> Markup {
    let title_id = format!("{id}-title");
    html! {dialog id=(id) class="ghtui-dialog ghtui-modal" aria-labelledby=(title_id)
        data-ui-modal data-ui-size="medium" data-ui-modal-backdrop="true" {
        header class="ghtui-modal-header" {h2 id=(title_id) {(title)}
            button type="button" class="ghtui-dismiss ghtui-modal-close" data-ui-modal-close aria-label="Close" {"×"}}
        div class="ghtui-modal-body" tabindex="0" role="region" aria-labelledby=(title_id) {(body)}
        footer class="ghtui-modal-footer" {(footer)}
    }}
}
