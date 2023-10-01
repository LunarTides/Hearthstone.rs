use crate::card::Card;

pub struct Game {
    pub cards: Vec<Card>,
}

impl Game {
    pub fn new() -> Self {
        Game { cards: vec![] }
    }
}
