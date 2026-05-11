use crate::event::PlayerId;

#[derive(Debug, Clone, thiserror::Error)]
pub enum GameError {
    #[error("not your turn")]
    NotYourTurn,

    #[error("that card is not in your hand")]
    CardNotInHand,

    #[error("that card cannot be played on the current top card")]
    InvalidCardPlay,

    #[error("wild card requires a declared color")]
    MissingDeclaredColor,

    #[error("game is not in progress")]
    GameNotInProgress,

    #[error("game has already started")]
    AlreadyStarted,

    #[error("at least 2 players are required")]
    NotEnoughPlayers,

    #[error("maximum 10 players allowed")]
    TooManyPlayers,

    #[error("player {0} not found")]
    PlayerNotFound(PlayerId),

    #[error("no card on the discard pile")]
    EmptyDiscard,
}
