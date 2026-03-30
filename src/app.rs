use std::io;

use crate::combat::{Battle, CombatError};
use crate::model::{BattleOutcome, Encounter, Player};
use crate::state::{shop_offers, GameOverReason, GamePhase, GameState};
use crate::ui::{self, BattleChoice, ExplorationChoice, GameoverChoice, ShopChoice, TitleChoice};

pub fn run() -> io::Result<()> {
    let mut game = GameState::new();

    loop {
        match game.phase {
            GamePhase::Title => {
                ui::render_title();

                match ui::prompt_title_choice()? {
                    TitleChoice::Start => game.start_adventure(),
                    TitleChoice::Quit => return Ok(()),
                }
            }
            GamePhase::Exploration => {
                let encounter = game.current_encounter().clone();
                ui::render_exploration(&game.player, &encounter);

                match ui::prompt_exploration_choice()? {
                    ExplorationChoice::Battle => game.phase = GamePhase::Battle,
                    ExplorationChoice::Shop => game.phase = GamePhase::Shop,
                    ExplorationChoice::Stats => {
                        ui::render_stats(&game.player);
                        ui::prompt_continue("Press Enter to return to exploration.")?;
                    }
                    ExplorationChoice::Save => {
                        ui::show_message("Save is not implemented yet.");
                        ui::prompt_continue("Press Enter to return to exploration.")?;
                    }
                    ExplorationChoice::Quit => game.set_game_over(GameOverReason::Retired),
                }
            }
            GamePhase::Battle => {
                let encounter = game.current_encounter().clone();

                match play_battle(&mut game.player, encounter)? {
                    BattleExit::Victory => game.phase = GamePhase::Reward,
                    BattleExit::Defeat => game.set_game_over(GameOverReason::Defeated),
                    BattleExit::Retreat => game.phase = GamePhase::Exploration,
                }
            }
            GamePhase::Reward => {
                let encounter = game.current_encounter().clone();
                apply_reward(&mut game.player, &encounter);
                ui::render_reward(&game.player, &encounter);
                ui::prompt_continue("Press Enter to continue to the next encounter.")?;
                game.prepare_next_encounter();
                game.phase = GamePhase::Exploration;
            }
            GamePhase::Shop => {
                run_shop(&mut game)?;
                game.phase = GamePhase::Exploration;
            }
            GamePhase::Gameover => {
                let reason = game.game_over_reason.unwrap_or(GameOverReason::Defeated);
                ui::render_gameover(reason, &game.player);

                match ui::prompt_gameover_choice()? {
                    GameoverChoice::Restart => game = GameState::new(),
                    GameoverChoice::Quit => return Ok(()),
                }
            }
        }
    }
}

enum BattleExit {
    Victory,
    Defeat,
    Retreat,
}

fn play_battle(player: &mut Player, encounter: Encounter) -> io::Result<BattleExit> {
    let mut battle = Battle::new(player.character.clone(), encounter.enemy);

    loop {
        ui::render_battle(&battle);

        match ui::prompt_battle_choice()? {
            BattleChoice::Quit => {
                player.character = battle.hero;
                return Ok(BattleExit::Retreat);
            }
            BattleChoice::Action(action) => match battle.step(action) {
                Ok(events) => {
                    ui::render_events(&events);

                    if battle.is_finished() {
                        player.character = battle.hero;
                        return Ok(match battle.outcome {
                            BattleOutcome::HeroWon => BattleExit::Victory,
                            BattleOutcome::EnemyWon => BattleExit::Defeat,
                            BattleOutcome::InProgress => BattleExit::Retreat,
                        });
                    }
                }
                Err(CombatError::NotEnoughMp { cost, current_mp }) => {
                    ui::show_message(&format!(
                        "You need {cost} MP for that action, but you only have {current_mp}."
                    ));
                }
                Err(CombatError::BattleAlreadyFinished) => {
                    player.character = battle.hero;
                    return Ok(match battle.outcome {
                        BattleOutcome::HeroWon => BattleExit::Victory,
                        BattleOutcome::EnemyWon => BattleExit::Defeat,
                        BattleOutcome::InProgress => BattleExit::Retreat,
                    });
                }
            },
        }
    }
}

fn apply_reward(player: &mut Player, encounter: &Encounter) {
    player.apply_reward(encounter.reward);
}

fn run_shop(game: &mut GameState) -> io::Result<()> {
    let offers = shop_offers();

    loop {
        ui::render_shop(&game.player, &offers);

        match ui::prompt_shop_choice(offers.len())? {
            ShopChoice::Leave => return Ok(()),
            ShopChoice::Buy(index) => {
                let offer = offers[index];

                if game.player.spend_gold(offer.price) {
                    game.player.add_item(offer.item);
                    ui::show_message(&format!("You bought a {}.", offer.item));
                } else {
                    ui::show_message(&format!("You need {} gold for that item.", offer.price));
                }
            }
        }
    }
}
