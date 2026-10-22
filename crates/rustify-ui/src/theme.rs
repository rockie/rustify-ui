//! The theme a scope draws with: the core's one table, plus the GPU family.
//!
//! The table and its catalogue are the core's, so a DOM scope and a region read
//! the same numbers. What lives here is the part that needs a Makepad `Cx` to
//! exist and therefore cannot live where the catalogue is.

pub use rustify_dom::theme::*;

/// Build the GPU family from the resolved resource, with CJK and emoji fallbacks.
#[cfg(all(target_arch = "wasm32", feature = "gpu"))]
pub fn gpu_font_family(
    cx: &mut crate::makepad_widgets::Cx,
    font: &ResolvedFont,
) -> Option<crate::makepad_widgets::makepad_draw::shader::draw_text::FontFamily> {
    let primary = font.resource_path.as_deref()?;
    let mut paths = vec![primary];
    for fallback in FONT_GLYPH_FALLBACKS.map(|font| font.resource_path) {
        if fallback != primary {
            paths.push(fallback);
        }
    }
    crate::makepad_widgets::makepad_draw::shader::draw_text::FontFamily::from_resource_paths(
        cx, &paths,
    )
}

/// The packaged faces are the GPU build's resources; a DOM build ships none, so
/// the catalogue's paths are only checkable from the side that ships them.
#[cfg(feature = "gpu")]
#[cfg(test)]
mod tests {
    use rustify_dom::theme::{FontSlot, THEME_FONTS};

    #[test]
    fn catalogue_resources_exist_and_cover_all_slots() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../makepad/widgets/resources");
        for font in THEME_FONTS {
            let file = font.face().resource_path.rsplit('/').next().unwrap();
            assert!(root.join(file).is_file(), "missing font resource: {file}");
        }
        for slot in [FontSlot::Sans, FontSlot::Serif, FontSlot::Mono] {
            assert!(THEME_FONTS.iter().any(|font| font.slot() == slot));
        }
    }
}
