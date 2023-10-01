// TODO: Split into multiple files.

use anyhow::Result;
use card::Card;
use lazy_static::lazy_static;
use std::sync::{Mutex, MutexGuard};

use crate::{enums::Ability, game::Game};

mod cards;
mod enums;

mod card;
mod game;

lazy_static! {
    static ref GAME: Mutex<Game> = Mutex::from(Game::new());
}

pub fn get_game() -> MutexGuard<'static, Game> {
    GAME.lock().unwrap()
}

fn main() -> Result<()> {
    cards::execute_blueprints();

    let game = get_game();

    println!("Registered cards: {:?}", game.cards);

    // Activate all card's cast ability
    for card in &game.cards {
        card.clone().activate(Ability::Cast);
    }

    Ok(())
}
