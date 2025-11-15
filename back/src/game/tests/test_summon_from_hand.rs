// FUNCTIONAL RULES: SummonFromHand Effect
//
// 1. Summoning moves card from hand to field at specified position
// 2. On_play effects are triggered when monster is summoned
// 3. Newly summoned monsters are asleep (cannot attack this turn)
// 4. Monsters with Charge keyword can attack immediately (not asleep)

#[cfg(test)]
mod tests {
    use super::super::test_utils::{
        create_test_game, create_test_monster, create_test_monster_in_hand,
    };
    use crate::game::{
        card::{CardTypeInstance, Keyword},
        effects::{Effect, Target},
        types::Location,
    };

    #[test]
    fn test_summon_moves_card_from_hand_to_field_at_position() {
        let mut game = create_test_game();
        let player_a = game.player_id_a;

        let monster_id = create_test_monster_in_hand(&mut game, player_a, 3, 4, vec![], vec![]);
        game.players.get_mut(&player_a).unwrap().mana = 5;

        let monster = game.entities.get(&monster_id).unwrap();
        assert_eq!(monster.location, Location::Hand);

        game.play_monster(player_a, monster_id, 2, None).unwrap();
        game.compute_commands().unwrap();

        let monster = game.entities.get(&monster_id).unwrap();
        assert_eq!(monster.location, Location::Field(2));

        let field = game.get_field(player_a);
        assert!(field.contains_key(&2));
        assert_eq!(field.get(&2).unwrap().id, monster_id);
    }

    #[test]
    fn test_summon_triggers_on_play_effects() {
        let mut game = create_test_game();
        let player_a = game.player_id_a;
        let player_b = game.player_id_b;

        let enemy_monster_id = create_test_monster(&mut game, player_b, 1, 5, 5);
        let monster_with_on_play_id = create_test_monster_in_hand(
            &mut game,
            player_a,
            2,
            3,
            vec![],
            vec![Effect::DealDamage {
                initiator: 0,
                target: Target::Id(enemy_monster_id),
                amount: 2,
            }],
        );
        game.players.get_mut(&player_a).unwrap().mana = 5;

        game.play_monster(player_a, monster_with_on_play_id, 0, None)
            .unwrap();
        game.compute_commands().unwrap();

        let enemy_monster = game.entities.get(&enemy_monster_id).unwrap();
        match &enemy_monster.card_type {
            CardTypeInstance::Monster(monster_instance) => {
                assert_eq!(monster_instance.hp, 3);
            }
            _ => panic!("Expected monster"),
        }
    }

    #[test]
    fn test_newly_summoned_monsters_are_asleep() {
        let mut game = create_test_game();
        let player_a = game.player_id_a;
        let player_b = game.player_id_b;

        let monster_id = create_test_monster_in_hand(&mut game, player_a, 3, 4, vec![], vec![]);
        let enemy_monster_id = create_test_monster(&mut game, player_b, 1, 5, 5);
        game.players.get_mut(&player_a).unwrap().mana = 5;

        game.play_monster(player_a, monster_id, 0, None).unwrap();
        game.compute_commands().unwrap();

        {
            let monster = game.entities.get(&monster_id).unwrap();
            assert_eq!(monster.location, Location::Field(0));

            match &monster.card_type {
                CardTypeInstance::Monster(monster_instance) => {
                    assert_eq!(monster_instance.asleep, true);
                }
                _ => panic!("Expected monster"),
            }
        }

        let attack_result = game.attack(player_a, monster_id, enemy_monster_id);
        assert!(attack_result.is_err());
        assert_eq!(
            attack_result.unwrap_err().to_string(),
            "Game Logic Error: This monster can't attack on his first turn"
        );
    }

    #[test]
    fn test_monsters_with_charge_can_attack_immediately() {
        let mut game = create_test_game();
        let player_a = game.player_id_a;
        let player_b = game.player_id_b;

        let charge_monster_id =
            create_test_monster_in_hand(&mut game, player_a, 3, 4, vec![Keyword::Charge], vec![]);
        let enemy_monster_id = create_test_monster(&mut game, player_b, 1, 5, 5);
        game.players.get_mut(&player_a).unwrap().mana = 5;

        game.play_monster(player_a, charge_monster_id, 0, None).unwrap();
        game.compute_commands().unwrap();

        let monster = game.entities.get(&charge_monster_id).unwrap();
        assert_eq!(monster.location, Location::Field(0));

        match &monster.card_type {
            CardTypeInstance::Monster(monster_instance) => {
                assert_eq!(monster_instance.asleep, false);
                assert!(monster_instance.keywords.contains(&Keyword::Charge));
            }
            _ => panic!("Expected monster"),
        }

        let attack_result = game.attack(player_a, charge_monster_id, enemy_monster_id);
        assert!(attack_result.is_ok());

        game.compute_commands().unwrap();

        let enemy_monster = game.entities.get(&enemy_monster_id).unwrap();
        match &enemy_monster.card_type {
            CardTypeInstance::Monster(monster_instance) => {
                assert_eq!(monster_instance.hp, 2);
            }
            _ => panic!("Expected monster"),
        }
    }
}
