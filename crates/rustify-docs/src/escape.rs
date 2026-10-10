//! Escaping for text and attribute values in generated HTML.

/// Appends `text` with the five characters that can end a text run or an
/// attribute value replaced by references. One function serves both, so a
/// value never has to know where it lands.
pub fn escape_html_into(out: &mut String, text: &str) {
    let mut start = 0;
    for (index, byte) in text.bytes().enumerate() {
        let replacement = match byte {
            b'&' => "&amp;",
            b'<' => "&lt;",
            b'>' => "&gt;",
            b'"' => "&quot;",
            b'\'' => "&#39;",
            _ => continue,
        };
        out.push_str(&text[start..index]);
        out.push_str(replacement);
        start = index + 1;
    }
    out.push_str(&text[start..]);
}

pub fn escape_html(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    escape_html_into(&mut out, text);
    out
}

/// Appends a URL for an `href` or `src`: percent-encodes the bytes a URL
/// may not carry raw (controls, space, non-ASCII and `"<>\\^`{|}`), keeps
/// existing `%xx` escapes, then escapes it as an attribute value.
pub fn escape_url_into(out: &mut String, url: &str) {
    let mut encoded = String::with_capacity(url.len());
    for byte in url.bytes() {
        let raw = byte.is_ascii_graphic()
            && !matches!(
                byte,
                b'"' | b'<' | b'>' | b'\\' | b'^' | b'`' | b'{' | b'|' | b'}'
            );
        if raw {
            encoded.push(byte as char);
        } else {
            encoded.push_str(&format!("%{byte:02X}"));
        }
    }
    escape_html_into(out, &encoded);
}

/// Percent-encodes a URL path built from file names: every byte but the
/// unreserved ones, `/` and the sub-delimiters a path segment may carry raw.
/// Unlike [`escape_url_into`] it encodes `%`, `#` and `?` too, because here
/// they are characters of a name, not URL syntax.
pub fn encode_path(path: &str) -> String {
    let mut out = String::with_capacity(path.len());
    for byte in path.bytes() {
        let raw = byte.is_ascii_alphanumeric()
            || matches!(
                byte,
                b'-' | b'.'
                    | b'_'
                    | b'~'
                    | b'/'
                    | b'!'
                    | b'$'
                    | b'&'
                    | b'\''
                    | b'('
                    | b')'
                    | b'*'
                    | b'+'
                    | b','
                    | b';'
                    | b'='
                    | b':'
                    | b'@'
            );
        if raw {
            out.push(byte as char);
        } else {
            out.push_str(&format!("%{byte:02X}"));
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_and_attributes() {
        assert_eq!(escape_html(r#"a<b>&"c'"#), "a&lt;b&gt;&amp;&quot;c&#39;");
        assert_eq!(escape_html("中文 plain"), "中文 plain");
    }

    #[test]
    fn urls_are_encoded_then_escaped() {
        let mut out = String::new();
        escape_url_into(&mut out, "/a b/?q=1&r=\"2\"#x");
        assert_eq!(out, "/a%20b/?q=1&amp;r=%222%22#x");
        out.clear();
        escape_url_into(&mut out, "/中文/%20ok/it's");
        assert_eq!(out, "/%E4%B8%AD%E6%96%87/%20ok/it&#39;s");
    }

    #[test]
    fn paths_from_file_names_encode_url_syntax() {
        assert_eq!(encode_path("/guide/setup/"), "/guide/setup/");
        assert_eq!(encode_path("/a b/50%#1?/"), "/a%20b/50%25%231%3F/");
        assert_eq!(encode_path("/指南/"), "/%E6%8C%87%E5%8D%97/");
    }
}
