use crate::{
    card::{Card, Blueprint},
    player::Player,
};

pub struct Game<'a> {
    pub cards: Vec<Card<'a>>,
    pub blueprints: Vec<Blueprint>,
    pub player1: Player,
    pub player2: Player,
}

impl Game<'_> {
    pub fn new(player1: Player, player2: Player) -> Self {
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
