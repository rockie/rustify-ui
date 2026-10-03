//! Finite CSS color syntax resolved to unpremultiplied sRGB.

/// Unpremultiplied, gamma-encoded sRGB channels and alpha in `0..=1`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rgba {
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub a: f64,
}

/// A parsed color and whether its OKLCH input required sRGB channel clipping.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ParsedColor {
    pub rgba: Rgba,
    pub out_of_gamut: bool,
}

/// CSS output notation for an already resolved sRGB color.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorFormat {
    Hex,
    Rgb,
    Hsl,
    Oklch,
}

/// HSL with hue in degrees, saturation/lightness in `0..=1` and separate alpha.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Hsla {
    pub h: f64,
    pub s: f64,
    pub l: f64,
    pub a: f64,
}

/// Parses HEX, RGB, HSL, OKLCH and `transparent`; alpha must be in `0..=1`.
///
/// RGB/HSL channels and OKLCH lightness follow CSS range clamping. Unsupported
/// syntax, nonfinite numbers and conversions that overflow return an error.
pub fn parse_color(input: &str) -> Result<ParsedColor, String> {
    let input = input.trim();
    if input.eq_ignore_ascii_case("transparent") {
        return Ok(parsed([0.0; 3], 0.0));
    }
    if let Some(hex) = input.strip_prefix('#') {
        return parse_hex(hex);
    }
    let (name, body) = input
        .split_once('(')
        .ok_or_else(|| "expected HEX, RGB, HSL, OKLCH or transparent".to_string())?;
    let body = body
        .strip_suffix(')')
        .filter(|body| !body.contains(['(', ')']))
        .ok_or_else(|| "invalid color function".to_string())?;
    let name = name.to_ascii_lowercase();
    if !matches!(name.as_str(), "rgb" | "rgba" | "hsl" | "hsla" | "oklch") {
        return Err(format!("unsupported color function: {name}"));
    }
    let (channels, alpha, legacy) = split_components(body, name != "oklch")?;
    let alpha = alpha.map(parse_alpha).transpose()?.unwrap_or(1.0);
    match name.as_str() {
        "rgb" | "rgba" => {
            if legacy {
                let percentages = channels.map(|channel| channel.ends_with('%'));
                if percentages[0] != percentages[1] || percentages[1] != percentages[2] {
                    return Err("legacy RGB channels must use the same units".into());
                }
            }
            let mut rgb = [0.0; 3];
            for (resolved, channel) in rgb.iter_mut().zip(channels) {
                *resolved = if let Some(percent) = channel.strip_suffix('%') {
                    (parse_number(percent)? / 100.0).clamp(0.0, 1.0)
                } else {
                    (parse_number(channel)? / 255.0).clamp(0.0, 1.0)
                };
            }
            Ok(parsed(rgb, alpha))
        }
        "hsl" | "hsla" => Ok(ParsedColor {
            rgba: hsla_to_rgba(Hsla {
                h: parse_hue(channels[0])?,
                s: parse_percent(channels[1])?.clamp(0.0, 1.0),
                l: parse_percent(channels[2])?.clamp(0.0, 1.0),
                a: alpha,
            }),
            out_of_gamut: false,
        }),
        _ => {
            let lightness = parse_scaled(channels[0], 0.01)?.clamp(0.0, 1.0);
            let chroma = parse_scaled(channels[1], 0.004)?.max(0.0);
            let hue = parse_hue(channels[2])?;
            let rgb = oklch_to_srgb(lightness, chroma, hue);
            if rgb.iter().any(|channel| !channel.is_finite()) {
                return Err("OKLCH conversion exceeds the finite numeric range".into());
            }
            // Matrix roundoff at exact sRGB primaries must not produce a warning.
            let out_of_gamut = rgb
                .iter()
                .any(|channel| *channel < -1e-7 || *channel > 1.0 + 1e-7);
            let mut color = parsed(rgb.map(|channel| channel.clamp(0.0, 1.0)), alpha);
            color.out_of_gamut = out_of_gamut;
            Ok(color)
        }
    }
}

