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
    let player1 = Player::new("Player 1".into());
    let player2 = Player::new("Player 2".into());
    let mut game = Game::new(player1, player2);

    cards::execute_blueprints(&mut game);

    println!("Registered cards: {:?}", game.blueprints);

    // Activate all card's cast ability
    let binding = &mut game;
    for card in &binding.blueprints {
        let player1binding = &mut binding.player1;
        let mut card = Card::new(card.get_name(), player1binding, binding);
        card.activate(Ability::Cast, binding, player1binding);
    }

    //interact::main()?;

    Ok(())
}
