use rustify_ui::theme::{
    export_css, export_json, font_context, import_css, import_json, resolve, ColorFormat,
    CssProfile, FontSlot, Theme, ThemeDocument, ThemeMode, FONT_FACES, FONT_GLYPH_FALLBACKS,
    THEME_FONTS,
};
use serde_json::json;
use std::{
    collections::BTreeSet,
    error::Error,
    fs,
    path::{Path, PathBuf},
};

#[allow(dead_code)]
#[path = "../../../../examples/theme-studio/src/state.rs"]
mod state;

#[allow(dead_code)]
#[path = "../../../../examples/theme-studio/src/persistence.rs"]
mod persistence;

#[allow(dead_code)]
mod data_tools {
    pub fn current_rust_snippet() -> &'static str {
        rust_snippet()
    }
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../examples/theme-studio/src/data_tools.rs"
    ));
}

#[path = "../../../../xtask/src/bridge.rs"]
mod bridge;

const CLASSES: &str = "bg-background text-foreground text-primary font-sans font-serif font-mono p-4 rounded-sm rounded-md rounded-lg rounded-xl shadow-lg shadow-none shadow-red-500 ring-2 ring-ring";

fn repo_root() -> Result<PathBuf, Box<dyn Error>> {
    Ok(Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .canonicalize()?)
}

fn write(path: &Path, content: impl AsRef<[u8]>) -> Result<(), Box<dyn Error>> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, content)?;
    Ok(())
}

fn document(alternate: bool) -> ThemeDocument {
    let mut document = ThemeDocument::from_legacy(Theme::light(), Theme::dark());
    document.id = if alternate { "fixture-b" } else { "fixture-a" }.into();
    document.name = if alternate {
        "Export fixture B"
    } else {
        "Export fixture A"
    }
    .into();
    let sans: Vec<_> = THEME_FONTS
        .iter()
        .filter(|font| font.slot() == FontSlot::Sans)
        .collect();
    let serif: Vec<_> = THEME_FONTS
        .iter()
        .filter(|font| font.slot() == FontSlot::Serif)
        .collect();
    let mono: Vec<_> = THEME_FONTS
        .iter()
        .filter(|font| font.slot() == FontSlot::Mono)
        .collect();
    for mode in [ThemeMode::Light, ThemeMode::Dark] {
        let dark = mode == ThemeMode::Dark;
        let values = &mut document.values_mut(mode).0;
        for (token, value) in [
            (
                "primary",
                if alternate {
                    if dark {
                        "#86d2bd"
                    } else {
                        "#148a6a"
                    }
                } else if dark {
                    "rgb(255 180 190 / .8)"
                } else {
                    "oklch(.7 .3 20 / .8)"
                },
            ),
            (
                "font-size",
                if dark {
                    "20px"
                } else if alternate {
                    "19px"
                } else {
                    "16px"
                },
            ),
            (
                "radius",
                if alternate {
                    if dark {
                        "9px"
                    } else {
                        "5px"
                    }
                } else if dark {
                    "18px"
                } else {
                    ".75rem"
                },
            ),
            (
                "spacing",
                if alternate {
                    if dark {
                        "10px"
                    } else {
                        "12px"
                    }
                } else if dark {
                    ".5rem"
                } else {
                    ".375rem"
                },
            ),
            ("layout-gap", if alternate { "14px" } else { "9px" }),
            ("letter-spacing", if dark { ".06em" } else { ".02em" }),
            (
                "shadow-color",
                if alternate {
                    "rgb(20 80 70 / .7)"
                } else {
                    "rgb(30 10 20 / .6)"
                },
            ),
            ("shadow-opacity", if alternate { ".3" } else { ".4" }),
            ("shadow-blur", "8px"),
            ("shadow-spread", "1px"),
            ("shadow-offset-x", "2px"),
            ("shadow-offset-y", "4px"),
        ] {
            values.insert(token.into(), value.into());
        }
        let varied = dark != alternate;
        let sans = if varied {
            sans[0]
        } else {
            sans[sans.len() - 1]
        }
        .face();
        let mono = if varied {
            mono[mono.len() - 1]
        } else {
            mono[0]
        }
        .face();
        for (token, face, generic) in [
            ("font-sans", sans, "sans-serif"),
            ("font-serif", serif[0].face(), "serif"),
            ("font-mono", mono, "monospace"),
        ] {
            values.insert(token.into(), format!("\"{}\", {generic}", face.family));
        }
    }
    document
}

