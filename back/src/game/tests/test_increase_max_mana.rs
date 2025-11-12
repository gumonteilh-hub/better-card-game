// FUNCTIONAL RULES: IncreaseMaxMana Effect
//
// 1. Permanently increases player's maximum mana (base_mana)
// 2. Current mana may not increase immediately
// 3. Can target self or opponent player

#[cfg(test)]
mod tests {
    use super::super::test_utils::{create_test_game, create_test_spell};
    use crate::game::effects::{Effect, PlayerTarget};

    #[test]
    fn test_increases_base_mana_permanently() {
        let mut game = create_test_game();
        let player_a = game.player_id_a;

        game.players.get_mut(&player_a).unwrap().base_mana = 5;

        let spell = create_test_spell(
            &mut game,
            player_a,
            vec![Effect::IncreaseMaxMana {
                initiator: 0,
                player: PlayerTarget::Player,
                amount: 3,
            }],
        );
        game.play_spell(player_a, spell).unwrap();
        game.compute_commands().unwrap();

        assert_eq!(game.players.get(&player_a).unwrap().base_mana, 8);
    }

    #[test]
    fn test_current_mana_unchanged() {
        let mut game = create_test_game();
        let player_a = game.player_id_a;

        game.players.get_mut(&player_a).unwrap().base_mana = 10;
        game.players.get_mut(&player_a).unwrap().mana = 3;

        let spell = create_test_spell(
            &mut game,
            player_a,
            vec![Effect::IncreaseMaxMana {
                initiator: 0,
                player: PlayerTarget::Player,
                amount: 2,
            }],
        );
        game.play_spell(player_a, spell).unwrap();
        game.compute_commands().unwrap();

        assert_eq!(game.players.get(&player_a).unwrap().base_mana, 12);
        assert_eq!(game.players.get(&player_a).unwrap().mana, 3);
    }

    #[test]
    fn test_target_self() {
        let mut game = create_test_game();
        let player_a = game.player_id_a;
        let player_b = game.player_id_b;

        game.players.get_mut(&player_a).unwrap().base_mana = 5;
        game.players.get_mut(&player_b).unwrap().base_mana = 5;

        let spell = create_test_spell(
            &mut game,
            player_a,
            vec![Effect::IncreaseMaxMana {
                initiator: 0,
                player: PlayerTarget::Player,
                amount: 3,
            }],
        );
        game.play_spell(player_a, spell).unwrap();
        game.compute_commands().unwrap();

        assert_eq!(game.players.get(&player_a).unwrap().base_mana, 8);
        assert_eq!(game.players.get(&player_b).unwrap().base_mana, 5);
    }

    #[test]
    fn test_target_opponent() {
        let mut game = create_test_game();
        let player_a = game.player_id_a;
        let player_b = game.player_id_b;

        game.players.get_mut(&player_a).unwrap().base_mana = 5;
        game.players.get_mut(&player_b).unwrap().base_mana = 5;

        let spell = create_test_spell(
            &mut game,
            player_a,
            vec![Effect::IncreaseMaxMana {
                initiator: 0,
                player: PlayerTarget::EnnemyPlayer,
                amount: 3,
            }],
        );
        game.play_spell(player_a, spell).unwrap();
        game.compute_commands().unwrap();

        assert_eq!(game.players.get(&player_a).unwrap().base_mana, 5);
        assert_eq!(game.players.get(&player_b).unwrap().base_mana, 8);
    }

    #[test]
    fn test_multiple_increases_stack() {
        let mut game = create_test_game();
        let player_a = game.player_id_a;

        game.players.get_mut(&player_a).unwrap().base_mana = 5;

        let spell1 = create_test_spell(
            &mut game,
            player_a,
            vec![Effect::IncreaseMaxMana {
                initiator: 0,
                player: PlayerTarget::Player,
                amount: 2,
            }],
        );
        game.play_spell(player_a, spell1).unwrap();
        game.compute_commands().unwrap();

        let spell2 = create_test_spell(
            &mut game,
            player_a,
            vec![Effect::IncreaseMaxMana {
                initiator: 0,
                player: PlayerTarget::Player,
                amount: 2,
            }],
        );
        game.play_spell(player_a, spell2).unwrap();
        game.compute_commands().unwrap();

        assert_eq!(game.players.get(&player_a).unwrap().base_mana, 9);
    }
}
