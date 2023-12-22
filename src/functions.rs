use crate::{game::Game, card::Card};

pub fn get_cards(game: &mut Game) {
    let mut names: Vec<String> = Vec::new();

    for card in &game.blueprints {
        let name = card.get_name();
        names.push(name);
    }

    for name in names {
        let card = Card::new(name, 0, game);
        game.cards.push(card);
    }
}