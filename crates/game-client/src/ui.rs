use bevy::ecs::hierarchy::ChildSpawnerCommands;
use bevy::prelude::*;
use game_core::card::{Card, CardKind, Color as CardColor};

pub mod palette {
    use bevy::prelude::Color;
    pub const BG: Color = Color::srgb(0.07, 0.09, 0.11);
    pub const PANEL: Color = Color::srgb(0.11, 0.14, 0.18);
    pub const BTN: Color = Color::srgb(0.16, 0.20, 0.26);
    pub const BTN_HOVER: Color = Color::srgb(0.22, 0.28, 0.36);
    pub const BTN_PRESS: Color = Color::srgb(0.10, 0.13, 0.18);
    pub const ACCENT: Color = Color::srgb(0.96, 0.72, 0.08);
    pub const TEXT: Color = Color::srgb(0.93, 0.93, 0.91);
    pub const TEXT_DIM: Color = Color::srgb(0.50, 0.52, 0.54);

    pub const CARD_RED: Color = Color::srgb(0.87, 0.18, 0.18);
    pub const CARD_GREEN: Color = Color::srgb(0.18, 0.74, 0.30);
    pub const CARD_BLUE: Color = Color::srgb(0.18, 0.40, 0.90);
    pub const CARD_YELLOW: Color = Color::srgb(0.96, 0.82, 0.05);
    pub const CARD_WILD: Color = Color::srgb(0.22, 0.22, 0.26);
}

#[derive(Component)]
pub struct StateRoot;

#[derive(Component)]
pub struct UiBtn;

/// Applies hover/press visuals to buttons when they are interacted with.
pub fn btn_visuals(
    mut q: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<UiBtn>)>,
) {
    for (interaction, mut bg) in &mut q {
        bg.0 = match interaction {
            Interaction::Pressed => palette::BTN_PRESS,
            Interaction::Hovered => palette::BTN_HOVER,
            Interaction::None => palette::BTN,
        };
    }
}

/// Spawn a standard button as a child. `M` is the per-screen marker component.
// ChildSpawnerCommands is the correct Bevy 0.18 type for the with_children closure parameter.
pub fn spawn_btn<M: Bundle>(parent: &mut ChildSpawnerCommands, label: String, marker: M) {
    parent
        .spawn((
            Button,
            UiBtn,
            marker,
            Node {
                width: Val::Px(260.0),
                height: Val::Px(52.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(1.0)),
                border_radius: BorderRadius::all(Val::Px(8.0)),
                ..default()
            },
            BackgroundColor(palette::BTN),
            // BorderColor is a struct with named fields — use ::all(), not ()
            BorderColor::all(Color::srgba(1.0, 1.0, 1.0, 0.07)),
        ))
        .with_children(|b| {
            b.spawn((
                Text::new(label),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(palette::TEXT),
            ));
        });
}

pub fn card_bg(card: &Card) -> Color {
    match &card.color {
        Some(CardColor::Red) => palette::CARD_RED,
        Some(CardColor::Green) => palette::CARD_GREEN,
        Some(CardColor::Blue) => palette::CARD_BLUE,
        Some(CardColor::Yellow) => palette::CARD_YELLOW,
        None => palette::CARD_WILD,
    }
}

pub fn card_label(card: &Card) -> &'static str {
    match card.kind {
        CardKind::Number(0) => "0",
        CardKind::Number(1) => "1",
        CardKind::Number(2) => "2",
        CardKind::Number(3) => "3",
        CardKind::Number(4) => "4",
        CardKind::Number(5) => "5",
        CardKind::Number(6) => "6",
        CardKind::Number(7) => "7",
        CardKind::Number(8) => "8",
        CardKind::Number(9) => "9",
        CardKind::Number(_) => "?",
        CardKind::Skip => "⊘",
        CardKind::Reverse => "↺",
        CardKind::DrawTwo => "+2",
        CardKind::Wild => "W",
        CardKind::WildDrawFour => "+4",
    }
}

pub fn spawn_card_node<M: Bundle>(
    parent: &mut ChildSpawnerCommands,
    card: &Card,
    clickable: bool,
    marker: M,
) {
    let mut e = parent.spawn((
        marker,
        Node {
            width: Val::Px(72.0),
            height: Val::Px(104.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            border: UiRect::all(Val::Px(2.0)),
            border_radius: BorderRadius::all(Val::Px(10.0)),
            ..default()
        },
        BackgroundColor(card_bg(card)),
        BorderColor::all(Color::srgba(1.0, 1.0, 1.0, 0.25)),
    ));

    if clickable {
        e.insert((Button, UiBtn));
    }

    let label = card_label(card).to_string();
    e.with_children(|b| {
        b.spawn((
            Text::new(label),
            TextFont {
                font_size: 30.0,
                ..default()
            },
            TextColor(Color::WHITE),
        ));
    });
}
