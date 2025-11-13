use crate::game::card::{CardTypeInstance, PassivBoost};

/// compute the passiv boost off the monsters on the board
/// this effect is triggered everytime the board is updated
/// when a monster is summoned/destroyed/moved
pub fn compute(context: &mut crate::Game) {
    for player in context.get_players() {
        for entity in context.get_mut_field(player.player_id).values_mut() {
            if let CardTypeInstance::Monster(monster) = &mut entity.card_type {
                monster.passiv_boost = PassivBoost::default();
            }
        }
    }

    for player in context.get_players() {
        for entity in context.get_field(player.player_id) {
            if let CardTypeInstance::Monster(monster) = entity.1.card_type {
                for effect in monster.passiv_effect {
                    
                }
            }

        }
    }
}
