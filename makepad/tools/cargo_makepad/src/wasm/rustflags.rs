/// Makepad's own link flags come first; whatever the caller already had in
/// `RUSTFLAGS` (cfgs, extra link args) is kept instead of being clobbered.
pub fn compose_rustflags(base: &str, inherited: Option<&str>) -> String {
    match inherited.map(str::trim) {
        Some(extra) if !extra.is_empty() => format!("{base} {extra}"),
        _ => base.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inherited_rustflags_are_appended_after_the_wasm_flags() {
        assert_eq!(compose_rustflags("-C opt-level=z", None), "-C opt-level=z");
        assert_eq!(compose_rustflags("-C opt-level=z", Some("")), "-C opt-level=z");
        assert_eq!(
            compose_rustflags("-C opt-level=z", Some("--cfg getrandom_backend=\"wasm_js\"")),
            "-C opt-level=z --cfg getrandom_backend=\"wasm_js\""
        );
    }
}
