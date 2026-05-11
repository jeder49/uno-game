use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Resource, Default)]
pub struct Locale(HashMap<String, String>);

impl Locale {
    // Explicit lifetime: the returned &str lives as long as `key`, not `self`
    pub fn t(&self, key: &str) -> String {
        self.0.get(key).cloned().unwrap_or_else(|| key.to_string())
    }

    fn parse(src: &str) -> HashMap<String, String> {
        src.lines()
            .filter(|l| !l.trim_start().starts_with('#') && l.contains('='))
            .filter_map(|l| {
                let mut it = l.splitn(2, '=');
                Some((it.next()?.trim().to_owned(), it.next()?.trim().to_owned()))
            })
            .collect()
    }
}

pub struct I18nPlugin;

impl Plugin for I18nPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Locale>()
            .add_systems(Startup, load)
            .add_systems(Update, reload_on_change);
    }
}

fn load(mut locale: ResMut<Locale>, settings: Res<crate::settings::Settings>) {
    locale.0 = Locale::parse(ftl_for(&settings.language));
}

fn reload_on_change(mut locale: ResMut<Locale>, settings: Res<crate::settings::Settings>) {
    if settings.is_changed() {
        locale.0 = Locale::parse(ftl_for(&settings.language));
    }
}

fn ftl_for(lang: &crate::settings::Language) -> &'static str {
    use crate::settings::Language;
    match lang {
        // src/i18n/mod.rs → ../../../.. → workspace root → assets/
        Language::EnUs => include_str!("../../../../assets/i18n/en-US.ftl"),
        Language::DeDe => include_str!("../../../../assets/i18n/de-DE.ftl"),
    }
}
