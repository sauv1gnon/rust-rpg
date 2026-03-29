use std::io;

use crate::combat::{Battle, CombatError};
use crate::model::{BattleOutcome, Character, Stats};
use crate::ui::{self, BattleChoice, MenuChoice};

pub fn run() -> io::Result<()> {
    loop {
        ui::render_title();

        match ui::prompt_title_choice()? {
            MenuChoice::Quit => return Ok(()),
            MenuChoice::Start => {
                let Some(outcome) = play_battle()? else {
                    return Ok(());
                };

                ui::render_outcome(outcome);

                match ui::prompt_return_to_title()? {
                    MenuChoice::Start => continue,
                    MenuChoice::Quit => return Ok(()),
                }
            }
        }
    }
}

fn play_battle() -> io::Result<Option<BattleOutcome>> {
    let mut battle = Battle::new(create_hero(), create_enemy());

    loop {
        ui::render_battle(&battle);

        match ui::prompt_battle_choice()? {
            BattleChoice::Quit => return Ok(None),
            BattleChoice::Action(action) => match battle.step(action) {
                Ok(events) => {
                    ui::render_events(&events);

                    if battle.is_finished() {
                        return Ok(Some(battle.outcome));
                    }
                }
                Err(CombatError::NotEnoughMp { cost, current_mp }) => {
                    ui::show_action_error(&format!(
                        "You need {cost} MP for that action, but you only have {current_mp}."
                    ));
                }
                Err(CombatError::BattleAlreadyFinished) => {
                    return Ok(Some(battle.outcome));
                }
            },
        }
    }
}

fn create_hero() -> Character {
    Character::new("Player One", Stats::new(32, 12, 8, 4, 6))
}

fn create_enemy() -> Character {
    Character::new("The Unyielding Banshee of the Old War", Stats::new(24, 0, 6, 2, 4))
}
