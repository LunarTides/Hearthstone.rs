use crate::card::Card;

#[derive(Debug, Clone)]
pub struct Player {
    pub name: String,
    pub id: u8,

    pub deck: Vec<Card>,
    pub hand: Vec<Card>,
}

impl Player {
    pub fn new(name: String, id: u8) -> Self {
        Player {
            name,
            id,

            deck: Vec::default(),
            hand: Vec::default(),
        }
    }
}
