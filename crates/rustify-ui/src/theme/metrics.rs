use super::ThemeError;

#[derive(Clone, Copy, Debug)]
pub enum LengthRule {
    Positive,
    NonNegative,
    Signed,
}

/// Resolve the finite px/rem subset using the host's actual root size.
pub fn parse_length(value: &str, rem_px: f64, rule: LengthRule) -> Result<f64, ThemeError> {
    if !rem_px.is_finite() || rem_px <= 0. || rem_px > 4096. {
        return Err(ThemeError::new(
            "rem_px",
            "root size must be finite and in (0, 4096]",
        ));
    }
    let value = value.trim();
    let (raw, multiplier) = if let Some(raw) = value.strip_suffix("rem") {
        (raw, rem_px)
    } else if let Some(raw) = value.strip_suffix("px") {
        (raw, 1.)
    } else if value.parse::<f64>() == Ok(0.) {
        (value, 1.)
    } else {
        return Err(ThemeError::new(
            "length",
            "use px or rem (or unitless zero)",
        ));
    };
    let parsed = number(raw, -4096., 4096.)? * multiplier;
    if parsed.abs() > 4096.
        || match rule {
            LengthRule::Positive => parsed <= 0.,
            LengthRule::NonNegative => parsed < 0.,
            LengthRule::Signed => false,
        }
    {
        return Err(ThemeError::new(
            "length",
            "length is outside the allowed pixel range",
        ));
    }
    Ok(parsed)
}

/// Parse family names, without accepting CSS declarations or functions.
pub fn parse_font_stack(value: &str) -> Result<Vec<String>, ThemeError> {
    let invalid = || ThemeError::new("font", "use a comma separated list of family names");
    if value.len() > 4096
        || value
            .chars()
            .any(|c| c.is_control() || ";{}()\\:/<>".contains(c))
    {
        return Err(invalid());
    }
    let mut families = Vec::new();
    let mut quote = None;
    let mut start = 0;
    for (index, character) in value.char_indices() {
        match (quote, character) {
            (Some(open), close) if open == close => quote = None,
            (None, '\'' | '"') => quote = Some(character),
            (None, ',') => {
                families.push(family(&value[start..index])?);
                start = index + 1;
            }
            _ => {}
        }
    }
    if quote.is_some() {
        return Err(invalid());
    }
    families.push(family(&value[start..])?);
    Ok(families)
}

fn family(raw: &str) -> Result<String, ThemeError> {
    let raw = raw.trim();
    let quoted = raw.starts_with(['\'', '"']);
    let name = if quoted && raw.len() >= 2 && raw.as_bytes().first() == raw.as_bytes().last() {
        &raw[1..raw.len() - 1]
    } else {
        raw
    };
    if name.trim().is_empty()
        || name.contains(['\'', '"'])
        || name
            .chars()
            .any(|c| !(c.is_alphanumeric() || " -_.,".contains(c)))
        || (!quoted && name.contains(','))
    {
        return Err(ThemeError::new("font", "invalid font family name"));
    }
    Ok(name.to_owned())
}

pub(crate) fn letter_spacing(value: &str, font_size: f64, rem_px: f64) -> Result<f64, ThemeError> {
    let value = value.trim();
    if value == "normal" || value == "0" {
        return Ok(0.);
    }
    if value.ends_with("px") || value.ends_with("rem") {
        let em = parse_length(value, rem_px, LengthRule::Signed)? / font_size;
        return number(&em.to_string(), -0.5, 0.5);
    }
    let raw = value
        .strip_suffix("em")
        .ok_or_else(|| ThemeError::new("letter-spacing", "use em, px, rem or normal"))?;
    number(raw, -0.5, 0.5)
}

pub(crate) fn number(value: &str, minimum: f64, maximum: f64) -> Result<f64, ThemeError> {
    let parsed = value
        .parse::<f64>()
        .map_err(|_| ThemeError::new("number", "expected a finite number"))?;
    if !parsed.is_finite() || !(minimum..=maximum).contains(&parsed) {
        return Err(ThemeError::new(
            "number",
            format!("use a finite number in {minimum}…{maximum}"),
        ));
    }
    Ok(parsed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lengths_use_the_supplied_root_size_and_allow_zero_radius() {
        assert_eq!(
            parse_length(".5rem", 20., LengthRule::NonNegative).unwrap(),
            10.
        );
        assert_eq!(parse_length("0", 20., LengthRule::NonNegative).unwrap(), 0.);
        assert_eq!(parse_length("-2px", 20., LengthRule::Signed).unwrap(), -2.);
        assert_eq!(
            parse_length("4096px", 20., LengthRule::Positive).unwrap(),
            4096.
        );
    }

    #[test]
    fn lengths_reject_unbounded_nonfinite_and_unsupported_values() {
        for value in [
            "4097px",
            "-4097px",
            "256rem",
            "NaNpx",
            "infpx",
            "1e999px",
            "calc(1px)",
            "var(--radius)",
            "3em",
            "3",
        ] {
            assert!(
                parse_length(value, 20., LengthRule::Signed).is_err(),
                "{value}"
            );
        }
        assert!(parse_length("0", 20., LengthRule::Positive).is_err());
        assert!(parse_length("-1px", 20., LengthRule::NonNegative).is_err());
        assert!(parse_length("1px", f64::NAN, LengthRule::Positive).is_err());
    }

    #[test]
    fn font_families_preserve_quoted_commas_and_unicode() {
        assert_eq!(
            parse_font_stack("'Family, One', \"霞鹜文楷\", sans-serif").unwrap(),
            vec!["Family, One", "霞鹜文楷", "sans-serif"]
        );
    }

    #[test]
    fn font_stacks_reject_declaration_injection_and_incomplete_input() {
        for value in [
            "",
            "sans-serif,",
            "'broken",
            "sans-serif; color:red",
            "url(https://x)",
            "a}body{color:red",
            "\"x\\22; color:red\"",
            "''",
        ] {
            assert!(parse_font_stack(value).is_err(), "{value}");
        }
    }

    #[test]
    fn letter_spacing_handles_normal_and_checks_em_bounds() {
        assert_eq!(letter_spacing("normal", 16., 20.).unwrap(), 0.);
        assert_eq!(letter_spacing("-.5em", 16., 20.).unwrap(), -0.5);
        assert_eq!(letter_spacing("0.5em", 16., 20.).unwrap(), 0.5);
        for value in ["0.51em", "-0.51em", "9px", "NaNem"] {
            assert!(letter_spacing(value, 16., 20.).is_err(), "{value}");
        }
    }

    #[test]
    fn reference_px_and_rem_letter_spacing_resolves_to_em() {
        assert_eq!(letter_spacing("0.5px", 20., 24.).unwrap(), 0.025);
        assert_eq!(letter_spacing("0rem", 20., 24.).unwrap(), 0.);
        assert_eq!(letter_spacing(".25rem", 20., 24.).unwrap(), 0.3);
    }

    #[test]
    fn opacity_rejects_nonfinite_and_outside_unit_interval() {
        assert_eq!(number("0.5", 0., 1.).unwrap(), 0.5);
        for value in ["-0.1", "1.1", "NaN", "inf"] {
            assert!(number(value, 0., 1.).is_err(), "{value}");
        }
    }
}
