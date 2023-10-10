use crate::{
    card::{Blueprint, Card},
    player::Player,
};

pub struct Game {
    pub cards: Vec<Card>,
    pub blueprints: Vec<Blueprint>,
    pub player1: Player,
    pub player2: Player,
}

impl Game {
    pub fn new(player1: Player, player2: Player) -> Self {
        Game {
            cards: vec![],
            blueprints: vec![],
            player1,
            player2,
        }
    }

    pub fn setup(&self) {
        // TODO: Choose random player
    }

    pub fn id_to_player(&mut self, id: u8) -> &mut Player {
        if id == 0 {
            return &mut self.player1;
        } else if id == 1 {
            return &mut self.player2;
        }

        unreachable!()
    }
}
