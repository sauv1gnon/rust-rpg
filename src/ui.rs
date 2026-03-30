use std::io::{self, Write};

use crate::combat::{Battle, CombatEvent, HEAL_COST};
use crate::model::{Action, BattleOutcome, Combatant, CombatantState, Encounter, Item, Player, ShopOffer};
use crate::state::GameOverReason;

pub enum TitleChoice {
    Start,
    Quit,
}

pub use TitleChoice as MenuChoice;

pub enum ExplorationChoice {
    Battle,
    Shop,
    Stats,
    Save,
    Quit,
}

pub enum BattleChoice {
    Action(Action),
    Quit,
}

pub enum ShopChoice {
    Buy(usize),
    Leave,
}

pub enum GameoverChoice {
    Restart,
    Quit,
}

pub fn render_title() {
    println!();
    println!("========================================");
    println!("           RUST RPG: FRONTIER          ");
    println!("========================================");
    println!("A plain-text adventure of battles, trade, and momentum.");
    println!();
}

pub fn prompt_title_choice() -> io::Result<TitleChoice> {
    loop {
        print!("Start a new adventure? [s]tart / [q]uit: ");
        io::stdout().flush()?;

        match read_trimmed_line()?.to_lowercase().as_str() {
            "s" | "start" | "" => return Ok(TitleChoice::Start),
            "q" | "quit" => return Ok(TitleChoice::Quit),
            _ => println!("Please enter 's' to start or 'q' to quit."),
        }
    }
}

#[allow(dead_code)]
pub fn prompt_return_to_title() -> io::Result<MenuChoice> {
    match prompt_title_choice()? {
        TitleChoice::Start => Ok(MenuChoice::Start),
        TitleChoice::Quit => Ok(MenuChoice::Quit),
    }
}

pub fn render_exploration(player: &Player, encounter: &Encounter) {
    println!();
    println!("-- Exploration --");
    println!("Current hero: {}", player.character.name);
    println!(
        "HP {:>3}/{:<3}  MP {:>3}/{:<3}  ATK {:>2}  DEF {:>2}  SPD {:>2}",
        player.character.hp,
        player.character.stats.max_hp,
        player.character.mp,
        player.character.stats.max_mp,
        player.character.stats.atk,
        player.character.stats.def,
        player.character.stats.spd,
    );
    println!("Gold: {}  EXP: {}", player.gold, player.experience);
    println!("Inventory: {}", inventory_summary(&player.inventory));
    println!("Next encounter: {}", encounter.name);
    println!();
    println!("Choose your next move:");
    println!("  [b]attle again");
    println!("  [s]hop");
    println!("  [v]iew stats");
    println!("  [e]xport save placeholder");
    println!("  [q]uit adventure");
}

pub fn prompt_exploration_choice() -> io::Result<ExplorationChoice> {
    loop {
        print!("Choice: ");
        io::stdout().flush()?;

        match read_trimmed_line()?.to_lowercase().as_str() {
            "b" | "battle" => return Ok(ExplorationChoice::Battle),
            "s" | "shop" => return Ok(ExplorationChoice::Shop),
            "v" | "view" | "stats" => return Ok(ExplorationChoice::Stats),
            "e" | "save" => return Ok(ExplorationChoice::Save),
            "q" | "quit" => return Ok(ExplorationChoice::Quit),
            _ => println!("Please choose battle, shop, stats, save, or quit."),
        }
    }
}

pub fn render_stats(player: &Player) {
    println!();
    println!("-- Stats --");
    println!("Name: {}", player.character.name);
    println!("Gold: {}", player.gold);
    println!("EXP: {}", player.experience);
    println!("Inventory: {}", inventory_summary(&player.inventory));
    println!(
        "HP {:>3}/{:<3}  MP {:>3}/{:<3}  ATK {:>2}  DEF {:>2}  SPD {:>2}",
        player.character.hp,
        player.character.stats.max_hp,
        player.character.mp,
        player.character.stats.max_mp,
        player.character.stats.atk,
        player.character.stats.def,
        player.character.stats.spd,
    );
}

pub fn prompt_continue(message: &str) -> io::Result<()> {
    println!("{message}");
    let _ = read_trimmed_line()?;
    Ok(())
}

