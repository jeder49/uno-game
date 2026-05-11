use crate::{
    action::PlayerAction,
    card::{Card, CardKind, Color, Deck},
    error::GameError,
    event::{GameEvent, PlayerId},
    player::Player,
};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Direction {
    Clockwise,
    CounterClockwise,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GamePhase {
    Lobby,
    Playing,
    Finished,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone)]
pub struct GameState {
    pub phase: GamePhase,
    pub players: Vec<Player>,
    pub turn_index: usize,
    pub direction: Direction,
    pub draw_pile: Deck,
    pub discard_pile: Vec<Card>,
    /// The currently active color (matters after a Wild is played).
    pub active_color: Color,
    /// Stacked draw penalty waiting to be resolved by the next player.
    pub stacked_draw: u8,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            phase: GamePhase::Lobby,
            players: Vec::new(),
            turn_index: 0,
            direction: Direction::Clockwise,
            draw_pile: Deck::standard(),
            discard_pile: Vec::new(),
            active_color: Color::Red, // overwritten on game start
            stacked_draw: 0,
        }
    }

    // ── Lobby ────────────────────────────────────────────────────────────────

    pub fn add_player(&mut self, player: Player) -> Result<(), GameError> {
        match self.phase {
            GamePhase::Lobby => {}
            _ => return Err(GameError::AlreadyStarted),
        }
        if self.players.len() >= 10 {
            return Err(GameError::TooManyPlayers);
        }
        self.players.push(player);
        Ok(())
    }

    pub fn start(&mut self, rng: &mut impl rand::Rng) -> Result<Vec<GameEvent>, GameError> {
        if self.phase != GamePhase::Lobby {
            return Err(GameError::AlreadyStarted);
        }
        if self.players.len() < 2 {
            return Err(GameError::NotEnoughPlayers);
        }

        self.draw_pile.shuffle(rng);

        // Deal 7 cards to each player
        for player in &mut self.players {
            for _ in 0..7 {
                if let Some(card) = self.draw_pile.draw() {
                    player.hand.add(card);
                }
            }
        }

        // Flip the first card; re-draw until it is a number card
        let first_card = loop {
            match self.draw_pile.draw() {
                Some(card) if matches!(card.kind, CardKind::Number(_)) => break card,
                Some(other) => self
                    .draw_pile
                    .refill_from_discard(&mut std::iter::once(other).collect(), rng),
                None => return Err(GameError::GameNotInProgress),
            }
        };

        self.active_color = first_card
            .color
            .clone()
            .expect("number card always has a color");
        self.discard_pile.push(first_card.clone());
        self.phase = GamePhase::Playing;

        let player_order = self.players.iter().map(|p| p.id).collect();
        let first_player = self.current_player_id();

        Ok(vec![
            GameEvent::GameStarted {
                player_order,
                first_card,
            },
            GameEvent::TurnStarted {
                player: first_player,
            },
        ])
    }

    // ── Main action entry point ───────────────────────────────────────────────

    /// Apply an action from `actor`. Returns the list of events that occurred.
    /// Callers (server) broadcast these events to all connected clients.
    pub fn apply(
        &mut self,
        actor: PlayerId,
        action: PlayerAction,
        rng: &mut impl rand::Rng,
    ) -> Result<Vec<GameEvent>, GameError> {
        if self.phase != GamePhase::Playing {
            return Err(GameError::GameNotInProgress);
        }
        if self.current_player_id() != actor {
            return Err(GameError::NotYourTurn);
        }

        match action {
            PlayerAction::PlayCard {
                card,
                declared_color,
            } => self.apply_play_card(actor, card, declared_color),
            PlayerAction::DrawCard => self.apply_draw_card(actor, rng),
            PlayerAction::CallUno => self.apply_call_uno(actor),
            PlayerAction::ChallengeFour => self.apply_challenge_four(actor, rng),
        }
    }

    // ── Private action handlers ──────────────────────────────────────────────

    fn apply_play_card(
        &mut self,
        actor: PlayerId,
        card: Card,
        declared_color: Option<Color>,
    ) -> Result<Vec<GameEvent>, GameError> {
        let top = self.discard_pile.last().ok_or(GameError::EmptyDiscard)?;

        if !card.can_play_on(top, &self.active_color) {
            return Err(GameError::InvalidCardPlay);
        }
        if card.is_wild() && declared_color.is_none() {
            return Err(GameError::MissingDeclaredColor);
        }

        let idx = self.player_index(actor)?;
        if !self.players[idx].hand.remove(&card) {
            return Err(GameError::CardNotInHand);
        }

        // Update active color
        self.active_color = declared_color
            .clone()
            .or_else(|| card.color.clone())
            .unwrap_or(Color::Red);

        let mut events = vec![GameEvent::CardPlayed {
            player: actor,
            card: card.clone(),
            declared_color: declared_color.clone(),
        }];

        // Win condition
        if self.players[idx].hand.is_empty() {
            self.phase = GamePhase::Finished;
            self.discard_pile.push(card);
            events.push(GameEvent::GameOver { winner: actor });
            return Ok(events);
        }

        // UNO penalty check — if a player has exactly one card and didn't call UNO, catch them
        if self.players[idx].hand.len() == 1 && !self.players[idx].called_uno {
            // Note: real UNO allows other players to *catch* this; here we auto-penalise
            // for simplicity. Replace with a timed window if you want competitive rules.
            events.push(GameEvent::UnoPenalty { player: actor });
        }
        self.players[idx].called_uno = false;

        // Card effects
        match &card.kind {
            CardKind::Skip => {
                self.advance();
                let skipped = self.current_player_id();
                events.push(GameEvent::TurnSkipped { player: skipped });
                self.advance();
            }
            CardKind::Reverse => {
                self.flip_direction();
                events.push(GameEvent::DirectionReversed);
                self.advance();
            }
            CardKind::DrawTwo | CardKind::WildDrawFour => {
                self.stacked_draw += card.penalty_draw_count();
                self.advance();
                // Next player must immediately resolve the draw
            }
            _ => {
                self.advance();
            }
        }

        self.discard_pile.push(card);

        let next = self.current_player_id();
        events.push(GameEvent::TurnStarted { player: next });
        Ok(events)
    }

    fn apply_draw_card(
        &mut self,
        actor: PlayerId,
        rng: &mut impl rand::Rng,
    ) -> Result<Vec<GameEvent>, GameError> {
        let count = if self.stacked_draw > 0 {
            std::mem::replace(&mut self.stacked_draw, 0)
        } else {
            1
        };

        let mut events = vec![];
        let idx = self.player_index(actor)?;
        let mut drawn = 0u8;

        for _ in 0..count {
            if self.draw_pile.is_empty() {
                self.draw_pile
                    .refill_from_discard(&mut self.discard_pile, rng);
                events.push(GameEvent::DeckReshuffled);
            }
            if let Some(card) = self.draw_pile.draw() {
                self.players[idx].hand.add(card);
                drawn += 1;
            }
        }

        events.push(GameEvent::CardDrawn {
            player: actor,
            count: drawn,
        });
        self.advance();
        events.push(GameEvent::TurnStarted {
            player: self.current_player_id(),
        });
        Ok(events)
    }

    fn apply_call_uno(&mut self, actor: PlayerId) -> Result<Vec<GameEvent>, GameError> {
        let idx = self.player_index(actor)?;
        self.players[idx].called_uno = true;
        Ok(vec![GameEvent::UnoCalled { player: actor }])
    }

    fn apply_challenge_four(
        &mut self,
        challenger: PlayerId,
        rng: &mut impl rand::Rng,
    ) -> Result<Vec<GameEvent>, GameError> {
        let offender_idx = self.previous_index();
        let offender = self.players[offender_idx].id;

        // The offender played an illegal +4 if they held a card playable on the card
        // that was on top *before* the +4 (two cards back in the discard pile).
        let card_before_four = self.discard_pile.iter().rev().nth(1);
        let had_legal_play = if let Some(prev_top) = card_before_four {
            let prev_color = prev_top.color.clone().unwrap_or(self.active_color.clone());
            self.players[offender_idx]
                .hand
                .cards()
                .iter()
                .any(|c| !c.is_wild() && c.can_play_on(prev_top, &prev_color))
        } else {
            false
        };

        let mut events = vec![];

        if had_legal_play {
            // Challenge succeeds: offender draws 4, challenger is safe
            let idx = self.player_index(offender)?;
            let mut drawn = 0u8;
            for _ in 0..4 {
                if let Some(card) = self.draw_pile.draw() {
                    self.players[idx].hand.add(card);
                    drawn += 1;
                }
            }
            events.push(GameEvent::FourChallengeSuccess {
                offender,
                challenger,
            });
            events.push(GameEvent::CardDrawn {
                player: offender,
                count: drawn,
            });
        } else {
            // Challenge fails: challenger draws 6 (4 penalty + 2 for failed challenge)
            let idx = self.player_index(challenger)?;
            let mut drawn = 0u8;
            for _ in 0..6 {
                if self.draw_pile.is_empty() {
                    self.draw_pile
                        .refill_from_discard(&mut self.discard_pile, rng);
                    events.push(GameEvent::DeckReshuffled);
                }
                if let Some(card) = self.draw_pile.draw() {
                    self.players[idx].hand.add(card);
                    drawn += 1;
                }
            }
            events.push(GameEvent::FourChallengeFailure { challenger });
            events.push(GameEvent::CardDrawn {
                player: challenger,
                count: drawn,
            });
        }

        self.advance();
        events.push(GameEvent::TurnStarted {
            player: self.current_player_id(),
        });
        Ok(events)
    }

    // ── Turn helpers ─────────────────────────────────────────────────────────

    fn advance(&mut self) {
        let n = self.players.len();
        self.turn_index = match self.direction {
            Direction::Clockwise => (self.turn_index + 1) % n,
            Direction::CounterClockwise => (self.turn_index + n - 1) % n,
        };
    }

    fn flip_direction(&mut self) {
        self.direction = match self.direction {
            Direction::Clockwise => Direction::CounterClockwise,
            Direction::CounterClockwise => Direction::Clockwise,
        };
    }

    fn previous_index(&self) -> usize {
        let n = self.players.len();
        match self.direction {
            Direction::Clockwise => (self.turn_index + n - 1) % n,
            Direction::CounterClockwise => (self.turn_index + 1) % n,
        }
    }

    pub fn current_player_id(&self) -> PlayerId {
        self.players[self.turn_index].id
    }

    fn player_index(&self, id: PlayerId) -> Result<usize, GameError> {
        self.players
            .iter()
            .position(|p| p.id == id)
            .ok_or(GameError::PlayerNotFound(id))
    }

    pub fn top_card(&self) -> Option<&Card> {
        self.discard_pile.last()
    }
}

impl Default for GameState {
    fn default() -> Self {
        Self::new()
    }
}
