use crate::{AssetPrefix, head};
use maud::{DOCTYPE, Markup, html};
/// Render a complete document with shared defaults and caller-owned extra head content.
/// The scoped root uses Standard/System appearance; the runtime persists user choices.
pub fn page(title: &str, extra_head: Markup, body: Markup) -> Markup {
    html! {
        (DOCTYPE) html lang="en" {
            head {meta charset="utf-8";meta name="viewport" content="width=device-width, initial-scale=1";
                title {(title)} (head(&AssetPrefix::default(),false,None)) (extra_head)}
            body class="ghtui-document" {
                div class="ghtui-app" data-ui-app data-ui-theme="standard" data-ui-mode="system"
                    data-ui-density="comfortable" data-ui-storage-key="rust-htmx-ui:appearance" {(body)}
            }
        }
    }
}
/// Render a sun/moon button for the next light/dark appearance.
/// System appearance follows the OS until the first explicit toggle. The shared
/// runtime enables the button and synchronizes its icon, accessible name and title.
pub fn appearance_toggle() -> Markup {
    html! {button type="button" class="ghtui-button ghtui-button-secondary ghtui-mode-toggle"
        data-ui-mode-toggle aria-label="Switch to dark mode" title="Switch to dark mode" disabled {
        svg data-ui-mode-icon="moon" viewBox="0 0 24 24" fill="none" stroke="currentColor"
            stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true" {
            path d="M20.9 13A9 9 0 0 1 11 3.1 9 9 0 1 0 20.9 13Z" {}
        }
        svg data-ui-mode-icon="sun" viewBox="0 0 24 24" fill="none" stroke="currentColor"
            stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true" hidden {
            circle cx="12" cy="12" r="4" {}
            path d="M12 2v2m0 16v2M2 12h2m16 0h2M4.93 4.93l1.42 1.42m11.3 11.3 1.42 1.42M4.93 19.07l1.42-1.42m11.3-11.3 1.42-1.42" {}
        }
    }}
}
/// Render a theme selector and the shared sun/moon appearance button.
pub fn theme_switcher() -> Markup {
    html! {div class="ghtui-theme-switcher" {
        label {span {"Theme"} select class="ghtui-input" data-ui-theme-select aria-label="Theme" disabled {
            @for (value,label) in [("standard","Standard"),("ocean","Ocean"),("forest","Forest")]{
                option value=(value) selected[value=="standard"] {(label)}
            }
        }}
        (appearance_toggle())
    }}
}
/// Shared shell with accessible mobile navigation, skip link and theme controls.
/// Supply a unique ID per shell. Domain navigation, actions and body are caller-owned.
pub fn app_shell(
    id: &str,
    title: &str,
    description: &str,
    navigation: Markup,
    actions: Markup,
    body: Markup,
) -> Markup {
    let main = format!("{id}-main");
    let nav = format!("{id}-navigation");
    html! {div id=(id) class="ghtui-shell" data-ui-shell {
        a class="ghtui-skip-link" href=(format!("#{main}")) {"Skip to content"}
        header class="ghtui-shell-header" {
            div class="ghtui-shell-brand" {
                button type="button" class="ghtui-button ghtui-button-secondary" data-ui-shell-toggle aria-controls=(nav) aria-expanded="false" hidden {"Menu"}
                div {a href="/" {(title)} p {(description)}}
            }
            div class="ghtui-shell-actions" {(actions)(theme_switcher())}
        }
        div class="ghtui-shell-body ghtui-shell-with-sidebar" {
            aside id=(nav) class="ghtui-shell-sidebar" data-ui-shell-nav {(navigation)}
            main id=(main) class="ghtui-shell-main" tabindex="-1" {(body)}
        }
    }}
}
/// Responsive shared grid; unsupported column counts use the four-column maximum.
pub fn grid(columns: u8, body: Markup) -> Markup {
    html! {div class="ghtui-grid" data-ui-columns=(columns.clamp(1,4)) {(body)}}
}
/// Shared vertical rhythm.
pub fn stack(body: Markup) -> Markup {
    html! {div class="ghtui-stack" {(body)}}
}

/// Stable SPA fragment target. Shared navigation owns its request cancellation group.
pub fn view_host(id: &str, body: Markup) -> Markup {
    html! {div id=(id) data-ui-view hx-sync="this:replace" {(body)}}
}
