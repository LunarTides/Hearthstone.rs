use std::borrow::BorrowMut;

use anyhow::Result;
use card::Card;
use player::Player;

use crate::{enums::Ability, game::Game};

mod cards;
mod enums;

mod card;
mod game;
mod interact;
mod functions;
mod player;

fn main() -> Result<()> {
    let player1 = Player::new("Player 1".into(), 0);
    let player2 = Player::new("Player 2".into(), 1);
    let mut game = Game::new(player1, player2);

    cards::execute_blueprints(&mut game);

    println!("Registered cards: {:?}", game.blueprints);

    // Activate all card's cast ability
    functions::get_cards(&mut game);

    let binding = &mut game;
    let cards: &mut Vec<Card> = binding.cards.borrow_mut();

    for card in cards {
        card.activate(Ability::Cast, binding, 0);
    }

    Ok(())
}
