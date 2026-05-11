pub mod action;
pub mod card;
pub mod error;
pub mod event;
pub mod player;
pub mod state;

// Flatten the most-used types for convenience
pub use action::PlayerAction;
pub use card::{Card, CardKind, Color, Deck};
pub use error::GameError;
pub use event::{GameEvent, PlayerId};
pub use player::{Hand, Player};
pub use state::{Direction, GamePhase, GameState};
