use crate::{card_count::CardCount, cards::Cards};

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum HandType {
    HighCard = 0,
    TwoOfAKind = 1,
    TwoPair = 2,
    ThreeOfAKind = 3,
    FullHouse = 4,
    FourOfAKind = 5,
    FiveOfAKind = 6,
}

impl HandType {
    pub fn compute(cards: &Cards) -> HandType {
        let card_count = CardCount::new(cards);

        if card_count.pintuples == 1 {
            HandType::FiveOfAKind
        } else if card_count.quaduples == 1 {
            match card_count.joker {
                0 => HandType::FourOfAKind,
                _ => HandType::FiveOfAKind,
            }
        } else if card_count.triples == 1 && card_count.doubles == 1 {
            match card_count.joker {
                0 => HandType::FullHouse,
                _ => HandType::FiveOfAKind,
            }
        } else if card_count.triples == 1 {
            match card_count.joker {
                0 => HandType::ThreeOfAKind,
                _ => HandType::FourOfAKind,
            }
        } else if card_count.doubles == 2 {
            match card_count.joker {
                0 => HandType::TwoPair,
                1 => HandType::FullHouse,
                _ => HandType::FourOfAKind,
            }
        } else if card_count.doubles == 1 {
            match card_count.joker {
                0 => HandType::TwoOfAKind,
                _ => HandType::ThreeOfAKind,
            }
        } else {
            match card_count.joker {
                0 => HandType::HighCard,
                _ => HandType::TwoOfAKind,
            }
        }
    }
}
