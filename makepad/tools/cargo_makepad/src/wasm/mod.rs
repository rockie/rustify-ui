mod bindgen_glue;
mod compile;
mod rustflags;

pub use compile::WasmConfig;

fn parse_wasm_option(config: &mut WasmConfig, v: &str) -> bool {
    match v {
        "--strip-custom-sections" => {
            config.strip = true;
            true
        }
        "--strip" => {
            config.strip = true;
            config.optimize_size = true;
            true
        }
        _ => false,
    }
}

fn strip_wasm_options(config: &mut WasmConfig, args: &[String]) -> Vec<String> {
    args.iter()
        .filter(|v| !parse_wasm_option(config, v))
        .cloned()
        .collect()
}

pub fn handle_wasm(mut args: &[String]) -> Result<(), String> {
    let mut config = WasmConfig::default();

    for i in 0..args.len() {
        if !parse_wasm_option(&mut config, &args[i]) {
            args = &args[i..];
            break;
        }
    }

    match args.first().map(String::as_str) {
        Some("build") => {
            let build_args = strip_wasm_options(&mut config, &args[1..]);
            compile::build(config, &build_args)?;
            Ok(())
        }
        Some(other) => Err(format!("{other} is not a valid command or option")),
        None => Err("missing command; expected `build`".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn args(values: &[&str]) -> Vec<String> {
        values.iter().map(|value| value.to_string()).collect()
    }

    #[test]
    fn strip_options_are_removed_from_the_cargo_arguments() {
        let mut config = WasmConfig::default();
        let rest = strip_wasm_options(&mut config, &args(&["--strip", "-p", "app", "--release"]));
        assert_eq!(rest, vec!["-p", "app", "--release"]);
        assert!(config.strip);
        assert!(config.optimize_size);
    }

    #[test]
    fn custom_section_stripping_alone_does_not_optimize() {
        let mut config = WasmConfig::default();
        strip_wasm_options(&mut config, &args(&["--strip-custom-sections", "-p", "app"]));
        assert!(config.strip);
        assert!(!config.optimize_size);
    }

    #[test]
    fn plain_builds_keep_every_section() {
        let mut config = WasmConfig::default();
        let rest = strip_wasm_options(&mut config, &args(&["-p", "app", "--profile=small"]));
        assert_eq!(rest, vec!["-p", "app", "--profile=small"]);
        assert!(!config.strip);
    }
}
