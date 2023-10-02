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
        Card::new(card.get_name(), &mut player1, binding).activate(Ability::Cast, binding);
    }

    //interact::main()?;

    Ok(())
}
