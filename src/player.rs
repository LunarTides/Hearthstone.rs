#[derive(Debug, Clone)]
pub struct Player {
    pub name: String,
    pub id: u8,
}

impl Player {
    pub fn new(name: String, id: u8) -> Self {
        Player { name, id }
    }
}
