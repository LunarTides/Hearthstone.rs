use crate::{
    card::{Blueprint, Card},
    player::Player,
};

pub struct Game {
    pub cards: Vec<Card>,
    pub blueprints: Vec<Blueprint>,
    pub player1: &'static Player,
    pub player2: &'static Player,
}

impl Game {
    pub fn new(player1: &'static Player, player2: &'static Player) -> Self {
        Game {
            cards: vec![],
            blueprints: vec![],
            player1,
            player2,
        }
    }

    pub fn setup() {
        // TODO: Choose random player
    }
}
