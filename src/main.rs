use anyhow::Result;
use card::Card;
use player::Player;

use crate::{enums::Ability, game::Game};

mod cards;
mod enums;

mod card;
mod game;
mod interact;
mod player;

fn main() -> Result<()> {
    let player1 = Player::new("Player 1".into(), 0);
    let player2 = Player::new("Player 2".into(), 1);
    let mut game = Game::new(player1, player2);

    cards::execute_blueprints(&mut game);

    println!("Registered cards: {:?}", game.blueprints);

    // Activate all card's cast ability
    let mut names: Vec<String> = Vec::new();

    let binding = &mut game;
    for card in &binding.blueprints {
        let name = card.get_name();
        names.push(name);
    }

    for name in names {
        let mut card = Card::new(name, 0, binding);
        card.activate(Ability::Cast, binding, 0);
    }

    //interact::main()?;

    Ok(())
}
