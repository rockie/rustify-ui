//! An image's intrinsic size, read from its header, so a page can reserve
//! the image's box before it loads and nothing below it moves.

/// An image's size in CSS pixels, and whether it is an absolute size or only
/// a shape: an SVG with a `viewBox` and no `width`/`height` scales to its
/// container, and only its aspect ratio is known.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Size {
    pub width: u32,
    pub height: u32,
    pub absolute: bool,
}

fn be16(bytes: &[u8], at: usize) -> Option<u32> {
    Some(u32::from(u16::from_be_bytes([
        *bytes.get(at)?,
        *bytes.get(at + 1)?,
    ])))
}

fn le16(bytes: &[u8], at: usize) -> Option<u32> {
    Some(u32::from(u16::from_le_bytes([
        *bytes.get(at)?,
        *bytes.get(at + 1)?,
    ])))
}

fn le24(bytes: &[u8], at: usize) -> Option<u32> {
    Some(u32::from_le_bytes([
        *bytes.get(at)?,
        *bytes.get(at + 1)?,
        *bytes.get(at + 2)?,
        0,
    ]))
}

fn png(bytes: &[u8]) -> Option<(u32, u32)> {
    if !bytes.starts_with(b"\x89PNG\r\n\x1a\n") || bytes.get(12..16)? != b"IHDR" {
        return None;
    }
    let width = u32::from_be_bytes(bytes.get(16..20)?.try_into().ok()?);
    let height = u32::from_be_bytes(bytes.get(20..24)?.try_into().ok()?);
    Some((width, height))
}

fn gif(bytes: &[u8]) -> Option<(u32, u32)> {
    if !(bytes.starts_with(b"GIF87a") || bytes.starts_with(b"GIF89a")) {
        return None;
    }
    Some((le16(bytes, 6)?, le16(bytes, 8)?))
}

fn jpeg(bytes: &[u8]) -> Option<(u32, u32)> {
    if !bytes.starts_with(&[0xff, 0xd8]) {
        return None;
    }
    let mut at = 2;
    while at + 9 < bytes.len() {
        if bytes[at] != 0xff {
            return None;
        }
        let marker = bytes[at + 1];
        if marker == 0xff {
            at += 1;
            continue;
        }
        let length = be16(bytes, at + 2)? as usize;
        // Start of frame, any coding but the DHT/JPG/DAC markers.
        if (0xc0..=0xcf).contains(&marker) && !matches!(marker, 0xc4 | 0xc8 | 0xcc) {
            return Some((be16(bytes, at + 7)?, be16(bytes, at + 5)?));
        }
        at += 2 + length;
    }
    None
}

fn webp(bytes: &[u8]) -> Option<(u32, u32)> {
    if bytes.get(0..4)? != b"RIFF" || bytes.get(8..12)? != b"WEBP" {
        return None;
    }
    match bytes.get(12..16)? {
        b"VP8 " => Some((le16(bytes, 26)? & 0x3fff, le16(bytes, 28)? & 0x3fff)),
        b"VP8L" => {
            let bits = u32::from_le_bytes(bytes.get(21..25)?.try_into().ok()?);
            Some(((bits & 0x3fff) + 1, ((bits >> 14) & 0x3fff) + 1))
        }
        b"VP8X" => Some((le24(bytes, 24)? + 1, le24(bytes, 27)? + 1)),
        _ => None,
    }
}

/// A length in an SVG's root attribute, in pixels: a number, or a number in
/// `px`. Relative and other units are not a size.
fn svg_length(value: &str) -> Option<f64> {
    let value = value.trim();
    let number = value.strip_suffix("px").unwrap_or(value);
    number
        .parse::<f64>()
        .ok()
        .filter(|n| n.is_finite() && *n > 0.0)
}

fn svg_attribute<'a>(tag: &'a str, name: &str) -> Option<&'a str> {
    let mut rest = tag;
    while let Some(found) = rest.find(name) {
        let before = rest[..found].chars().last();
        let after = &rest[found + name.len()..];
        if before.is_some_and(char::is_whitespace) {
            let after = after.trim_start();
            if let Some(after) = after.strip_prefix('=') {
                let after = after.trim_start();
                let quote = after.chars().next()?;
                if quote == '"' || quote == '\'' {
                    let value = &after[1..];
                    return value.find(quote).map(|end| &value[..end]);
                }
            }
        }
        rest = &rest[found + name.len()..];
    }
    None
}

fn svg(bytes: &[u8]) -> Option<Size> {
    let text = String::from_utf8_lossy(&bytes[..bytes.len().min(4096)]);
    let start = text.find("<svg")?;
    let end = start + text[start..].find('>')?;
    let tag = &text[start..end];
    let width = svg_attribute(tag, "width").and_then(svg_length);
    let height = svg_attribute(tag, "height").and_then(svg_length);
    if let (Some(width), Some(height)) = (width, height) {
        return Some(Size {
            width: width.round() as u32,
            height: height.round() as u32,
            absolute: true,
        });
    }
    let view_box: Vec<f64> = svg_attribute(tag, "viewBox")?
        .split([' ', ','])
        .filter(|part| !part.is_empty())
        .filter_map(|part| part.parse().ok())
        .collect();
    let [_, _, width, height] = view_box[..] else {
        return None;
    };
    (width > 0.0 && height > 0.0).then(|| Size {
        width: width.round().max(1.0) as u32,
        height: height.round().max(1.0) as u32,
        absolute: false,
    })
}

/// The size of the image in `bytes`, by its content, not its name.
pub fn size(bytes: &[u8]) -> Option<Size> {
    let raster = png(bytes)
        .or_else(|| gif(bytes))
        .or_else(|| jpeg(bytes))
        .or_else(|| webp(bytes));
    match raster {
        Some((width, height)) if width > 0 && height > 0 => Some(Size {
            width,
            height,
            absolute: true,
        }),
        Some(_) => None,
        None => svg(bytes),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rasters_by_their_headers() {
        let share = size(crate::generation::SHARE_IMAGE).unwrap();
        assert_eq!(
            (share.width, share.height, share.absolute),
            (1200, 630, true)
        );
        let mut gif89 = b"GIF89a".to_vec();
        gif89.extend([0x40, 0x01, 0xf0, 0x00]);
        assert_eq!(size(&gif89).map(|s| (s.width, s.height)), Some((320, 240)));
        // A baseline JPEG: SOI, an APP0 segment, then SOF0 with 480 x 640.
        let jpeg = [
            0xff, 0xd8, 0xff, 0xe0, 0x00, 0x04, 0x00, 0x00, 0xff, 0xc0, 0x00, 0x11, 0x08, 0x02,
            0x80, 0x01, 0xe0, 0x03, 0x01, 0x22, 0x00,
        ];
        assert_eq!(size(&jpeg).map(|s| (s.width, s.height)), Some((480, 640)));
        assert_eq!(size(b"not an image"), None);
    }

    #[test]
    fn svg_sizes_and_shapes() {
        let fixed = size(b"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"120px\" height='40' viewBox=\"0 0 12 4\">").unwrap();
        assert_eq!((fixed.width, fixed.height, fixed.absolute), (120, 40, true));
        let shape = size(b"<?xml version=\"1.0\"?>\n<svg viewBox=\"0 0 32 16\" xmlns=\"http://www.w3.org/2000/svg\">").unwrap();
        assert_eq!((shape.width, shape.height, shape.absolute), (32, 16, false));
        assert_eq!(size(b"<svg width=\"100%\" height=\"2em\">"), None);
    }
}
