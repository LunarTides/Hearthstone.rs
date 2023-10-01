use anyhow::Result;

use crate::{
    enums::{Ability, CardClass, CardRarity, CardType},
    Card,
};

pub fn blueprint() -> Card {
    Card::new(
        String::from("The Coin"),
        String::from("Gain 1 Mana Crystal this turn only."),
        0,
        vec![CardType::Spell],
        vec![CardClass::Neutral],
        vec![CardRarity::Free],
        false,
        2,
        |this| {
            this.abilities.insert(Ability::Cast, cast);
            Ok(())
        },
    )
}

pub fn cast(_: &mut Card) -> Result<()> {
    println!("Test");
    Ok(())
}
