use crate::card::Card;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Cards(pub Card, pub Card, pub Card, pub Card, pub Card);

impl Cards {
    pub fn parse(line: &str) -> Cards {
        line.chars().map(|c| Card::parse(c)).collect::<Cards>()
    }
}

impl FromIterator<Card> for Cards {
    fn from_iter<T: IntoIterator<Item = Card>>(iter: T) -> Self {
        let mut iter = iter.into_iter();

        Cards(
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
        )
    }
}
