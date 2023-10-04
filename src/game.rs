use crate::card::{Card, CardEffect, CardNumber, Suit};
use crate::player::Player;

#[derive(Clone, Debug)]
pub enum ActivePlayer {
    Player1,
    Player2
}

#[derive(Clone, Debug)]
pub struct Game {
    player1: Player,
    player2: Player,
    active_player: ActivePlayer
}

impl Game {
    fn generate_default_deck(shuffle: bool) -> Vec<Card> {
        vec![
            Card::create(Suit::HEART('D'), CardNumber::TWO('d'), 1200, CardEffect::DrawOneWhenDestroyed),
        ]
    }

    pub fn new() -> Self {
        let player1 = Player {
            deck: Game::generate_default_deck(true),
            hand: vec![],
            graveyard: vec![],
            monsters: vec![],
            lifepoints: 8000
        };

        let player2 = Player {
            deck: Game::generate_default_deck(true),
            hand: vec![],
            graveyard: vec![],
            monsters: vec![],
            lifepoints: 8000
        };
        Self {
            player1,
            player2,
            active_player: ActivePlayer::Player1
        }
    }

}
