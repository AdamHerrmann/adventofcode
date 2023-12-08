#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Card {
    Joker = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    // Jack = 11,
    Queen = 12,
    King = 13,
    Ace = 14,
}

impl Card {
    pub fn parse(c: char) -> Card {
        match c {
            'J' => Card::Joker,
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            'T' => Card::Ten,
            // 'J' => Card::Jack,
            'Q' => Card::Queen,
            'K' => Card::King,
            'A' => Card::Ace,
            _ => panic!("Unknown card"),
        }
    }
}
