use std::collections::HashMap;

use crate::{
    enums::{
        Ability, CardClass, CardKeyword, CardRarity, CardRunes, CardType, MinionTribe, SpellSchool,
    },
    get_game,
};

type AbilityCallback = fn(&mut Card);
type AbilityCallbacks = HashMap<Ability, AbilityCallback>;

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
}

impl Card {
    pub fn new(
        name: String,
        text: String,
        cost: usize,
        card_types: Vec<CardType>,
        classes: Vec<CardClass>,
        rarities: Vec<CardRarity>,
        collectible: bool,
        id: usize,
        create_ability: AbilityCallback,
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

        // Activate the `setup` ability
        card.abilities.insert(Ability::Create, create_ability);
        card.activate(Ability::Create);

        // Add the card to the list of cards
        let mut game = get_game();
        if !game.cards.iter().any(|c| c.id == card.id) {
            game.cards.push(card.clone());
        }

        card
    }

    pub fn activate(&mut self, ability: Ability) {
        if let Some(ability) = self.clone().abilities.get(&ability) {
            ability(self);
        }
    }
}
