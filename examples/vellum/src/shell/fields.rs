//! Controlled property fields retain their draft while the user is editing.

use leptos::prelude::*;
use rustify_ui::Draft;
use serde_json::{json, Value};

use crate::app::Editor;
use crate::document::Node;
use crate::icons::icon;
use crate::shell;

pub type Selection = Memo<Vec<Node>>;

pub fn selected_nodes(editor: Editor) -> Selection {
    Memo::new(move |_| {
        editor.selection.with(|ids| {
            editor
                .doc
                .with(|doc| ids.iter().filter_map(|id| doc.get(id).cloned()).collect())
        })
    })
}

pub fn number(nodes: Selection, prop: &str) -> f64 {
    nodes.with(|nodes| nodes.first().map_or(0.0, |node| node_number(node, prop)))
}

fn node_number(node: &Node, prop: &str) -> f64 {
    match prop {
        "x" => node.x,
        "y" => node.y,
        "w" => node.w,
        "h" => node.h,
        "rotation" => node.rotation,
        "radius" => node.radius,
        "opacity" => node.opacity * 100.0,
        "fillOpacity" => node.fill_opacity * 100.0,
        "fontSize" => node.font_size,
        "fontWeight" => node.font_weight,
        "lineHeight" => node.line_height * 100.0,
        "letterSpacing" => node.letter_spacing,
        "gradientAngle" => node.gradient_angle,
        "strokeWidth" => node.stroke_width,
        "shadowX" => node.shadow_x,
        "shadowY" => node.shadow_y,
        "shadowBlur" => node.shadow_blur,
        "shadowOpacity" => node.shadow_opacity * 100.0,
        "gap" | "padding" => node.number(prop, 16.0),
        _ => node.number(prop, 0.0),
    }
}

pub fn text(nodes: Selection, prop: &str) -> String {
    nodes.with(|nodes| {
        nodes.first().map_or_else(String::new, |node| {
            match prop {
                "fill" => &node.fill,
                "fill2" => &node.fill2,
                "fillType" => &node.fill_type,
                "stroke" => &node.stroke,
                "shadowColor" => &node.shadow_color,
                "fontFamily" => &node.font_family,
                "fontWeight" => return node.font_weight.to_string(),
                "fontStyle" => &node.font_style,
                "textAlign" => &node.text_align,
                "textDecoration" => &node.text_decoration,
                "textCase" => &node.text_case,
                "direction" => &node.direction,
                _ => {
                    return node
                        .string(prop)
                        .filter(|value| !value.is_empty())
                        .unwrap_or(match prop {
                            "layout" => "none",
                            "layoutAlign" => "start",
                            "constraintH" => "left",
                            "constraintV" => "top",
                            _ => "",
                        })
                        .to_owned()
                }
            }
            .clone()
        })
    })
}

pub fn formatted(value: f64, precision: f64) -> String {
    let value = js_sys::Math::round(value * precision) / precision;
    if value == 0.0 {
        "0".into()
    } else {
        value.to_string()
    }
}

pub fn safe_color(value: &str) -> String {
    if value.starts_with('#')
        && (4..=9).contains(&value.len())
        && value[1..].bytes().all(|byte| byte.is_ascii_hexdigit())
    {
        value.to_owned()
    } else {
        "#8462e8".into()
    }
}

fn hex_display(value: &str) -> String {
    if value == "none" {
        "None".into()
    } else if value.is_empty() {
        "000000".into()
    } else {
        value.replacen('#', "", 1).to_uppercase()
    }
}

fn parse_hex(value: &str) -> Option<String> {
    let value = value.trim_matches(|ch: char| ch.is_whitespace() || ch == '\u{feff}');
    if value.eq_ignore_ascii_case("none") {
        return Some("none".into());
    }
    let digits = value.strip_prefix('#').unwrap_or(value);
    if !matches!(digits.len(), 3 | 6) || !digits.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return None;
    }
    if digits.len() == 3 {
        Some(format!(
            "#{}",
            digits.chars().flat_map(|ch| [ch, ch]).collect::<String>()
        ))
    } else {
        Some(format!("#{digits}"))
    }
}

