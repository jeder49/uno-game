use crate::card::Card;
use crate::event::PlayerId;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Default)]
pub struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    pub fn add(&mut self, card: Card) {
        self.cards.push(card);
    }

    /// Remove a specific card. Returns `false` if the card was not in the hand.
    pub fn remove(&mut self, card: &Card) -> bool {
        if let Some(pos) = self.cards.iter().position(|c| c == card) {
            self.cards.remove(pos);
            true
        } else {
            false
        }
    }

    pub fn cards(&self) -> &[Card] {
        &self.cards
    }

    pub fn len(&self) -> usize {
        self.cards.len()
    }

    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone)]
pub struct Player {
    pub id: PlayerId,
    pub name: String,
    pub hand: Hand,
    /// Whether the player has called UNO this round.
    pub called_uno: bool,
}

impl Player {
    pub fn new(id: PlayerId, name: impl Into<String>) -> Self {
        Self {
            id,
            name: name.into(),
            hand: Hand::default(),
            called_uno: false,
        }
    }
}
