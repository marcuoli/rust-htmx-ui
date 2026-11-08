use maud::html;
use rust_htmx_ui as ui;
use std::io::{self, Write};
fn main() -> io::Result<()> {
    let prefix = ui::AssetPrefix::default();
    let document = html! {
        (maud::DOCTYPE) html lang="en" {
            head {meta charset="utf-8";meta name="viewport" content="width=device-width, initial-scale=1";
                title {"Rust UI showcase"} (ui::head(&prefix,true,None))}
            body class="ghtui-document" {div class="ghtui-app" data-ui-app data-ui-theme="standard" data-ui-mode="system"
                data-ui-density="comfortable" data-ui-storage-key="rust-htmx-ui:showcase" {
                (ui::app_shell("showcase","Rust UI","Synthetic component examples",html!{nav {a class="ghtui-shell-link" href="#components" {"Components"}}},html!{},html!{
                    h1 id="components" {"Component showcase"}
                    (ui::stack(html!{
                        (ui::widget("Controls","Shared presentation without a framework runtime.",html!{},html!{
                            (ui::field(&ui::Field{id:"project",label:"Project",value:Some("Example"),hint:Some("A labeled field"),..Default::default()}))
                            (ui::button(&ui::Button{label:"Primary action",..Default::default()}))
                            (ui::status(&ui::Status{label:"Healthy",tone:ui::Tone::Success,pulse:true,..Default::default()}))
                        }))
                        (ui::alert(ui::Tone::Info,"Synthetic information only.",None,None))
                        (ui::grid(2,html!{
                            (ui::metric("Requests","123456","Exact values remain available",None))
                            (ui::metric("Latency","4.2 ms","Synthetic fixture",None))
                        }))
                        (ui::widget("Native chart","Synthetic data.",html!{},ui::chart("requests","Requests",
                            &serde_json::json!({"xAxis":{"type":"category","data":["One","Two","Three"]},"yAxis":{"type":"value"},"series":[{"type":"line","data":[1,null,3]}]}),
                            &serde_json::json!({}))))
                        button type="button" class="ghtui-button ghtui-button-secondary" data-ui-modal-open="sample-dialog" {"Open dialog"}
                        (ui::modal("sample-dialog","Example dialog",html!{p {"Native modal and shared focus behavior."}},
                            html!{button type="button" class="ghtui-button ghtui-button-secondary" data-ui-modal-close {"Close"}}))
                    }))
                }))
            }}
        }
    };
    let mut output = io::stdout().lock();
    output.write_all(document.into_string().as_bytes())?;
    output.flush()
}
