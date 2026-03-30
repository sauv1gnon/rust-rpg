use crate::model::{Action, BattleOutcome, Combatant, CombatantState};

pub const HEAL_COST: i32 = 5;
pub const HEAL_AMOUNT: i32 = 12;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CombatEvent {
    GuardRaised {
        target: Combatant,
    },
    GuardConsumed {
        target: Combatant,
    },
    DamageDealt {
        source: Combatant,
        target: Combatant,
        amount: i32,
    },
    Healed {
        target: Combatant,
        amount: i32,
    },
    BattleEnded(BattleOutcome),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CombatError {
    BattleAlreadyFinished,
    NotEnoughMp {
        cost: i32,
        current_mp: i32,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Battle<Hero, Enemy>
where
    Hero: CombatantState,
    Enemy: CombatantState,
{
    pub hero: Hero,
    pub enemy: Enemy,
    pub hero_guarding: bool,
    pub outcome: BattleOutcome,
}

impl<Hero, Enemy> Battle<Hero, Enemy>
where
    Hero: CombatantState,
    Enemy: CombatantState,
{
    pub fn new(hero: Hero, enemy: Enemy) -> Self {
        Self {
            hero,
            enemy,
            hero_guarding: false,
            outcome: BattleOutcome::InProgress,
        }
    }

    pub fn hero_acts_first(&self) -> bool {
        self.hero.stats().spd >= self.enemy.stats().spd
    }

    pub fn is_finished(&self) -> bool {
        self.outcome != BattleOutcome::InProgress
    }

    pub fn step(&mut self, action: Action) -> Result<Vec<CombatEvent>, CombatError> {
        if self.is_finished() {
            return Err(CombatError::BattleAlreadyFinished);
        }

        if matches!(action, Action::Heal) && self.hero.mp() < HEAL_COST {
            return Err(CombatError::NotEnoughMp {
                cost: HEAL_COST,
                current_mp: self.hero.mp(),
            });
        }

        let mut events = Vec::new();

        if self.hero_acts_first() {
            self.resolve_hero_action(action, &mut events);
            if self.is_finished() {
                return Ok(events);
            }

            self.resolve_enemy_action(&mut events);
        } else {
            self.resolve_enemy_action(&mut events);
            if self.is_finished() {
                return Ok(events);
            }

            self.resolve_hero_action(action, &mut events);
        }

        Ok(events)
    }

    fn resolve_hero_action(&mut self, action: Action, events: &mut Vec<CombatEvent>) {
        match action {
            Action::Attack => {
                let damage = calculate_damage(self.hero.stats().atk, self.enemy.stats().def);
                self.enemy.take_damage(damage);
                events.push(CombatEvent::DamageDealt {
                    source: Combatant::Hero,
                    target: Combatant::Enemy,
                    amount: damage,
                });

                if !self.enemy.is_alive() {
                    self.outcome = BattleOutcome::HeroWon;
                    events.push(CombatEvent::BattleEnded(self.outcome));
                }
            }
            Action::Defend => {
                self.hero_guarding = true;
                events.push(CombatEvent::GuardRaised {
                    target: Combatant::Hero,
                });
            }
            Action::Heal => {
                let _ = self.hero.spend_mp(HEAL_COST);
                let healed_amount = self.hero.heal(HEAL_AMOUNT);
                events.push(CombatEvent::Healed {
                    target: Combatant::Hero,
                    amount: healed_amount,
                });
            }
        }
    }

    fn resolve_enemy_action(&mut self, events: &mut Vec<CombatEvent>) {
        let mut damage = calculate_damage(self.enemy.stats().atk, self.hero.stats().def);

        if self.hero_guarding {
            self.hero_guarding = false;
            damage = (damage / 2).max(1);
            events.push(CombatEvent::GuardConsumed {
                target: Combatant::Hero,
            });
        }

        self.hero.take_damage(damage);
        events.push(CombatEvent::DamageDealt {
            source: Combatant::Enemy,
            target: Combatant::Hero,
            amount: damage,
        });

        if !self.hero.is_alive() {
            self.outcome = BattleOutcome::EnemyWon;
            events.push(CombatEvent::BattleEnded(self.outcome));
        }
    }
}

fn calculate_damage(attack: i32, defense: i32) -> i32 {
    (attack - defense).max(1)
}
