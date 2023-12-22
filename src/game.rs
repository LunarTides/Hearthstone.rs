use crate::{
    card::{Blueprint, Card},
    player::Player,
};

pub struct Game {
    pub cards: Vec<Card>,
    pub blueprints: Vec<Blueprint>,
    pub player1: Player,
    pub player2: Player,
    pub current_player: u8,

    pub board: [Vec<Card>; 2],
}

impl Game {
    pub fn new(player1: Player, player2: Player) -> Self {
        Game {
            cards: vec![],
            blueprints: vec![],
            player1,
            player2,
            current_player: 0,

            board: [Vec::default(), Vec::default()],
        }
    }

    pub fn get_current_player(&mut self) -> &mut Player {
        self.id_to_player(self.current_player)
    }

    pub fn get_opponent_from_id(&mut self, id: u8) -> &mut Player {
        let opponent_id = 1 - id;

        self.id_to_player(opponent_id)
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
