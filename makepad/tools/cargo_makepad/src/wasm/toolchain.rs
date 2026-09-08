use makepad_toml_parser::{parse_toml, Toml};
use std::path::Path;

const TOOLCHAIN_FILE: &str = "rust-toolchain.toml";

/// The wasm build must run on the exact nightly this repository pins, so the
/// channel is read from the nearest `rust-toolchain.toml` instead of being a
/// floating `nightly` alias.
pub fn pinned_channel(start: &Path) -> Result<String, String> {
    let file = start
        .ancestors()
        .map(|dir| dir.join(TOOLCHAIN_FILE))
        .find(|candidate| candidate.is_file())
        .ok_or_else(|| {
            format!(
                "no {TOOLCHAIN_FILE} found above {}; the wasm toolchain must be pinned",
                start.display()
            )
        })?;
    let text = std::fs::read_to_string(&file)
        .map_err(|e| format!("cannot read {}: {e}", file.display()))?;
    parse_toolchain_channel(&text).map_err(|e| format!("{}: {e}", file.display()))
}

pub fn parse_toolchain_channel(text: &str) -> Result<String, String> {
    let toml = parse_toml(text).map_err(|e| format!("invalid toolchain file: {e:?}"))?;
    match toml.get("toolchain.channel") {
        Some(Toml::Str(channel, _)) if !channel.is_empty() => Ok(channel.clone()),
        _ => Err("missing [toolchain] channel".to_string()),
    }
}

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
    fn channel_comes_from_the_toolchain_table() {
        let text = "[toolchain]\nchannel = \"nightly-2026-05-20\"\ncomponents = [\"rust-src\"]\n";
        assert_eq!(
            parse_toolchain_channel(text).unwrap(),
            "nightly-2026-05-20"
        );
    }

    #[test]
    fn missing_channel_is_an_error() {
        assert!(parse_toolchain_channel("[toolchain]\ncomponents = []\n").is_err());
        assert!(parse_toolchain_channel("").is_err());
    }

    #[test]
    fn toolchain_file_is_found_walking_up_from_a_nested_dir() {
        let root = std::env::temp_dir().join(format!(
            "cargo-makepad-toolchain-{}",
            std::process::id()
        ));
        let nested = root.join("a").join("b");
        std::fs::create_dir_all(&nested).unwrap();
        std::fs::write(
            root.join("rust-toolchain.toml"),
            "[toolchain]\nchannel = \"nightly-2026-05-20\"\n",
        )
        .unwrap();
        assert_eq!(pinned_channel(&nested).unwrap(), "nightly-2026-05-20");
        std::fs::remove_dir_all(&root).unwrap();
    }

    #[test]
    fn missing_toolchain_file_is_an_error() {
        let root = std::env::temp_dir().join(format!(
            "cargo-makepad-no-toolchain-{}",
            std::process::id()
        ));
        std::fs::create_dir_all(&root).unwrap();
        assert!(pinned_channel(&root).is_err());
        std::fs::remove_dir_all(&root).unwrap();
    }

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
