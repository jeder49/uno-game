use crate::{
    game::LocalGameView,
    states::AppState,
    ui::{self, palette},
};
use bevy::prelude::*;

#[derive(Component)]
struct BoardRoot;
#[derive(Component)]
struct TopCardDisplay;
#[derive(Component)]
struct ActiveColorDisplay;
#[derive(Component)]
struct DirectionDisplay;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), spawn)
            .add_systems(OnExit(AppState::InGame), despawn)
            .add_systems(Update, refresh.run_if(in_state(AppState::InGame)));
    }
}

fn spawn(mut commands: Commands) {
    commands
        .spawn((
            BoardRoot,
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                column_gap: Val::Px(40.0),
                ..default()
            },
        ))
        .with_children(|root| {
            root.spawn((
                DirectionDisplay,
                Text::new("↻"),
                TextFont {
                    font_size: 48.0,
                    ..default()
                },
                TextColor(palette::TEXT_DIM),
            ));

            // Discard pile
            root.spawn((
                TopCardDisplay,
                Node {
                    width: Val::Px(80.0),
                    height: Val::Px(112.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    border_radius: BorderRadius::all(Val::Px(10.0)),
                    border: UiRect::all(Val::Px(3.0)),
                    ..default()
                },
                BackgroundColor(palette::PANEL),
                // BorderColor is a named-field struct; use ::all() not ()
                BorderColor::all(palette::ACCENT),
            ))
            .with_children(|c| {
                c.spawn((
                    Text::new("?"),
                    TextFont {
                        font_size: 32.0,
                        ..default()
                    },
                    TextColor(palette::TEXT),
                ));
            });

            // Active colour dot
            root.spawn((
                ActiveColorDisplay,
                Node {
                    width: Val::Px(28.0),
                    height: Val::Px(28.0),
                    border_radius: BorderRadius::all(Val::Percent(50.0)),
                    ..default()
                },
                BackgroundColor(palette::TEXT_DIM),
            ));

            // Draw pile
            root.spawn(Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                row_gap: Val::Px(4.0),
                ..default()
            })
            .with_children(|draw| {
                draw.spawn((
                    Node {
                        width: Val::Px(80.0),
                        height: Val::Px(112.0),
                        border_radius: BorderRadius::all(Val::Px(10.0)),
                        border: UiRect::all(Val::Px(2.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.10, 0.14, 0.20)),
                    BorderColor::all(palette::TEXT_DIM),
                ));
                draw.spawn((
                    Text::new("52"),
                    TextFont {
                        font_size: 14.0,
                        ..default()
                    },
                    TextColor(palette::TEXT_DIM),
                ));
            });
        });
}

fn despawn(mut commands: Commands, q: Query<Entity, With<BoardRoot>>) {
    for e in &q {
        commands.entity(e).despawn();
    }
}

fn refresh(
    view: Res<LocalGameView>,
    mut top_q: Query<&mut BackgroundColor, With<TopCardDisplay>>,
    mut color_q: Query<&mut BackgroundColor, (With<ActiveColorDisplay>, Without<TopCardDisplay>)>,
    mut dir_q: Query<&mut Text, With<DirectionDisplay>>,
) {
    let Some(view) = view.0.as_ref() else { return };
    if !view_is_changed() {
        return;
    }

    let top_bg = ui::card_bg(&view.top_card);
    for mut bg in &mut top_q {
        bg.0 = top_bg;
    }

    let dot = match &view.active_color {
        game_core::card::Color::Red => palette::CARD_RED,
        game_core::card::Color::Green => palette::CARD_GREEN,
        game_core::card::Color::Blue => palette::CARD_BLUE,
        game_core::card::Color::Yellow => palette::CARD_YELLOW,
    };
    for mut bg in &mut color_q {
        bg.0 = dot;
    }

    let arrow = if view.direction_clockwise {
        "↻"
    } else {
        "↺"
    };
    for mut t in &mut dir_q {
        **t = arrow.into();
    }
}

fn view_is_changed() -> bool {
    true
}
