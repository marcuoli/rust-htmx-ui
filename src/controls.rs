use maud::{Markup, html};
/// Button visual treatment.
#[derive(Default, Clone, Copy)]
pub enum ButtonKind {
    /// Primary action.
    #[default]
    Primary,
    /// Secondary action.
    Secondary,
    /// Destructive action.
    Danger,
}
/// Server-defined behavior bindings; never populate expressions with untrusted text.
#[derive(Default)]
pub struct Button<'a> {
    /// Visible action label.
    pub label: &'a str,
    /// Visual action treatment.
    pub kind: ButtonKind,
    /// Submit the owning form.
    pub submit: bool,
    /// Trusted Alpine action.
    pub click: Option<&'a str>,
    /// Trusted Alpine disabled-state expression.
    pub disabled: Option<&'a str>,
}
/// Render an accessible shared action button.
pub fn button(props: &Button<'_>) -> Markup {
    let class = match props.kind {
        ButtonKind::Primary => "ghtui-button ghtui-button-primary",
        ButtonKind::Secondary => "ghtui-button ghtui-button-secondary",
        ButtonKind::Danger => "ghtui-button ghtui-button-danger",
    };
    html! {button type=(if props.submit{"submit"}else{"button"}) class=(class)
    x-on:click=[props.click] x-bind:disabled=[props.disabled] {(props.label)}}
}
/// Supported input presentations.
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub enum FieldKind {
    /// Single-line text.
    #[default]
    Text,
    /// Password/token input.
    Password,
    /// Numeric input with Alpine numeric conversion.
    Number,
    /// Multiline code/text.
    Textarea,
    /// Boolean checkbox.
    Checkbox,
}
/// A labeled field. Expressions are trusted developer code; text/values are escaped.
#[derive(Default)]
pub struct Field<'a> {
    /// Stable fallback ID and form name.
    pub id: &'a str,
    /// Visible label.
    pub label: &'a str,
    /// Trusted Alpine model expression; empty omits the binding.
    pub model: &'a str,
    /// Optional repeated-field ID expression, shared by the label and control.
    pub dynamic_id: Option<&'a str>,
    /// Presentation/input type.
    pub kind: FieldKind,
    /// Browser required validation.
    pub required: bool,
    /// Maximum character count.
    pub max_length: Option<u32>,
    /// Minimum numeric value.
    pub min: Option<u32>,
    /// Maximum numeric value.
    pub max: Option<u32>,
    /// Example input.
    pub placeholder: Option<&'a str>,
    /// Initial escaped value for server-rendered forms.
    pub value: Option<&'a str>,
    /// Additional visible guidance.
    pub hint: Option<&'a str>,
}
/// Render a field with associated label, appropriate input semantics and shared styling.
pub fn field(props: &Field<'_>) -> Markup {
    let model = (!props.model.is_empty()).then_some(props.model);
    let hint_id = format!("{}-hint", props.id);
    let description = props.hint.map(|_| hint_id.as_str());
    let input_type = match props.kind {
        FieldKind::Password => "password",
        FieldKind::Number => "number",
        _ => "text",
    };
    html! {div class="ghtui-field" {
        @if props.kind==FieldKind::Checkbox {
            label for=(props.id) x-bind:for=[props.dynamic_id] {
                input id=(props.id) name=(props.id) type="checkbox" x-bind:id=[props.dynamic_id] x-model=[model] required[props.required];
                " " (props.label)
            }
        } @else {
            label for=(props.id) x-bind:for=[props.dynamic_id] {(props.label)}
            @if props.kind==FieldKind::Textarea {
                textarea id=(props.id) name=(props.id) class="ghtui-input" rows="5"
                    x-bind:id=[props.dynamic_id] x-model=[model] required[props.required]
                    maxlength=[props.max_length] aria-describedby=[description] spellcheck="false" autocomplete="off" {(props.value.unwrap_or_default())}
            } @else {
                input id=(props.id) name=(props.id) class="ghtui-input" type=(input_type)
                    x-bind:id=[props.dynamic_id] x-model=[if props.kind!=FieldKind::Number{model}else{None}]
                    "x-model.number"=[if props.kind==FieldKind::Number{model}else{None}]
                    value=[props.value] required[props.required] maxlength=[props.max_length]
                    min=[props.min] max=[props.max] placeholder=[props.placeholder]
                    aria-describedby=[description] spellcheck="false"
                    autocomplete=(if props.kind==FieldKind::Password{"current-password"}else{"off"});
            }
        }
        @if let Some(hint)=props.hint{p id=(hint_id) class="ghtui-field-hint" {(hint)}}
    }}
}
/// Link navigation enhanced with application-owned HTMX fragments.
/// URLs and Alpine expressions must be trusted server-defined values.
pub fn nav_item(label: &str, href: &str, fragment: &str, target: &str, current: &str) -> Markup {
    html! {a class="ghtui-shell-link" href=(href) hx-get=(fragment) hx-target=(target)
    hx-swap="innerHTML" hx-sync=(format!("{target}:replace")) x-bind:aria-current=(current) {(label)}}
}
