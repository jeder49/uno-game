use crate::{
    i18n::Locale,
    states::{AppState, PauseState},
    ui::{self, palette},
};
use bevy::prelude::*;

#[derive(Component)]
struct PauseRoot;
#[derive(Component)]
struct ResumeBtn;
#[derive(Component)]
struct QuitBtn;

pub struct PausePlugin;

impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(PauseState::Paused), spawn)
            .add_systems(OnExit(PauseState::Paused), despawn)
            .add_systems(
                Update,
                (ui::btn_visuals, on_resume, on_quit).run_if(in_state(PauseState::Paused)),
            );
    }
}

fn spawn(mut commands: Commands, locale: Res<Locale>) {
    commands
        .spawn((
            PauseRoot,
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.70)),
        ))
        .with_children(|root| {
            root.spawn((
                Node {
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    padding: UiRect::all(Val::Px(40.0)),
                    row_gap: Val::Px(16.0),
                    border_radius: BorderRadius::all(Val::Px(14.0)),
                    ..default()
                },
                BackgroundColor(palette::PANEL),
            ))
            .with_children(|panel| {
                panel.spawn((
                    Text::new(locale.t("pause.title")),
                    TextFont {
                        font_size: 36.0,
                        ..default()
                    },
                    TextColor(palette::ACCENT),
                ));
                ui::spawn_btn(panel, locale.t("pause.resume"), ResumeBtn);
                ui::spawn_btn(panel, locale.t("pause.quit"), QuitBtn);
            });
        });
}

fn despawn(mut commands: Commands, q: Query<Entity, With<PauseRoot>>) {
    for e in &q {
        commands.entity(e).despawn();
    }
}

fn on_resume(
    q: Query<&Interaction, (Changed<Interaction>, With<ResumeBtn>)>,
    mut next: ResMut<NextState<PauseState>>,
) {
    for i in &q {
        if *i == Interaction::Pressed {
            next.set(PauseState::Running);
        }
    }
}

fn on_quit(
    q: Query<&Interaction, (Changed<Interaction>, With<QuitBtn>)>,
    mut next: ResMut<NextState<AppState>>,
) {
    for i in &q {
        if *i == Interaction::Pressed {
            next.set(AppState::MainMenu);
        }
    }
}
