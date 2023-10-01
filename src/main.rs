// TODO: Split into multiple files.

use anyhow::Result;
use card::Card;
use enums::Guard;
use lazy_static::lazy_static;
use player::Player;
use std::sync::Mutex;

use crate::{enums::Ability, game::Game};

mod cards;
mod enums;

mod card;
mod game;
mod player;

lazy_static! {
    static ref PLAYER1: Player = Player::new("Player 1".into());
    static ref PLAYER2: Player = Player::new("Player 2".into());
    static ref GAME: Mutex<Game> = Mutex::from(Game::new(&PLAYER1, &PLAYER2));
}

pub fn get_game() -> Guard<Game> {
    GAME.lock().unwrap()
}

fn main() -> Result<()> {
    cards::execute_blueprints();

    let mut game = get_game();

    println!("Registered cards: {:?}", game.blueprints);

    // Activate all card's cast ability
    for card in &game.blueprints.clone() {
        Card::new(card.get_name(), game.player1, &mut game).activate(Ability::Cast, &mut game);
    }

    Ok(())
}
