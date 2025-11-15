// FUNCTIONAL RULES: Destroy Effect
//
// 1. Destroyed cards are moved to graveyard (from field, hand, or deck)
// 2. On_death effects are triggered when a card is destroyed

#[cfg(test)]
mod tests {
    use super::super::test_utils::{
        create_test_game, create_test_monster, create_test_monster_with_on_death, create_test_spell,
    };
    use crate::game::effects::{Effect, Target};
    use crate::game::types::Location;

    #[test]
    fn test_destroyed_card_moved_to_graveyard() {
        let mut game = create_test_game();
        let player_a = game.player_id_a;
        let player_b = game.player_id_b;

        let monster = create_test_monster(&mut game, player_b, 0, 5, 5);

        let spell = create_test_spell(
            &mut game,
            player_a,
            vec![Effect::Destroy {
                initiator: 0,
                target: Target::Id(monster),
            }],
        );
        game.play_spell(player_a, spell, None).unwrap();
        game.compute_commands().unwrap();

        assert_eq!(
            game.entities.get(&monster).unwrap().location,
            Location::Graveyard
        );
    }

    #[test]
    fn test_on_death_effects_trigger() {
        let mut game = create_test_game();
        let player_a = game.player_id_a;
        let player_b = game.player_id_b;

        let initial_hp = game.players.get(&player_a).unwrap().hp;

        let monster = create_test_monster_with_on_death(
            &mut game,
            player_b,
            0,
            5,
            5,
            vec![Effect::DealDamage {
                initiator: 0,
                target: Target::EnnemyPlayer,
                amount: 3,
            }],
        );

        let spell = create_test_spell(
            &mut game,
            player_a,
            vec![Effect::Destroy {
                initiator: 0,
                target: Target::Id(monster),
            }],
        );
        game.play_spell(player_a, spell, None).unwrap();
        game.compute_commands().unwrap();

        assert_eq!(
            game.entities.get(&monster).unwrap().location,
            Location::Graveyard
        );
        assert_eq!(game.players.get(&player_a).unwrap().hp, initial_hp - 3);
    }

    #[test]
    fn test_on_death_with_multiple_effects() {
        let mut game = create_test_game();
        let player_a = game.player_id_a;
        let player_b = game.player_id_b;

        let initial_hp_a = game.players.get(&player_a).unwrap().hp;
        let initial_hp_b = game.players.get(&player_b).unwrap().hp;

        let monster = create_test_monster_with_on_death(
            &mut game,
            player_b,
            0,
            5,
            5,
            vec![
                Effect::DealDamage {
                    initiator: 0,
                    target: Target::EnnemyPlayer,
                    amount: 3,
                },
                Effect::DealDamage {
                    initiator: 0,
                    target: Target::Player,
                    amount: 2,
                },
            ],
        );

        let spell = create_test_spell(
            &mut game,
            player_a,
            vec![Effect::Destroy {
                initiator: 0,
                target: Target::Id(monster),
            }],
        );
        game.play_spell(player_a, spell, None).unwrap();
        game.compute_commands().unwrap();

        assert_eq!(
            game.entities.get(&monster).unwrap().location,
            Location::Graveyard
        );
        assert_eq!(game.players.get(&player_a).unwrap().hp, initial_hp_a - 3);
        assert_eq!(game.players.get(&player_b).unwrap().hp, initial_hp_b - 2);
    }

    #[test]
    fn test_destroy_multiple_targets() {
        let mut game = create_test_game();
        let player_a = game.player_id_a;
        let player_b = game.player_id_b;

        let monster1 = create_test_monster(&mut game, player_b, 0, 5, 5);
        let monster2 = create_test_monster(&mut game, player_b, 2, 3, 3);

        let spell = create_test_spell(
            &mut game,
            player_a,
            vec![Effect::Destroy {
                initiator: 0,
                target: Target::Ennemies,
            }],
        );
        game.play_spell(player_a, spell, None).unwrap();
        game.compute_commands().unwrap();

        assert_eq!(
            game.entities.get(&monster1).unwrap().location,
            Location::Graveyard
        );
        assert_eq!(
            game.entities.get(&monster2).unwrap().location,
            Location::Graveyard
        );
    }

    #[test]
    fn test_monster_without_on_death_no_effects() {
        let mut game = create_test_game();
        let player_a = game.player_id_a;
        let player_b = game.player_id_b;

        let initial_hp_a = game.players.get(&player_a).unwrap().hp;
        let initial_hp_b = game.players.get(&player_b).unwrap().hp;

        let monster = create_test_monster(&mut game, player_b, 0, 5, 5);

        let spell = create_test_spell(
            &mut game,
            player_a,
            vec![Effect::Destroy {
                initiator: 0,
                target: Target::Id(monster),
            }],
        );
        game.play_spell(player_a, spell, None).unwrap();
        game.compute_commands().unwrap();

        assert_eq!(
            game.entities.get(&monster).unwrap().location,
            Location::Graveyard
        );
        assert_eq!(game.players.get(&player_a).unwrap().hp, initial_hp_a);
        assert_eq!(game.players.get(&player_b).unwrap().hp, initial_hp_b);
    }
}
