use lazy_static::lazy_static;
use std::{
    collections::HashMap,
    sync::{Mutex, MutexGuard},
};

mod cards;

// TODO: Complete
#[derive(Debug, Clone)]
pub enum CardType {
    Minion,
    Spell,
}

// TODO: Complete
#[derive(Debug, Clone)]
pub enum MinionTribe {
    None,
}

// TODO: Complete
#[derive(Debug, Clone)]
pub enum SpellSchool {
    None,
}

// TODO: Complete
#[derive(Debug, Clone)]
pub enum CardClass {
    Neutral,
}

// TODO: Complete
#[derive(Debug, Clone)]
pub enum CardRarity {
    Free,
}

// TODO: Complete
#[derive(Debug, Clone)]
pub enum CardKeyword {
    Taunt,
}

// TODO: Complete
#[derive(Debug, Clone)]
pub enum CardRunes {
    Blood,
    Frost,
    Unholy,
}

// TODO: Complete
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Ability {
    Battlecry,
    Cast,
    Setup,
}

type AbilityCallback = fn(&mut Card);
type AbilityCallbacks = HashMap<Ability, AbilityCallback>;

#[derive(Debug, Clone)]
pub struct Card {
    name: String,
    text: String,
    cost: usize,
    card_types: Vec<CardType>,
    classes: Vec<CardClass>,
    rarities: Vec<CardRarity>,
    collectible: bool,
    id: usize,
    abilities: AbilityCallbacks,

    display_name: Option<String>,
    stats: Option<[usize; 2]>,
    tribes: Option<Vec<MinionTribe>>,
    spell_schools: Option<Vec<SpellSchool>>,
    durability: Option<usize>,
    cooldown: Option<usize>,
    heropower_text: Option<String>,
    heropower_cost: Option<usize>,
    keywords: Option<Vec<CardKeyword>>,
    runes: Option<Vec<CardRunes>>,
    dormant: Option<usize>,
    colossal: Option<Vec<String>>,
    corrupt: Option<String>,
    deck_settings: Option<String>,
    conditioned: Option<Ability>,
    storage: Option<HashMap<String, String>>,
}

impl Card {
    fn new(
        name: String,
        text: String,
        cost: usize,
        card_types: Vec<CardType>,
        classes: Vec<CardClass>,
        rarities: Vec<CardRarity>,
        collectible: bool,
        id: usize,
        setup: AbilityCallback,
    ) -> Self {
        let mut card = Card {
            name,
            text,
            card_types,
            cost,
            classes,
            rarities,
            collectible,
            id,
            abilities: HashMap::new(),

            display_name: None,
            stats: None,
            tribes: None,
            spell_schools: None,
            durability: None,
            cooldown: None,
            heropower_text: None,
            heropower_cost: None,
            keywords: None,
            runes: None,
            dormant: None,
            colossal: None,
            corrupt: None,
            deck_settings: None,
            conditioned: None,
            storage: None,
        };

        card.abilities.insert(Ability::Setup, setup);
        card.activate(Ability::Setup);

        let mut game = get_game();
        if !game.cards.iter().any(|c| c.id == card.id) {
            game.cards.push(card.clone());
        }

        card
    }

    fn activate(&mut self, ability: Ability) {
        if let Some(ability) = self.clone().abilities.get(&ability) {
            ability(self);
        }
    }
}

pub struct Game {
    cards: Vec<Card>,
}

impl Game {
    fn new() -> Self {
        Game { cards: vec![] }
    }
}

lazy_static! {
    static ref GAME: Mutex<Game> = Mutex::from(Game::new());
}

pub fn get_game() -> MutexGuard<'static, Game> {
    GAME.lock().unwrap()
}

fn main() {
    cards::execute_blueprints();

    println!("Registered cards: {:?}", get_game().cards);

    let game = get_game();

    // Activate all card's cast ability
    for card in &game.cards {
        card.clone().activate(Ability::Cast);
    }
}
