use anyhow::Result;

use crate::{
    card::Blueprint,
    enums::{Ability, CardClass, CardRarity, CardType},
    game::Game,
    Card,
};

pub fn blueprint(game: &mut Game) -> Blueprint {
    Blueprint::new()
        .named("The Coin")
        .with_text("Gain 1 Mana Crystal this turn only.")
        .costing(0)
        .with_type(CardType::Spell)
        .with_class(CardClass::Neutral)
        .with_rarity(CardRarity::Free)
        .collectible(false)
        .with_id(2)
        .with_ability(Ability::Cast, cast)
        .build(game)
}

pub fn cast(this: &mut Card, _: &mut Game) -> Result<()> {
    println!("{}", this.owner.name.to_owned());
    Ok(())
}
