use anyhow::Result;

use crate::{
    card::Blueprint,
    enums::{Ability, CardClass, CardKeyword, CardRarity, CardType, SpellSchool},
    game::Game,
    Card,
};

pub fn blueprint(game: &mut Game) -> Blueprint {
    Blueprint::new()
        .named("The Coin")
        .with_text("Gain 1 Mana Crystal this turn only.")
        .costing(0)
        .with_type(CardType::Spell)
        .with_spell_school(SpellSchool::None)
        .with_class(CardClass::Neutral)
        .with_rarity(CardRarity::Free)
        .collectible(false)
        .with_id(2)
        .with_ability(Ability::Cast, cast)
        .build(game)
}

pub fn cast(owner: u8, game: &mut Game, this: &mut Card) -> Result<()> {
    // Ideally this would happen automatically, but oh well.
    let owner = game.id_to_player(owner);

    // The "2" should get converted to usize by some `handle_dormant` method
    this.add_keyword(CardKeyword::Dormant, Some("2"));

    let id = owner.id;

    println!("{}", owner.name);
    println!("{}", game.get_opponent_from_id(id).name);
    Ok(())
}
