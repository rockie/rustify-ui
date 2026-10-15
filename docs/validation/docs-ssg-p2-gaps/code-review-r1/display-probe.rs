pub fn display(tokens: &str) -> String {
    const SPACED: [&str; 10] = ["->", "=>", "=", "==", "!=", "<=", ">=", "+", "&&", "||"];
    let word = |ch: char| ch.is_alphanumeric() || matches!(ch, '_' | '"' | '\'');
    let mut out = String::new();
    let mut previous: Option<String> = None;
    for piece in pieces(tokens) {
        if let Some(previous) = &previous {
            let spaced = previous.ends_with([',', ';'])
                || SPACED.contains(&previous.as_str())
                || SPACED.contains(&piece.as_str())
                || (previous.ends_with(word) && piece.starts_with(word));
            if spaced {
                out.push(' ');
            }
        }
        out.push_str(&piece);
        previous = Some(piece);
    }
    out
}

fn pieces(tokens: &str) -> Vec<String> {
    let chars: Vec<char> = tokens.chars().collect();
    let mut pieces = Vec::new();
    let mut current = String::new();
    let mut index = 0;
    while index < chars.len() {
        let ch = chars[index];
        if ch == ' ' {
            if !current.is_empty() {
                pieces.push(std::mem::take(&mut current));
            }
            index += 1;
            continue;
        }
        let closing = match ch {
            '"' => Some('"'),
            // A character literal, not a lifetime such as `'static`.
            '\'' if chars.get(index + 1) == Some(&'\\') || chars.get(index + 2) == Some(&'\'') => {
                Some('\'')
            }
            _ => None,
        };
        current.push(ch);
        index += 1;
        if let Some(closing) = closing {
            while index < chars.len() {
                let ch = chars[index];
                current.push(ch);
                index += 1;
                if ch == '\\' {
                    if let Some(&escaped) = chars.get(index) {
                        current.push(escaped);
                        index += 1;
                    }
                } else if ch == closing {
                    break;
                }
            }
        }
    }
    if !current.is_empty() {
        pieces.push(current);
    }
    pieces
}


fn main() {
    for input in [
        r##"r#"a"   b   "c"# . to_string ()"##,
        r##"r#"a\"  b  \"c"# . to_string ()"##,
        r##""ordinary   quoted string" . to_string ()"##,
        "Vec < u32 >",
    ] { println!("input={input:?}\noutput={:?}", display(input)); }
}
