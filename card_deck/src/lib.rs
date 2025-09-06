use rand::prelude::*;

#[derive(Debug,PartialEq, Eq)]
pub enum Suit {
    Heart,
    Diamond,
    Spade,
    Club,
}
#[derive(Debug,PartialEq, Eq)]
pub enum Rank {
    Ace,
    King,
    Queen,
    Jack,
    Number(u8),
}

impl Suit {
    pub fn random() -> Suit {
        let mut rng = rand::thread_rng();
        let n: u8 = rng.gen_range(1..=4);
        return match n {
            1 => Suit::Heart,
            2 => Suit::Diamond,
            3 => Suit::Spade,
            _ => Suit::Club,
        };
    }
    pub fn translate(value: u8) -> Suit {
        return match value {
            1 => Suit::Heart,
            2 => Suit::Diamond,
            3 => Suit::Spade,
            _ => Suit::Club,
        };
    }
}

impl Rank {
    pub fn random() -> Rank {
        let mut rng = rand::thread_rng();
        let n: u8 = rng.gen_range(1..=13);
        return match n {
            1 => Rank::Ace,
            11 => Rank::Jack,
            13 => Rank::King,
            12 => Rank::Queen,
            _ => Rank::Number(n),
        };
    }

    pub fn translate(value: u8) -> Rank {
        return match value {
            1 => Rank::Ace,
            11 => Rank::Jack,
            13 => Rank::King,
            12 => Rank::Queen,
            _ => Rank::Number(value),
        };
    }
}
#[derive(Debug,PartialEq, Eq)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

pub fn winner_card(card: &Card) -> bool {
    if card.rank == Rank::Ace && card.suit == Suit::Spade {
        return true;
    }
    false
}
