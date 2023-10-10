use anyhow::Result;
use std::collections::HashMap;

use crate::{
    enums::{
        Ability, CardClass, CardKeyword, CardRarity, CardRunes, CardType, CostType, MinionTribe,
        SpellSchool,
    },
    game::Game,
};

pub type AbilityCallback = fn(u8, &mut Game, &mut Card) -> Result<()>;
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

    pub stats: Option<[usize; 2]>,
    pub tribes: Option<Vec<MinionTribe>>,
    pub spell_schools: Option<Vec<SpellSchool>>,
    pub durability: Option<usize>,
    pub cooldown: Option<usize>,
    pub heropower_text: Option<String>,
    pub heropower_cost: Option<usize>,
    pub keywords: HashMap<CardKeyword, Option<&'static str>>,
    pub runes: Option<Vec<CardRunes>>,
    pub deck_settings: Option<String>,
    pub conditioned: Option<Ability>,
    pub storage: Option<HashMap<String, String>>,

    pub cost_type: CostType,
}

impl Card {
    pub fn new(name: String, owner: u8, game: &mut Game) -> Self {
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

            stats: None,
            tribes: None,
            spell_schools: None,
            durability: None,
            cooldown: None,
            heropower_text: None,
            heropower_cost: None,
            keywords: HashMap::new(),
            runes: None,
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

    pub fn activate(&mut self, ability: Ability, game: &mut Game, owner: u8) {
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

    pub fn add_keyword(&mut self, keyword: CardKeyword, value: Option<&'static str>) {
        self.keywords.insert(keyword, value);
    }
}

#[derive(Debug, Clone)]
pub struct Blueprint {
    // Common / Required
    name: String,
    text: String,
    cost: usize,
    card_types: Vec<CardType>,

    // Type-specific
    stats: Option<[usize; 2]>,
    tribes: Option<Vec<MinionTribe>>,
    spell_schools: Option<Vec<SpellSchool>>,
    durability: Option<usize>,
    cooldown: Option<usize>,
    heropower_text: Option<String>,
    heropower_cost: Option<usize>,

    // More required
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

            stats: None,
            tribes: None,
            spell_schools: None,
            durability: None,
            cooldown: None,
            heropower_text: None,
            heropower_cost: None,

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

    // Type-specific
    pub fn with_stats(mut self, stats: [usize; 2]) -> Self {
        self.stats = Some(stats);
        self
    }

    pub fn with_tribe(mut self, tribe: MinionTribe) -> Self {
        match self.tribes {
            None => {
                self.tribes = Some(vec![tribe]);
            },
            Some(ref mut tribes) => {
                tribes.push(tribe);
            }
        }
        self
    }

    pub fn with_spell_school(mut self, spell_school: SpellSchool) -> Self {
        match self.spell_schools {
            None => {
                self.spell_schools = Some(vec![spell_school]);
            },
            Some(ref mut spell_schools) => {
                spell_schools.push(spell_school);
            }
        }
        self
    }

    pub fn with_durability(mut self, durability: usize) -> Self {
        self.durability = Some(durability);
        self
    }

    pub fn with_cooldown(mut self, cooldown: usize) -> Self {
        self.cooldown = Some(cooldown);
        self
    }

    pub fn with_heropower_text(mut self, heropower_text: &str) -> Self {
        self.heropower_text = Some(heropower_text.into());
        self
    }

    pub fn with_heropower_cost(mut self, heropower_cost: usize) -> Self {
        self.heropower_cost = Some(heropower_cost);
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
