use bevy::prelude::*;

#[derive(Resource, Clone, Debug)]
pub struct Settings {
    pub language: Language,
    pub server_url: String,
    pub ai_api_enabled: bool,
    pub player_name: String,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            language: Language::EnUs,
            server_url: "ws://127.0.0.1:9001".into(),
            ai_api_enabled: false,
            player_name: "Player".into(),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Language {
    EnUs,
    DeDe,
}

impl Language {
    pub fn display_name(&self) -> &'static str {
        match self {
            Language::EnUs => "English",
            Language::DeDe => "Deutsch",
        }
    }

    pub fn all() -> &'static [Language] {
        &[Language::EnUs, Language::DeDe]
    }
}
