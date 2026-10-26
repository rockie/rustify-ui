use rustify_ui::theme::{
    export_json, resolve, AccentColor, Appearance, FontResolution, GrayColor, ResolveContext,
    ThemeDocument, ThemeMode, ThemeRadius, ThemeScaling,
};
use std::{collections::BTreeMap, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = PathBuf::from(
        std::env::args_os()
            .nth(1)
            .ok_or("expected author project directory")?,
    );
    let mut document = ThemeDocument::standard("docs-source-v2", "Docs source v2 fixture");
    document.config.accent_color = Some(AccentColor::Blue);
    document.config.gray_color = Some(GrayColor::Auto);
    document.config.appearance = Appearance::Dark;
    document.config.radius = Some(ThemeRadius::Full);
    document.config.scaling = ThemeScaling::P110;
    for (mode, primary, foreground) in [
        (ThemeMode::Light, "#123456", "#ffffff"),
        (ThemeMode::Dark, "#cceeff", "#102030"),
    ] {
        let base = match mode {
            ThemeMode::Light => &mut document.base.light,
            ThemeMode::Dark => &mut document.base.dark,
        };
        for (token, value) in [
            ("font-sans", "sans-serif"),
            ("font-serif", "serif"),
            ("font-mono", "monospace"),
        ] {
            base.0.insert(token.into(), value.into());
        }
        for (token, value) in [
            ("primary", primary),
            ("primary-foreground", foreground),
            ("font-size", "18px"),
            ("spacing", "0.5rem"),
            ("layout-gap", "12px"),
            ("radius", "8px"),
            ("shadow-color", "rgb(51 68 85 / .5)"),
            ("shadow-opacity", ".35"),
            ("shadow-offset-x", "2px"),
            ("shadow-offset-y", "3px"),
            ("shadow-blur", "8px"),
            ("shadow-spread", "1px"),
        ] {
            document
                .overrides
                .values_mut(mode)
                .0
                .insert(token.into(), value.into());
        }
    }
    let context = ResolveContext {
        font_resolution: FontResolution::Css,
        ..Default::default()
    };
    let json = export_json(&document, &context)?;
    let properties = |mode| -> Result<BTreeMap<String, String>, rustify_ui::ThemeError> {
        Ok(resolve(&document, mode, &context)?
            .properties()
            .into_iter()
            .collect())
    };
    let expected = serde_json::json!({
        "document_id": document.id, "config": document.config,
        "light": properties(ThemeMode::Light)?, "dark": properties(ThemeMode::Dark)?,
    });
    std::fs::write(root.join("theme.json"), json)?;
    std::fs::create_dir_all(root.join(".rustify-docs"))?;
    std::fs::write(
        root.join(".rustify-docs/theme-v2-expected.json"),
        serde_json::to_vec_pretty(&expected)?,
    )?;
    println!("exported full v2 and both resolved property maps");
    Ok(())
}