fn expectation(
    document: &ThemeDocument,
    mode: ThemeMode,
) -> Result<serde_json::Value, Box<dyn Error>> {
    let theme = resolve(document, mode, &font_context(16., false))?;
    let primary = theme.colors["primary"];
    Ok(json!({
        "document_id":document.id, "mode":mode.as_str(),
        "radii_px":theme.radii_px, "spacing_px":theme.spacing_px, "layout_gap_px":theme.layout_gap_px,
        "font_size_px":theme.font_size_px, "letter_spacing_px":theme.letter_spacing_em*theme.font_size_px,
        "primary_rgba":[primary.r,primary.g,primary.b,primary.a],
        "fonts":theme.fonts.iter().map(|font|json!({"family":font.family,"resource_path":font.resource_path})).collect::<Vec<_>>(),
        "properties":theme.properties(),
        "shadow_lg":theme.shadows["shadow-lg"].iter().map(|shadow|json!({
            "offset_x":shadow.offset_x,"offset_y":shadow.offset_y,"blur":shadow.blur,"spread":shadow.spread,
            "rgba":[shadow.color.r,shadow.color.g,shadow.color.b,shadow.color.a]
        })).collect::<Vec<_>>()
    }))
}

fn sample(document: &ThemeDocument, mode: ThemeMode, scoped: bool) -> String {
    let prefix = format!("{}-{}", document.id, mode.as_str());
    let marker = if scoped {
        format!(
            "data-rustify-scope=\"{prefix}\" data-rustify-theme-css=\"{}\" data-theme=\"{}\"",
            document.id,
            mode.as_str()
        )
    } else {
        String::new()
    };
    format!(
        r#"<section {marker} data-testid="{prefix}" class="{dark} sample bg-background text-foreground">
<h2>{prefix}</h2>
<p class="font-sans" data-testid="{prefix}-sans">English 中文 é fi 😀</p>
<p class="font-serif" data-testid="{prefix}-serif">Serif 中文</p>
<p class="font-mono" data-testid="{prefix}-mono">const theme = true;</p>
<button class="p-4 rounded-sm bg-background text-primary" data-testid="{prefix}-radius-sm">Small radius / four units</button>
<div class="p-4 rounded-md" data-testid="{prefix}-radius-md">Medium radius</div>
<div class="p-4 rounded-lg shadow-lg" data-testid="{prefix}-shadow">Large radius / authored shadow</div>
<div class="p-4 rounded-xl" data-testid="{prefix}-radius-xl">Extra large radius</div>
<button class="p-4 rounded-sm ring-2 ring-ring shadow-none" data-testid="{prefix}-ring">Ring with no shadow</button>
<div class="p-4 rounded-lg shadow-lg shadow-red-500" data-testid="{prefix}-shadow-color">Utility shadow color</div>
</section>"#,
        dark = if !scoped && mode == ThemeMode::Dark {
            "dark"
        } else {
            ""
        }
    )
}

fn page(title: &str, css: &str, body: &str, runtime: bool) -> String {
    format!(
        r#"<!doctype html>
<html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1">
<title>{title}</title><link rel="stylesheet" href="./fixture.css"><link rel="stylesheet" href="./{css}">
{font_link}</head><body><h1>{title}</h1>{body}{script}</body></html>"#,
        font_link = if runtime {
            r#"<link rel="stylesheet" href="./fonts.css"><link rel="stylesheet" href="./runtime.css">"#
        } else {
            ""
        },
        script = if runtime {
            r#"<script type="module" src="./runtime.js"></script>"#
        } else {
            ""
        }
    )
}

