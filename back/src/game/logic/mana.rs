use crate::error::Result;
use crate::game::action::Action;
use crate::game::effects::PlayerTarget;
use crate::game::types::InstanceId;

pub fn compute_increase_max_mana(
    context: &mut crate::Game,
    initiator: &InstanceId,
    player: &PlayerTarget,
    amount: &usize,
) -> Result<Vec<Action>> {
    let mut actions = Vec::new();
    let targets = super::resolve_player_target(*initiator, player, context)?;

    for target in targets {
        context.get_mut_player(target)?.base_mana += amount;
        actions.push(Action::IncreaseMaxMana {
            player: target,
            amount: *amount,
        });
    }

    Ok(actions)
}

pub fn compute_refresh_mana(
    context: &mut crate::Game,
    initiator: &InstanceId,
    player: &PlayerTarget,
    amount: &usize,
) -> Result<Vec<Action>> {
    let mut actions = Vec::new();
    let targets = super::resolve_player_target(*initiator, player, context)?;

    for target in targets {
        let player = context.get_mut_player(target)?;
        let effective_refresh;
        if player.mana + amount >= player.base_mana {
            effective_refresh = player.base_mana - player.mana;
            player.mana = player.base_mana;
        } else {
            effective_refresh = *amount;
            player.mana += amount;
        }

        actions.push(Action::RefreshMana {
            player: target,
            amount: effective_refresh,
        });
    }

    Ok(actions)
}
