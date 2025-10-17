use crate::{
    error::{Error, Result},
    game::{
        action::Action,
        card::{CardInstance, Keyword},
        types::{EntityId, PlayerId},
        Game,
    },
    ia::{
        get_opponent_id, IaBehavior, CREATURE_VALUE_ATK_WEIGHT, CREATURE_VALUE_HP_WEIGHT,
        FACE_DAMAGE_VALUE, MIN_ACCEPTABLE_TRADE_SCORE, SURVIVAL_BONUS, WOUNDED_ATTACKER_BONUS,
    },
};

pub fn ai_attack_sequence(
    game: &mut Game,
    player_id: PlayerId,
    mode: IaBehavior,
) -> Result<Vec<Action>> {
    execute_greedy_attack_strategy(game, player_id, mode)
}

fn get_available_attackers(game: &Game, player_id: PlayerId) -> Vec<EntityId> {
    game.get_field(player_id)
        .into_iter()
        .filter(|(_, creature)| {
            if creature.asleep {
                return false;
            }

            if creature.template.keywords.contains(&Keyword::Windfury) {
                creature.attack_count < 2
            } else {
                creature.attack_count < 1
            }
        })
        .map(|(id, _)| *id)
        .collect()
}

fn has_taunt_on_field(game: &Game, player_id: PlayerId) -> bool {
    game.get_field(player_id)
        .values()
        .any(|creature| creature.template.keywords.contains(&Keyword::Taunt))
}

fn evaluate_attack_control(attacker: &CardInstance, target: &CardInstance) -> f32 {
    let mut score = 0.0;

    let damage_dealt = attacker.atk.min(target.hp);
    score += damage_dealt as f32 * CREATURE_VALUE_HP_WEIGHT;

    if damage_dealt >= target.hp {
        score += target.atk as f32 * CREATURE_VALUE_ATK_WEIGHT;
    }

    let damage_received = target.atk.min(attacker.hp);
    score -= damage_received as f32 * CREATURE_VALUE_HP_WEIGHT;

    if damage_received >= attacker.hp {
        score -= attacker.atk as f32 * CREATURE_VALUE_ATK_WEIGHT;
    } else {
        score += SURVIVAL_BONUS;
    }

    let attacker_missing_hp = attacker.template.base_hp.saturating_sub(attacker.hp);
    if attacker_missing_hp > 0 {
        score += WOUNDED_ATTACKER_BONUS * attacker_missing_hp as f32;
    }

    score
}

fn evaluate_attack_survival(attacker: &CardInstance, target: &CardInstance) -> f32 {
    let mut score = 0.0;

    let damage_dealt = attacker.atk.min(target.hp);
    score += damage_dealt as f32 * 10.0;

    if damage_dealt >= target.hp {
        score += 100.0;
        score += target.atk as f32 * 5.0;
    }

    score
}

fn evaluate_attack_aggressive(attacker: &CardInstance, target: &CardInstance) -> f32 {
    let mut score = 0.0;

    let damage_dealt = attacker.atk.min(target.hp);

    if damage_dealt >= target.hp {
        score += 50.0;
    } else {
        score += damage_dealt as f32;
    }

    let overkill = attacker.atk.saturating_sub(target.hp);
    score -= overkill as f32 * 0.5;

    score
}

fn calculate_threat_level(game: &Game, player_id: PlayerId) -> Result<f32> {
    let opponent_id = get_opponent_id(game, player_id)?;
    let ai_hp = game
        .players
        .get(&player_id)
        .ok_or_else(|| Error::Game(format!("AI player {} not found", player_id)))?
        .hp;

    let enemy_total_atk: usize = game.get_field(opponent_id).values().map(|c| c.atk).sum();

    let threat = enemy_total_atk as f32 / ai_hp.max(1) as f32;

    Ok(threat)
}

fn find_best_attack(
    game: &Game,
    player_id: PlayerId,
    mode: IaBehavior,
) -> Result<Option<(EntityId, EntityId, f32)>> {
    let opponent_id = get_opponent_id(game, player_id)?;
    let attackers = get_available_attackers(game, player_id);

    if attackers.is_empty() {
        return Ok(None);
    }

    let enemy_field = game.get_field(opponent_id);
    let has_taunt = has_taunt_on_field(game, opponent_id);

    let mut best_attack: Option<(EntityId, EntityId, f32)> = None;

    for attacker_id in attackers {
        let attacker = game.get_entity(attacker_id)?;

        if !has_taunt {
            let score = match mode {
                IaBehavior::Control => {
                    let threat = calculate_threat_level(game, player_id)?;

                    let threat_penalty = (1.0 - threat).max(0.0);
                    let adjusted_face_value = FACE_DAMAGE_VALUE * threat_penalty;

                    adjusted_face_value * attacker.atk as f32
                }
                IaBehavior::Survival => -1000.0,
                IaBehavior::Aggressive => attacker.atk as f32 * 100.0,
            };

            if best_attack.is_none() || score > best_attack.unwrap().2 {
                best_attack = Some((attacker_id, opponent_id, score));
            }
        }

        for enemy in enemy_field.values() {
            if has_taunt && !enemy.template.keywords.contains(&Keyword::Taunt) {
                continue;
            }

            let score = match mode {
                IaBehavior::Control => evaluate_attack_control(attacker, enemy),
                IaBehavior::Survival => evaluate_attack_survival(attacker, enemy),
                IaBehavior::Aggressive => evaluate_attack_aggressive(attacker, enemy),
            };

            if best_attack.is_none() || score > best_attack.unwrap().2 {
                best_attack = Some((attacker_id, enemy.id, score));
            }
        }
    }

    if let Some((_, _, score)) = best_attack {
        if mode == IaBehavior::Control && score < MIN_ACCEPTABLE_TRADE_SCORE {
            return Ok(None);
        }
    }

    Ok(best_attack)
}

fn execute_greedy_attack_strategy(
    game: &mut Game,
    player_id: PlayerId,
    mode: IaBehavior,
) -> Result<Vec<Action>> {
    let mut all_actions = Vec::new();

    loop {
        let best_attack = find_best_attack(game, player_id, mode)?;

        match best_attack {
            Some((attacker_id, target_id, _score)) => {
                game.attack(attacker_id, target_id)?;

                let mut actions = game.compute_commands()?;
                all_actions.append(&mut actions);
            }
            None => {
                break;
            }
        }
    }

    Ok(all_actions)
}
