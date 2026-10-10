//! GitHub's extended autolinks: a bare `www.`, `http://` or `https://` URL,
//! or an email address, in ordinary text becomes a link, as it does on
//! GitHub. pulldown-cmark implements CommonMark's `<…>` autolinks only.
//!
//! The rules follow the GFM specification's "Autolinks (extension)": a URL
//! starts at the beginning of the text, after whitespace or after one of
//! `*_~(`; its domain has at least one period and no underscore in its last
//! two labels; trailing `?!.,:*_~` is not part of it, nor a `)` without a
//! matching `(`, nor an `&name;` entity at the very end.

use std::ops::Range;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Autolink {
    Url { range: Range<usize>, href: String },
    Email { range: Range<usize>, href: String },
}

impl Autolink {
    pub fn range(&self) -> &Range<usize> {
        match self {
            Self::Url { range, .. } | Self::Email { range, .. } => range,
        }
    }

    pub fn href(&self) -> &str {
        match self {
            Self::Url { href, .. } | Self::Email { href, .. } => href,
        }
    }
}

fn boundary_before(text: &str, at: usize) -> bool {
    text[..at]
        .chars()
        .next_back()
        .is_none_or(|c| c.is_whitespace() || matches!(c, '*' | '_' | '~' | '('))
}

/// The length of a valid domain at the start of `text`, if there is one.
fn domain(text: &str) -> Option<usize> {
    let end = text
        .find(|c: char| !(c.is_alphanumeric() || c == '-' || c == '_' || c == '.'))
        .unwrap_or(text.len());
    let candidate = &text[..end];
    let labels: Vec<&str> = candidate.split('.').collect();
    // A trailing period ends the sentence, not the domain.
    let labels: Vec<&str> = match labels.last() {
        Some(&"") => labels[..labels.len() - 1].to_vec(),
        _ => labels,
    };
    if labels.len() < 2 || labels.iter().any(|label| label.is_empty()) {
        return None;
    }
    if labels[labels.len() - 2..]
        .iter()
        .any(|label| label.contains('_'))
    {
        return None;
    }
    Some(labels.iter().map(|label| label.len()).sum::<usize>() + labels.len() - 1)
}

/// Trims what GFM leaves out of the end of an autolink's path.
fn trim_path(url: &str) -> usize {
    let mut end = url.len();
    loop {
        let current = &url[..end];
        let Some(last) = current.chars().next_back() else {
            return end;
        };
        if matches!(
            last,
            '?' | '!' | '.' | ',' | ':' | '*' | '_' | '~' | '\'' | '"'
        ) {
            end -= last.len_utf8();
            continue;
        }
        if last == ')' {
            let open = current.matches('(').count();
            let close = current.matches(')').count();
            if close > open {
                end -= 1;
                continue;
            }
        }
        if last == ';' {
            if let Some(amp) = current.rfind('&') {
                let name = &current[amp + 1..current.len() - 1];
                if !name.is_empty() && name.chars().all(|c| c.is_ascii_alphanumeric()) {
                    end = amp;
                    continue;
                }
            }
        }
        return end;
    }
}

fn url_at(text: &str, at: usize) -> Option<Autolink> {
    let rest = &text[at..];
    let lower: String = rest
        .chars()
        .take(8)
        .collect::<String>()
        .to_ascii_lowercase();
    let (scheme_len, prefix) = if lower.starts_with("https://") {
        (8, "")
    } else if lower.starts_with("http://") {
        (7, "")
    } else if lower.starts_with("www.") {
        (0, "http://")
    } else {
        return None;
    };
    let host = domain(&rest[scheme_len..])?;
    let end = rest
        .find(|c: char| c.is_whitespace() || c == '<')
        .unwrap_or(rest.len());
    if end < scheme_len + host {
        return None;
    }
    let end = trim_path(&rest[..end]).max(scheme_len + host);
    let url = &rest[..end];
    Some(Autolink::Url {
        range: at..at + end,
        href: format!("{prefix}{url}"),
    })
}

/// Every autolink in `text`, in order.
pub fn find(text: &str) -> Vec<Autolink> {
    let mut links = Vec::new();
    let mut at = 0;
    while at < text.len() {
        let rest = &text[at..];
        let next = rest
            .char_indices()
            .find(|(i, c)| {
                let lower = c.to_ascii_lowercase();
                (lower == 'h' || lower == 'w' || *c == '@') && boundary_or_email(text, at + i, *c)
            })
            .map(|(i, _)| at + i);
        let Some(start) = next else {
            break;
        };
        if text[start..].starts_with('@') {
            if let Some(link) = email_around(
                text,
                start,
                links.last().map_or(0, |link: &Autolink| link.range().end),
            ) {
                at = link.range().end;
                links.push(link);
                continue;
            }
            at = start + 1;
            continue;
        }
        match url_at(text, start) {
            Some(link) => {
                at = link.range().end;
                links.push(link);
            }
            None => at = start + text[start..].chars().next().map_or(1, char::len_utf8),
        }
    }
    links
}

