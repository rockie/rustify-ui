use super::{
    conditions,
    values::{self, Values},
    *,
};
use crate::{
    i18n::Direction,
    theme::{
        css_px_to_points, parse_color, parse_font_stack, ResolvedFont, ResolvedTheme, Rgba,
        ShadowLayer,
    },
};
use rustify_style::{
    program::{GpuProgram, GpuValue},
    text::Text,
};
use std::collections::BTreeMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Field {
    Color,
    FontSize,
    Width,
    Height,
    MinWidth,
    MinHeight,
    MaxWidth,
    MaxHeight,
    Background,
    BorderColor(usize),
    Padding(usize),
    Gap(usize),
    Radius(usize),
    BorderWidth(usize),
    BorderStyle(usize),
    Opacity,
    Display,
    Flow,
    Items,
    Justify,
    FontFamily,
    FontWeight,
    LineHeight,
    Tracking,
    Shadow,
}
#[derive(Clone, Copy)]
enum Part {
    Whole,
    Box(usize),
    Pair(usize),
    Radius(usize),
    Border(u8),
}
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Key {
    important: bool,
    specificity: usize,
    order: usize,
    candidate: Vec<u16>,
    declaration: usize,
}
struct Entry<'a> {
    field: Field,
    part: Part,
    value: &'a GpuValue,
    candidate: &'a Text,
    property: &'a str,
    key: Key,
}
impl Entry<'_> {
    fn text(&self, values: &Values) -> Result<String, String> {
        let text = values.resolve(self.value)?;
        Ok(match self.part {
            Part::Whole | Part::Radius(_) => text,
            Part::Box(index) => box_part(&text, index)?.into(),
            Part::Pair(index) => pair_part(&text, index)?.into(),
            Part::Border(part) => border_part(&text, part)?,
        })
    }
}
fn push_diagnostic(
    out: &mut Vec<GpuDiagnostic>,
    candidate: &Text,
    property: Option<&str>,
    kind: GpuDiagnosticKind,
    reason: impl Into<String>,
) {
    let diagnostic = GpuDiagnostic {
        candidate: candidate.clone(),
        property: property.map(str::to_owned),
        kind,
        reason: reason.into(),
    };
    if !out.contains(&diagnostic) {
        out.push(diagnostic);
    }
}
fn destinations(
    property: &str,
    direction: Direction,
    cap: GpuCapabilities,
) -> Result<Vec<(Field, Part)>, String> {
    let [start, end] = if direction == Direction::LeftToRight {
        [3, 1]
    } else {
        [1, 3]
    };
    let single = |field| vec![(field, Part::Whole)];
    let edges = |field: fn(usize) -> Field| {
        (0..4)
            .map(|index| (field(index), Part::Box(index)))
            .collect()
    };
    let pair = |field: fn(usize) -> Field, indices: [usize; 2]| {
        indices
            .into_iter()
            .enumerate()
            .map(|(part, index)| (field(index), Part::Pair(part)))
            .collect()
    };
    Ok(match property {
        "color" => single(Field::Color),
        "background-color" => single(Field::Background),
        "padding" => edges(Field::Padding),
        "padding-top" => single(Field::Padding(0)),
        "padding-right" => single(Field::Padding(1)),
        "padding-bottom" => single(Field::Padding(2)),
        "padding-left" => single(Field::Padding(3)),
        "padding-inline" => pair(Field::Padding, [start, end]),
        "padding-block" => pair(Field::Padding, [0, 2]),
        "padding-inline-start" => single(Field::Padding(start)),
        "padding-inline-end" => single(Field::Padding(end)),
        "padding-block-start" => single(Field::Padding(0)),
        "padding-block-end" => single(Field::Padding(2)),
        "gap" => vec![
            (Field::Gap(0), Part::Pair(0)),
            (Field::Gap(1), Part::Pair(1)),
        ],
        "row-gap" => single(Field::Gap(0)),
        "column-gap" => single(Field::Gap(1)),
        "width" => single(Field::Width),
        "height" => single(Field::Height),
        "min-width" => single(Field::MinWidth),
        "min-height" => single(Field::MinHeight),
        "max-width" => single(Field::MaxWidth),
        "max-height" => single(Field::MaxHeight),
        "inline-size" => single(Field::Width),
        "block-size" => single(Field::Height),
        "min-inline-size" => single(Field::MinWidth),
        "min-block-size" => single(Field::MinHeight),
        "max-inline-size" => single(Field::MaxWidth),
        "max-block-size" => single(Field::MaxHeight),
        "border-color" => edges(Field::BorderColor),
        "border-top-color" => single(Field::BorderColor(0)),
        "border-right-color" => single(Field::BorderColor(1)),
        "border-bottom-color" => single(Field::BorderColor(2)),
        "border-left-color" => single(Field::BorderColor(3)),
        "border-inline-color" => pair(Field::BorderColor, [start, end]),
        "border-block-color" => pair(Field::BorderColor, [0, 2]),
        "border-inline-start-color" => single(Field::BorderColor(start)),
        "border-inline-end-color" => single(Field::BorderColor(end)),
        "border-block-start-color" => single(Field::BorderColor(0)),
        "border-block-end-color" => single(Field::BorderColor(2)),
        "border-width" => edges(Field::BorderWidth),
        "border-top-width" => single(Field::BorderWidth(0)),
        "border-right-width" => single(Field::BorderWidth(1)),
        "border-bottom-width" => single(Field::BorderWidth(2)),
        "border-left-width" => single(Field::BorderWidth(3)),
        "border-inline-width" => pair(Field::BorderWidth, [start, end]),
        "border-block-width" => pair(Field::BorderWidth, [0, 2]),
        "border-inline-start-width" => single(Field::BorderWidth(start)),
        "border-inline-end-width" => single(Field::BorderWidth(end)),
        "border-block-start-width" => single(Field::BorderWidth(0)),
        "border-block-end-width" => single(Field::BorderWidth(2)),
        "border-style" => edges(Field::BorderStyle),
        "border-top-style" => single(Field::BorderStyle(0)),
        "border-right-style" => single(Field::BorderStyle(1)),
        "border-bottom-style" => single(Field::BorderStyle(2)),
        "border-left-style" => single(Field::BorderStyle(3)),
        "border-inline-style" => pair(Field::BorderStyle, [start, end]),
        "border-block-style" => pair(Field::BorderStyle, [0, 2]),
        "border-inline-start-style" => single(Field::BorderStyle(start)),
        "border-inline-end-style" => single(Field::BorderStyle(end)),
        "border-block-start-style" => single(Field::BorderStyle(0)),
        "border-block-end-style" => single(Field::BorderStyle(2)),
        "border" => {
            let mut fields = Vec::new();
            for index in 0..4 {
                fields.extend([
                    (Field::BorderWidth(index), Part::Border(0)),
                    (Field::BorderColor(index), Part::Border(1)),
                    (Field::BorderStyle(index), Part::Border(2)),
                ]);
            }
            fields
        }
        "border-radius" => (0..4)
            .map(|index| (Field::Radius(index), Part::Radius(index)))
            .collect(),
        "border-top-left-radius" => single(Field::Radius(0)),
        "border-top-right-radius" => single(Field::Radius(1)),
        "border-bottom-right-radius" => single(Field::Radius(2)),
        "border-bottom-left-radius" => single(Field::Radius(3)),
        "border-start-start-radius" => single(Field::Radius(if start == 3 { 0 } else { 1 })),
        "border-start-end-radius" => single(Field::Radius(if end == 1 { 1 } else { 0 })),
        "border-end-start-radius" => single(Field::Radius(if start == 3 { 3 } else { 2 })),
        "border-end-end-radius" => single(Field::Radius(if end == 1 { 2 } else { 3 })),
        "opacity" if cap.independent_leaf_opacity => single(Field::Opacity),
        "opacity" => return Err("opacity requires group composition for this control".into()),
        "display" if cap.flex_layout => single(Field::Display),
        "flex-direction" if cap.flex_layout => single(Field::Flow),
        "align-items" if cap.flex_layout => single(Field::Items),
        "justify-content" if cap.flex_layout => single(Field::Justify),
        "font-family" if cap.text => single(Field::FontFamily),
        "font-size" if cap.text => single(Field::FontSize),
        "font-weight" if cap.text => single(Field::FontWeight),
        "line-height" if cap.text => single(Field::LineHeight),
        "letter-spacing" if cap.text => single(Field::Tracking),
        "box-shadow" if cap.outer_shadows => single(Field::Shadow),
        _ => return Err(format!("no GPU declaration capability for {property}")),
    })
}
fn box_part(text: &str, index: usize) -> Result<&str, String> {
    let parts = values::tokens(text, None)?;
    let which = match parts.len() {
        1 => 0,
        2 => index % 2,
        3 => [0, 1, 2, 1][index],
        4 => index,
        _ => return Err("box shorthand needs one to four values".into()),
    };
    Ok(parts[which])
}
fn pair_part(text: &str, index: usize) -> Result<&str, String> {
    let parts = values::tokens(text, None)?;
    match parts.len() {
        1 => Ok(parts[0]),
        2 => Ok(parts[index]),
        _ => Err("axis shorthand needs one or two values".into()),
    }
}
fn border_part(text: &str, part: u8) -> Result<String, String> {
    let tokens = values::tokens(text, None)?;
    if tokens.len() != 3 {
        return Err("border shorthand needs explicit width, style and color".into());
    }
    Ok(tokens[match part {
        0 => 0,
        1 => 2,
        _ => 1,
    }]
    .into())
}
fn choose<'a, T>(
    entries: &'a [Entry<'a>],
    field: Field,
    diagnostics: &mut Vec<GpuDiagnostic>,
    mut parse: impl FnMut(&Entry<'a>) -> Result<T, String>,
) -> Option<(T, &'a Entry<'a>)> {
    let mut best: Option<(T, &Entry<'_>)> = None;
    for entry in entries.iter().filter(|entry| entry.field == field) {
        match parse(entry) {
            Ok(value) => {
                if best.as_ref().is_none_or(|(_, old)| entry.key > old.key) {
                    best = Some((value, entry));
                }
            }
            Err(reason) => push_diagnostic(
                diagnostics,
                entry.candidate,
                Some(entry.property),
                GpuDiagnosticKind::UnsupportedValue,
                reason,
            ),
        }
    }
    best
}
fn nonnegative(value: f64) -> Result<f64, String> {
    if value < 0. {
        Err("negative value is invalid for this property".into())
    } else {
        Ok(value)
    }
}
fn color(text: &str, current: Rgba) -> Result<Rgba, String> {
    let text = text.trim();
    if let Some(body) = text
        .strip_prefix("color-mix(")
        .and_then(|body| body.strip_suffix(')'))
    {
        let parts = values::tokens(body, Some(','))?;
        if parts.len() != 3 || parts[0] != "in oklab" || parts[2] != "transparent" {
            return Err("only compiler oklab-to-transparent alpha mixes are supported".into());
        }
        let source = values::tokens(parts[1], None)?;
        if source.len() != 2 || !source[1].ends_with('%') {
            return Err("alpha mix requires an explicit percentage".into());
        }
        let alpha = values::number(source[1])?;
        if !(0. ..=1.).contains(&alpha) {
            return Err("alpha mix percentage must be in 0..=100%".into());
        }
        let mut rgba = base_color(source[0], current)?;
        rgba.a *= alpha;
        return Ok(rgba);
    }
    base_color(text, current)
}
fn base_color(text: &str, current: Rgba) -> Result<Rgba, String> {
    match text.trim() {
        "currentColor" | "currentcolor" => Ok(current),
        "black" => Ok(parse_color("#000")?.rgba),
        "white" => Ok(parse_color("#fff")?.rgba),
        _ => parse_color(text).map(|parsed| parsed.rgba),
    }
}
fn dimension(
    text: &str,
    max: bool,
    rem: f64,
    em: f64,
    percent: Option<f64>,
) -> Result<GpuLength, String> {
    match text.trim() {
        "auto" if !max => Ok(GpuLength::Auto),
        "none" if max => Ok(GpuLength::Unbounded),
        _ => Ok(GpuLength::Pixels(nonnegative(values::length(
            text, rem, em, percent,
        )?)?)),
    }
}
fn clamp_dimension(value: &mut GpuLength, min: GpuLength, max: GpuLength) {
    if let GpuLength::Pixels(value) = value {
        let min = if let GpuLength::Pixels(min) = min {
            min
        } else {
            0.
        };
        let max = if let GpuLength::Pixels(max) = max {
            max.max(min)
        } else {
            f64::MAX
        };
        *value = value.max(min).min(max);
    }
}
fn control_size(style: &GpuComputedStyle, context: &GpuResolveContext) -> [Option<f64>; 2] {
    [
        match style.width {
            GpuLength::Pixels(px) => Some(px),
            _ => context.control_size[0],
        },
        match style.height {
            GpuLength::Pixels(px) => Some(px),
            _ => context.control_size[1],
        },
    ]
}
fn radius(
    entry: &Entry<'_>,
    variables: &Values,
    style: &GpuComputedStyle,
    context: &GpuResolveContext,
) -> Result<f64, String> {
    let text = entry.text(variables)?;
    let size = control_size(style, context);
    let (x, y) = if let Part::Radius(index) = entry.part {
        let axes = values::tokens(&text, Some('/'))?;
        if axes.len() > 2 {
            return Err("invalid radius shorthand".into());
        }
        (
            box_part(axes[0], index)?,
            box_part(axes.get(1).copied().unwrap_or(axes[0]), index)?,
        )
    } else {
        let parts = values::tokens(&text, None)?;
        match parts.len() {
            1 => (parts[0], parts[0]),
            2 => (parts[0], parts[1]),
            _ => return Err("corner radius needs one or two lengths".into()),
        }
    };
    let x = nonnegative(values::length(
        x,
        context.rem_px,
        style.font_size_px,
        size[0],
    )?)?;
    let y = nonnegative(values::length(
        y,
        context.rem_px,
        style.font_size_px,
        size[1],
    )?)?;
    if x != y {
        return Err("elliptical corners have no GPU draw capability".into());
    }
    Ok(x)
}
fn font_stack(
    requested: Vec<String>,
    weight: u16,
    theme: &ResolvedTheme,
    context: &GpuResolveContext,
) -> Result<ResolvedFont, String> {
    for requested_family in &requested {
        let family = match requested_family.as_str() {
            "sans-serif" | "ui-sans-serif" | "system-ui" => theme.fonts[0].family.as_str(),
            "serif" | "ui-serif" => theme.fonts[1].family.as_str(),
            "monospace" | "ui-monospace" => theme.fonts[2].family.as_str(),
            _ => requested_family,
        };
        if context
            .fonts
            .iter()
            .any(|face| face.family.eq_ignore_ascii_case(family))
        {
            let face = context
                .fonts
                .iter()
                .find(|face| {
                    face.family.eq_ignore_ascii_case(family)
                        && face.weight == weight
                        && !face.resource_path.is_empty()
                })
                .ok_or_else(|| {
                    format!("no packaged font resource for {family} at weight {weight}")
                })?;
            return Ok(ResolvedFont {
                requested: requested.clone(),
                family: face.family.clone(),
                resource_path: Some(face.resource_path.clone()),
                fallback: !requested[0].eq_ignore_ascii_case(&face.family),
            });
        }
    }
    Err(format!(
        "no packaged font resource for {}",
        requested.join(", ")
    ))
}
fn shadow(text: &str, current: Rgba, rem: f64, font: f64) -> Result<Vec<ShadowLayer>, String> {
    if text.trim() == "none" {
        return Ok(Vec::new());
    }
    let mut layers = Vec::new();
    for layer in values::tokens(text, Some(','))? {
        let tokens = values::tokens(layer, None)?;
        if tokens.contains(&"inset") {
            return Err("inset shadows cannot be represented by ShadowLayer".into());
        }
        let mut lengths = Vec::new();
        let mut rgba = None;
        for token in tokens {
            if let Ok(value) = values::length(token, rem, font, None) {
                lengths.push(value);
            } else if rgba.is_none() {
                rgba = Some(color(token, current)?);
            } else {
                return Err("unsupported shadow layer".into());
            }
        }
        if !(2..=4).contains(&lengths.len()) {
            return Err("outer shadow needs two to four lengths".into());
        }
        let blur = nonnegative(*lengths.get(2).unwrap_or(&0.))?;
        let color = rgba.unwrap_or(current);
        if color.a == 0. {
            continue;
        }
        layers.push(ShadowLayer {
            color,
            offset_x: lengths[0],
            offset_y: lengths[1],
            blur,
            spread: *lengths.get(3).unwrap_or(&0.),
        });
    }
    Ok(layers)
}
fn valid_style(style: &GpuComputedStyle) -> Result<(), String> {
    for rgba in [style.color, style.background]
        .into_iter()
        .chain(style.border_colors)
        .chain(style.shadows.iter().map(|shadow| shadow.color))
    {
        for value in [rgba.r, rgba.g, rgba.b, rgba.a] {
            if !(0. ..=1.).contains(&values::finite(value)?) {
                return Err("snapshot color channels must be in 0..=1".into());
            }
        }
    }
    for value in style
        .padding
        .into_iter()
        .chain(style.gap)
        .chain(style.radii)
        .chain(style.border_widths)
    {
        nonnegative(values::finite(value)?)?;
    }
    for length in [
        style.width,
        style.height,
        style.min_width,
        style.min_height,
        style.max_width,
        style.max_height,
    ] {
        if let GpuLength::Pixels(value) = length {
            nonnegative(values::finite(value)?)?;
        }
    }
    if values::finite(style.font_size_px)? <= 0.
        || !style.font_size_points.is_finite()
        || style.font_size_points <= 0.
    {
        return Err("snapshot font size must be finite and positive".into());
    }
    if !(0. ..=1.).contains(&values::finite(style.opacity)?) {
        return Err("snapshot opacity must be in 0..=1".into());
    }
    values::finite(style.letter_spacing_em)?;
    if let GpuLineHeight::Pixels(value) = style.line_height {
        nonnegative(values::finite(value)?)?;
    }
    for layer in &style.shadows {
        for value in [layer.offset_x, layer.offset_y, layer.blur, layer.spread] {
            values::finite(value)?;
        }
        nonnegative(layer.blur)?;
    }
    Ok(())
}
fn valid_context(context: &GpuResolveContext) -> Result<(), String> {
    valid_style(&context.base_style)?;
    if let Some(style) = &context.previous_style {
        valid_style(style)?;
    }
    for value in context
        .viewport
        .into_iter()
        .chain(context.parent_size.into_iter().flatten())
        .chain(context.control_size.into_iter().flatten())
    {
        nonnegative(values::finite(value)?)?;
    }
    for (value, name) in [
        (context.rem_px, "rem"),
        (context.media_rem_px, "media rem"),
        (context.parent_font_size_px, "parent font size"),
    ] {
        if values::finite(value)? <= 0. {
            return Err(format!("{name} must be positive"));
        }
    }
    Ok(())
}
fn validate_uniform(
    style: &mut GpuComputedStyle,
    context: &GpuResolveContext,
    entries: &[Entry<'_>],
    diagnostics: &mut Vec<GpuDiagnostic>,
    mut radius_applied: bool,
) {
    let mut report = |field: Field, reason: &str| {
        if let Some(entry) = entries
            .iter()
            .filter(|e| e.field == field)
            .max_by(|a, b| a.key.cmp(&b.key))
        {
            push_diagnostic(
                diagnostics,
                entry.candidate,
                Some(entry.property),
                GpuDiagnosticKind::UnsupportedUtility,
                reason,
            );
        }
    };
    if !context.capabilities.nonuniform_corners && style.radii.iter().any(|r| *r != style.radii[0])
    {
        for index in 0..4 {
            report(
                Field::Radius(index),
                "control cannot draw nonuniform corners",
            );
        }
        style.radii = context.base_style.radii;
        radius_applied = false;
    }
    if !context.capabilities.nonuniform_borders
        && (style
            .border_widths
            .iter()
            .any(|v| *v != style.border_widths[0])
            || style
                .border_colors
                .iter()
                .any(|v| *v != style.border_colors[0]))
    {
        for index in 0..4 {
            report(
                Field::BorderWidth(index),
                "control cannot draw nonuniform borders",
            );
            report(
                Field::BorderColor(index),
                "control cannot draw nonuniform borders",
            );
        }
        style.border_widths = context.base_style.border_widths;
        style.border_colors = context.base_style.border_colors;
    }
    let size = if radius_applied {
        control_size(style, context)
    } else {
        [None, None]
    };
    if let [Some(width), Some(height)] = size {
        let [r0, r1, r2, r3] = style.radii;
        let mut scale = 1f64;
        for (available, total) in [
            (width, r0 + r1),
            (width, r3 + r2),
            (height, r0 + r3),
            (height, r1 + r2),
        ] {
            if total > available && total > 0. {
                scale = scale.min(available / total);
            }
        }
        for radius in &mut style.radii {
            *radius *= scale;
        }
    }
}

pub(super) fn resolve(
    programs: &[GpuProgram],
    theme: &ResolvedTheme,
    context: &GpuResolveContext,
) -> GpuResolution {
    let mut style = context.base_style.clone();
    let mut diagnostics = Vec::new();
    let mut entries = Vec::new();
    let mut variable_winners: BTreeMap<String, (String, Key)> = BTreeMap::new();
    if let Err(reason) = valid_context(context) {
        push_diagnostic(
            &mut diagnostics,
            &Text::default(),
            None,
            GpuDiagnosticKind::InvalidContext,
            reason,
        );
        return finish(style, diagnostics, context, true);
    }
    for program in programs {
        let mut active = true;
        let mut specificity = 1;
        let mut supported = true;
        for cond in &program.conditions {
            match conditions::condition(cond, theme, context) {
                Ok((matches, weight)) => {
                    active &= matches;
                    specificity += weight;
                }
                Err(reason) => {
                    supported = false;
                    push_diagnostic(
                        &mut diagnostics,
                        &program.candidate,
                        None,
                        GpuDiagnosticKind::UnsupportedVariant,
                        reason,
                    );
                }
            }
        }
        if program.declarations.is_empty() {
            push_diagnostic(
                &mut diagnostics,
                &program.candidate,
                None,
                GpuDiagnosticKind::UnsupportedUtility,
                "compiler IR contains no declarations",
            );
        }
        for (index, declaration) in program.declarations.iter().enumerate() {
            let key = Key {
                important: declaration.important,
                specificity,
                order: program.cascade_order,
                candidate: program.candidate.units().to_vec(),
                declaration: index,
            };
            if declaration.property.starts_with("--") {
                if active && supported {
                    let replace = variable_winners
                        .get(&declaration.property)
                        .is_none_or(|(_, old)| key > *old);
                    if replace {
                        variable_winners.insert(
                            declaration.property.clone(),
                            (values::raw(&declaration.value), key),
                        );
                    }
                }
                continue;
            }
            match destinations(
                &declaration.property,
                context.direction,
                context.capabilities,
            ) {
                Ok(fields) => {
                    if active && supported {
                        for (field, part) in fields {
                            entries.push(Entry {
                                field,
                                part,
                                value: &declaration.value,
                                candidate: &program.candidate,
                                property: &declaration.property,
                                key: key.clone(),
                            });
                        }
                    }
                }
                Err(reason) => push_diagnostic(
                    &mut diagnostics,
                    &program.candidate,
                    Some(&declaration.property),
                    GpuDiagnosticKind::UnsupportedUtility,
                    reason,
                ),
            }
        }
    }
    let mut variables: BTreeMap<rustify_style::text::Text, rustify_style::text::Text> = theme
        .values
        .0
        .iter()
        .filter(|(key, _)| key.as_str() != "spacing")
        .map(|(key, value)| (format!("--{key}").into(), value.clone().into()))
        .collect();
    for (name, value) in theme.properties() {
        if name != "--spacing" {
            variables.entry(name.into()).or_insert(value.into());
        }
    }
    variables.extend(context.variables.clone());
    variables.extend(
        variable_winners
            .into_iter()
            .map(|(name, (value, _))| (name.into(), value.into())),
    );
    let variables = Values::new(variables);
    if let Some((rgba, _)) = choose(&entries, Field::Color, &mut diagnostics, |entry| {
        color(&entry.text(&variables)?, context.base_style.color)
    }) {
        style.color = rgba;
    }
    if let Some((px, _)) = choose(&entries, Field::FontSize, &mut diagnostics, |entry| {
        let px = values::length(
            &entry.text(&variables)?,
            context.rem_px,
            context.parent_font_size_px,
            Some(context.parent_font_size_px),
        )?;
        if px <= 0. {
            return Err("font size must be positive".into());
        }
        Ok(px)
    }) {
        style.font_size_px = px;
        style.font_size_points = css_px_to_points(px);
    }
    let mut dimension_applied = [false; 2];
    for (field, axis, max) in [
        (Field::Width, 0, false),
        (Field::Height, 1, false),
        (Field::MinWidth, 0, false),
        (Field::MinHeight, 1, false),
        (Field::MaxWidth, 0, true),
        (Field::MaxHeight, 1, true),
    ] {
        if let Some((value, _)) = choose(&entries, field, &mut diagnostics, |entry| {
            dimension(
                &entry.text(&variables)?,
                max,
                context.rem_px,
                style.font_size_px,
                context.parent_size[axis],
            )
        }) {
            dimension_applied[axis] = true;
            match field {
                Field::Width => style.width = value,
                Field::Height => style.height = value,
                Field::MinWidth => style.min_width = value,
                Field::MinHeight => style.min_height = value,
                Field::MaxWidth => style.max_width = value,
                Field::MaxHeight => style.max_height = value,
                _ => {}
            }
        }
    }
    if dimension_applied[0] {
        clamp_dimension(&mut style.width, style.min_width, style.max_width);
    }
    if dimension_applied[1] {
        clamp_dimension(&mut style.height, style.min_height, style.max_height);
    }
    if let Some((value, _)) = choose(&entries, Field::Background, &mut diagnostics, |entry| {
        color(&entry.text(&variables)?, style.color)
    }) {
        style.background = value;
    }
    let mut radius_applied = false;
    for index in 0..4 {
        if let Some((value, _)) =
            choose(&entries, Field::Padding(index), &mut diagnostics, |entry| {
                nonnegative(values::length(
                    &entry.text(&variables)?,
                    context.rem_px,
                    style.font_size_px,
                    context.parent_size[0],
                )?)
            })
        {
            style.padding[index] = value;
        }
        if let Some((value, _)) = choose(
            &entries,
            Field::BorderWidth(index),
            &mut diagnostics,
            |entry| {
                nonnegative(values::length(
                    &entry.text(&variables)?,
                    context.rem_px,
                    style.font_size_px,
                    None,
                )?)
            },
        ) {
            style.border_widths[index] = value;
        }
        if let Some((value, _)) = choose(
            &entries,
            Field::BorderColor(index),
            &mut diagnostics,
            |entry| color(&entry.text(&variables)?, style.color),
        ) {
            style.border_colors[index] = value;
        }
        if let Some((value, _)) =
            choose(&entries, Field::Radius(index), &mut diagnostics, |entry| {
                radius(entry, &variables, &style, context)
            })
        {
            style.radii[index] = value;
            radius_applied = true;
        }
        if let Some((hidden, _)) = choose(
            &entries,
            Field::BorderStyle(index),
            &mut diagnostics,
            |entry| match entry.text(&variables)?.trim() {
                "solid" => Ok(false),
                "none" | "hidden" => Ok(true),
                _ => Err("only solid or absent GPU borders are supported".into()),
            },
        ) {
            if hidden {
                style.border_widths[index] = 0.;
            }
        }
    }
    let size = control_size(&style, context);
    for index in 0..2 {
        if let Some((value, _)) = choose(&entries, Field::Gap(index), &mut diagnostics, |entry| {
            let text = entry.text(&variables)?;
            if text.trim() == "normal" {
                return Ok(0.);
            }
            nonnegative(values::length(
                &text,
                context.rem_px,
                style.font_size_px,
                size[1 - index],
            )?)
        }) {
            style.gap[index] = value;
        }
    }
    if let Some((value, _)) = choose(&entries, Field::Opacity, &mut diagnostics, |entry| {
        Ok(values::number(&entry.text(&variables)?)?.clamp(0., 1.))
    }) {
        style.opacity = value;
    }
    choose(&entries, Field::Display, &mut diagnostics, |entry| {
        if entry.text(&variables)?.trim() == "flex" {
            Ok(())
        } else {
            Err("only flex display is supported by this container".into())
        }
    });
    if let Some((value, _)) = choose(&entries, Field::Flow, &mut diagnostics, |entry| match entry
        .text(&variables)?
        .trim()
    {
        "row" => Ok(GpuFlow::Row),
        "column" => Ok(GpuFlow::Column),
        _ => Err("only non-reversed row/column flex containers are supported".into()),
    }) {
        style.flow = value;
    }
    if let Some((value, _)) = choose(
        &entries,
        Field::Items,
        &mut diagnostics,
        |entry| match entry.text(&variables)?.trim() {
            "start" | "flex-start" => Ok(GpuAlignment::Start),
            "center" => Ok(GpuAlignment::Center),
            "end" | "flex-end" => Ok(GpuAlignment::End),
            "stretch" => Ok(GpuAlignment::Stretch),
            _ => Err("unsupported GPU cross-axis alignment".into()),
        },
    ) {
        style.items = value;
    }
    if let Some((value, _)) = choose(
        &entries,
        Field::Justify,
        &mut diagnostics,
        |entry| match entry.text(&variables)?.trim() {
            "start" | "flex-start" => Ok(GpuJustification::Start),
            "center" => Ok(GpuJustification::Center),
            "end" | "flex-end" => Ok(GpuJustification::End),
            "space-between" => Ok(GpuJustification::SpaceBetween),
            "space-around" => Ok(GpuJustification::SpaceAround),
            "space-evenly" => Ok(GpuJustification::SpaceEvenly),
            _ => Err("unsupported GPU main-axis justification".into()),
        },
    ) {
        style.justify = value;
    }
    let family = choose(&entries, Field::FontFamily, &mut diagnostics, |entry| {
        parse_font_stack(&entry.text(&variables)?).map_err(|err| err.message)
    });
    let weight = choose(&entries, Field::FontWeight, &mut diagnostics, |entry| {
        let text = entry.text(&variables)?;
        let value = match text.trim() {
            "normal" => 400.,
            "bold" => 700.,
            _ if !text.trim().ends_with('%') => values::number(&text)?,
            _ => return Err("font weight does not accept percentages".into()),
        };
        if !(1. ..=1000.).contains(&value) || value.fract() != 0. {
            return Err("font weight requires an integral value in 1..=1000".into());
        }
        Ok(value as u16)
    });
    if family.is_some() || weight.is_some() {
        let requested = family
            .as_ref()
            .map(|(family, _)| family.clone())
            .unwrap_or_else(|| vec![style.font.family.clone()]);
        let requested_weight = weight
            .as_ref()
            .map_or(style.font_weight, |(weight, _)| *weight);
        match font_stack(requested, requested_weight, theme, context) {
            Ok(font) => {
                style.font = font;
                style.font_weight = requested_weight;
            }
            Err(reason) => {
                let entry = weight
                    .as_ref()
                    .map(|(_, entry)| *entry)
                    .or_else(|| family.as_ref().map(|(_, entry)| *entry));
                if let Some(entry) = entry {
                    push_diagnostic(
                        &mut diagnostics,
                        entry.candidate,
                        Some(entry.property),
                        GpuDiagnosticKind::UnsupportedValue,
                        reason,
                    );
                }
            }
        }
    }
    if let Some((value, _)) = choose(&entries, Field::LineHeight, &mut diagnostics, |entry| {
        let text = entry.text(&variables)?;
        if text.trim() == "normal" {
            return Ok(GpuLineHeight::Normal);
        }
        let pixels = if let Ok(number) = values::number(&text) {
            number * style.font_size_px
        } else {
            values::length(
                &text,
                context.rem_px,
                style.font_size_px,
                Some(style.font_size_px),
            )?
        };
        Ok(GpuLineHeight::Pixels(nonnegative(values::finite(pixels)?)?))
    }) {
        style.line_height = value;
    }
    if let Some((value, _)) = choose(&entries, Field::Tracking, &mut diagnostics, |entry| {
        let text = entry.text(&variables)?;
        if text.trim() == "normal" {
            return Ok(0.);
        }
        values::finite(
            values::length(&text, context.rem_px, style.font_size_px, None)? / style.font_size_px,
        )
    }) {
        style.letter_spacing_em = value;
    }
    if let Some((value, _)) = choose(&entries, Field::Shadow, &mut diagnostics, |entry| {
        shadow(
            &entry.text(&variables)?,
            style.color,
            context.rem_px,
            style.font_size_px,
        )
    }) {
        style.shadows = value;
    }
    validate_uniform(
        &mut style,
        context,
        &entries,
        &mut diagnostics,
        radius_applied,
    );
    finish(style, diagnostics, context, false)
}
fn finish(
    style: GpuComputedStyle,
    diagnostics: Vec<GpuDiagnostic>,
    context: &GpuResolveContext,
    invalid_context: bool,
) -> GpuResolution {
    if !diagnostics.is_empty() && (context.policy == GpuApplyPolicy::Strict || invalid_context) {
        return GpuResolution {
            status: GpuResolutionStatus::Rejected,
            style: context
                .previous_style
                .as_ref()
                .unwrap_or(&context.base_style)
                .clone(),
            diagnostics,
            changed: false,
        };
    }
    let changed = context
        .previous_style
        .as_ref()
        .is_none_or(|previous| !previous.rendered_eq(&style));
    GpuResolution {
        status: if diagnostics.is_empty() {
            GpuResolutionStatus::Applied
        } else {
            GpuResolutionStatus::Partial
        },
        style,
        diagnostics,
        changed,
    }
}

#[cfg(test)]
mod text_identity_tests {
    use super::*;
    use crate::theme::{font_context, resolve as resolve_theme, Theme, ThemeDocument, ThemeMode};
    use rustify_style::program::{GpuCondition, GpuDeclaration};

    fn theme() -> ResolvedTheme {
        resolve_theme(
            &ThemeDocument::from_legacy(Theme::light(), Theme::dark()),
            ThemeMode::Light,
            &font_context(16., false),
        )
        .unwrap()
    }
    fn program(candidate: &[u16], property: &str, value: &str) -> GpuProgram {
        GpuProgram {
            candidate: Text::from_units(candidate.to_vec()),
            conditions: Vec::new(),
            declarations: vec![GpuDeclaration {
                property: property.into(),
                value: GpuValue::CssExpression {
                    value: value.into(),
                },
                important: false,
            }],
            cascade_order: 0,
        }
    }

    #[test]
    fn tie_order_uses_candidate_units_and_diagnostics_keep_distinct_lone_identities() {
        let theme = theme();
        let context = GpuResolveContext::new(GpuComputedStyle::from_theme(&theme));
        let output = resolve(
            &[
                program(&[0xd83d, 0xde00], "width", "10px"),
                program(&[0xe000], "width", "20px"),
            ],
            &theme,
            &context,
        );
        assert_eq!(output.style.width, GpuLength::Pixels(20.));
        let output = resolve(
            &[
                program(&[0xd800], "unknown", "x"),
                program(&[0xdc00], "unknown", "x"),
                program(&[0xfffd], "unknown", "x"),
            ],
            &theme,
            &context,
        );
        assert_eq!(output.diagnostics.len(), 3);
        let identities: Vec<_> = output
            .diagnostics
            .iter()
            .map(|diagnostic| diagnostic.candidate.units())
            .collect();
        assert_eq!(identities, vec![&[0xd800][..], &[0xdc00], &[0xfffd]]);
    }

    #[test]
    fn non_scalar_selector_and_media_are_visible_failures_with_original_candidate_identity() {
        let theme = theme();
        let context = GpuResolveContext::new(GpuComputedStyle::from_theme(&theme));
        for condition in [
            GpuCondition::Selector {
                selector: Text::from_units(vec![38, 58, 0xd800]),
            },
            GpuCondition::Media {
                query: Text::from_units(vec![40, 0xdc00, 41]),
            },
        ] {
            let mut program = program(&[0xd800, 0xdc00, 0xd800], "width", "20px");
            program.conditions.push(condition);
            let output = resolve(&[program], &theme, &context);
            assert_eq!(output.status, GpuResolutionStatus::Rejected);
            assert_eq!(output.style, context.base_style);
            assert!(output.diagnostics.iter().any(|diagnostic| diagnostic.kind
                == GpuDiagnosticKind::UnsupportedVariant
                && diagnostic.candidate.units() == [0xd800, 0xdc00, 0xd800]
                && diagnostic.reason.contains("non-scalar")));
        }
    }
}
