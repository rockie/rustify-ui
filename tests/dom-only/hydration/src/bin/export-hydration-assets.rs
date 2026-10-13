use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out = std::env::args_os()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("target/dom/hydration"));
    std::fs::create_dir_all(&out)?;
    let mut scopes = String::new();
    let mut empty_scopes = String::new();
    let mut geometry = String::new();
    for scope in hydration_harness::SCOPES {
        let (html, css) = hydration_harness::render_scope(scope);
        geometry.push_str(&css);
        scopes.push_str(&format!(
            "<section class=\"harness-scope\" data-harness-root=\"{scope}\"><h2>{scope}</h2>{html}</section>"
        ));
        empty_scopes.push_str(&format!(
            "<section class=\"harness-scope\" data-harness-root=\"{scope}\"><h2>{scope}</h2><div></div></section>"
        ));
    }
    let (rejection, rejection_css) = hydration_harness::render_scope("rejected");
    geometry.push_str(&rejection_css);
    let template = format!("<template id=\"rejection-root\">{rejection}</template>");
    let categories: Vec<_> = rustify_components::CATALOG
        .iter()
        .map(|entry| entry.category.name())
        .collect();
    let files = [
        ("index.html", page("hydrate", &scopes, &template)),
        ("csr.html", page("csr", &empty_scopes, &template)),
        ("preboot.html", page("preboot", &scopes, &template)),
        ("geometry.css", geometry),
        ("catalog.json", serde_json::to_string_pretty(&categories)?),
        ("app.js", include_str!("../../public/app.js").to_string()),
        (
            "harness.css",
            include_str!("../../public/harness.css").to_string(),
        ),
        (
            "rustify-dom-loader.js",
            rustify_ui::runtime::DOM_LOADER_JS.to_string(),
        ),
        (
            "rustify-runtime.css",
            rustify_ui::runtime::DOM_RUNTIME_CSS.to_string(),
        ),
        ("rustify.css", rustify_components::CSS.to_string()),
    ];
    for (name, contents) in files {
        let path = out.join(name);
        std::fs::write(&path, contents)?;
        println!("{}", path.display());
    }
    Ok(())
}

fn page(mode: &str, scopes: &str, template: &str) -> String {
    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <title>SDK {mode} comparison</title>
    <link rel="stylesheet" href="./rustify-runtime.css">
    <link rel="stylesheet" href="./rustify.css">
    <link rel="stylesheet" href="./harness.css">
    <link rel="stylesheet" href="./geometry.css">
    <script type="module" src="./app.js"></script>
</head>
<body data-harness-mode="{mode}">
    <main>
        <h1>SDK {mode} comparison</h1>
        <p>One deterministic DOM view, three independent scopes.</p>
        <button id="attach" type="button">Attach all scopes</button>
        <p id="status" data-testid="status" role="status">Loading the runtime.</p>
        {scopes}
    </main>
    {template}
</body>
</html>"#
    )
}