fn boundary_or_email(text: &str, at: usize, c: char) -> bool {
    c == '@' || boundary_before(text, at)
}

/// An email address whose `@` is at `at`, starting no earlier than `floor`.
fn email_around(text: &str, at: usize, floor: usize) -> Option<Autolink> {
    let local_start = text[floor..at]
        .rfind(|c: char| !(c.is_ascii_alphanumeric() || matches!(c, '.' | '-' | '_' | '+')))
        .map_or(floor, |i| floor + i + 1);
    if local_start == at {
        return None;
    }
    let after = &text[at + 1..];
    let mut end = after
        .find(|c: char| !(c.is_ascii_alphanumeric() || matches!(c, '.' | '-' | '_')))
        .unwrap_or(after.len());
    while end > 0 && after[..end].ends_with('.') {
        end -= 1;
    }
    let host = &after[..end];
    if !host.contains('.') || host.ends_with(['-', '_']) || host.split('.').any(str::is_empty) {
        return None;
    }
    let range = local_start..at + 1 + end;
    Some(Autolink::Email {
        href: format!("mailto:{}", &text[range.clone()]),
        range,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn hrefs(text: &str) -> Vec<String> {
        find(text)
            .iter()
            .map(|link| link.href().to_owned())
            .collect()
    }

    #[test]
    fn gfm_examples() {
        assert_eq!(hrefs("www.commonmark.org"), ["http://www.commonmark.org"]);
        assert_eq!(
            hrefs("Visit www.commonmark.org/help for more information."),
            ["http://www.commonmark.org/help"]
        );
        assert_eq!(
            hrefs("Visit www.commonmark.org."),
            ["http://www.commonmark.org"]
        );
        assert_eq!(
            hrefs("Visit www.commonmark.org/a.b."),
            ["http://www.commonmark.org/a.b"]
        );
        assert_eq!(
            hrefs("www.google.com/search?q=Markup+(business)"),
            ["http://www.google.com/search?q=Markup+(business)"]
        );
        assert_eq!(
            hrefs("www.google.com/search?q=Markup+(business)))"),
            ["http://www.google.com/search?q=Markup+(business)"]
        );
        assert_eq!(
            hrefs("(www.google.com/search?q=Markup+(business))"),
            ["http://www.google.com/search?q=Markup+(business)"]
        );
        assert_eq!(
            hrefs("www.google.com/search?q=(business))+ok"),
            ["http://www.google.com/search?q=(business))+ok"]
        );
        assert_eq!(
            hrefs("www.google.com/search?q=commonmark&hl=en"),
            ["http://www.google.com/search?q=commonmark&hl=en"]
        );
        assert_eq!(
            hrefs("www.google.com/search?q=commonmark&hl;"),
            ["http://www.google.com/search?q=commonmark"]
        );
        assert_eq!(hrefs("http://commonmark.org"), ["http://commonmark.org"]);
        assert_eq!(
            hrefs("(Visit https://encrypted.google.com/search?q=Markup+(business))"),
            ["https://encrypted.google.com/search?q=Markup+(business)"]
        );
        assert_eq!(hrefs("foo@bar.baz"), ["mailto:foo@bar.baz"]);
        assert_eq!(
            hrefs("hello@mail+xyz.example isn't valid, but hello+xyz@mail.example is."),
            ["mailto:hello+xyz@mail.example"]
        );
        assert_eq!(hrefs("a.b-c_d@a.b"), ["mailto:a.b-c_d@a.b"]);
        assert_eq!(hrefs("a.b-c_d@a.b."), ["mailto:a.b-c_d@a.b"]);
        assert!(hrefs("a.b-c_d@a.b-").is_empty());
        assert!(hrefs("a.b-c_d@a.b_").is_empty());
        assert!(hrefs("www.example_x.com").is_empty());
        assert!(hrefs("xwww.example.com").is_empty());
        assert_eq!(
            hrefs("中文 https://例子.测试/路径 后文"),
            ["https://例子.测试/路径"]
        );
    }
}
