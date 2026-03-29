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
