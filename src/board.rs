// Purpose: Defines the board and its components.

use crate::card::{Card, CardEffect, CardNumber, Suit, MonsterCardProperties};
use std::rc::Rc;

#[derive(Debug)]
pub struct Deck(Vec<Rc<Card>>);

impl Deck {
    pub fn create(cards: &[Rc<Card>]) -> Self {
        // for each card in cards, add it to the deck
        Self(cards.to_vec())
    }
}

#[derive(Debug)]
pub struct MonsterSlots([Option<CardPlacement>; 1]);

#[derive(Debug)]
pub struct Graveyard([Option<Rc<Card>>; 1]);

#[derive(Debug)]
pub struct PlayableCards([Rc<Card>; 1]);

#[derive(Debug)]
pub struct PlayerCards {
    cards: PlayableCards,
    deck: Deck,
    monster_slots: MonsterSlots,
    graveyard: Graveyard,
}

impl PlayerCards {
    pub fn new() -> Self {
        let playable_cards = PlayableCards([Rc::new(Card::create(CardNumber::JOKER))]);
        let test_deck = Deck::create(&playable_cards.0);
        Self {
            cards: playable_cards,
            deck: test_deck,
            monster_slots: MonsterSlots([None]),
            graveyard: Graveyard([None])
        }
    }

    pub fn create(playable_cards: PlayableCards) -> Self {
        let test_deck = Deck::create(&playable_cards.0);
        Self {
            cards: playable_cards,
            deck: test_deck,
            monster_slots: MonsterSlots([None]),
            graveyard: Graveyard([None])
        }
    }
}

#[derive(Debug)]
pub struct Board {
    board_id: u32,
    player_one: PlayerCards,
    player_two: PlayerCards,
}

#[derive(Debug)]
pub struct CardPlacement {
    card: Card,
    position: MonsterPosition,
}

#[derive(Clone, Debug)]
pub enum MonsterPosition {
    Attack,
    Defense,
    FaceDown
}

impl Board {
    pub fn new() -> Self {
        Self {
            board_id: 0,
            player_one: PlayerCards::new(),
            player_two: PlayerCards::new(),
        }
    }
}

