//! Picks the interface font from the fonts installed on this system.
//!
//! The same face is used on screen and for export, so text measures the same in both.

use std::sync::OnceLock;

use ab_glyph::{Font, FontVec};
use fontdb::{Database, Family, Query};

/// Tried in order. Segoe UI keeps the look on Windows, the rest ship with Linux desktops.
const PREFERRED: &[&str] =
    &["Segoe UI", "Noto Sans", "Liberation Sans", "DejaVu Sans", "Cantarell", "Ubuntu", "Arial"];

/// A font must draw these to be usable for a Turkish interface.
const REQUIRED_GLYPHS: &str = "ğĞşŞıİçÇöÖüÜ";

struct Picked {
    family: String,
    outlines: Option<FontVec>,
}

fn load(db: &Database, family: Family) -> Option<Picked> {
    let id = db.query(&Query { families: &[family], ..Query::default() })?;
    let name = db.face(id)?.families.first()?.0.clone();
    let font = db.with_face_data(id, |data, index| FontVec::try_from_vec_and_index(data.to_vec(), index).ok())??;
    let complete = REQUIRED_GLYPHS.chars().all(|c| font.glyph_id(c).0 != 0);
    complete.then_some(Picked { family: name, outlines: Some(font) })
}

fn picked() -> &'static Picked {
    static PICKED: OnceLock<Picked> = OnceLock::new();
    PICKED.get_or_init(|| {
        let mut db = Database::new();
        db.load_system_fonts();
        PREFERRED
            .iter()
            .find_map(|name| load(&db, Family::Name(name)))
            .or_else(|| load(&db, Family::SansSerif))
            // GPUI falls back to a system font of its own when the family is missing.
            .unwrap_or_else(|| Picked { family: PREFERRED[0].to_string(), outlines: None })
    })
}

/// The family name to hand to GPUI.
pub fn family() -> &'static str {
    &picked().family
}

/// The same face as outlines, for measuring and exporting text.
pub fn outlines() -> Option<&'static FontVec> {
    picked().outlines.as_ref()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn picked_font_draws_turkish_letters() {
        let Some(font) = outlines() else { return };
        assert!(REQUIRED_GLYPHS.chars().all(|c| font.glyph_id(c).0 != 0), "{} lacks Turkish letters", family());
    }
}
