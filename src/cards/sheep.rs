use crate::{Card, CardClass, CardRarity, CardType};

pub fn blueprint() -> Card {
    Card::new(
        String::from("Sheep"),
        String::from(""),
        1,
        vec![CardType::Minion],
        vec![CardClass::Neutral],
        vec![CardRarity::Free],
        false,
        1,
        |_| {},
    )
}
