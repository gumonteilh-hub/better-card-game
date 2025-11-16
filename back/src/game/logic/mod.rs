use crate::{
    error::{Error, Result},
    game::{
        Game,
        action::Action,
        effects::{Effect, PlayerTarget, Target},
        types::{InstanceId, Location, PlayerId},
    },
};

mod summon;
mod draw;
mod damage;
mod destroy;
mod heal;
mod attack;
mod win;
mod mana;
mod boost;

pub fn execute_effect(effect: &Effect, context: &mut Game) -> Result<Vec<Action>> {
    let mut actions: Vec<Action> = Vec::new();
    match effect {
        Effect::MakeDraw {
            initiator,
            player,
            amount,
        } => {
            let make_draw_actions = draw::compute_make_draw(context, initiator, player, amount)?;
            actions.extend(make_draw_actions);
        }
        Effect::DealDamage {
            initiator,
            target,
            amount,
        } => {
            let deal_damage_actions = damage::compute(context, initiator, target, amount)?;
            actions.extend(deal_damage_actions);
        }
        Effect::Destroy { initiator, target } => {
            let destroy_actions = destroy::compute(context, initiator, target)?;
            actions.extend(destroy_actions);
        }
        Effect::Heal {
            initiator,
            target,
            amount,
        } => {
            let heal_actions = heal::compute(context, initiator, target, amount)?;
            actions.extend(heal_actions);
        }
        Effect::Attack { initiator, target } => {
            let attack_actions = attack::compute(context, initiator, target)?;
            actions.extend(attack_actions);
        }
        Effect::Win(player_id) => {
            let win_actions = win::compute(context, player_id)?;
            actions.extend(win_actions);
        }
        Effect::AutoDraw { player, amount } => {
            let auto_draw_actions = draw::compute_auto_draw(context, player, amount)?;
            actions.extend(auto_draw_actions);
        }
        Effect::IncreaseMaxMana {
            initiator,
            player,
            amount,
        } => {
            let increase_max_mana_actions =
                mana::compute_increase_max_mana(context, initiator, player, amount)?;
            actions.extend(increase_max_mana_actions);
        }
        Effect::RefreshMana {
            initiator,
            player,
            amount,
        } => {
            let refresh_mana_actions =
                mana::compute_refresh_mana(context, initiator, player, amount)?;
            actions.extend(refresh_mana_actions);
        }
        Effect::Boost {
            initiator,
            attack,
            hp,
            target,
        } => {
            let boost_actions = boost::compute(context, initiator, attack, hp, target)?;
            actions.extend(boost_actions);
        }
        Effect::Summon {
            initiator,
            side,
            target,
        } => {
            let summon_actions = summon::compute(context, initiator, side, target)?;
            actions.extend(summon_actions);
        }
    }

    Ok(actions)
}

fn is_player_id(id: usize) -> bool {
    id < 2
}

pub(super) fn resolve_target(
    initiator: InstanceId,
    target: &Target,
    context: &Game,
) -> Result<Vec<InstanceId>> {
    let mut targets = Vec::new();
    let mut player_targets = resolve_target_player_only(initiator, target, context)?;
    targets.append(&mut player_targets);
    let mut field_targets = resolve_field_target(initiator, target, context)?;
    targets.append(&mut field_targets);

    Ok(targets)
}
pub(super) fn resolve_target_player_only(
    initiator: InstanceId,
    target: &Target,
    context: &Game,
) -> Result<Vec<PlayerId>> {
    let player_side = if is_player_id(initiator) {
        initiator
    } else {
        context.get_entity(initiator)?.owner
    };

    let opponent_id = get_opponent_player_id(player_side, context)?;
    let targets = match target {
        Target::Player => vec![player_side],
        Target::EnnemyPlayer => vec![opponent_id],
        Target::BothPlayers | Target::All => vec![player_side, opponent_id],
        Target::Id(id) => {
            if context.players.contains_key(id) {
                vec![*id]
            } else {
                vec![]
            }
        }
        _ => vec![], // Not a player target
    };
    Ok(targets)
}

pub(super) fn resolve_player_target(
    initiator: InstanceId,
    target: &PlayerTarget,
    context: &Game,
) -> Result<Vec<PlayerId>> {
    let player_side = if is_player_id(initiator) {
        initiator
    } else {
        context.get_entity(initiator)?.owner
    };

    let opponent_id = get_opponent_player_id(player_side, context)?;
    let targets = match target {
        PlayerTarget::Player => vec![player_side],
        PlayerTarget::EnnemyPlayer => vec![opponent_id],
        PlayerTarget::BothPlayers => vec![player_side, opponent_id],
        PlayerTarget::Id(id) => {
            if context.players.contains_key(id) {
                vec![*id]
            } else {
                vec![]
            }
        }
    };
    Ok(targets)
}

pub(super) fn get_opponent_player_id(player_id: PlayerId, context: &Game) -> Result<PlayerId> {
    context
        .players
        .keys()
        .find(|p| **p != player_id)
        .copied()
        .ok_or_else(|| {
            Error::Game(format!(
                "Opponent not found for player with id {}",
                player_id
            ))
        })
}

pub(super) fn resolve_field_target(
    initiator: InstanceId,
    target: &Target,
    context: &Game,
) -> Result<Vec<InstanceId>> {
    let player_side = if is_player_id(initiator) {
        initiator
    } else {
        context.get_entity(initiator)?.owner
    };

    let opponent_id = get_opponent_player_id(player_side, context)?;
    let targets = match target {
        Target::ItSelf => vec![initiator],
        Target::Allies => context
            .get_field(player_side)
            .iter()
            .map(|e| e.1.id)
            .collect(),
        Target::Ennemies => context
            .get_field(opponent_id)
            .iter()
            .map(|e| e.1.id)
            .collect(),
        Target::AllMonsters => context
            .get_field(player_side)
            .iter()
            .map(|e| e.1.id)
            .chain(context.get_field(opponent_id).iter().map(|e| e.1.id))
            .collect(),
        Target::All => context
            .entities
            .values()
            .filter(|e| matches!(e.location, Location::Field(_)))
            .map(|e| e.id)
            .collect(),
        Target::Id(id) => {
            if context.entities.contains_key(id) {
                vec![*id]
            } else {
                vec![]
            }
        }
        Target::Ids(ids) => ids.clone(),
        _ => vec![], // Not an entity target
    };
    Ok(targets)
}
