use game_core::{state::Direction, state::GameState, Card, Color, PlayerId};
use serde::{Deserialize, Serialize};

/// What you can see of an opponent — their hand size, not their cards.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpponentView {
    pub id: PlayerId,
    pub name: String,
    pub card_count: u8,
    pub called_uno: bool,
}

/// A snapshot of the game as seen by one specific player.
/// The server rebuilds and sends this after every state-changing event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameStateView {
    pub your_id: PlayerId,
    pub your_hand: Vec<Card>,
    /// Other players in turn order, starting from the player after you
    pub opponents: Vec<OpponentView>,
    pub top_card: Card,
    pub active_color: Color,
    pub draw_pile_size: usize,
    pub current_player: PlayerId,
    pub stacked_draw: u8,
    pub direction_clockwise: bool,
}

impl GameStateView {
    /// Build the view of `game_state` from the perspective of `player_id`.
    /// Returns `None` if the player is not in the game.
    pub fn for_player(state: &GameState, player_id: PlayerId) -> Option<Self> {
        let your_idx = state.players.iter().position(|p| p.id == player_id)?;
        let top_card = state.top_card()?.clone();

        // Opponents in turn order starting from the seat after yours
        let n = state.players.len();
        let opponents = (1..n)
            .map(|offset| &state.players[(your_idx + offset) % n])
            .map(|p| OpponentView {
                id: p.id,
                name: p.name.clone(),
                card_count: p.hand.len() as u8,
                called_uno: p.called_uno,
            })
            .collect();

        Some(Self {
            your_id: player_id,
            your_hand: state.players[your_idx].hand.cards().to_vec(),
            opponents,
            top_card,
            active_color: state.active_color.clone(),
            draw_pile_size: state.draw_pile.len(),
            current_player: state.current_player_id(),
            stacked_draw: state.stacked_draw,
            direction_clockwise: state.direction == Direction::Clockwise,
        })
    }
}