fn generate(site: &Path) -> Result<(), Box<dyn Error>> {
    fs::create_dir_all(site)?;
    let root = repo_root()?;
    let documents = [document(false), document(true)];
    let context = font_context(16., false);
    let inputs = site.join("inputs");
    let generated = Path::new(env!("CARGO_MANIFEST_DIR")).join("../generated");
    let json = export_json(&documents[0], &context)?;
    assert_eq!(import_json(&json, &context)?.document, documents[0]);
    write(&site.join("theme.json"), &json)?;
    write(&generated.join("theme.json"), &json)?;
    write(
        &site.join("theme-b.json"),
        export_json(&documents[1], &context)?,
    )?;
    write(
        &generated.join("theme.rs"),
        data_tools::current_rust_snippet(),
    )?;
    write(&site.join("theme.rs"), data_tools::current_rust_snippet())?;
    let mut expectations = Vec::new();
    for document in &documents {
        for mode in [ThemeMode::Light, ThemeMode::Dark] {
            expectations.push(expectation(document, mode)?);
        }
    }
    write(
        &site.join("expected.json"),
        serde_json::to_string_pretty(
            &json!({"rem_px":16,"themes":expectations,"font_catalog":FONT_FACES.iter().chain(&FONT_GLYPH_FALLBACKS).map(|face|json!({"family":face.family,"resource_path":face.resource_path})).collect::<Vec<_>>()}),
        )?,
    )?;

    let mut font_css = String::new();
    let mut font_resources = BTreeSet::new();
    for face in FONT_FACES.iter().chain(&FONT_GLYPH_FALLBACKS) {
        font_css.push_str(&format!(
            "@font-face{{font-family:{};src:url({}) format(\"truetype\");font-display:swap;}}\n",
            serde_json::to_string(face.family)?,
            serde_json::to_string(face.resource_path)?
        ));
        if font_resources.insert(face.resource_path) {
            let filename = face
                .resource_path
                .strip_prefix("makepad_widgets/resources/")
                .ok_or("unrecognized SDK font resource namespace")?;
            write(
                &site.join(face.resource_path),
                fs::read(root.join("makepad/widgets/resources").join(filename))?,
            )?;
        }
    }
    for notice in ["FONT-LICENSES.md", "LICENSE-NOTO-SERIF.txt"] {
        write(
            &site.join("makepad_widgets/resources").join(notice),
            fs::read(root.join("makepad/widgets/resources").join(notice))?,
        )?;
    }
    write(&site.join("fonts.css"), font_css)?;
    let sdk = serde_json::to_string(
        &root
            .join("crates/rustify-components/css/sdk.css")
            .to_string_lossy(),
    )?;
    let v4 = serde_json::to_string(
        &root
            .join("crates/rustify-components/css/theme-v4.css")
            .to_string_lossy(),
    )?;
    let tailwind = format!("@import \"tailwindcss/theme\";\n@import \"tailwindcss/utilities\" source(none);\n@import {sdk};\n");
    let source = format!("\n@source inline(\"{CLASSES}\");\n");
    let mut links = Vec::new();
    for (profile_name, profile) in [
        ("standard", CssProfile::TailwindV4),
        ("scoped", CssProfile::RustifyScoped),
    ] {
        for (format_name, format) in [
            ("hex", ColorFormat::Hex),
            ("rgb", ColorFormat::Rgb),
            ("hsl", ColorFormat::Hsl),
            ("oklch", ColorFormat::Oklch),
        ] {
            let stem = format!("{profile_name}-{format_name}");
            let mut exported = export_css(&documents[0], profile, &context, format)?;
            assert!(import_css(&exported, &documents[0], &context).is_ok());
            if profile == CssProfile::RustifyScoped {
                exported.push_str(&export_css(&documents[1], profile, &context, format)?);
            }
            write(&site.join(format!("{stem}.export.css")), &exported)?;
            let input = if profile == CssProfile::TailwindV4 {
                exported.replacen(
                    "@import \"tailwindcss\";",
                    "@import \"tailwindcss\" source(none);",
                    1,
                )
            } else {
                format!("{tailwind}{exported}")
            };
            write(
                &inputs.join(format!("{stem}.css")),
                format!("{input}{source}"),
            )?;
            let mut body = String::from(
                r#"<p data-testid="host-sentinel" class="font-sans p-4 rounded-sm shadow-lg bg-background text-foreground">Outside all export scopes</p><div class="samples">"#,
            );
            for document in documents
                .iter()
                .take(if profile == CssProfile::RustifyScoped {
                    2
                } else {
                    1
                })
            {
                for mode in [ThemeMode::Light, ThemeMode::Dark] {
                    body.push_str(&sample(
                        document,
                        mode,
                        profile == CssProfile::RustifyScoped,
                    ));
                }
            }
            body.push_str("</div>");
            write(
                &site.join(format!("{stem}.html")),
                page(&stem, &format!("{stem}.css"), &body, false),
            )?;
            links.push(format!("<li><a href=\"./{stem}.html\">{stem}</a> · <a href=\"./{stem}.export.css\">raw exported CSS</a></li>"));
        }
    }
    write(
        &inputs.join("runtime-theme.css"),
        format!("{tailwind}@import {v4};\n{source}"),
    )?;
    write(&site.join("fixture.css"),"html{font-size:16px}body{margin:1rem;font-family:system-ui,sans-serif}h1{font-size:1.25rem}h2{font-size:1rem}.samples{display:grid;grid-template-columns:repeat(auto-fit,minmax(280px,1fr));gap:1rem}.sample{padding:1rem;border:1px solid #888}.sample>*{margin-block:.5rem}#runtime-light,#runtime-dark{min-height:12rem}a{color:inherit}\n")?;
    write(
        &site.join("rust.html"),
        page(
            "Rust snippet / JSON consumer",
            "runtime-theme.css",
            r#"<p id="runtime-status" data-testid="runtime-status" data-status="starting" role="status">Starting independent consumer…</p><div class="samples"><section id="runtime-light" data-testid="runtime-light"></section><section id="runtime-dark" data-testid="runtime-dark"></section></div>"#,
            true,
        ),
    )?;
    links.push(
        "<li><a href=\"./rust.html\">Compiled Rust snippet and JSON / ThemeScope</a></li>".into(),
    );
    write(&site.join("index.html"),page("Theme export consumers","fixture.css",&format!("<ul>{}</ul><p><a href=\"./expected.json\">SDK-resolved expectations</a> · <a href=\"./theme.json\">JSON</a> · <a href=\"./theme.rs\">Rust snippet</a></p>",links.join("")),false))?;
    println!("Generated SDK JSON, eight CSS profile/notation pages, exact Studio Rust snippet and {} font files at {}",font_resources.len(),site.display());
    Ok(())
}

