// FUNCTIONAL RULES: Heal Effect
//
// 1. Heal restores HP to the target (player or monster)
// 2. Healing cannot exceed the target's maximum HP
// 3. Healing a player at maximum HP has no effect
// 4. Healing a monster at maximum HP has no effect

#[cfg(test)]
mod tests {
    use super::super::test_utils::{create_test_game, create_test_monster, create_test_spell};
    use crate::game::{card::CardTypeInstance, effects::Effect};

    #[test]
    fn test_heal_restores_hp_to_damaged_player() {
        let mut game = create_test_game();
        let player_a = game.player_id_a;

        game.players.get_mut(&player_a).unwrap().hp = 20;
        game.players.get_mut(&player_a).unwrap().mana = 5;

        let heal_spell = create_test_spell(
            &mut game,
            player_a,
            vec![Effect::Heal {
                initiator: 0,
                target: crate::game::effects::Target::Player,
                amount: 5,
            }],
        );
        game.play_spell(player_a, heal_spell, None).unwrap();
        game.compute_commands().unwrap();

        assert_eq!(game.players.get(&player_a).unwrap().hp, 25);
    }

    #[test]
    fn test_heal_restores_hp_to_damaged_monster() {
        let mut game = create_test_game();
        let player_a = game.player_id_a;

        let monster_id = create_test_monster(&mut game, player_a, 0, 2, 10);
        game.players.get_mut(&player_a).unwrap().mana = 5;

        let heal_spell = create_test_spell(
            &mut game,
            player_a,
            vec![Effect::Heal {
                initiator: 0,
                target: crate::game::effects::Target::Id(monster_id),
                amount: 3,
            }],
        );
        game.play_spell(player_a, heal_spell, None).unwrap();
        game.compute_commands().unwrap();

        let monster = game.entities.get(&monster_id).unwrap();
        match &monster.card_type {
            CardTypeInstance::Monster(monster_instance) => {
                assert_eq!(monster_instance.hp, 5);
            }
            _ => panic!("Expected monster"),
        }
    }

    #[test]
    fn test_heal_cannot_exceed_player_maximum_hp() {
        let mut game = create_test_game();
        let player_a = game.player_id_a;

        game.players.get_mut(&player_a).unwrap().hp = 25;
        game.players.get_mut(&player_a).unwrap().mana = 5;

        let heal_spell = create_test_spell(
            &mut game,
            player_a,
            vec![Effect::Heal {
                initiator: 0,
                target: crate::game::effects::Target::Player,
                amount: 10,
            }],
        );
        game.play_spell(player_a, heal_spell, None).unwrap();
        game.compute_commands().unwrap();

        assert_eq!(game.players.get(&player_a).unwrap().hp, 30);
    }

    #[test]
    fn test_heal_cannot_exceed_monster_maximum_hp() {
        let mut game = create_test_game();
        let player_a = game.player_id_a;

        let monster_id = create_test_monster(&mut game, player_a, 0, 3, 5);
        game.players.get_mut(&player_a).unwrap().mana = 5;

        let heal_spell = create_test_spell(
            &mut game,
            player_a,
            vec![Effect::Heal {
                initiator: 0,
                target: crate::game::effects::Target::Id(monster_id),
                amount: 10,
            }],
        );
        game.play_spell(player_a, heal_spell, None).unwrap();
        game.compute_commands().unwrap();

        let monster = game.entities.get(&monster_id).unwrap();
        match &monster.card_type {
            CardTypeInstance::Monster(monster_instance) => {
                assert_eq!(monster_instance.hp, 5);
                assert_eq!(monster_instance.max_hp, 5);
            }
            _ => panic!("Expected monster"),
        }
    }
}
