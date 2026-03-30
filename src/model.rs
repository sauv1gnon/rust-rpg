use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Stats {
    pub max_hp: i32,
    pub max_mp: i32,
    pub atk: i32,
    pub def: i32,
    pub spd: i32,
}

impl Stats {
    pub const fn new(max_hp: i32, max_mp: i32, atk: i32, def: i32, spd: i32) -> Self {
        Self {
            max_hp,
            max_mp,
            atk,
            def,
            spd,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Character {
    pub name: String,
    pub stats: Stats,
    pub hp: i32,
    pub mp: i32,
}

impl Character {
    pub fn new(name: impl Into<String>, stats: Stats) -> Self {
        Self {
            name: name.into(),
            hp: stats.max_hp,
            mp: stats.max_mp,
            stats,
        }
    }

    pub fn is_alive(&self) -> bool {
        self.hp > 0
    }

    pub fn spend_mp(&mut self, amount: i32) -> bool {
        if self.mp < amount {
            false
        } else {
            self.mp -= amount;
            true
        }
    }

    pub fn heal(&mut self, amount: i32) -> i32 {
        let previous_hp = self.hp;
        self.hp = (self.hp + amount).min(self.stats.max_hp).max(0);
        self.hp - previous_hp
    }

    pub fn take_damage(&mut self, amount: i32) -> i32 {
        let actual_damage = amount.max(0);
        let previous_hp = self.hp;
        self.hp = (self.hp - actual_damage).max(0);
        previous_hp - self.hp
    }
}

pub trait CombatantState {
    fn name(&self) -> &str;

    fn stats(&self) -> &Stats;

    fn hp(&self) -> i32;

    fn mp(&self) -> i32;

    fn spend_mp(&mut self, amount: i32) -> bool;

    fn heal(&mut self, amount: i32) -> i32;

    fn take_damage(&mut self, amount: i32) -> i32;

    fn is_alive(&self) -> bool {
        self.hp() > 0
    }
}

impl CombatantState for Character {
    fn name(&self) -> &str {
        &self.name
    }

    fn stats(&self) -> &Stats {
        &self.stats
    }

    fn hp(&self) -> i32 {
        self.hp
    }

    fn mp(&self) -> i32 {
        self.mp
    }

    fn spend_mp(&mut self, amount: i32) -> bool {
        Character::spend_mp(self, amount)
    }

    fn heal(&mut self, amount: i32) -> i32 {
        Character::heal(self, amount)
    }

    fn take_damage(&mut self, amount: i32) -> i32 {
        Character::take_damage(self, amount)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Item {
    Potion,
    Ether,
    Tonic,
}

impl Item {
    pub const fn label(self) -> &'static str {
        match self {
            Item::Potion => "Potion",
            Item::Ether => "Ether",
            Item::Tonic => "Tonic",
        }
    }
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.label())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Reward {
    pub gold: i32,
    pub exp: i32,
    pub item_drop: Option<Item>,
}

impl Reward {
    pub const fn new(gold: i32, exp: i32, item_drop: Option<Item>) -> Self {
        Self {
            gold,
            exp,
            item_drop,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Encounter {
    pub name: String,
    pub enemy: Character,
    pub reward: Reward,
}

impl Encounter {
    pub fn new(name: impl Into<String>, enemy: Character, reward: Reward) -> Self {
        Self {
            name: name.into(),
            enemy,
            reward,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ShopOffer {
    pub item: Item,
    pub price: i32,
}

impl ShopOffer {
    pub const fn new(item: Item, price: i32) -> Self {
        Self { item, price }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Player {
    pub character: Character,
    pub gold: i32,
    pub experience: i32,
    pub inventory: Vec<Item>,
}

impl Player {
    pub fn new(character: Character) -> Self {
        Self {
            character,
            gold: 0,
            experience: 0,
            inventory: Vec::new(),
        }
    }

    pub fn add_gold(&mut self, amount: i32) {
        self.gold = self.gold.saturating_add(amount.max(0));
    }

    pub fn gain_experience(&mut self, amount: i32) {
        self.experience = self.experience.saturating_add(amount.max(0));
    }

    pub fn add_item(&mut self, item: Item) {
        self.inventory.push(item);
    }

    pub fn apply_reward(&mut self, reward: Reward) {
        self.add_gold(reward.gold);
        self.gain_experience(reward.exp);

        if let Some(item) = reward.item_drop {
            self.add_item(item);
        }
    }

    pub fn can_afford(&self, amount: i32) -> bool {
        self.gold >= amount
    }

    pub fn spend_gold(&mut self, amount: i32) -> bool {
        if self.gold < amount {
            false
        } else {
            self.gold -= amount;
            true
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Action {
    Attack,
    Defend,
    Heal,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BattleOutcome {
    InProgress,
    HeroWon,
    EnemyWon,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Combatant {
    Hero,
    Enemy,
}
