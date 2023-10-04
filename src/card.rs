#[derive(Clone, Debug)]
pub enum CardEffect {
    DrawOneWhenDestroyed,
    DrawThreeDiscardTwoWhenDestroyed,
    DestroyAttackingMonster,
    PiercingDamage,
    PreemptiveSpecialSummonAttack, // Can summon during opponent's attack phase
    MonsterReborn,
    None
}

#[derive(Clone, Debug)]
pub enum Suit {
    CLUB(char),
    SPADE(char),
    HEART(char),
    DIAMOND(char)
}

#[derive(Clone, Debug)]
pub enum CardNumber {
    TWO(char),
    THREE(char),
    FOUR(char),
    FIVE(char),
    SIX(char),
    SEVEN(char),
    EIGHT(char),
    NINE(char),
    TEN(char),
    JACK(char),
    QUEEN(char),
    KING(char),
    ACE(char),
    JOKER(char)
}

#[derive(Clone, Debug)]
pub struct Card {
    suit: Suit,
    number: CardNumber,
    power: i32,
    effect: CardEffect
}

impl Card {
    fn new(suit: Suit, number: CardNumber, power: i32, effect: CardEffect) -> Self {
        Self{ 
            suit,
            number,
            power,
            effect
        }
    }

    pub fn create(suit: Suit, number: CardNumber, power: i32, effect: CardEffect) -> Self {
        Self::new( 
            suit,
            number,
            power,
            effect
        )
    }
}

#[derive(Clone, Debug)]
pub struct CardInHand {
    pub card: Card,
    pub is_playable: bool
}
