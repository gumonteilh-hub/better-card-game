use crate::{
    Game,
    error::{Error, Result},
    game::{
        action::Action,
        card::{self, CardTypeInstance},
        effects::{self, Effect},
        types::PlayerId,
    },
    ia,
};

pub fn end_turn(context: &mut Game, ending_player_id: PlayerId) -> Result<Vec<Action>> {
    let mut actions = Vec::new();

    let mut ending_effects = Vec::new();
    for (card_id, monster) in context.get_field(ending_player_id) {
        if let CardTypeInstance::Monster(monster) = &monster.card_type
            && !monster.on_turn_end.is_empty()
        {
            actions.push(Action::TriggerOnTurnEnd(*card_id));
            ending_effects.extend(monster.on_turn_end.clone());
        }
    }
    context.effect_queue.extend(ending_effects);
    actions.push(Action::EndTurn(ending_player_id));

    let starting_player_id = context.get_opponent(&ending_player_id)?.player_id;

    actions.push(Action::StartTurn(starting_player_id));
    let mut starting_effects = Vec::new();
    for (card_id, monster) in context.get_field(starting_player_id) {
        if let CardTypeInstance::Monster(monster) = &monster.card_type
            && !monster.on_turn_start.is_empty()
        {
            actions.push(Action::TriggerOnTurnStart(*card_id));
            starting_effects.extend(monster.on_turn_start.clone());
        }
    }
    context.effect_queue.extend(starting_effects);

    context.current_player = starting_player_id;
    context.effect_queue.push_back(Effect::AutoDraw {
        player: starting_player_id,
        amount: 1,
    });

    let current_player_instance = context.get_mut_player(starting_player_id)?;

    if current_player_instance.max_mana < current_player_instance.absolute_max_mana {
        context.effect_queue.push_back(Effect::IncreaseMaxMana {
            initiator: starting_player_id,
            player: effects::PlayerTarget::Player,
            amount: 1,
        });
    }

    let max_mana = context.get_player(context.current_player)?.max_mana;
    context.effect_queue.push_back(Effect::RefreshMana {
        initiator: starting_player_id,
        player: effects::PlayerTarget::Player,
        amount: max_mana + 1,
    });

    context.get_mut_player(starting_player_id)?.move_count = 3;

    for (_, monster) in context.get_mut_field(starting_player_id) {
        match &mut monster.card_type {
            card::CardTypeInstance::Monster(monster_instance) => {
                monster_instance.attack_count = 0;
                monster_instance.asleep = false;
            }
            card::CardTypeInstance::Spell(spell_instance) => {
                return Err(Error::Game("There shouldn't be spell on the field".into()));
            }
        }
    }

    let mut reset_turn_actions = context.compute_commands()?;
    actions.append(&mut reset_turn_actions);

    if starting_player_id == context.player_id_b && context.vs_ia {
        let mut ia_actions = ia::ai_play_turn(context, context.player_id_b)?;
        actions.append(&mut ia_actions);
    }
    Ok(actions)
}