fn copy_tree(source: &Path, destination: &Path) -> Result<(), Box<dyn Error>> {
    fs::create_dir_all(destination)?;
    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let target = destination.join(entry.file_name());
        if entry.file_type()?.is_dir() {
            copy_tree(&entry.path(), &target)?;
        } else {
            fs::copy(entry.path(), target)?;
        }
    }
    Ok(())
}

fn package(site: &Path, wasm_app: &Path, base: &str) -> Result<(), Box<dyn Error>> {
    let root = repo_root()?;
    copy_tree(wasm_app, site)?;
    let wasm = fs::read(site.join("theme_export_consumer.wasm"))?;
    let source = bridge::extract_bridge(&wasm)?;
    write(
        &site.join("rustify_makepad/message_bridge.js"),
        bridge::render_bridge_module(&source.source, source.hash),
    )?;
    write(
        &site.join("rustify_makepad/embedded.js"),
        fs::read(root.join("crates/rustify-makepad/web/embedded.js"))?,
    )?;
    for name in ["loader.js", "runtime.css"] {
        write(&site.join(name), fs::read(root.join("web").join(name))?)?;
    }
    write(
        &site.join("build-manifest.json"),
        serde_json::to_string_pretty(
            &json!({"base":base,"example":"theme_export_consumer","bridge":source.hash.to_string()}),
        )?,
    )?;
    write(&site.join("runtime.js"), include_str!("runtime.js"))?;
    println!(
        "Packaged the actual wasm/bridge consumer for base {base} at {}",
        site.display()
    );
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    match args.as_slice() {
        [operation, site] if operation == "generate" => generate(Path::new(site)),
        [operation, site, wasm_app, base] if operation == "package" => {
            package(Path::new(site), Path::new(wasm_app), base)
        }
        _ => Err("use generate <site> or package <site> <cargo-makepad-app> <base>".into()),
    }
}
