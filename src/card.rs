#[derive(Clone, Debug)]
pub enum CardEffect {
    DrawOneWhenDestroyed,
    DrawThreeDiscardTwoWhenDestroyed,
    DestroyAttackingMonster,
    PiercingDamage,
    PreemptiveSpecialSummonAttack, // Can summon during opponent's attack phase
    ResurrectMonster,
    None
}

#[derive(Clone, Debug)]
pub enum Suit {
    CLUB,
    SPADE,
    HEART,
    DIAMOND
}

#[derive(Clone, Debug)]
pub enum CardNumber {
    TWO(Suit, MonsterCardProperties),
    THREE(Suit, MonsterCardProperties),
    FOUR(Suit, MonsterCardProperties),
    FIVE(Suit, MonsterCardProperties),
    SIX(Suit, MonsterCardProperties),
    SEVEN(Suit, MonsterCardProperties),
    EIGHT(Suit, MonsterCardProperties),
    NINE(Suit, MonsterCardProperties),
    TEN(Suit, MonsterCardProperties),
    JACK(Suit, MonsterCardProperties),
    QUEEN(Suit, MonsterCardProperties),
    KING(Suit, MonsterCardProperties),
    ACE(Suit, MonsterCardProperties),
    JOKER
}

#[derive(Debug)]
pub struct Card {
    pub number: CardNumber,
}

#[derive(Clone, Debug)]
pub struct MonsterCardProperties {
    pub power: i32,
    pub effect: CardEffect
}


impl Card {
    fn new(number: CardNumber) -> Self {
        Self{ 
            number,
        }
    }

    pub fn create(number: CardNumber) -> Self {
        Self::new( 
            number,
        )
    }
}

#[derive(Debug)]
pub struct CardInHand {
    card: Card,
    is_playable: bool
}
