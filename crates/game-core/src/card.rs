#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Color {
    Red,
    Green,
    Blue,
    Yellow,
    // Wild cards carry no color until one is declared
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CardKind {
    Number(u8), // 0–9
    Skip,
    Reverse,
    DrawTwo,
    Wild,
    WildDrawFour,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Card {
    pub color: Option<Color>, // None for unplayed Wild / WildDrawFour
    pub kind: CardKind,
}

impl Card {
    pub fn new(color: Color, kind: CardKind) -> Self {
        Self {
            color: Some(color),
            kind,
        }
    }

    pub fn wild(kind: CardKind) -> Self {
        debug_assert!(matches!(kind, CardKind::Wild | CardKind::WildDrawFour));
        Self { color: None, kind }
    }

    pub fn is_wild(&self) -> bool {
        matches!(self.kind, CardKind::Wild | CardKind::WildDrawFour)
    }

    /// How many cards the next player must draw because of this card (0 if none).
    pub fn penalty_draw_count(&self) -> u8 {
        match self.kind {
            CardKind::DrawTwo => 2,
            CardKind::WildDrawFour => 4,
            _ => 0,
        }
    }

    /// Whether this card may be played on top of `top` given the current active `color`.
    pub fn can_play_on(&self, top: &Card, active_color: &Color) -> bool {
        if self.is_wild() {
            return true;
        }
        // Same color as the currently active color, or same kind
        self.color.as_ref().map_or(false, |c| c == active_color) || self.kind == top.kind
    }
}

// ── Deck ────────────────────────────────────────────────────────────────────
// TODO: put Deck in it's own file

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone)]
pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    /// Standard 108-card UNO deck.
    pub fn standard() -> Self {
        let mut cards = Vec::with_capacity(108);

        for color in [Color::Red, Color::Green, Color::Blue, Color::Yellow] {
            // One 0 per color
            cards.push(Card::new(color.clone(), CardKind::Number(0)));

            // Two of everything else per color
            for _ in 0..2 {
                for n in 1..=9 {
                    cards.push(Card::new(color.clone(), CardKind::Number(n)));
                }
                cards.push(Card::new(color.clone(), CardKind::Skip));
                cards.push(Card::new(color.clone(), CardKind::Reverse));
                cards.push(Card::new(color.clone(), CardKind::DrawTwo));
            }
        }

        // Four of each wild
        for _ in 0..4 {
            cards.push(Card::wild(CardKind::Wild));
            cards.push(Card::wild(CardKind::WildDrawFour));
        }

        Self { cards }
    }

    pub fn shuffle(&mut self, rng: &mut impl rand::Rng) {
        use rand::seq::SliceRandom;
        self.cards.shuffle(rng);
    }

    pub fn draw(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    pub fn len(&self) -> usize {
        self.cards.len()
    }

    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }

    /// Move all discard cards (except the top one) back into the draw pile and shuffle.
    pub fn refill_from_discard(&mut self, discard: &mut Vec<Card>, rng: &mut impl rand::Rng) {
        if let Some(top) = discard.pop() {
            self.cards.extend(discard.drain(..));
            self.shuffle(rng);
            discard.push(top);
        }
    }
}
