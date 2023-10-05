use crate::card::{Card, CardEffect, CardNumber, Suit, MonsterCardProperties};
use crate::board::{Board, PlayerCards};
use std::rc::Rc;

#[derive(Clone, Debug)]
pub enum ActivePlayer {
    Player1,
    Player2
}

#[derive(Clone, Debug)]
pub enum Phase {
    DRAW,
    STANDY,
    MAIN_1,
    BATTLE(BattlePhase),
    MAIN_2,
    END
}

#[derive(Clone, Debug)]
pub enum BattlePhase {
    START,
    BATTLE,
    DAMAGE,
    END
}

#[derive(Debug)]
pub struct Player {
    player_id: u32,
    player_name: Rc<str>,
    player_cards: PlayerCards,
}

#[derive(Debug)]
pub struct Game {
    player1: Player,
    player2: Player,
    active_player: ActivePlayer,
    board: Board,
    phase: Phase
}


impl Game {
    pub fn new() -> Self {
        let player1 = Player {
            player_id: 1,
            player_name: Rc::from("Player 1"),
            player_cards: PlayerCards::new(),
        };

        let player2 = Player {
            player_id: 2,
            player_name: Rc::from("Player 2"),
            player_cards: PlayerCards::new(),
        };

        let board = Board::new();

        Self {
            player1,
            player2,
            board,
            active_player: ActivePlayer::Player1,
            phase: Phase::DRAW
        }
    }

}
