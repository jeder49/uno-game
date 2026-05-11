use crate::{
    game::{LocalGameView, LocalPlayer},
    network::SendMsg,
    states::{AppState, PauseState},
    ui::palette,
};
use bevy::prelude::*;
use game_core::PlayerAction;
use game_protocol::ClientMessage;

#[derive(Component)]
struct HudRoot;
#[derive(Component)]
struct TurnLabel;
#[derive(Component)]
struct OpponentRow;
#[derive(Component)]
struct UnoBtn;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), spawn)
            .add_systems(OnExit(AppState::InGame), despawn)
            .add_systems(
                Update,
                (
                    refresh_turn_label,
                    refresh_opponents,
                    uno_btn_visibility,
                    on_uno_btn,
                )
                    .into_configs()
                    .run_if(in_state(AppState::InGame))
                    .run_if(in_state(PauseState::Running)),
            );
    }
}

fn spawn(mut commands: Commands) {
    commands
        .spawn((
            HudRoot,
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::SpaceBetween,
                padding: UiRect::all(Val::Px(16.0)),
                ..default()
            },
        ))
        .with_children(|root| {
            root.spawn((
                TurnLabel,
                Text::new("Waiting…"),
                TextFont {
                    font_size: 22.0,
                    ..default()
                },
                TextColor(palette::TEXT),
            ));
            root.spawn((
                OpponentRow,
                Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(20.0),
                    justify_content: JustifyContent::Center,
                    ..default()
                },
            ));
            root.spawn((
                UnoBtn,
                Button,
                crate::ui::UiBtn,
                Node {
                    align_self: AlignSelf::FlexEnd,
                    padding: UiRect::axes(Val::Px(24.0), Val::Px(12.0)),
                    border_radius: BorderRadius::all(Val::Px(28.0)),
                    ..default()
                },
                BackgroundColor(palette::ACCENT),
                Visibility::Hidden,
            ))
            .with_children(|b| {
                b.spawn((
                    Text::new("UNO!"),
                    TextFont {
                        font_size: 22.0,
                        ..default()
                    },
                    TextColor(Color::BLACK),
                ));
            });
        });
}

fn despawn(mut commands: Commands, q: Query<Entity, With<HudRoot>>) {
    for e in &q {
        commands.entity(e).despawn();
    }
}

fn refresh_turn_label(
    view: Res<LocalGameView>,
    local: Option<Res<LocalPlayer>>,
    mut q: Query<&mut Text, With<TurnLabel>>,
) {
    if !view.is_changed() {
        return;
    }
    let Some(view) = view.0.as_ref() else { return };
    let your_turn = local.map_or(false, |l| view.current_player == l.id);
    let label = if your_turn {
        "Your turn!".into()
    } else {
        "Waiting for player…".to_string()
    };
    for mut t in &mut q {
        **t = label.clone().into();
    }
}

fn refresh_opponents(
    mut commands: Commands,
    view: Res<LocalGameView>,
    row_q: Query<Entity, With<OpponentRow>>,
) {
    if !view.is_changed() {
        return;
    }
    let Some(view) = view.0.as_ref() else { return };
    let Ok(row) = row_q.single() else { return };

    commands.entity(row).despawn_related::<Children>();
    commands.entity(row).with_children(|row| {
        for opp in &view.opponents {
            let label = format!(
                "{} ({}🃏{})",
                opp.name,
                opp.card_count,
                if opp.called_uno { " UNO" } else { "" }
            );
            row.spawn((
                Text::new(label),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(palette::TEXT_DIM),
            ));
        }
    });
}

fn uno_btn_visibility(view: Res<LocalGameView>, mut q: Query<&mut Visibility, With<UnoBtn>>) {
    if !view.is_changed() {
        return;
    }
    let visible = view.0.as_ref().map_or(false, |v| v.your_hand.len() == 1);
    for mut vis in &mut q {
        *vis = if visible {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
}

fn on_uno_btn(
    q: Query<&Interaction, (Changed<Interaction>, With<UnoBtn>)>,
    mut send: MessageWriter<SendMsg>,
) {
    for i in &q {
        if *i == Interaction::Pressed {
            send.write(SendMsg(ClientMessage::Action(PlayerAction::CallUno)));
        }
    }
}
