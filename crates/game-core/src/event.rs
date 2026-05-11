use crate::card::{Card, Color};

pub type PlayerId = uuid::Uuid;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone)]
pub enum GameEvent {
    // ── Lifecycle ───────────────────────────────────────────────────────────
    GameStarted {
        player_order: Vec<PlayerId>,
        first_card: Card,
    },
    GameOver {
        winner: PlayerId,
    },

    // ── Turn flow ───────────────────────────────────────────────────────────
    TurnStarted {
        player: PlayerId,
    },
    TurnSkipped {
        player: PlayerId,
    },
    DirectionReversed,

    // ── Cards ───────────────────────────────────────────────────────────────
    CardPlayed {
        player: PlayerId,
        card: Card,
        /// Set when a Wild or WildDrawFour was played.
        declared_color: Option<Color>,
    },
    CardDrawn {
        player: PlayerId,
        /// Actual number drawn (may differ if the deck ran out).
        count: u8,
    },
    DeckReshuffled,

    // ── UNO calls ───────────────────────────────────────────────────────────
    UnoCalled {
        player: PlayerId,
    },
    /// A player forgot to call UNO and was caught — they draw 2 as penalty.
    UnoPenalty {
        player: PlayerId,
    },

    // ── Wild +4 challenge ───────────────────────────────────────────────────
    FourChallengeSuccess {
        /// Player whose +4 was challenged successfully.
        offender: PlayerId,
        challenger: PlayerId,
    },
    FourChallengeFailure {
        challenger: PlayerId,
    },
}
