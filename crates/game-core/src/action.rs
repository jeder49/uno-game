use crate::card::{Card, Color};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlayerAction {
    /// Play a card. Wild cards must include a `declared_color`.
    PlayCard {
        card: Card,
        declared_color: Option<Color>,
    },
    /// Draw one card from the pile (or accept a stacked penalty).
    DrawCard,
    /// Announce "UNO!" when the hand drops to one card.
    CallUno,
    /// Challenge the previous player's WildDrawFour as illegal.
    ChallengeFour,
}
