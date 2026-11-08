use rust_htmx_ui::{self as ui, maud::html};

#[test]
fn fields_escape_values_and_keep_dynamic_labels_connected() {
    let output = ui::field(&ui::Field {
        id: "name",
        label: "<script>bad</script>",
        model: "client.name",
        dynamic_id: Some("'name-' + index"),
        value: Some("\" onfocus=\"bad"),
        ..Default::default()
    })
    .into_string();
    assert!(!output.contains("<script>"));
    assert!(output.contains("&lt;script&gt;"));
    assert!(
        output.contains("x-bind:for=\"&#39;name-&#39; + index\"")
            || output.contains("x-bind:for=\"'name-' + index\"")
    );
    assert!(output.contains("x-bind:id="));
    assert!(!output.contains("value=\"\" onfocus="));
}
#[test]
fn asset_prefixes_are_local_and_traversal_is_rejected() {
    for prefix in [
        "https://example.test/",
        "//example.test/",
        "/a/../b",
        "/a?token=x",
        "/a\\b",
        "/x\n",
    ] {
        assert!(ui::AssetPrefix::new(prefix).is_err(), "{prefix:?}");
    }
    let prefix = ui::AssetPrefix::new("/custom/ui").unwrap();
    let output = ui::head(&prefix, true, None).into_string();
    assert!(output.contains("/custom/ui/ui.css"));
    assert!(output.contains("/custom/ui/echarts.min.js"));
    assert!(ui::asset("../Cargo.toml").is_none());
    assert!(ui::asset("ui.css").unwrap().mime.starts_with("text/css"));
}
#[test]
fn chart_json_is_escaped_and_native_options_are_preserved() {
    let value = serde_json::json!({"title":{"text":"</div><script>bad</script>"},"series":[{"type":"line","data":[1,null,3]}]});
    let output = ui::chart("chart", "Requests", &value, &serde_json::json!({})).into_string();
    assert!(!output.contains("<script>"));
    assert!(output.contains("data-ui-echarts="));
    assert!(output.contains("null"));
    assert!(output.contains("&quot;"));
}
#[test]
fn defaults_and_controls_compose_without_consumer_framework_setup() {
    let output=ui::page("Example", html! {}, html! {
        (ui::widget("Overview","Live information",html! {},html! {
            (ui::button(&ui::Button{label:"Save",..Default::default()}))
            (ui::status(&ui::Status{label:"Connected",tone:ui::Tone::Success,..Default::default()}))
        }))
    }).into_string();
    assert!(output.starts_with("<!DOCTYPE html>"));
    assert!(output.contains("data-ui-theme=\"standard\""));
    assert!(output.contains("ghtui-widget"));
    assert!(output.contains("ghtui-button"));
    assert!(output.contains("data-tone=\"success\""));
    assert!(!output.contains("https://"));
    assert_eq!(output.matches("ui-runtime.js").count(), 1);
}

#[test]
fn shared_head_owns_framework_assets_and_registration_order() {
    let markup = ui::head(&ui::AssetPrefix::default(), false, None).into_string();
    assert_eq!(markup.matches("ui-core.js").count(), 1);
    assert!(markup.contains("frontend.css"));
    let runtime = ui::asset("ui-core.js").expect("framework runtime must be embedded");
    let script = String::from_utf8_lossy(runtime.bytes);
    assert!(script.contains("RustHTMXUI"));
    let licenses = ui::asset("frontend-LICENSES.txt").expect("runtime licenses must be embedded");
    assert!(String::from_utf8_lossy(licenses.bytes).contains("htmx.org"));
}
#[test]
fn shared_view_host_owns_request_synchronization() {
    let markup = ui::view_host("view", html! {p {"Content"}}).into_string();
    assert!(markup.contains("data-ui-view"));
    assert!(markup.contains("hx-sync=\"this:replace\""));
}

#[test]
fn appearance_control_is_an_accessible_progressive_sun_moon_button() {
    let markup = ui::theme_switcher().into_string();
    assert!(markup.contains("data-ui-mode-toggle"));
    assert!(markup.contains("aria-label=\"Switch to dark mode\""));
    assert!(markup.contains("data-ui-mode-icon=\"sun\""));
    assert!(markup.contains("data-ui-mode-icon=\"moon\""));
    assert!(markup.contains("disabled"));
    assert!(!markup.contains("data-ui-mode-select"));
}

#[test]
fn stable_status_labels_include_escaped_full_state_for_assistive_technology() {
    let markup = ui::status(&ui::Status {
        label: "Live updates",
        description: Some("Live updates: <Disconnected>"),
        description_binding: Some("connectionDescription"),
        tone: ui::Tone::Warning,
        ..Default::default()
    })
    .into_string();
    assert!(markup.contains(">Live updates</span>"));
    assert!(markup.contains("aria-label=\"Live updates: &lt;Disconnected&gt;\""));
    assert!(markup.contains("title=\"Live updates: &lt;Disconnected&gt;\""));
    assert!(markup.contains("x-bind:aria-label=\"connectionDescription\""));
}

#[test]
fn appearance_style_defaults_to_plain_and_boxed_treatment_is_explicit() {
    let default = ui::appearance_toggle().into_string();
    assert!(default.contains("data-ui-appearance-style=\"plain\""));
    assert!(!default.contains("ghtui-button-secondary"));
    let boxed = ui::appearance_toggle_with_style(ui::AppearanceToggleStyle::Button).into_string();
    assert!(boxed.contains("data-ui-appearance-style=\"button\""));
    assert!(boxed.contains("ghtui-button-secondary"));
    let switcher = ui::theme_switcher_with_style(ui::AppearanceToggleStyle::Button).into_string();
    assert!(switcher.contains("data-ui-appearance-style=\"button\""));
    let shell = ui::app_shell_with_style(
        "shell",
        "Example",
        "",
        html! {},
        html! {},
        html! {},
        ui::AppearanceToggleStyle::Button,
    )
    .into_string();
    assert!(shell.contains("data-ui-appearance-style=\"button\""));
}
