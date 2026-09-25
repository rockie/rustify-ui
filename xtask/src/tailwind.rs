//! The Tailwind CLI: which binary runs, and whether it is the version the
//! committed stylesheet was generated with.
//!
//! `mise.toml` pins Tailwind's standalone binary, so generating CSS needs no
//! Node. Any other version is refused rather than trusted: two releases can
//! emit different bytes for the same input, and `css --check` would then report
//! drift that no class string caused.

use std::path::{Path, PathBuf};
use std::process::Command;

/// The version `mise.toml` pins.
pub const VERSION: &str = "4.1.13";

/// Names the binary outright, for a fault test or a CLI mise did not install.
const OVERRIDE: &str = "RUSTIFY_TAILWIND";

/// The CLI to run: `RUSTIFY_TAILWIND` when it is set, otherwise the first
/// `tailwindcss` on PATH, which is the one mise installs.
pub fn cli() -> Result<PathBuf, String> {
    let (binary, label, remedy) = match std::env::var_os(OVERRIDE).filter(|v| !v.is_empty()) {
        Some(value) => {
            let binary = PathBuf::from(value);
            let label = format!("{} (from {OVERRIDE})", binary.display());
            let remedy = format!(
                "point {OVERRIDE} at tailwindcss {VERSION}, or unset it and run `mise install` \
                 for the one mise.toml pins"
            );
            (binary, label, remedy)
        }
        None => {
            let remedy = format!(
                "run `mise install` for the tailwindcss {VERSION} that mise.toml pins, \
                 or point {OVERRIDE} at one"
            );
            let binary = on_path("tailwindcss")
                .ok_or_else(|| format!("tailwindcss is not on PATH; {remedy}"))?;
            let label = binary.display().to_string();
            (binary, label, remedy)
        }
    };
    // The banner is coloured when FORCE_COLOR is set, which would split the
    // version from the name it follows.
    let help = Command::new(&binary)
        .arg("--help")
        .env("NO_COLOR", "1")
        .output()
        .map_err(|e| format!("cannot run {label}: {e}; {remedy}"))?;
    check(&String::from_utf8_lossy(&help.stdout))
        .map_err(|problem| format!("{label} {problem}; {remedy}"))?;
    Ok(binary)
}

/// Compiles `input` into `output`. A failure carries the CLI's stderr, which
/// is where Tailwind names the part of the input it could not read.
pub fn compile(input: &Path, output: &Path, minify: bool) -> Result<(), String> {
    let cli = cli()?;
    let mut command = Command::new(&cli);
    command.arg("-i").arg(input).arg("-o").arg(output);
    if minify {
        command.arg("--minify");
    }
    let out = command
        .current_dir(crate::build::repo_root())
        .output()
        .map_err(|e| format!("cannot run {}: {e}", cli.display()))?;
    if !out.status.success() {
        return Err(format!(
            "tailwindcss exited {}: {}",
            out.status,
            String::from_utf8_lossy(&out.stderr).trim()
        ));
    }
    Ok(())
}

fn on_path(name: &str) -> Option<PathBuf> {
    std::env::split_paths(&std::env::var_os("PATH")?)
        .map(|dir| dir.join(name))
        .find(|candidate| candidate.is_file())
}

fn check(help: &str) -> Result<(), String> {
    match banner_version(help) {
        Some(version) if version == VERSION => Ok(()),
        Some(version) => Err(format!("is tailwindcss v{version}, not v{VERSION}")),
        None => Err(
            "printed no `tailwindcss v<version>` banner for --help, so its version is unknown"
                .to_string(),
        ),
    }
}

/// The version in the `≈ tailwindcss v4.1.13` line `--help` opens with.
fn banner_version(help: &str) -> Option<&str> {
    help.lines().find_map(|line| {
        let (_, rest) = line.split_once("tailwindcss v")?;
        rest.split_whitespace()
            .next()
            .filter(|version| version.starts_with(|c: char| c.is_ascii_digit()))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const HELP: &str = "≈ tailwindcss v4.1.13\n\nUsage:\n  tailwindcss [--input input.css] [--output output.css] [--watch] [options…]\n";

    #[test]
    fn the_banner_names_the_version() {
        assert_eq!(banner_version(HELP), Some("4.1.13"));
        assert_eq!(
            banner_version("≈ tailwindcss v4.2.0-beta.1\n"),
            Some("4.2.0-beta.1")
        );
        // Tailwind 3 prints its banner without the leading mark.
        assert_eq!(
            banner_version("\ntailwindcss v3.4.17\n\nUsage:\n"),
            Some("3.4.17")
        );
        assert_eq!(banner_version(""), None);
        assert_eq!(banner_version("Usage: tailwindcss [options]\n"), None);
        assert_eq!(banner_version("≈ tailwindcss vnext\n"), None);
        assert_eq!(banner_version("≈ tailwindcss v\n"), None);
    }

    #[test]
    fn only_the_pinned_version_is_accepted() {
        assert_eq!(check(HELP), Ok(()));
        assert_eq!(
            check("≈ tailwindcss v4.1.12\n"),
            Err("is tailwindcss v4.1.12, not v4.1.13".to_string())
        );
        assert!(check("sh: 1: tailwindcss: not found\n").is_err());
    }

    #[test]
    fn mise_pins_the_version_this_checks_for() {
        let pinned = crate::doctor::mise_pin(
            &crate::build::repo_root(),
            "\"github:tailwindlabs/tailwindcss\"",
        );
        assert_eq!(pinned.as_deref(), Ok(VERSION));
    }
}
