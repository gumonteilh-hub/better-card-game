// FUNCTIONAL RULES: RefreshMana Effect
//
// 1. Restores available mana up to player's base_mana maximum
// 2. Cannot exceed base_mana (excess is wasted)
// 3. Can target self or opponent player
// 4. Multiple refreshes in one turn stack (still capped at base_mana)

#[cfg(test)]
mod tests {
    use super::super::test_utils::{create_test_game, create_test_spell};
    use crate::game::effects::{Effect, PlayerTarget};

    #[test]
    fn test_refresh_mana_restores_available_mana() {
        let mut game = create_test_game();
        let player_a = game.player_id_a;

        game.players.get_mut(&player_a).unwrap().base_mana = 10;
        game.players.get_mut(&player_a).unwrap().mana = 3;

        let refresh_spell = create_test_spell(
            &mut game,
            player_a,
            vec![Effect::RefreshMana {
                initiator: 0,
                player: PlayerTarget::Player,
                amount: 5,
            }],
        );
        game.play_spell(player_a, refresh_spell, None).unwrap();
        game.compute_commands().unwrap();

        assert_eq!(game.players.get(&player_a).unwrap().mana, 8);
        assert_eq!(game.players.get(&player_a).unwrap().base_mana, 10);
    }

    #[test]
    fn test_refresh_mana_cannot_exceed_base_mana() {
        let mut game = create_test_game();
        let player_a = game.player_id_a;

        game.players.get_mut(&player_a).unwrap().base_mana = 10;
        game.players.get_mut(&player_a).unwrap().mana = 7;

        let refresh_spell = create_test_spell(
            &mut game,
            player_a,
            vec![Effect::RefreshMana {
                initiator: 0,
                player: PlayerTarget::Player,
                amount: 10,
            }],
        );
        game.play_spell(player_a, refresh_spell, None).unwrap();
        game.compute_commands().unwrap();

        assert_eq!(game.players.get(&player_a).unwrap().mana, 10);
        assert_eq!(game.players.get(&player_a).unwrap().base_mana, 10);
    }

    #[test]
    fn test_refresh_mana_can_target_opponent() {
        let mut game = create_test_game();
        let player_a = game.player_id_a;
        let player_b = game.player_id_b;

        game.players.get_mut(&player_a).unwrap().mana = 5;
        game.players.get_mut(&player_b).unwrap().base_mana = 8;
        game.players.get_mut(&player_b).unwrap().mana = 2;

        let refresh_spell = create_test_spell(
            &mut game,
            player_a,
            vec![Effect::RefreshMana {
                initiator: 0,
                player: PlayerTarget::EnnemyPlayer,
                amount: 4,
            }],
        );
        game.play_spell(player_a, refresh_spell, None).unwrap();
        game.compute_commands().unwrap();

        assert_eq!(game.players.get(&player_b).unwrap().mana, 6);
        assert_eq!(game.players.get(&player_a).unwrap().mana, 5);
    }

    #[test]
    fn test_multiple_refreshes_stack_capped_at_base_mana() {
        let mut game = create_test_game();
        let player_a = game.player_id_a;

        game.players.get_mut(&player_a).unwrap().base_mana = 10;
        game.players.get_mut(&player_a).unwrap().mana = 2;

        let multi_refresh_spell = create_test_spell(
            &mut game,
            player_a,
            vec![
                Effect::RefreshMana {
                    initiator: 0,
                    player: PlayerTarget::Player,
                    amount: 4,
                },
                Effect::RefreshMana {
                    initiator: 0,
                    player: PlayerTarget::Player,
                    amount: 3,
                },
                Effect::RefreshMana {
                    initiator: 0,
                    player: PlayerTarget::Player,
                    amount: 5,
                },
            ],
        );
        game.play_spell(player_a, multi_refresh_spell, None).unwrap();
        game.compute_commands().unwrap();

        assert_eq!(game.players.get(&player_a).unwrap().mana, 10);
        assert_eq!(game.players.get(&player_a).unwrap().base_mana, 10);
    }

    #[test]
    fn test_refresh_mana_from_empty_to_full() {
        let mut game = create_test_game();
        let player_a = game.player_id_a;

        game.players.get_mut(&player_a).unwrap().base_mana = 7;
        game.players.get_mut(&player_a).unwrap().mana = 0;

        let refresh_spell = create_test_spell(
            &mut game,
            player_a,
            vec![Effect::RefreshMana {
                initiator: 0,
                player: PlayerTarget::Player,
                amount: 7,
            }],
        );
        game.play_spell(player_a, refresh_spell, None).unwrap();
        game.compute_commands().unwrap();

        assert_eq!(game.players.get(&player_a).unwrap().mana, 7);
        assert_eq!(game.players.get(&player_a).unwrap().base_mana, 7);
    }
}
