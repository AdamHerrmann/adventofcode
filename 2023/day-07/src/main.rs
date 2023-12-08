use crate::input::Input;

mod card;
mod card_count;
mod cards;
mod hand;
mod hand_type;
mod input;

fn main() {
    let value: u32 = Input::parse("input.txt")
        .hands
        .iter()
        .enumerate()
        .map(|(i, h)| (i as u32 + 1) * h.bid)
        .sum();

    println!("Value: {}", value);
}
