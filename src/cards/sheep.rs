use crate::{
    card::Blueprint,
    enums::{CardClass, CardRarity, CardType},
    game::Game,
};

pub fn blueprint(game: &mut Game) -> Blueprint {
    Blueprint::new()
        .named("Sheep")
        .costing(1)
        .with_type(CardType::Minion)
        .with_class(CardClass::Neutral)
        .with_rarity(CardRarity::Free)
        .collectible(false)
        .with_id(1)
        .build(game)
}
