use crate::model::{Character, Encounter, Item, Player, Reward, ShopOffer, Stats};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GamePhase {
    Title,
    Exploration,
    Battle,
    Reward,
    Shop,
    Gameover,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GameOverReason {
    Defeated,
    Retired,
}

#[derive(Debug)]
pub struct GameState {
    pub phase: GamePhase,
    pub player: Player,
    pub encounter_index: usize,
    pub current_encounter: Option<Encounter>,
    pub game_over_reason: Option<GameOverReason>,
}

impl Default for GameState {
    fn default() -> Self {
        Self::new()
    }
}

impl GameState {
    pub fn new() -> Self {
        Self {
            phase: GamePhase::Title,
            player: default_player(),
            encounter_index: 0,
            current_encounter: None,
            game_over_reason: None,
        }
    }

    pub fn start_adventure(&mut self) {
        self.player = default_player();
        self.encounter_index = 0;
        self.current_encounter = Some(encounter_for_index(0));
        self.game_over_reason = None;
        self.phase = GamePhase::Exploration;
    }

    pub fn current_encounter(&mut self) -> &Encounter {
        self.current_encounter
            .get_or_insert_with(|| encounter_for_index(self.encounter_index))
    }

    pub fn clear_current_encounter(&mut self) {
        self.current_encounter = None;
    }

    pub fn prepare_next_encounter(&mut self) {
        self.encounter_index = self.encounter_index.saturating_add(1);
        self.current_encounter = Some(encounter_for_index(self.encounter_index));
    }

    pub fn set_game_over(&mut self, reason: GameOverReason) {
        self.game_over_reason = Some(reason);
        self.phase = GamePhase::Gameover;
    }
}

pub fn default_player() -> Player {
    Player::new(Character::new("Player One", Stats::new(32, 12, 8, 4, 6)))
}

pub fn encounter_for_index(index: usize) -> Encounter {
    let tier = index as i32;
    let enemy = Character::new(
        format!("Wandering Hollow Stalker {number}", number = index + 1),
        Stats::new(24 + tier * 4, 0, 6 + tier * 2, 2 + tier, 4 + tier),
    );

    let reward = Reward::new(
        8 + tier * 4,
        6 + tier * 3,
        if index % 2 == 0 {
            Some(Item::Potion)
        } else {
            Some(Item::Tonic)
        },
    );

    Encounter::new(format!("Encounter {number}", number = index + 1), enemy, reward)
}

pub fn shop_offers() -> Vec<ShopOffer> {
    vec![
        ShopOffer::new(Item::Potion, 6),
        ShopOffer::new(Item::Ether, 9),
        ShopOffer::new(Item::Tonic, 12),
    ]
}