/// Formats finite `0..=1` sRGB channels; HEX rounds channels to eight bits.
/// Other formats retain eight fractional decimal places and omit opaque alpha.
pub fn format_color(rgba: Rgba, format: ColorFormat) -> String {
    if format == ColorFormat::Hex {
        let byte = |channel: f64| (channel * 255.0).round() as u8;
        let rgb = format!(
            "#{:02x}{:02x}{:02x}",
            byte(rgba.r),
            byte(rgba.g),
            byte(rgba.b)
        );
        return if rgba.a == 1.0 {
            rgb
        } else {
            format!("{rgb}{:02x}", byte(rgba.a))
        };
    }
    let alpha = if rgba.a == 1.0 {
        String::new()
    } else {
        format!(" / {}", number(rgba.a))
    };
    match format {
        ColorFormat::Rgb => format!(
            "rgb({} {} {}{alpha})",
            number(rgba.r * 255.0),
            number(rgba.g * 255.0),
            number(rgba.b * 255.0)
        ),
        ColorFormat::Hsl => {
            let hsl = rgba_to_hsla(rgba);
            format!(
                "hsl({} {}% {}%{alpha})",
                number(hsl.h),
                number(hsl.s * 100.0),
                number(hsl.l * 100.0)
            )
        }
        _ => {
            let [lightness, chroma, hue] = srgb_to_oklch(rgba);
            format!(
                "oklch({} {} {}{alpha})",
                number(lightness),
                number(chroma),
                number(hue)
            )
        }
    }
}

/// Converts finite sRGB to HSL; achromatic colors use hue zero.
pub fn rgba_to_hsla(rgba: Rgba) -> Hsla {
    let max = rgba.r.max(rgba.g).max(rgba.b);
    let min = rgba.r.min(rgba.g).min(rgba.b);
    let chroma = max - min;
    let lightness = (max + min) / 2.0;
    if chroma == 0.0 {
        return Hsla {
            h: 0.0,
            s: 0.0,
            l: lightness,
            a: rgba.a,
        };
    }
    let hue = if max == rgba.r {
        ((rgba.g - rgba.b) / chroma).rem_euclid(6.0)
    } else if max == rgba.g {
        (rgba.b - rgba.r) / chroma + 2.0
    } else {
        (rgba.r - rgba.g) / chroma + 4.0
    };
    Hsla {
        h: hue * 60.0,
        s: chroma
            / if lightness <= 0.5 {
                max + min
            } else {
                2.0 - max - min
            },
        l: lightness,
        a: rgba.a,
    }
}

/// Converts finite HSL to sRGB; hue wraps and saturation/lightness are clamped.
pub fn hsla_to_rgba(hsla: Hsla) -> Rgba {
    let hue = hsla.h.rem_euclid(360.0) / 60.0;
    let lightness = hsla.l.clamp(0.0, 1.0);
    let chroma = (1.0 - (2.0 * lightness - 1.0).abs()) * hsla.s.clamp(0.0, 1.0);
    let x = chroma * (1.0 - (hue.rem_euclid(2.0) - 1.0).abs());
    let rgb = match hue as u8 {
        0 => [chroma, x, 0.0],
        1 => [x, chroma, 0.0],
        2 => [0.0, chroma, x],
        3 => [0.0, x, chroma],
        4 => [x, 0.0, chroma],
        _ => [chroma, 0.0, x],
    };
    parsed(
        rgb.map(|channel| channel + lightness - chroma / 2.0),
        hsla.a,
    )
    .rgba
}

/// Source-over composition in gamma-encoded sRGB, returning separate alpha.
pub fn composite_over(foreground: Rgba, background: Rgba) -> Rgba {
    let background_weight = background.a * (1.0 - foreground.a);
    let alpha = foreground.a + background_weight;
    if alpha == 0.0 {
        return parsed([0.0; 3], 0.0).rgba;
    }
    let blend = |front: f64, back: f64| (front * foreground.a + back * background_weight) / alpha;
    Rgba {
        r: blend(foreground.r, background.r),
        g: blend(foreground.g, background.g),
        b: blend(foreground.b, background.b),
        a: alpha,
    }
}

