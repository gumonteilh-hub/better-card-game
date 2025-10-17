use crate::error::{Error, Result};
use crate::game::action::Action;
use crate::game::{types::PlayerId, Game};
use crate::ia::summon::summon_max_cards;

mod attack;
mod summon;

const HP_THRESHOLD: usize = 15; // 50% of max HP (30)
const MAX_HP: usize = 30;
const CREATURE_VALUE_ATK_WEIGHT: f32 = 1.0;
const CREATURE_VALUE_HP_WEIGHT: f32 = 1.0;
const SURVIVAL_BONUS: f32 = 5.0; // Bonus when our creature survives
const WOUNDED_ATTACKER_BONUS: f32 = 2.0; // Bonus for using wounded creatures
const FACE_DAMAGE_VALUE: f32 = 0.5; // Value of face damage in control mode
const MIN_ACCEPTABLE_TRADE_SCORE: f32 = -10.0; // Minimum score to accept a trade

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum IaBehavior {
    Control,
    Survival,
    Aggressive,
}

pub fn ai_play_turn(game_state: &mut Game, player_id: PlayerId) -> Result<Vec<Action>> {
    let mut all_actions = Vec::new();
    let behavior = determine_game_mode(&game_state, player_id)?;

    let mut attack_actions_1 = attack::ai_attack_sequence(game_state, player_id, behavior)?;
    all_actions.append(&mut attack_actions_1);

    let mut summon_actions = summon_max_cards(game_state, player_id)?;
    all_actions.append(&mut summon_actions);

    let mut attack_actions_2 = attack::ai_attack_sequence(game_state, player_id, behavior)?;
    all_actions.append(&mut attack_actions_2);

    let mut end_turn_actions = game_state.next_turn()?;
    all_actions.append(&mut end_turn_actions);

    Ok(all_actions)
}

fn determine_game_mode(game: &Game, player_id: PlayerId) -> Result<IaBehavior> {
    let ai_hp = game
        .players
        .get(&player_id)
        .ok_or_else(|| Error::Game(format!("AI player {} not found", player_id)))?
        .hp;

    let opponent_id = get_opponent_id(game, player_id)?;
    let opponent_hp = game
        .players
        .get(&opponent_id)
        .ok_or_else(|| Error::Game(format!("Opponent player {} not found", opponent_id)))?
        .hp;

    if opponent_hp <= HP_THRESHOLD {
        Ok(IaBehavior::Aggressive)
    } else if ai_hp <= HP_THRESHOLD {
        Ok(IaBehavior::Survival)
    } else {
        Ok(IaBehavior::Control)
    }
}

fn get_opponent_id(game: &Game, player_id: PlayerId) -> Result<PlayerId> {
    game.players
        .keys()
        .find(|&id| *id != player_id)
        .copied()
        .ok_or_else(|| Error::Game(format!("Opponent not found for player {}", player_id)))
}
