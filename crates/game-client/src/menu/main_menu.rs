use crate::{
    i18n::Locale,
    network::ConnectCmd,
    settings::Settings,
    states::AppState,
    ui::{self, StateRoot, palette},
};
use bevy::prelude::*;

// #[derive(Component)]
// struct LocalBtn;
#[derive(Component)]
struct HostBtn;
#[derive(Component)]
struct JoinBtn;
#[derive(Component)]
struct SettingsBtn;
#[derive(Component)]
struct QuitBtn;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), spawn)
            .add_systems(OnExit(AppState::MainMenu), despawn)
            .add_systems(
                Update,
                (ui::btn_visuals, on_host, on_join, on_settings, on_quit)
                    .into_configs()
                    .run_if(in_state(AppState::MainMenu)),
            );
    }
}

fn spawn(mut commands: Commands, locale: Res<Locale>) {
    commands
        .spawn((
            StateRoot,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                row_gap: Val::Px(14.0),
                ..default()
            },
            BackgroundColor(palette::BG),
        ))
        .with_children(|root| {
            root.spawn((
                Text::new("UNO"),
                TextFont {
                    font_size: 80.0,
                    ..default()
                },
                TextColor(palette::ACCENT),
            ));
            root.spawn(Node {
                height: Val::Px(28.0),
                ..default()
            });

            ui::spawn_btn(root, locale.t("menu.host"), HostBtn);
            ui::spawn_btn(root, locale.t("menu.join"), JoinBtn);
            ui::spawn_btn(root, locale.t("menu.settings"), SettingsBtn);
            ui::spawn_btn(root, locale.t("menu.quit"), QuitBtn);
        });
}

fn despawn(mut commands: Commands, q: Query<Entity, With<StateRoot>>) {
    for e in &q {
        commands.entity(e).despawn();
    }
}

fn on_host(
    q: Query<&Interaction, (Changed<Interaction>, With<HostBtn>)>,
    mut next: ResMut<NextState<AppState>>,
    mut conn: MessageWriter<ConnectCmd>,
    settings: Res<Settings>,
) {
    for i in &q {
        if *i == Interaction::Pressed {
            conn.write(ConnectCmd {
                url: settings.server_url.clone(),
            });
            next.set(AppState::Lobby);
        }
    }
}

/*
fn on_local(
    q: Query<&Interaction, (Changed<Interaction>, With<JoinBtn>)>,
    mut next: ResMut<NextState<AppState>>,
    settings: Res<Settings>,
) {
    for i in &q {
        if *i == Interaction::Pressed {
            next.set(AppState::LocalLobby);
        }
    }
}
*/

fn on_join(
    q: Query<&Interaction, (Changed<Interaction>, With<JoinBtn>)>,
    mut next: ResMut<NextState<AppState>>,
    mut conn: MessageWriter<ConnectCmd>,
    settings: Res<Settings>,
) {
    for i in &q {
        if *i == Interaction::Pressed {
            conn.write(ConnectCmd {
                url: settings.server_url.clone(),
            });
            next.set(AppState::Lobby);
        }
    }
}

fn on_settings(
    q: Query<&Interaction, (Changed<Interaction>, With<SettingsBtn>)>,
    mut visible: ResMut<super::settings::SettingsOpen>,
) {
    for i in &q {
        if *i == Interaction::Pressed {
            visible.0 = true;
        }
    }
}

fn on_quit(
    q: Query<&Interaction, (Changed<Interaction>, With<QuitBtn>)>,
    mut exit: MessageWriter<AppExit>,
) {
    for i in &q {
        if *i == Interaction::Pressed {
            exit.write(AppExit::Success);
        }
    }
}