/// WCAG contrast after alpha composition; an unknown final backdrop returns `None`.
/// Supply an opaque canvas when the background is translucent.
pub fn contrast_ratio(foreground: Rgba, background: Rgba, canvas: Option<Rgba>) -> Option<f64> {
    let background = if background.a == 1.0 {
        background
    } else {
        composite_over(background, canvas?)
    };
    if background.a != 1.0 {
        return None;
    }
    let foreground = composite_over(foreground, background);
    let luminance = |color: Rgba| {
        0.2126 * srgb_to_linear(color.r)
            + 0.7152 * srgb_to_linear(color.g)
            + 0.0722 * srgb_to_linear(color.b)
    };
    let front = luminance(foreground);
    let back = luminance(background);
    Some((front.max(back) + 0.05) / (front.min(back) + 0.05))
}

fn parsed([r, g, b]: [f64; 3], a: f64) -> ParsedColor {
    ParsedColor {
        rgba: Rgba { r, g, b, a },
        out_of_gamut: false,
    }
}

fn parse_hex(hex: &str) -> Result<ParsedColor, String> {
    let bytes = hex.as_bytes();
    if !matches!(bytes.len(), 3 | 4 | 6 | 8) {
        return Err("HEX requires 3, 4, 6 or 8 digits".into());
    }
    let digit = |byte: u8| -> Result<u8, String> {
        match byte {
            b'0'..=b'9' => Ok(byte - b'0'),
            b'a'..=b'f' => Ok(byte - b'a' + 10),
            b'A'..=b'F' => Ok(byte - b'A' + 10),
            _ => Err("invalid HEX digit".into()),
        }
    };
    let mut channels = [1.0; 4];
    let width = if bytes.len() <= 4 { 1 } else { 2 };
    for (channel, digits) in channels.iter_mut().zip(bytes.chunks(width)) {
        let value = if width == 1 {
            digit(digits[0])? * 17
        } else {
            digit(digits[0])? * 16 + digit(digits[1])?
        };
        *channel = f64::from(value) / 255.0;
    }
    Ok(parsed([channels[0], channels[1], channels[2]], channels[3]))
}

fn split_components(
    body: &str,
    allow_comma: bool,
) -> Result<([&str; 3], Option<&str>, bool), String> {
    if body.contains(',') {
        if !allow_comma || body.contains('/') {
            return Err("unsupported comma and slash combination".into());
        }
        let values: Vec<_> = body.splitn(5, ',').map(str::trim).collect();
        return match values.as_slice() {
            [r, g, b] if ![r, g, b].contains(&&"") => Ok(([*r, *g, *b], None, true)),
            [r, g, b, a] if ![r, g, b, a].contains(&&"") => Ok(([*r, *g, *b], Some(*a), true)),
            _ => Err("expected three color channels and optional alpha".into()),
        };
    }
    let (channels, alpha) = match body.split_once('/') {
        Some((channels, alpha)) if !alpha.contains('/') && !alpha.trim().is_empty() => {
            (channels, Some(alpha.trim()))
        }
        Some(_) => return Err("expected one alpha value after slash".into()),
        None => (body, None),
    };
    let values: Vec<_> = channels.split_ascii_whitespace().take(4).collect();
    match values.as_slice() {
        [r, g, b] => Ok(([*r, *g, *b], alpha, false)),
        _ => Err("expected three space-separated color channels".into()),
    }
}

fn parse_number(input: &str) -> Result<f64, String> {
    let bytes = input.as_bytes();
    let mut position = usize::from(matches!(bytes.first(), Some(b'+' | b'-')));
    let integer_start = position;
    while bytes.get(position).is_some_and(u8::is_ascii_digit) {
        position += 1;
    }
    let mut digits = position - integer_start;
    if bytes.get(position) == Some(&b'.') {
        position += 1;
        let fraction_start = position;
        while bytes.get(position).is_some_and(u8::is_ascii_digit) {
            position += 1;
        }
        if position == fraction_start {
            return Err(format!("invalid CSS number: {input}"));
        }
        digits += position - fraction_start;
    }
    if matches!(bytes.get(position), Some(b'e' | b'E')) {
        position += 1;
        if matches!(bytes.get(position), Some(b'+' | b'-')) {
            position += 1;
        }
        let exponent_start = position;
        while bytes.get(position).is_some_and(u8::is_ascii_digit) {
            position += 1;
        }
        if position == exponent_start {
            return Err(format!("invalid CSS number: {input}"));
        }
    }
    if digits == 0 || position != bytes.len() {
        return Err(format!("invalid CSS number: {input}"));
    }
    input
        .parse::<f64>()
        .ok()
        .filter(|value| value.is_finite())
        .ok_or_else(|| format!("color number must be finite: {input}"))
}

