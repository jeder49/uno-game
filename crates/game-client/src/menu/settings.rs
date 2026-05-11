use crate::{
    i18n::Locale,
    settings::{Language, Settings},
    states::AppState,
    ui::{self, palette},
};
use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct SettingsOpen(pub bool);

#[derive(Component)]
struct SettingsPanel;
#[derive(Component)]
struct CloseBtn;
#[derive(Component)]
struct LangBtn(pub Language);
#[derive(Component)]
struct AiToggleBtn;

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SettingsOpen>().add_systems(
            Update,
            (show_hide, ui::btn_visuals, on_lang, on_ai_toggle, on_close)
                .into_configs()
                .run_if(in_state(AppState::MainMenu)),
        );
    }
}

fn show_hide(
    open: Res<SettingsOpen>,
    mut commands: Commands,
    panel_q: Query<Entity, With<SettingsPanel>>,
    locale: Res<Locale>,
    settings: Res<Settings>,
) {
    if !open.is_changed() {
        return;
    }
    if open.0 {
        spawn_panel(&mut commands, &locale, &settings);
    } else {
        for e in &panel_q {
            commands.entity(e).despawn();
        }
    }
}

fn spawn_panel(commands: &mut Commands, locale: &Locale, settings: &Settings) {
    commands
        .spawn((
            SettingsPanel,
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
                    padding: UiRect::all(Val::Px(32.0)),
                    row_gap: Val::Px(16.0),
                    min_width: Val::Px(360.0),
                    border_radius: BorderRadius::all(Val::Px(12.0)),
                    ..default()
                },
                BackgroundColor(palette::PANEL),
            ))
            .with_children(|panel| {
                panel.spawn((
                    Text::new(locale.t("settings.title")),
                    TextFont {
                        font_size: 28.0,
                        ..default()
                    },
                    TextColor(palette::ACCENT),
                ));
                panel.spawn((
                    Text::new(locale.t("settings.language")),
                    TextFont {
                        font_size: 16.0,
                        ..default()
                    },
                    TextColor(palette::TEXT_DIM),
                ));
                panel
                    .spawn(Node {
                        flex_direction: FlexDirection::Row,
                        column_gap: Val::Px(8.0),
                        ..default()
                    })
                    .with_children(|row| {
                        for lang in Language::all() {
                            ui::spawn_btn(
                                row,
                                lang.display_name().to_string(),
                                LangBtn(lang.clone()),
                            );
                        }
                    });

                let ai_label = if settings.ai_api_enabled {
                    locale.t("settings.ai_on")
                } else {
                    locale.t("settings.ai_off")
                };
                ui::spawn_btn(panel, ai_label, AiToggleBtn);
                ui::spawn_btn(panel, locale.t("settings.close"), CloseBtn);
            });
        });
}

fn on_lang(
    q: Query<(&Interaction, &LangBtn), Changed<Interaction>>,
    mut settings: ResMut<Settings>,
) {
    for (interaction, LangBtn(lang)) in &q {
        if *interaction == Interaction::Pressed {
            settings.language = lang.clone();
        }
    }
}

fn on_ai_toggle(
    q: Query<&Interaction, (Changed<Interaction>, With<AiToggleBtn>)>,
    mut settings: ResMut<Settings>,
) {
    for i in &q {
        if *i == Interaction::Pressed {
            settings.ai_api_enabled = !settings.ai_api_enabled;
        }
    }
}

fn on_close(
    q: Query<&Interaction, (Changed<Interaction>, With<CloseBtn>)>,
    mut visible: ResMut<SettingsOpen>,
) {
    for i in &q {
        if *i == Interaction::Pressed {
            visible.0 = false;
        }
    }
}
