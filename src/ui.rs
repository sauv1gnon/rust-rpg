use std::io::{self, Write};

use crate::combat::{Battle, CombatEvent, HEAL_COST};
use crate::model::{Action, BattleOutcome, Combatant};

pub enum MenuChoice {
    Start,
    Quit,
}

pub enum BattleChoice {
    Action(Action),
    Quit,
}

pub fn render_title() {
    println!();
    println!("========================================");
    println!("         SINGLE-ROOM BATTLE RPG        ");
    println!("========================================");
    println!("A plain-text medieval duel of steel and resolve.");
    println!();
}

pub fn prompt_title_choice() -> io::Result<MenuChoice> {
    loop {
        print!("Start a new battle? [s]tart / [q]uit: ");
        io::stdout().flush()?;

        match read_trimmed_line()?.to_lowercase().as_str() {
            "s" | "start" | "" => return Ok(MenuChoice::Start),
            "q" | "quit" => return Ok(MenuChoice::Quit),
            _ => println!("Please enter 's' to start or 'q' to quit."),
        }
    }
}

pub fn render_battle(battle: &Battle) {
    println!();
    println!("-- Battle --");
    println!(
        "Hero  HP {:>3}/{:<3}  MP {:>3}/{:<3}  ATK {:>2}  DEF {:>2}  SPD {:>2}",
        battle.hero.hp,
        battle.hero.stats.max_hp,
        battle.hero.mp,
        battle.hero.stats.max_mp,
        battle.hero.stats.atk,
        battle.hero.stats.def,
        battle.hero.stats.spd,
    );
    println!(
        "Enemy HP {:>3}/{:<3}  MP {:>3}/{:<3}  ATK {:>2}  DEF {:>2}  SPD {:>2}",
        battle.enemy.hp,
        battle.enemy.stats.max_hp,
        battle.enemy.mp,
        battle.enemy.stats.max_mp,
        battle.enemy.stats.atk,
        battle.enemy.stats.def,
        battle.enemy.stats.spd,
    );
    if battle.hero_guarding {
        println!("Your guard is raised.");
    }
    println!();
    println!("Choose an action:");
    println!("  [a]ttack");
    println!("  [d]efend");
    println!("  [h]eal (cost {HEAL_COST} MP)");
    println!("  [q]uit");
}

pub fn prompt_battle_choice() -> io::Result<BattleChoice> {
    loop {
        print!("Action: ");
        io::stdout().flush()?;

        match read_trimmed_line()?.to_lowercase().as_str() {
            "a" | "attack" => return Ok(BattleChoice::Action(Action::Attack)),
            "d" | "defend" => return Ok(BattleChoice::Action(Action::Defend)),
            "h" | "heal" => return Ok(BattleChoice::Action(Action::Heal)),
            "q" | "quit" => return Ok(BattleChoice::Quit),
            _ => println!("Please choose attack, defend, heal, or quit."),
        }
    }
}

pub fn render_events(events: &[CombatEvent]) {
    for event in events {
        match event {
            CombatEvent::GuardRaised {
                target: Combatant::Hero,
            } => println!("You raise your shield and brace for the next strike."),
            CombatEvent::GuardRaised { .. } => println!("The enemy braces itself."),
            CombatEvent::GuardConsumed {
                target: Combatant::Hero,
            } => println!("Your guard absorbs part of the blow."),
            CombatEvent::GuardConsumed { .. } => println!("A guard was consumed."),
            CombatEvent::DamageDealt {
                source: Combatant::Hero,
                amount,
                ..
            } => println!("You deal {amount} damage."),
            CombatEvent::DamageDealt {
                source: Combatant::Enemy,
                amount,
                ..
            } => println!("The enemy deals {amount} damage."),
            CombatEvent::Healed {
                target: Combatant::Hero,
                amount,
            } => println!("You recover {amount} HP."),
            CombatEvent::Healed { .. } => println!("Something was healed."),
            CombatEvent::BattleEnded(BattleOutcome::HeroWon) => println!("The enemy falls."),
            CombatEvent::BattleEnded(BattleOutcome::EnemyWon) => println!("You have been defeated."),
            CombatEvent::BattleEnded(BattleOutcome::InProgress) => {}
        }
    }
}

pub fn render_outcome(outcome: BattleOutcome) {
    println!();
    match outcome {
        BattleOutcome::HeroWon => println!("Victory is yours."),
        BattleOutcome::EnemyWon => println!("Defeat settles over the battlefield."),
        BattleOutcome::InProgress => {}
    }
}

pub fn prompt_return_to_title() -> io::Result<MenuChoice> {
    loop {
        print!("Return to the title screen? [s]tart / [q]uit: ");
        io::stdout().flush()?;

        match read_trimmed_line()?.to_lowercase().as_str() {
            "s" | "start" => return Ok(MenuChoice::Start),
            "q" | "quit" => return Ok(MenuChoice::Quit),
            _ => println!("Please enter 's' to restart or 'q' to quit."),
        }
    }
}

pub fn show_action_error(message: &str) {
    println!("{message}");
}

fn read_trimmed_line() -> io::Result<String> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}
