use crate::{
    game::{HandState, LocalGameView, LocalPlayer},
    network::SendMsg,
    states::{AppState, PauseState},
    ui,
};
use bevy::prelude::*;
use game_core::{Card, PlayerAction, card::Color as CardColor};
use game_protocol::ClientMessage;

#[derive(Component)]
pub struct HandRoot;
#[derive(Component)]
pub struct CardBtn(pub Card);
#[derive(Component)]
pub struct ColorPickerRoot;
#[derive(Component)]
pub struct ColorPickBtn(pub CardColor);

pub struct HandPlugin;

impl Plugin for HandPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), spawn_hand_root)
            .add_systems(OnExit(AppState::InGame), despawn_all)
            .add_systems(
                Update,
                (ui::btn_visuals, rebuild_hand, on_card_click, on_color_pick)
                    .into_configs()
                    .run_if(in_state(AppState::InGame))
                    .run_if(in_state(PauseState::Running)),
            );
    }
}

fn spawn_hand_root(mut commands: Commands) {
    commands.spawn((
        HandRoot,
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(16.0),
            width: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::FlexEnd,
            column_gap: Val::Px(6.0),
            ..default()
        },
    ));
}

fn despawn_all(
    mut commands: Commands,
    roots: Query<Entity, Or<(With<HandRoot>, With<ColorPickerRoot>)>>,
) {
    for e in &roots {
        commands.entity(e).despawn();
    }
}

fn rebuild_hand(
    mut commands: Commands,
    view: Res<LocalGameView>,
    local: Option<Res<LocalPlayer>>,
    root_q: Query<Entity, With<HandRoot>>,
) {
    if !view.is_changed() {
        return;
    }
    let (Some(view), Some(local)) = (view.0.as_ref(), local) else {
        return;
    };
    // single() replaces get_single() in Bevy 0.16+
    let Ok(root) = root_q.single() else { return };

    commands.entity(root).despawn_related::<Children>();

    let my_turn = view.current_player == local.id;
    commands.entity(root).with_children(|hand| {
        for card in &view.your_hand {
            ui::spawn_card_node(hand, card, my_turn, CardBtn(card.clone()));
        }
    });
}

fn on_card_click(
    q: Query<(&Interaction, &CardBtn), Changed<Interaction>>,
    view: Res<LocalGameView>,
    local: Option<Res<LocalPlayer>>,
    mut hand_state: ResMut<HandState>,
    mut send: MessageWriter<SendMsg>,
    mut commands: Commands,
    picker_q: Query<Entity, With<ColorPickerRoot>>,
) {
    let (Some(view), Some(local)) = (view.0.as_ref(), local) else {
        return;
    };
    if view.current_player != local.id {
        return;
    }

    for (interaction, CardBtn(card)) in &q {
        if *interaction != Interaction::Pressed {
            continue;
        }

        if card.is_wild() {
            hand_state.pending_wild = Some(card.clone());
            for e in &picker_q {
                commands.entity(e).despawn();
            }
            spawn_color_picker(&mut commands);
        } else {
            send.write(SendMsg(ClientMessage::Action(PlayerAction::PlayCard {
                card: card.clone(),
                declared_color: None,
            })));
        }
    }
}

fn spawn_color_picker(commands: &mut Commands) {
    let colors = [
        (CardColor::Red, ui::palette::CARD_RED, "Red"),
        (CardColor::Green, ui::palette::CARD_GREEN, "Green"),
        (CardColor::Blue, ui::palette::CARD_BLUE, "Blue"),
        (CardColor::Yellow, ui::palette::CARD_YELLOW, "Yellow"),
    ];
    commands
        .spawn((
            ColorPickerRoot,
            Node {
                position_type: PositionType::Absolute,
                bottom: Val::Px(140.0),
                width: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                column_gap: Val::Px(12.0),
                ..default()
            },
        ))
        .with_children(|row| {
            for (color, bg, label) in colors {
                row.spawn((
                    Button,
                    ui::UiBtn,
                    ColorPickBtn(color),
                    Node {
                        width: Val::Px(80.0),
                        height: Val::Px(80.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        border_radius: BorderRadius::all(Val::Percent(50.0)),
                        ..default()
                    },
                    BackgroundColor(bg),
                ))
                .with_children(|b| {
                    b.spawn((
                        Text::new(label),
                        TextFont {
                            font_size: 14.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });
            }
        });
}

fn on_color_pick(
    q: Query<(&Interaction, &ColorPickBtn), Changed<Interaction>>,
    mut hand_state: ResMut<HandState>,
    mut send: MessageWriter<SendMsg>,
    mut commands: Commands,
    picker_q: Query<Entity, With<ColorPickerRoot>>,
) {
    for (interaction, ColorPickBtn(color)) in &q {
        if *interaction != Interaction::Pressed {
            continue;
        }
        if let Some(card) = hand_state.pending_wild.take() {
            send.write(SendMsg(ClientMessage::Action(PlayerAction::PlayCard {
                card,
                declared_color: Some(color.clone()),
            })));
            for e in &picker_q {
                commands.entity(e).despawn();
            }
        }
    }
}
