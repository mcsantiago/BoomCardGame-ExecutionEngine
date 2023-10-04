use crate::card::{Card, CardInHand};

#[derive(Clone, Debug)]
pub struct Hand {
    cards: Vec<CardInHand>
}

impl Hand {
    fn new(cards: Vec<CardInHand>) -> Self  {
        Self { cards }
    }
    
    pub fn create(cards: Vec<CardInHand>) -> Result<Self, &'static str> {
        if cards.len() > 9 {
            Err("A hand cannot have more than 9 cards.")
        } else {
            Ok(Self::new(cards))
        }
    }

    pub fn get_cards(&self) -> &Vec<CardInHand> {
        &self.cards
    }

    pub fn add_to_hand(self, card: CardInHand) -> Result<Self, &'static str> {
        if self.cards.len() >= 9 {
            Err("Cannot add one more to hand")
        } else {
            Ok(self)
        }
    }
}

#[derive(Clone, Debug)]
pub struct Player {
    pub deck: Vec<Card>,
    pub hand: Vec<Card>,
    pub graveyard: Vec<Card>,
    pub monsters: Vec<Card>,
    pub lifepoints: i32
}
