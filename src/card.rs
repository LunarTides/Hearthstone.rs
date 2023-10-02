use anyhow::Result;
use std::collections::HashMap;

use crate::{
    enums::{
        Ability, CardClass, CardKeyword, CardRarity, CardRunes, CardType, CostType,
        MinionTribe, SpellSchool,
    },
    game::Game,
    player::Player,
};

pub type AbilityCallback = fn(&mut Player, &mut Game, &mut Card) -> Result<()>;
type AbilityCallbacks = HashMap<Ability, Vec<AbilityCallback>>;

#[derive(Debug, Clone)]
pub struct Card {
    pub name: String,
    pub text: String,
    pub cost: usize,
    pub card_types: Vec<CardType>,
    pub classes: Vec<CardClass>,
    pub rarities: Vec<CardRarity>,
    pub collectible: bool,
    pub id: usize,
    pub abilities: AbilityCallbacks,

    pub display_name: Option<String>,
    pub stats: Option<[usize; 2]>,
    pub tribes: Option<Vec<MinionTribe>>,
    pub spell_schools: Option<Vec<SpellSchool>>,
    pub durability: Option<usize>,
    pub cooldown: Option<usize>,
    pub heropower_text: Option<String>,
    pub heropower_cost: Option<usize>,
    pub keywords: Option<Vec<CardKeyword>>,
    pub runes: Option<Vec<CardRunes>>,
    pub dormant: Option<usize>,
    pub colossal: Option<Vec<String>>,
    pub corrupt: Option<String>,
    pub deck_settings: Option<String>,
    pub conditioned: Option<Ability>,
    pub storage: Option<HashMap<String, String>>,

    pub cost_type: CostType,
}

impl Card {
    pub fn new(name: String, owner: &mut Player, game: &mut Game) -> Self {
        let blueprint = game
            .blueprints
            .iter()
            .find(|card| card.name == name)
            .expect("should find blueprint by name");

        let mut card = Card {
            name,
            text: blueprint.text.to_owned(),
            card_types: blueprint.card_types.to_owned(),
            cost: blueprint.cost.to_owned(),
            classes: blueprint.classes.to_owned(),
            rarities: blueprint.rarities.to_owned(),
            collectible: blueprint.collectible,
            id: blueprint.id,
            abilities: blueprint.abilities.to_owned(),

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

            cost_type: CostType::Mana,
        };

        // Activate the `create` ability
        card.activate(Ability::Create, game, owner);

        // Add the card to the list of cards
        if !game.cards.iter().any(|c| c.id == card.id) {
            game.cards.push(card.clone());
        }

        card
    }

    pub fn activate(&mut self, ability: Ability, game: &mut Game, owner: &mut Player) {
        if let Some(callbacks) = self.clone().abilities.get(&ability) {
            for callback in callbacks {
                callback(owner, game, self).unwrap_or_else(|err| {
                    panic!(
                        "Something went wrong when running the '{:#?}' ability for '{}': {}",
                        ability, self.name, err
                    )
                });
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Blueprint {
    name: String,
    text: String,
    cost: usize,
    card_types: Vec<CardType>,
    classes: Vec<CardClass>,
    rarities: Vec<CardRarity>,
    collectible: bool,
    id: usize,
    abilities: AbilityCallbacks,
}

impl Blueprint {
    pub fn new() -> Self {
        Self {
            name: String::default(),
            text: String::default(),
            cost: usize::default(),
            card_types: Vec::default(),
            classes: Vec::default(),
            rarities: Vec::default(),
            collectible: bool::default(),
            id: usize::default(),
            abilities: HashMap::default(),
        }
    }

    pub fn named(mut self, name: &str) -> Self {
        self.name = name.into();
        self
    }

    pub fn with_text(mut self, text: &str) -> Self {
        self.text = text.into();
        self
    }

    pub fn costing(mut self, cost: usize) -> Self {
        self.cost = cost;
        self
    }

    pub fn with_type(mut self, card_type: CardType) -> Self {
        self.card_types.push(card_type);
        self
    }

    pub fn with_class(mut self, class: CardClass) -> Self {
        self.classes.push(class);
        self
    }

    pub fn with_rarity(mut self, rarity: CardRarity) -> Self {
        self.rarities.push(rarity);
        self
    }

    pub fn collectible(mut self, collectible: bool) -> Self {
        self.collectible = collectible;
        self
    }

    pub fn with_id(mut self, id: usize) -> Self {
        self.id = id;
        self
    }

    pub fn with_ability(mut self, key: Ability, value: AbilityCallback) -> Self {
        if self.abilities.get(&key).is_some() {
            self.abilities.get_mut(&key).unwrap().push(value);
        } else {
            self.abilities.insert(key, vec![value]);
        }
        self
    }

    pub fn build(self, game: &mut Game) -> Self {
        game.blueprints.push(self.clone());
        self
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }
}

impl Default for Blueprint {
    fn default() -> Self {
        Self::new()
    }
}
