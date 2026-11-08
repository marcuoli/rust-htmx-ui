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
fn defaults_and_controls_compose_without_runtime_frameworks() {
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
