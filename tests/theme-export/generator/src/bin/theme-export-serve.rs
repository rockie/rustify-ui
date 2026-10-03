#[allow(dead_code)]
#[path = "../../../../../xtask/src/serve.rs"]
mod serve;

fn main() -> Result<(), String> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    let option = |name| {
        args.windows(2)
            .find(|pair| pair[0] == name)
            .map(|pair| pair[1].clone())
            .ok_or_else(|| format!("missing {name}"))
    };
    let root = std::path::PathBuf::from(option("--root")?)
        .canonicalize()
        .map_err(|error| error.to_string())?;
    let base = serve::normalize_base(&option("--base")?);
    let port = option("--port")?
        .parse::<u16>()
        .map_err(|error| error.to_string())?;
    serve::serve(serve::ServeConfig {
        root,
        base,
        port,
        csp: serve::CspMode::Strict,
        spa: false,
        fault: std::sync::Mutex::new(None),
    })
}
