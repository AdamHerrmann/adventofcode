use crate::card::Card;
use crate::cards::Cards;

pub struct CardCount {
    pub singles: u8,
    pub doubles: u8,
    pub triples: u8,
    pub quaduples: u8,
    pub pintuples: u8,

    pub joker: u8,
    pub two: u8,
    pub three: u8,
    pub four: u8,
    pub five: u8,
    pub six: u8,
    pub seven: u8,
    pub eight: u8,
    pub nine: u8,
    pub ten: u8,
    // pub jack: u8,
    pub queen: u8,
    pub king: u8,
    pub ace: u8,
}

impl CardCount {
    pub fn new(cards: &Cards) -> CardCount {
        CardCount {
            singles: 0,
            doubles: 0,
            triples: 0,
            quaduples: 0,
            pintuples: 0,
            joker: 0,
            two: 0,
            three: 0,
            four: 0,
            five: 0,
            six: 0,
            seven: 0,
            eight: 0,
            nine: 0,
            ten: 0,
            // jack: 0,
            queen: 0,
            king: 0,
            ace: 0,
        }
        .add_card(&cards.0)
        .add_card(&cards.1)
        .add_card(&cards.2)
        .add_card(&cards.3)
        .add_card(&cards.4)
    }

    pub fn add_card(mut self, card: &Card) -> Self {
        let new_count;
        match card {
            Card::Joker => {
                self.joker += 1;
                new_count = self.joker;
            }
            Card::Two => {
                self.two += 1;
                new_count = self.two;
            }
            Card::Three => {
                self.three += 1;
                new_count = self.three;
            }
            Card::Four => {
                self.four += 1;
                new_count = self.four;
            }
            Card::Five => {
                self.five += 1;
                new_count = self.five;
            }
            Card::Six => {
                self.six += 1;
                new_count = self.six;
            }
            Card::Seven => {
                self.seven += 1;
                new_count = self.seven;
            }
            Card::Eight => {
                self.eight += 1;
                new_count = self.eight;
            }
            Card::Nine => {
                self.nine += 1;
                new_count = self.nine;
            }
            Card::Ten => {
                self.ten += 1;
                new_count = self.ten;
            }
            // Card::Jack => {
            //     self.jack += 1;
            //     new_count = self.jack;
            // }
            Card::Queen => {
                self.queen += 1;
                new_count = self.queen;
            }
            Card::King => {
                self.king += 1;
                new_count = self.king;
            }
            Card::Ace => {
                self.ace += 1;
                new_count = self.ace;
            }
        }

        match new_count {
            1 => self.singles += 1,
            2 => {
                self.singles -= 1;
                self.doubles += 1;
            }
            3 => {
                self.doubles -= 1;
                self.triples += 1;
            }
            4 => {
                self.triples -= 1;
                self.quaduples += 1;
            }
            5 => {
                self.quaduples -= 1;
                self.pintuples += 1;
            }
            _ => panic!("How did you get this many cards"),
        }
        self
    }
}