fn parse_scaled(input: &str, percent_scale: f64) -> Result<f64, String> {
    match input.strip_suffix('%') {
        Some(percent) => Ok(parse_number(percent)? * percent_scale),
        None => parse_number(input),
    }
}

fn parse_percent(input: &str) -> Result<f64, String> {
    let percent = input
        .strip_suffix('%')
        .ok_or_else(|| "HSL saturation and lightness require percentages".to_string())?;
    Ok(parse_number(percent)? / 100.0)
}

fn parse_alpha(input: &str) -> Result<f64, String> {
    let alpha = parse_scaled(input, 0.01)?;
    if !(0.0..=1.0).contains(&alpha) {
        return Err("alpha must be between 0 and 1".into());
    }
    Ok(alpha)
}

fn parse_hue(input: &str) -> Result<f64, String> {
    let input = input.to_ascii_lowercase();
    for (suffix, period) in [
        ("deg", 360.0),
        ("grad", 400.0),
        ("rad", std::f64::consts::TAU),
        ("turn", 1.0),
    ] {
        if let Some(number) = input.strip_suffix(suffix) {
            return Ok(parse_number(number)?.rem_euclid(period) / period * 360.0);
        }
    }
    Ok(parse_number(&input)?.rem_euclid(360.0))
}

fn number(value: f64) -> String {
    let formatted = format!("{value:.8}");
    let trimmed = formatted.trim_end_matches('0').trim_end_matches('.');
    if trimmed == "-0" {
        "0".into()
    } else {
        trimmed.into()
    }
}

fn srgb_to_linear(value: f64) -> f64 {
    if value <= 0.04045 {
        value / 12.92
    } else {
        ((value + 0.055) / 1.055).powf(2.4)
    }
}

fn linear_to_srgb(value: f64) -> f64 {
    if value.abs() <= 0.0031308 {
        value * 12.92
    } else {
        value.signum() * (1.055 * value.abs().powf(1.0 / 2.4) - 0.055)
    }
}

fn multiply(matrix: [[f64; 3]; 3], vector: [f64; 3]) -> [f64; 3] {
    matrix.map(|row| row[0] * vector[0] + row[1] * vector[1] + row[2] * vector[2])
}

// CSS Color 4, https://www.w3.org/TR/css-color-4/#color-conversion-code
fn oklch_to_srgb(lightness: f64, chroma: f64, hue: f64) -> [f64; 3] {
    let angle = hue.to_radians();
    let lab = [lightness, chroma * angle.cos(), chroma * angle.sin()];
    let lms = multiply(
        [
            [1.0, 0.3963377773761749, 0.2158037573099136],
            [1.0, -0.1055613458156586, -0.0638541728258133],
            [1.0, -0.0894841775298119, -1.2914855480194092],
        ],
        lab,
    )
    .map(|value| value.powi(3));
    let xyz = multiply(
        [
            [1.2268798758459243, -0.5578149944602171, 0.2813910456659647],
            [-0.0405757452148008, 1.112286803280317, -0.0717110580655164],
            [-0.0763729366746601, -0.4214933324022432, 1.5869240198367816],
        ],
        lms,
    );
    multiply(
        [
            [12831.0 / 3959.0, -329.0 / 214.0, -1974.0 / 3959.0],
            [
                -851781.0 / 878810.0,
                1648619.0 / 878810.0,
                36519.0 / 878810.0,
            ],
            [705.0 / 12673.0, -2585.0 / 12673.0, 705.0 / 667.0],
        ],
        xyz,
    )
    .map(linear_to_srgb)
}

