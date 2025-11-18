use crate::{
    Game,
    error::{Error, Result},
    game::{
        action::Action,
        card,
        effects::{self, Effect},
        types::PlayerId,
    },
    ia,
};

pub fn end_turn(context: &mut Game, ending_player: PlayerId) -> Result<Vec<Action>> {
    let mut actions = Vec::new();
    let starting_player = *context
        .players
        .keys()
        .find(|p| **p != ending_player)
        .unwrap();

    actions.push(Action::StartTurn(starting_player));
    context.current_player = starting_player;
    context.effect_queue.push_back(Effect::AutoDraw {
        player: starting_player,
        amount: 1,
    });

    let current_player_instance = context.get_mut_player(starting_player)?;

    if current_player_instance.max_mana < current_player_instance.absolute_max_mana {
        context.effect_queue.push_back(Effect::IncreaseMaxMana {
            initiator: starting_player,
            player: effects::PlayerTarget::Player,
            amount: 1,
        });
    }

    let max_mana = context.get_player(context.current_player)?.max_mana;
    context.effect_queue.push_back(Effect::RefreshMana {
        initiator: starting_player,
        player: effects::PlayerTarget::Player,
        amount: max_mana + 1,
    });

    context.get_mut_player(starting_player)?.move_count = 3;

    for (_, monster) in context.get_mut_field(starting_player) {
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

    if starting_player == context.player_id_b && context.vs_ia {
        let mut ia_actions = ia::ai_play_turn(context, context.player_id_b)?;
        actions.append(&mut ia_actions);
    }
    Ok(actions)
}
