use crate::{
    game::LocalPlayer,
    i18n::Locale,
    network::{SendMsg, ServerMsg},
    settings::Settings,
    states::AppState,
    ui::{self, StateRoot, palette},
};
use bevy::prelude::*;
use game_protocol::{ClientMessage, JoinRequest, LobbyState, ServerMessage};

#[derive(Component)]
struct ReadyBtn;
#[derive(Component)]
struct StartBtn;
#[derive(Component)]
struct PlayerListNode;

pub struct LobbyPlugin;

impl Plugin for LobbyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Lobby), (spawn, send_join))
            .add_systems(OnExit(AppState::Lobby), despawn)
            .add_systems(
                Update,
                (ui::btn_visuals, handle_server_msgs, on_ready, on_start)
                    .into_configs()
                    .run_if(in_state(AppState::Lobby)),
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
                row_gap: Val::Px(20.0),
                ..default()
            },
            BackgroundColor(palette::BG),
        ))
        .with_children(|root| {
            root.spawn((
                Text::new(locale.t("lobby.title")),
                TextFont {
                    font_size: 40.0,
                    ..default()
                },
                TextColor(palette::ACCENT),
            ));
            root.spawn((
                PlayerListNode,
                Node {
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(8.0),
                    min_width: Val::Px(300.0),
                    ..default()
                },
            ));
            ui::spawn_btn(root, locale.t("lobby.ready"), ReadyBtn);
            ui::spawn_btn(root, locale.t("lobby.start"), StartBtn);
        });
}

fn despawn(mut commands: Commands, q: Query<Entity, With<StateRoot>>) {
    for e in &q {
        commands.entity(e).despawn();
    }
}

fn send_join(mut send: MessageWriter<SendMsg>, settings: Res<Settings>) {
    send.write(SendMsg(ClientMessage::Join(JoinRequest {
        name: settings.player_name.clone(),
        room_code: None,
        is_ai: false,
        auth_token: None,
    })));
}

fn handle_server_msgs(
    mut evs: MessageReader<ServerMsg>,
    mut commands: Commands,
    mut next: ResMut<NextState<AppState>>,
    list_q: Query<Entity, With<PlayerListNode>>,
    mut local_view: ResMut<crate::game::LocalGameView>,
) {
    for ServerMsg(msg) in evs.read() {
        match msg {
            ServerMessage::Welcome { your_id, lobby } => {
                // uuid::Uuid is Copy; pattern-matched from &ServerMessage gives Uuid directly.
                // Do NOT write *your_id — that tries to deref a value, not a reference.
                commands.insert_resource(LocalPlayer { id: *your_id });
                // lobby is &LobbyState here (matched from &ServerMessage)
                rebuild_list(&mut commands, &list_q, lobby);
            }
            ServerMessage::LobbyUpdated(lobby) => {
                rebuild_list(&mut commands, &list_q, lobby);
            }
            ServerMessage::GameSnapshot(view) => {
                local_view.0 = Some(view.clone());
                next.set(AppState::InGame);
            }
            _ => {}
        }
    }
}

fn rebuild_list(
    commands: &mut Commands,
    list_q: &Query<Entity, With<PlayerListNode>>,
    lobby: &LobbyState,
) {
    // single() replaces get_single() in Bevy 0.16+
    let Ok(entity) = list_q.single() else { return };
    commands.entity(entity).despawn_related::<Children>();
    commands.entity(entity).with_children(|list| {
        for p in &lobby.players {
            let label = format!(
                "{} {}{}",
                if p.is_host { "♛" } else { "●" },
                p.name,
                if p.is_ready { " ✓" } else { "" },
            );
            list.spawn((
                Text::new(label),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(palette::TEXT),
            ));
        }
    });
}

fn on_ready(
    q: Query<&Interaction, (Changed<Interaction>, With<ReadyBtn>)>,
    mut send: MessageWriter<SendMsg>,
) {
    for i in &q {
        if *i == Interaction::Pressed {
            send.write(SendMsg(ClientMessage::SetReady(true)));
        }
    }
}

fn on_start(
    q: Query<&Interaction, (Changed<Interaction>, With<StartBtn>)>,
    mut send: MessageWriter<SendMsg>,
) {
    for i in &q {
        if *i == Interaction::Pressed {
            send.write(SendMsg(ClientMessage::StartGame));
        }
    }
}
