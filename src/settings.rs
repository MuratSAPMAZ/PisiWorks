//! Preferences kept between runs, in the user's configuration folder.

use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::i18n::Language;
use crate::theme::APP_NAME;

#[derive(Default, Serialize, Deserialize)]
pub struct Settings {
    /// A language code such as `tr`. Empty until the user picks one, so the
    /// interface follows the desktop's language.
    #[serde(default)]
    language: Option<String>,
    /// What File > Page Setup was last set to.
    #[serde(default)]
    pub print: crate::print::PrintSetup,
    /// The words Check Spelling was told are right, though no dictionary has them.
    #[serde(default)]
    pub personal_words: Vec<String>,
}

impl Settings {
    pub fn language(&self) -> Option<Language> {
        self.language.as_deref().and_then(Language::from_code)
    }

    pub fn set_language(&mut self, language: Language) {
        self.language = Some(language.code().to_string());
    }

    /// Reads the settings. A missing or damaged file gives the defaults.
    pub fn load() -> Self {
        Self::path().map(|path| Self::load_from(&path)).unwrap_or_default()
    }

    pub fn save(&self) -> anyhow::Result<()> {
        match Self::path() {
            Some(path) => self.save_to(&path),
            None => anyhow::bail!("no configuration folder"),
        }
    }

    fn path() -> Option<PathBuf> {
        Some(dirs::config_dir()?.join(APP_NAME.to_lowercase()).join("settings.json"))
    }

    fn load_from(path: &Path) -> Self {
        std::fs::read_to_string(path).ok().and_then(|text| serde_json::from_str(&text).ok()).unwrap_or_default()
    }

    fn save_to(&self, path: &Path) -> anyhow::Result<()> {
        if let Some(folder) = path.parent() {
            std::fs::create_dir_all(folder)?;
        }
        std::fs::write(path, serde_json::to_string_pretty(self)?)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn language_survives_a_save_and_load() {
        let path = std::env::temp_dir().join("pisiworks_settings_test").join("settings.json");
        let mut settings = Settings::default();
        assert_eq!(settings.language(), None);
        settings.set_language(Language::Turkish);
        settings.save_to(&path).unwrap();
        assert_eq!(Settings::load_from(&path).language(), Some(Language::Turkish));
        std::fs::remove_dir_all(path.parent().unwrap()).unwrap();
    }

    #[test]
    fn damaged_file_gives_the_defaults() {
        let path = std::env::temp_dir().join("pisiworks_settings_damaged.json");
        std::fs::write(&path, "not json").unwrap();
        assert_eq!(Settings::load_from(&path).language(), None);
        std::fs::remove_file(path).unwrap();
    }
}
