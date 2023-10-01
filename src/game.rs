use crate::card::Card;

pub struct Game {
    pub cards: Vec<Card>,
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
