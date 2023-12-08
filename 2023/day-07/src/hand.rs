use crate::{cards::Cards, hand_type::HandType};

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Hand {
    pub hand_type: HandType,
    pub cards: Cards,
    pub bid: u32,
}

impl Hand {
    pub fn parse(line: String) -> Hand {
        let cards = Cards::parse(&line[..5]);

        Hand {
            hand_type: HandType::compute(&cards),
            cards,
            bid: line[6..].parse::<u32>().unwrap(),
        }
    }
}