fn srgb_to_oklch(rgba: Rgba) -> [f64; 3] {
    let linear = [rgba.r, rgba.g, rgba.b].map(srgb_to_linear);
    let xyz = multiply(
        [
            [506752.0 / 1228815.0, 87881.0 / 245763.0, 12673.0 / 70218.0],
            [87098.0 / 409605.0, 175762.0 / 245763.0, 12673.0 / 175545.0],
            [7918.0 / 409605.0, 87881.0 / 737289.0, 1001167.0 / 1053270.0],
        ],
        linear,
    );
    let lms = multiply(
        [
            [0.819022437996703, 0.3619062600528904, -0.1288737815209879],
            [0.0329836539323885, 0.9292868615863434, 0.0361446663506424],
            [0.0481771893596242, 0.2642395317527308, 0.6335478284694309],
        ],
        xyz,
    )
    .map(f64::cbrt);
    let [lightness, a, b] = multiply(
        [
            [0.210454268309314, 0.7936177747023054, -0.0040720430116193],
            [1.9779985324311684, -2.42859224204858, 0.450593709617411],
            [0.0259040424655478, 0.7827717124575296, -0.8086757549230774],
        ],
        lms,
    );
    let chroma = a.hypot(b);
    if rgba.r == rgba.g && rgba.g == rgba.b {
        [lightness, 0.0, 0.0]
    } else {
        [lightness, chroma, b.atan2(a).to_degrees().rem_euclid(360.0)]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_rgba(actual: Rgba, expected: [f64; 4], tolerance: f64) {
        for (actual, expected) in [actual.r, actual.g, actual.b, actual.a]
            .into_iter()
            .zip(expected)
        {
            assert!(
                (actual - expected).abs() <= tolerance,
                "{actual} != {expected}"
            );
        }
    }

    #[test]
    fn hex_supports_short_long_and_alpha_forms() {
        for (input, expected) in [
            ("#0f8", [0.0, 1.0, 136.0 / 255.0, 1.0]),
            ("#0f8a", [0.0, 1.0, 136.0 / 255.0, 170.0 / 255.0]),
            ("#123456", [18.0 / 255.0, 52.0 / 255.0, 86.0 / 255.0, 1.0]),
            (
                "#12345678",
                [18.0 / 255.0, 52.0 / 255.0, 86.0 / 255.0, 120.0 / 255.0],
            ),
        ] {
            assert_rgba(parse_color(input).unwrap().rgba, expected, 1e-12);
        }
    }

    #[test]
    fn rgb_supports_legacy_modern_percentages_and_alpha() {
        for input in [
            "rgb(255, 0, 127.5)",
            "RGB(100% 0% 50%)",
            "rgba(255,0,127.5,1)",
            "rgb(255 0 50% / 100%)",
        ] {
            assert_rgba(
                parse_color(input).unwrap().rgba,
                [1.0, 0.0, 0.5, 1.0],
                1e-12,
            );
        }
        assert_rgba(
            parse_color("rgba(255 0 0/.25)").unwrap().rgba,
            [1.0, 0.0, 0.0, 0.25],
            1e-12,
        );
    }

    #[test]
    fn hsl_supports_hue_units_wrapping_and_alpha() {
        for input in [
            "hsl(480, 100%, 50%)",
            "hsla(-240 100% 50% / 50%)",
            "hsl(0.3333333333333333turn 100% 50%/.5)",
            "hsl(133.33333333333333grad 100% 50%/.5)",
            "hsl(2.0943951023931953rad 100% 50%/.5)",
        ] {
            let expected_alpha = if input.contains(',') { 1.0 } else { 0.5 };
            assert_rgba(
                parse_color(input).unwrap().rgba,
                [0.0, 1.0, 0.0, expected_alpha],
                1e-12,
            );
        }
    }

    #[test]
    fn transparent_preserves_zero_alpha() {
        assert_rgba(parse_color(" transparent ").unwrap().rgba, [0.0; 4], 0.0);
    }

    #[test]
    fn oklch_fixed_srgb_primary_vectors_match() {
        // Independent vectors from the CSS Color 4 conversion matrices.
        for (input, expected) in [
            (
                "oklch(0.6279553639214313 0.25768330380536064 29.233880279627872)",
                [1.0, 0.0, 0.0, 1.0],
            ),
            (
                "oklch(0.8664396175234368 0.29482722454269544 142.49534504144387)",
                [0.0, 1.0, 0.0, 1.0],
            ),
            (
                "oklch(0.4520137181744237 0.3132143886344849 264.0520226163699)",
                [0.0, 0.0, 1.0, 1.0],
            ),
        ] {
            let parsed = parse_color(input).unwrap();
            assert_rgba(parsed.rgba, expected, 1e-6);
            assert!(!parsed.out_of_gamut);
        }
    }

    #[test]
    fn oklch_neutral_and_percentage_chroma_match() {
        assert_rgba(
            parse_color("oklch(50% 0 270 / 25%)").unwrap().rgba,
            [
                0.3885728590463344,
                0.3885728590463344,
                0.3885728590463344,
                0.25,
            ],
            1e-7,
        );
        let number = parse_color("oklch(.6 .1 20)").unwrap();
        let percentage = parse_color("oklch(60% 25% 20)").unwrap();
        assert_eq!(number, percentage);
    }

    #[test]
    fn oklch_clips_channels_and_reports_out_of_gamut() {
        let parsed = parse_color("oklch(.7 .4 40 / .3)").unwrap();
        assert!(parsed.out_of_gamut);
        assert_rgba(parsed.rgba, [1.0, 0.0, 0.0, 0.3], 1e-12);
    }

    #[test]
    fn unsupported_or_malformed_css_is_rejected() {
        for input in [
            "",
            "red",
            "currentColor",
            "var(--primary)",
            "color(display-p3 1 0 0)",
            "color-mix(in srgb, red, blue)",
            "rgb(from red r g b)",
            "rgb(calc(255) 0 0)",
            "#12",
            "#12345",
            "#gggggg",
            "#💜💜💜",
            "rgb(1 2)",
            "rgb(1 2 3 4)",
            "rgb(1,,2,3)",
            "rgb(1,2,3 / .5)",
            "rgb(1 2 3 / .5 / .5)",
            "rgb(1% , 2, 3)",
            "hsl(0 1 .5)",
            "oklch(.5,.1,20)",
            "rgb(1. 0 0)",
            "rgb(1 2 3))",
            "rgb (1 2 3)",
            "rgb(1 2 3);",
            "rgb(1 2 3 /)",
        ] {
            assert!(parse_color(input).is_err(), "accepted {input:?}");
        }
    }

    #[test]
    fn nonfinite_values_and_invalid_alpha_are_rejected() {
        for input in [
            "rgb(NaN 0 0)",
            "rgb(inf 0 0)",
            "rgb(1e999 0 0)",
            "rgb(0 0 0 / -0.1)",
            "rgba(0,0,0,101%)",
            "hsl(NaN 0% 0%)",
            "oklch(.5 Infinity 0)",
            "oklch(.5 1e308 20)",
            "oklch(.5 .1 0 / 1.01)",
        ] {
            assert!(parse_color(input).is_err(), "accepted {input:?}");
        }
    }

    #[test]
    fn formats_opaque_and_transparent_hex() {
        assert_eq!(
            format_color(
                Rgba {
                    r: 1.0,
                    g: 0.0,
                    b: 0.5,
                    a: 1.0
                },
                ColorFormat::Hex
            ),
            "#ff0080"
        );
        assert_eq!(
            format_color(
                Rgba {
                    r: 1.0,
                    g: 0.0,
                    b: 0.5,
                    a: 0.5
                },
                ColorFormat::Hex
            ),
            "#ff008080"
        );
    }

    #[test]
    fn all_output_formats_round_trip_the_resolved_color() {
        let color = parse_color("rgb(21 112 239 / 37%)").unwrap().rgba;
        for format in [
            ColorFormat::Hex,
            ColorFormat::Rgb,
            ColorFormat::Hsl,
            ColorFormat::Oklch,
        ] {
            let output = format_color(color, format);
            let parsed = parse_color(&output).unwrap();
            let tolerance = if format == ColorFormat::Hex {
                1.0 / 255.0
            } else {
                1e-6
            };
            assert_rgba(parsed.rgba, [color.r, color.g, color.b, color.a], tolerance);
        }
    }

    #[test]
    fn rgb_and_hsl_channels_follow_css_range_clamping() {
        assert_rgba(
            parse_color("rgb(-20 300 50%)").unwrap().rgba,
            [0.0, 1.0, 0.5, 1.0],
            0.0,
        );
        assert_rgba(
            parse_color("hsl(0 120% -20%)").unwrap().rgba,
            [0.0, 0.0, 0.0, 1.0],
            0.0,
        );
    }

    #[test]
    fn hsl_conversion_stays_finite_near_black() {
        let color = Rgba {
            r: 1e-20,
            g: 0.0,
            b: 0.0,
            a: 0.4,
        };
        assert_eq!(
            rgba_to_hsla(color),
            Hsla {
                h: 0.0,
                s: 1.0,
                l: 5e-21,
                a: 0.4
            }
        );
    }

    #[test]
    fn grayscale_hsl_has_zero_hue_and_retains_alpha() {
        let color = parse_color("rgb(127.5 127.5 127.5 / .3)").unwrap().rgba;
        assert_eq!(
            rgba_to_hsla(color),
            Hsla {
                h: 0.0,
                s: 0.0,
                l: 0.5,
                a: 0.3
            }
        );
    }

    #[test]
    fn oklch_format_retains_near_neutral_chroma() {
        let color = parse_color("rgb(100 100.001 100)").unwrap().rgba;
        let output = format_color(color, ColorFormat::Oklch);
        let parsed = parse_color(&output).unwrap().rgba;
        assert_rgba(parsed, [color.r, color.g, color.b, color.a], 1e-7);
    }

    #[test]
    fn numeric_formats_round_trip_srgb_cube_and_neutrals() {
        let values = [0.0, 1.0 / 255.0, 128.0 / 255.0, 254.0 / 255.0, 1.0];
        for r in values {
            for g in values {
                for b in values {
                    let color = Rgba { r, g, b, a: 0.37 };
                    for format in [ColorFormat::Rgb, ColorFormat::Hsl, ColorFormat::Oklch] {
                        let output = format_color(color, format);
                        assert_rgba(parse_color(&output).unwrap().rgba, [r, g, b, 0.37], 1e-6);
                    }
                }
            }
        }
    }

    #[test]
    fn source_over_returns_unpremultiplied_channels() {
        let foreground = parse_color("rgb(255 0 0 / .5)").unwrap().rgba;
        let background = parse_color("rgb(0 0 255 / .5)").unwrap().rgba;
        assert_rgba(
            composite_over(foreground, background),
            [2.0 / 3.0, 0.0, 1.0 / 3.0, 0.75],
            1e-12,
        );
    }

    #[test]
    fn source_over_transparent_colors_stays_finite() {
        let transparent = parse_color("transparent").unwrap().rgba;
        assert_rgba(composite_over(transparent, transparent), [0.0; 4], 0.0);
    }

    #[test]
    fn contrast_black_on_white_is_twenty_one() {
        let black = parse_color("#000").unwrap().rgba;
        let white = parse_color("#fff").unwrap().rgba;
        assert_eq!(contrast_ratio(black, white, None), Some(21.0));
    }

    #[test]
    fn contrast_composites_foreground_alpha() {
        let black = parse_color("rgb(0 0 0 / .5)").unwrap().rgba;
        let white = parse_color("#fff").unwrap().rgba;
        let ratio = contrast_ratio(black, white, None).unwrap();
        assert!((ratio - 3.976653024912438).abs() < 1e-12);
    }

    #[test]
    fn contrast_unknown_background_returns_none() {
        let black = parse_color("#000").unwrap().rgba;
        let background = parse_color("rgb(255 255 255 / .5)").unwrap().rgba;
        assert_eq!(contrast_ratio(black, background, None), None);
        assert_eq!(contrast_ratio(black, background, Some(background)), None);
    }

    #[test]
    fn contrast_known_canvas_composites_background_alpha() {
        let black = parse_color("#000").unwrap().rgba;
        let background = parse_color("rgb(255 255 255 / .5)").unwrap().rgba;
        let ratio = contrast_ratio(black, background, Some(black)).unwrap();
        assert!((ratio - 5.280822809644651).abs() < 1e-12);
    }
}