pub fn render_battle<Hero, Enemy>(battle: &Battle<Hero, Enemy>)
where
    Hero: CombatantState,
    Enemy: CombatantState,
{
    println!();
    println!("-- Battle --");
    println!(
        "Hero  HP {:>3}/{:<3}  MP {:>3}/{:<3}  ATK {:>2}  DEF {:>2}  SPD {:>2}",
        battle.hero.hp(),
        battle.hero.stats().max_hp,
        battle.hero.mp(),
        battle.hero.stats().max_mp,
        battle.hero.stats().atk,
        battle.hero.stats().def,
        battle.hero.stats().spd,
    );
    println!(
        "Enemy {} HP {:>3}/{:<3}  MP {:>3}/{:<3}  ATK {:>2}  DEF {:>2}  SPD {:>2}",
        battle.enemy.name(),
        battle.enemy.hp(),
        battle.enemy.stats().max_hp,
        battle.enemy.mp(),
        battle.enemy.stats().max_mp,
        battle.enemy.stats().atk,
        battle.enemy.stats().def,
        battle.enemy.stats().spd,
    );
    if battle.hero_guarding {
        println!("Your guard is raised.");
    }
    println!();
    println!("Choose an action:");
    println!("  [a]ttack");
    println!("  [d]efend");
    println!("  [h]eal (cost {HEAL_COST} MP)");
    println!("  [q]uit battle");
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

pub fn render_reward(player: &Player, encounter: &Encounter) {
    println!();
    println!("-- Reward --");
    println!("You defeated {}.", encounter.name);
    println!("Reward: +{} gold, +{} EXP", encounter.reward.gold, encounter.reward.exp);
    if let Some(item) = encounter.reward.item_drop {
        println!("Item drop: {item}");
    } else {
        println!("Item drop: none");
    }
    println!("Total gold: {}", player.gold);
    println!("Total EXP: {}", player.experience);
    println!("Inventory: {}", inventory_summary(&player.inventory));
}

pub fn render_shop(player: &Player, offers: &[ShopOffer]) {
    println!();
    println!("-- Shop --");
    println!("Gold: {}", player.gold);
    println!("Inventory: {}", inventory_summary(&player.inventory));
    println!();
    println!("Available goods:");
    for (index, offer) in offers.iter().enumerate() {
        println!("  [{}] {} - {} gold", index + 1, offer.item, offer.price);
    }
    println!("  [l]eave shop");
}

pub fn prompt_shop_choice(offer_count: usize) -> io::Result<ShopChoice> {
    loop {
        print!("Purchase choice: ");
        io::stdout().flush()?;

        let input = read_trimmed_line()?.to_lowercase();

        if matches!(input.as_str(), "l" | "leave" | "q" | "quit") {
            return Ok(ShopChoice::Leave);
        }

        if let Ok(choice) = input.parse::<usize>() {
            if (1..=offer_count).contains(&choice) {
                return Ok(ShopChoice::Buy(choice - 1));
            }
        }

        println!("Choose an item number or leave the shop.");
    }
}

pub fn render_gameover(reason: GameOverReason, player: &Player) {
    println!();
    println!("-- Game Over --");
    match reason {
        GameOverReason::Defeated => println!("Your journey ends in defeat."),
        GameOverReason::Retired => println!("You leave the road behind."),
    }
    println!("Gold: {}", player.gold);
    println!("EXP: {}", player.experience);
    println!("Inventory: {}", inventory_summary(&player.inventory));
}

pub fn prompt_gameover_choice() -> io::Result<GameoverChoice> {
    loop {
        print!("Restart adventure? [r]estart / [q]uit: ");
        io::stdout().flush()?;

        match read_trimmed_line()?.to_lowercase().as_str() {
            "r" | "restart" => return Ok(GameoverChoice::Restart),
            "q" | "quit" => return Ok(GameoverChoice::Quit),
            _ => println!("Please enter 'r' to restart or 'q' to quit."),
        }
    }
}

pub fn show_message(message: &str) {
    println!("{message}");
}

#[allow(dead_code)]
pub fn show_action_error(message: &str) {
    show_message(message);
}

#[allow(dead_code)]
pub fn render_outcome(outcome: BattleOutcome) {
    println!();
    match outcome {
        BattleOutcome::HeroWon => println!("Victory is yours."),
        BattleOutcome::EnemyWon => println!("Defeat settles over the battlefield."),
        BattleOutcome::InProgress => {}
    }
}

fn inventory_summary(items: &[Item]) -> String {
    if items.is_empty() {
        return String::from("Empty");
    }

    let mut counts: Vec<(Item, usize)> = Vec::new();

    for item in items {
        if let Some((_, quantity)) = counts.iter_mut().find(|(existing, _)| existing == item) {
            *quantity += 1;
        } else {
            counts.push((*item, 1));
        }
    }

    counts
        .into_iter()
        .map(|(item, quantity)| {
            if quantity == 1 {
                item.to_string()
            } else {
                format!("{} x{}", item, quantity)
            }
        })
        .collect::<Vec<_>>()
        .join(", ")
}

fn read_trimmed_line() -> io::Result<String> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}
