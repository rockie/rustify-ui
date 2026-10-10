//! Code included from the project's own source files.
//!
//! In Markdown, `<!-- rustify:include path="src/example.rs" region="setup"
//! lang="rust" -->` (GitHub shows nothing for it) becomes a code block
//! holding the lines between `rustify-region:setup` and
//! `rustify-endregion:setup` in that file, which can sit in any comment
//! syntax. The marker lines are left out; the lines between keep their bytes
//! and line endings. Nothing is compiled: the file is read as it is now.

use std::ops::Range;

/// An include directive's attributes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Directive {
    pub path: String,
    pub region: Option<String>,
    pub lang: Option<String>,
    pub title: Option<String>,
}

const OPEN: &str = "rustify:include";

/// Reads an include directive from an HTML comment, if `html` is exactly
/// one. `Err` is a malformed directive, with what is wrong.
pub fn directive(html: &str) -> Option<Result<Directive, String>> {
    let inner = html
        .trim()
        .strip_prefix("<!--")?
        .strip_suffix("-->")?
        .trim();
    let rest = inner.strip_prefix(OPEN)?;
    if !rest.starts_with(char::is_whitespace) && !rest.is_empty() {
        return None;
    }
    Some(parse_attributes(rest).and_then(|attributes| {
        let mut directive = Directive {
            path: String::new(),
            region: None,
            lang: None,
            title: None,
        };
        for (name, value) in attributes {
            match name.as_str() {
                "path" => directive.path = value,
                "region" => directive.region = Some(value),
                "lang" => directive.lang = Some(value),
                "title" => directive.title = Some(value),
                other => return Err(format!("`{other}` is not an include attribute")),
            }
        }
        if directive.path.is_empty() {
            return Err("an include needs path=\"…\"".into());
        }
        Ok(directive)
    }))
}

/// `name="value"` pairs, double-quoted.
pub fn parse_attributes(text: &str) -> Result<Vec<(String, String)>, String> {
    let mut out = Vec::new();
    let mut rest = text.trim();
    while !rest.is_empty() {
        let equals = rest
            .find('=')
            .ok_or_else(|| format!("`{rest}` is not name=\"value\""))?;
        let name = rest[..equals].trim().to_owned();
        let after = rest[equals + 1..].trim_start();
        let after = after
            .strip_prefix('"')
            .ok_or_else(|| format!("the value of `{name}` must be in double quotes"))?;
        let end = after
            .find('"')
            .ok_or_else(|| format!("the value of `{name}` has no closing quote"))?;
        out.push((name, after[..end].to_owned()));
        rest = after[end + 1..].trim_start();
    }
    Ok(out)
}

/// The lines of `region` in `source`, without the marker lines.
pub fn extract(source: &str, region: &str) -> Result<String, String> {
    let open = format!("rustify-region:{region}");
    let close = format!("rustify-endregion:{region}");
    let is_marker = |line: &str, marker: &str| {
        line.find(marker).is_some_and(|at| {
            // `rustify-region:a` must not match `rustify-region:ab`.
            line[at + marker.len()..]
                .chars()
                .next()
                .is_none_or(|c| !(c.is_alphanumeric() || c == '-' || c == '_'))
        })
    };
    let mut start: Option<usize> = None;
    let mut found: Option<Range<usize>> = None;
    let mut offset = 0;
    for line in source.split_inclusive('\n') {
        if is_marker(line, &open) {
            if start.is_some() || found.is_some() {
                return Err(format!("region `{region}` is opened more than once"));
            }
            start = Some(offset + line.len());
        } else if is_marker(line, &close) {
            match start.take() {
                Some(begin) => {
                    if found.is_some() {
                        return Err(format!("region `{region}` is closed more than once"));
                    }
                    found = Some(begin..offset);
                }
                None => return Err(format!("region `{region}` is closed before it is opened")),
            }
        }
        offset += line.len();
    }
    if start.is_some() {
        return Err(format!("region `{region}` is opened but never closed"));
    }
    found
        .map(|range| source[range].to_owned())
        .ok_or_else(|| format!("there is no region `{region}`"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn directives() {
        assert_eq!(
            directive("<!-- rustify:include path=\"src/a.rs\" region=\"x\" lang=\"rust\" -->\n"),
            Some(Ok(Directive {
                path: "src/a.rs".into(),
                region: Some("x".into()),
                lang: Some("rust".into()),
                title: None
            }))
        );
        assert_eq!(directive("<!-- an ordinary comment -->"), None);
        assert_eq!(directive("<!-- rustify:includes path=\"x\" -->"), None);
        assert!(matches!(
            directive("<!-- rustify:include path=src -->"),
            Some(Err(_))
        ));
        assert!(matches!(
            directive("<!-- rustify:include region=\"x\" -->"),
            Some(Err(_))
        ));
        assert!(matches!(
            directive("<!-- rustify:include path=\"x\" colour=\"red\" -->"),
            Some(Err(_))
        ));
    }

    #[test]
    fn regions_keep_bytes_and_drop_markers() {
        let source = "fn a() {}\n// rustify-region:setup\nlet x = 1;\r\n\n    let y = 2;\n// rustify-endregion:setup\n# rustify-region:setup2\nz\n# rustify-endregion:setup2\n";
        assert_eq!(
            extract(source, "setup").unwrap(),
            "let x = 1;\r\n\n    let y = 2;\n"
        );
        assert_eq!(extract(source, "setup2").unwrap(), "z\n");
        assert_eq!(
            extract(source, "nope").unwrap_err(),
            "there is no region `nope`"
        );
        assert!(extract("// rustify-region:a\nx\n", "a")
            .unwrap_err()
            .contains("never closed"));
        assert!(extract("// rustify-endregion:a\n", "a")
            .unwrap_err()
            .contains("before it is opened"));
        assert!(extract("// rustify-region:a\n// rustify-endregion:a\n// rustify-region:a\n// rustify-endregion:a\n", "a").is_err());
    }
}
