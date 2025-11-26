// FUNCTIONAL RULES: RefreshMana Effect
//
// 1. Restores available mana up to player's max_mana maximum
// 2. Cannot exceed max_mana (excess is wasted)
// 3. Can target self or opponent player
// 4. Multiple refreshes in one turn stack (still capped at max_mana)

#[cfg(test)]
mod tests {
    use super::super::test_utils::{create_test_game, create_test_spell};
    use crate::game::effects::{Effect, PlayerTarget};

    #[test]
    fn test_decrease_mana() {
        let mut game = create_test_game();
        let player_a = game.player_id_a;

        game.players.get_mut(&player_a).unwrap().max_mana = 3;
        game.players.get_mut(&player_a).unwrap().mana = 3;

        let refresh_spell = create_test_spell(
            &mut game,
            player_a,
            vec![Effect::DecreaseCurrentMana {
                initiator: 0,
                player: PlayerTarget::Player,
                amount: 2,
            }],
        );
        crate::game::user_actions::play_spell::play_spell(&mut game, player_a, refresh_spell, None)
            .unwrap();
        game.compute_commands().unwrap();

        assert_eq!(game.players.get(&player_a).unwrap().mana, 1);
        assert_eq!(game.players.get(&player_a).unwrap().max_mana, 3);
    }

    #[test]
    fn test_decrease_mana_saturating() {
        let mut game = create_test_game();
        let player_a = game.player_id_a;

        game.players.get_mut(&player_a).unwrap().max_mana = 3;
        game.players.get_mut(&player_a).unwrap().mana = 3;

        let refresh_spell = create_test_spell(
            &mut game,
            player_a,
            vec![Effect::DecreaseCurrentMana {
                initiator: 0,
                player: PlayerTarget::Player,
                amount: 5,
            }],
        );
        crate::game::user_actions::play_spell::play_spell(&mut game, player_a, refresh_spell, None)
            .unwrap();
        game.compute_commands().unwrap();

        assert_eq!(game.players.get(&player_a).unwrap().mana, 0);
        assert_eq!(game.players.get(&player_a).unwrap().max_mana, 3);
    }

    #[test]
    fn test_refresh_mana_cannot_exceed_max_mana() {
        let mut game = create_test_game();
        let player_a = game.player_id_a;

        game.players.get_mut(&player_a).unwrap().max_mana = 10;
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
        crate::game::user_actions::play_spell::play_spell(&mut game, player_a, refresh_spell, None)
            .unwrap();
        game.compute_commands().unwrap();

        assert_eq!(game.players.get(&player_a).unwrap().mana, 10);
        assert_eq!(game.players.get(&player_a).unwrap().max_mana, 10);
    }
}
