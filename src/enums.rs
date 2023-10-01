#[derive(Debug, Clone)]
pub enum CardType {
    Minion,
    Spell,
    Weapon,
    Hero,
    Location,
    None,
}

#[derive(Debug, Clone)]
pub enum MinionTribe {
    Beast,
    Demon,
    Dragon,
    Elemental,
    Mech,
    Murloc,
    Naga,
    Pirate,
    Quilboar,
    Totem,
    Undead,
    All,
    None,
}

#[derive(Debug, Clone)]
pub enum SpellSchool {
    Arcane,
    Fel,
    Fire,
    Frost,
    Holy,
    Nature,
    Shadow,
    None,
}

#[derive(Debug, Clone)]
pub enum CardClass {
    DeathKnight,
    DemonHunter,
    Druid,
    Hunter,
    Mage,
    Paladin,
    Priest,
    Rogue,
    Shaman,
    Warlock,
    Warrior,
    Neutral,
}

#[derive(Debug, Clone)]
pub enum CardRarity {
    Free,
    Common,
    Rare,
    Epic,
    Legendary,
}

#[derive(Debug, Clone)]
pub enum CostType {
    Mana,
    Armor,
    Health,
}

#[derive(Debug, Clone)]
pub enum CardKeyword {
    DivineShield,
    Dormant,
    Lifesteal,
    Poisonous,
    Reborn,
    Rush,
    Stealth,
    Taunt,
    Tradeable,
    Windfury,
    Outcast,
    CastOnDraw,
    Charge,
    MegaWindfury,
    Echo,
    Magnetic,
    Twinspell,
    Elusive,
    Cleave,
}

#[derive(Debug, Clone)]
pub enum CardRunes {
    Blood,
    Frost,
    Unholy,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Ability {
    Adapt,
    Battlecry,
    Cast,
    Combo,
    Deathrattle,
    Finale,
    Frenzy,
    HeroPower,
    HonorableKill,
    Infuse,
    Inspire,
    Invoke,
    Outcast,
    Overheal,
    Overkill,
    Passive,
    Spellburst,
    StartOfGame,
    Use,

    Placeholders,
    Condition,
    Remove,
    HandPassive,
    Tick,
    HandTick,
    Test,
    Create,
}
