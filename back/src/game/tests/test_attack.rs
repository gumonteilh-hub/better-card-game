// FUNCTIONAL RULES: Attack Effect
//
// 1. Monster attacking monster deals mutual damage (both take damage)
// 2. Monster attacking player deals one-way damage (only player takes damage)
// 3. On_attack effects trigger before damage is dealt
// 4. Monsters reduced to 0 HP are destroyed

#[cfg(test)]
mod tests {
    use super::super::test_utils::{
        create_test_game, create_test_monster_with_attack, create_test_monster_with_on_attack,
    };
    use crate::game::card::CardTypeInstance;
    use crate::game::effects::Effect;
    use crate::game::effects::Target;
    use crate::game::types::Location;

    #[test]
    fn test_monster_vs_monster_mutual_damage() {
        let mut game = create_test_game();
        let player_a = game.player_id_a;
        let player_b = game.player_id_b;

        let monster_a = create_test_monster_with_attack(&mut game, player_a, 0, 5, 10, 10);
        let monster_b = create_test_monster_with_attack(&mut game, player_b, 1, 3, 8, 8);

        let monster_a_instance = game.entities.get(&monster_a).unwrap();
        let monster_b_instance = game.entities.get(&monster_b).unwrap();

        if let CardTypeInstance::Monster(ma) = &monster_a_instance.card_type {
            assert_eq!(ma.hp, 10);
            assert_eq!(ma.attack, 5);
        }
        if let CardTypeInstance::Monster(mb) = &monster_b_instance.card_type {
            assert_eq!(mb.hp, 8);
            assert_eq!(mb.attack, 3);
        }

        game.attack(monster_a, monster_b).unwrap();
        game.compute_commands().unwrap();

        let monster_a_instance = game.entities.get(&monster_a).unwrap();
        let monster_b_instance = game.entities.get(&monster_b).unwrap();

        if let CardTypeInstance::Monster(ma) = &monster_a_instance.card_type {
            assert_eq!(ma.hp, 7);
        }
        if let CardTypeInstance::Monster(mb) = &monster_b_instance.card_type {
            assert_eq!(mb.hp, 3);
        }
    }

    #[test]
    fn test_monster_vs_player_one_way_damage() {
        let mut game = create_test_game();
        let player_a = game.player_id_a;
        let player_b = game.player_id_b;

        let monster_a = create_test_monster_with_attack(&mut game, player_a, 0, 5, 10, 10);

        let player_b_hp_before = game.players.get(&player_b).unwrap().hp;
        assert_eq!(player_b_hp_before, 30);

        let monster_a_instance = game.entities.get(&monster_a).unwrap();
        if let CardTypeInstance::Monster(ma) = &monster_a_instance.card_type {
            assert_eq!(ma.hp, 10);
        }

        game.attack(monster_a, player_b).unwrap();
        game.compute_commands().unwrap();

        let player_b_hp_after = game.players.get(&player_b).unwrap().hp;
        assert_eq!(player_b_hp_after, 25);

        let monster_a_instance = game.entities.get(&monster_a).unwrap();
        if let CardTypeInstance::Monster(ma) = &monster_a_instance.card_type {
            assert_eq!(ma.hp, 10);
        }
    }

    #[test]
    fn test_on_attack_effects_trigger_before_damage() {
        let mut game = create_test_game();
        let player_a = game.player_id_a;
        let player_b = game.player_id_b;

        let monster_b = create_test_monster_with_attack(&mut game, player_b, 1, 5, 10, 10);

        let monster_a = create_test_monster_with_on_attack(
            &mut game,
            player_a,
            0,
            2,
            10,
            vec![Effect::DealDamage {
                initiator: 0,
                target: Target::Id(monster_b),
                amount: 3,
            }],
        );

        let monster_b_instance = game.entities.get(&monster_b).unwrap();
        if let CardTypeInstance::Monster(mb) = &monster_b_instance.card_type {
            assert_eq!(mb.hp, 10);
        }

        game.attack(monster_a, monster_b).unwrap();
        game.compute_commands().unwrap();

        let monster_b_instance = game.entities.get(&monster_b).unwrap();
        if let CardTypeInstance::Monster(mb) = &monster_b_instance.card_type {
            assert_eq!(mb.hp, 5);
        }

        let monster_a_instance = game.entities.get(&monster_a).unwrap();
        if let CardTypeInstance::Monster(ma) = &monster_a_instance.card_type {
            assert_eq!(ma.hp, 5);
        }
    }

    #[test]
    fn test_monster_destroyed_at_zero_hp() {
        let mut game = create_test_game();
        let player_a = game.player_id_a;
        let player_b = game.player_id_b;

        let monster_a = create_test_monster_with_attack(&mut game, player_a, 0, 5, 10, 10);
        let monster_b = create_test_monster_with_attack(&mut game, player_b, 1, 3, 5, 5);

        assert_eq!(
            game.entities.get(&monster_b).unwrap().location,
            Location::Field(1)
        );

        game.attack(monster_a, monster_b).unwrap();
        game.compute_commands().unwrap();

        assert_eq!(
            game.entities.get(&monster_b).unwrap().location,
            Location::Graveyard
        );

        let graveyard_count = game
            .entities
            .values()
            .filter(|e| e.owner == player_b && e.location == Location::Graveyard)
            .count();
        assert_eq!(graveyard_count, 1);
    }

    #[test]
    fn test_mutual_destruction() {
        let mut game = create_test_game();
        let player_a = game.player_id_a;
        let player_b = game.player_id_b;

        let monster_a = create_test_monster_with_attack(&mut game, player_a, 0, 5, 5, 5);
        let monster_b = create_test_monster_with_attack(&mut game, player_b, 1, 5, 5, 5);

        assert_eq!(
            game.entities.get(&monster_a).unwrap().location,
            Location::Field(0)
        );
        assert_eq!(
            game.entities.get(&monster_b).unwrap().location,
            Location::Field(1)
        );

        game.attack(monster_a, monster_b).unwrap();
        game.compute_commands().unwrap();

        assert_eq!(
            game.entities.get(&monster_a).unwrap().location,
            Location::Graveyard
        );
        assert_eq!(
            game.entities.get(&monster_b).unwrap().location,
            Location::Graveyard
        );

        let total_graveyard_count = game
            .entities
            .values()
            .filter(|e| e.location == Location::Graveyard)
            .count();
        assert_eq!(total_graveyard_count, 2);
    }
}
