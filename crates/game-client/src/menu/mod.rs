mod lobby;
mod main_menu;
mod settings;

use bevy::prelude::*;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            main_menu::MainMenuPlugin,
            settings::SettingsPlugin,
            lobby::LobbyPlugin,
        ));
    }
}
