mod game;
mod i18n;
mod menu;
mod network;
mod settings;
mod states;
mod ui;

use bevy::prelude::*;
use game::GamePlugin;
use i18n::I18nPlugin;
use menu::MenuPlugin;
use network::NetworkPlugin;
use settings::Settings;
use states::{AppState, PauseState};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "UNO".into(),
                resolution: (1280_u32, 720_u32).into(),
                ..default()
            }),
            ..default()
        }))
        .init_state::<AppState>()
        .add_sub_state::<PauseState>()
        .init_resource::<Settings>()
        .add_plugins((NetworkPlugin, I18nPlugin, MenuPlugin, GamePlugin))
        .add_systems(Startup, spawn_camera)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
