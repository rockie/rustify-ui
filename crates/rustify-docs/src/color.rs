//! `#RRGGBB` colours and the WCAG contrast between two of them.

/// Parses `#RRGGBB` into its channels.
pub fn parse_hex(color: &str) -> Option<[u8; 3]> {
    let hex = color.strip_prefix('#')?;
    if hex.len() != 6 || !hex.bytes().all(|b| b.is_ascii_hexdigit()) {
        return None;
    }
    let channel = |i: usize| u8::from_str_radix(&hex[i..i + 2], 16).ok();
    Some([channel(0)?, channel(2)?, channel(4)?])
}

/// Relative luminance, as WCAG 2.x defines it.
pub fn luminance([r, g, b]: [u8; 3]) -> f64 {
    let linear = |c: u8| {
        let c = f64::from(c) / 255.0;
        if c <= 0.04045 {
            c / 12.92
        } else {
            ((c + 0.055) / 1.055).powf(2.4)
        }
    };
    0.2126 * linear(r) + 0.7152 * linear(g) + 0.0722 * linear(b)
}

/// The contrast ratio between two `#RRGGBB` colours, from 1 to 21.
pub fn contrast(a: &str, b: &str) -> Option<f64> {
    let (a, b) = (luminance(parse_hex(a)?), luminance(parse_hex(b)?));
    let (light, dark) = if a > b { (a, b) } else { (b, a) };
    Some((light + 0.05) / (dark + 0.05))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_ratios() {
        assert!((contrast("#000000", "#ffffff").unwrap() - 21.0).abs() < 1e-9);
        assert!((contrast("#777777", "#ffffff").unwrap() - 4.478).abs() < 0.01);
        assert_eq!(contrast("#fff", "#000000"), None);
        assert_eq!(parse_hex("#12345g"), None);
    }
}