#[component]
pub fn NumberField(
    editor: Editor,
    nodes: Selection,
    prop: &'static str,
    #[prop(default = "")] label: &'static str,
    #[prop(default = "")] symbol: &'static str,
    #[prop(default = "")] unit: &'static str,
    #[prop(default = 1.0)] step: f64,
    #[prop(optional)] min: Option<f64>,
    #[prop(optional)] max: Option<f64>,
) -> impl IntoView {
    let value = Memo::new(move |_| number(nodes, prop));
    // A native number input reports text on the way to a number ("-", "1e")
    // as empty, which the draft takes for text that is not a number yet. The
    // typed text stays in the field: a draft writes nothing back until the
    // edit ends.
    let draft = Draft::new(
        value.into(),
        |value: &f64| formatted(*value, 100.0),
        |text| text.parse::<f64>().ok().filter(|value| value.is_finite()),
    )
    .with_preview(move |value| shell::set_property(editor, prop, json!(value), true))
    .with_commit(move |value| shell::set_property(editor, prop, json!(value), false))
    .with_cancel(move || crate::pointer::cancel_history(editor));
    view! {
        <label class="field" title=prop>
            <span>{if symbol.is_empty() {label.into_any()} else {icon(symbol, 12)}}</span>
            <input type="number" data-prop=prop aria-label=prop step=step min=min max=max
                prop:value=draft.text()
                on:focus=draft.on_focus()
                on:input=draft.on_input()
                // The spin buttons end a step with `change` alone, no Enter or
                // blur: every step is an edit, and an undo step, of its own.
                on:change=move |_| draft.commit()
                on:blur=draft.on_blur()
                on:keydown=draft.on_keydown()
            />
            {(!unit.is_empty()).then(|| view! {<i class="unit">{unit}</i>})}
        </label>
    }
}

#[component]
pub fn SelectField(
    editor: Editor,
    nodes: Selection,
    prop: &'static str,
    options: Signal<Vec<(String, String)>>,
) -> impl IntoView {
    view! {
        <select class="full-width" data-prop=prop aria-label=prop prop:value=move || text(nodes, prop)
            on:change=move |event| shell::set_property(editor, prop, Value::String(event_target_value(&event)), false)>
            <For each=move || options.get() key=|option| option.clone() children=move |(value, label)| {
                let selected = value.clone();
                view! {<option value=value prop:selected=move || text(nodes, prop) == selected>{label}</option>}
            }/>
        </select>
    }
}

pub fn choices(options: &[(&str, &str)]) -> Signal<Vec<(String, String)>> {
    let options: Vec<_> = options
        .iter()
        .map(|(value, label)| ((*value).to_owned(), (*label).to_owned()))
        .collect();
    Signal::stored(options)
}

#[component]
pub fn ColorField(
    editor: Editor,
    nodes: Selection,
    prop: &'static str,
    #[prop(optional)] opacity: Option<&'static str>,
) -> impl IntoView {
    let picker_dirty = RwSignal::new(false);
    let source = Memo::new(move |_| text(nodes, prop));
    // The hex is judged once, when the edit ends, and not as it is typed:
    // nothing is previewed, and text that is not a colour is refused with a
    // message rather than settling on a colour typed earlier in the edit. So
    // the draft's value is the text itself.
    let hex = Draft::new(
        source.into(),
        |value: &String| hex_display(value),
        |text| Some(text.to_owned()),
    )
    .with_commit(move |text| match parse_hex(&text) {
        Some(color) => shell::set_property(editor, prop, Value::String(color), false),
        None => shell::toast(editor, "Enter a 3- or 6-digit hex color, or None."),
    });
    view! {
        <div class="fill-row">
            <div class="hex-field">
                <input type="color" data-prop=prop aria-label=format!("{prop} color") prop:value=move || safe_color(&source.get())
                    on:input=move |event| {picker_dirty.set(true); shell::set_property(editor, prop, Value::String(event_target_value(&event)), true);}
                    on:change=move |event| {shell::set_property(editor, prop, Value::String(event_target_value(&event)), false); picker_dirty.set(false);}
                    on:blur=move |_| {if picker_dirty.get_untracked() {shell::set_property(editor, prop, Value::String(source.get_untracked()), false); picker_dirty.set(false);}}
                />
                <input type="text" data-hex=prop aria-label=format!("{prop} hex color") spellcheck="false"
                    prop:value=hex.text()
                    on:focus=hex.on_focus()
                    on:input=hex.on_input()
                    on:blur=hex.on_blur()
                    on:keydown=hex.on_keydown()
                />
            </div>
            {opacity.map(|prop| view! {<NumberField editor nodes prop unit="%" min=0.0 max=100.0/>})}
        </div>
    }
}

#[component]
pub fn ActionButton(
    editor: Editor,
    action: &'static str,
    title: &'static str,
    #[prop(default = "")] symbol: &'static str,
) -> impl IntoView {
    view! {
        <button class="icon-button small" data-action=action title=title aria-label=title on:click=move |_| shell::dispatch(editor, action)>
            {if symbol.is_empty() {"···".into_any()} else {icon(symbol, 14)}}
        </button>
    }
}
