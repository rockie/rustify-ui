//! The theme's few icons, drawn here as inline SVG: one 24-unit grid, one
//! 2-unit round stroke, coloured by the text around them. They are
//! decoration beside words that say the same thing, so they are hidden from
//! assistive technology.

fn icon(paths: &str) -> String {
    format!(
        "<svg class=\"rd-icon\" viewBox=\"0 0 24 24\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"2\" stroke-linecap=\"round\" stroke-linejoin=\"round\" aria-hidden=\"true\" focusable=\"false\">{paths}</svg>"
    )
}

/// The icon of one of GitHub's five alerts, by its class name.
pub fn alert(kind: &str) -> String {
    icon(match kind {
        // A circled i.
        "note" => "<circle cx=\"12\" cy=\"12\" r=\"9\"/><path d=\"M12 11v5M12 8h.01\"/>",
        // A bulb.
        "tip" => "<path d=\"M9 18h6M10 21h4\"/><path d=\"M12 3a6 6 0 0 0-3.5 10.9c.6.4 1 1.1 1 1.9V16h5v-.2c0-.8.4-1.5 1-1.9A6 6 0 0 0 12 3z\"/>",
        // A speech balloon with a mark.
        "important" => "<path d=\"M4 5h16v11H9l-5 4z\"/><path d=\"M12 8v3M12 13.5h.01\"/>",
        // A warning triangle.
        "warning" => "<path d=\"M12 3 2 20h20z\"/><path d=\"M12 9v5M12 17h.01\"/>",
        // A stop octagon.
        _ => "<path d=\"M8 3h8l5 5v8l-5 5H8l-5-5V8z\"/><path d=\"M12 8v5M12 16h.01\"/>",
    })
}

/// Back to where a footnote is referenced.
pub fn back() -> String {
    icon("<path d=\"M9 14 4 9l5-5\"/><path d=\"M4 9h10a6 6 0 0 1 0 12h-2\"/>")
}

/// Copying a code block.
pub fn copy() -> String {
    icon("<rect x=\"8\" y=\"8\" width=\"12\" height=\"12\" rx=\"2\"/><path d=\"M16 8V6a2 2 0 0 0-2-2H6a2 2 0 0 0-2 2v8a2 2 0 0 0 2 2h2\"/>")
}

/// A colour mode: the system's, light or dark.
pub fn mode(mode: &str) -> String {
    icon(match mode {
        // A screen.
        "system" => "<rect x=\"3\" y=\"4\" width=\"18\" height=\"12\" rx=\"2\"/><path d=\"M8 20h8M12 16v4\"/>",
        // A sun.
        "light" => "<circle cx=\"12\" cy=\"12\" r=\"4\"/><path d=\"M12 2v2M12 20v2M4.9 4.9l1.4 1.4M17.7 17.7l1.4 1.4M2 12h2M20 12h2M4.9 19.1l1.4-1.4M17.7 6.3l1.4-1.4\"/>",
        // A crescent.
        _ => "<path d=\"M20 14.5A8 8 0 1 1 9.5 4a6.5 6.5 0 0 0 10.5 10.5z\"/>",
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn icons_are_hidden_decoration_with_no_style() {
        for svg in [
            alert("note"),
            alert("tip"),
            alert("important"),
            alert("warning"),
            alert("caution"),
            back(),
            copy(),
            mode("system"),
            mode("light"),
            mode("dark"),
        ] {
            assert!(svg.starts_with("<svg class=\"rd-icon\""));
            assert!(svg.contains("aria-hidden=\"true\""));
            assert!(!svg.contains("style"));
            assert!(crate::html_check::check_document(&svg).is_empty());
        }
    }
}
