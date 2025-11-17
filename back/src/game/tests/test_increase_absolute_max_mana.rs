// FUNCTIONAL RULES: IncreaseAbsoluteMaxMana Effect
//
// 1. Permanently increases player's absolute maximum mana cap (absolute_max_mana)
// 2. Does not affect current max_mana or current mana
// 3. Can target self, opponent, or both players
// 4. Multiple increases stack additively
// 5. Affects the ceiling that max_mana cannot exceed during end turn

#[cfg(test)]
mod tests {
    use super::super::test_utils::{create_test_game, create_test_spell};
    use crate::game::effects::{Effect, PlayerTarget};

    #[test]
    fn test_increases_absolute_max_mana_permanently() {
        // a) Initialize
        let mut game = create_test_game();
        let player_a = game.player_id_a;

        // b) Modify state: set player A's absolute_max_mana to 10
        game.players.get_mut(&player_a).unwrap().absolute_max_mana = 10;

        // c) Test as user: play spell that increases absolute_max_mana by 3
        let spell = create_test_spell(
            &mut game,
            player_a,
            vec![Effect::IncreaseAbsoluteMaxMana {
                initiator: 0,
                player: PlayerTarget::Player,
                amount: 3,
            }],
        );
        crate::game::user_actions::play_spell::play_spell(&mut game, player_a, spell, None)
            .unwrap();
        game.compute_commands().unwrap();

        // d) Assert absolute_max_mana increased to 13
        assert_eq!(
            game.players.get(&player_a).unwrap().absolute_max_mana,
            13
        );
    }

    #[test]
    fn test_does_not_affect_current_max_mana() {
        // a) Initialize
        let mut game = create_test_game();
        let player_a = game.player_id_a;

        // b) Modify state: set different values for max_mana and absolute_max_mana
        game.players.get_mut(&player_a).unwrap().max_mana = 5;
        game.players.get_mut(&player_a).unwrap().absolute_max_mana = 10;

        // c) Test as user: increase absolute_max_mana
        let spell = create_test_spell(
            &mut game,
            player_a,
            vec![Effect::IncreaseAbsoluteMaxMana {
                initiator: 0,
                player: PlayerTarget::Player,
                amount: 5,
            }],
        );
        crate::game::user_actions::play_spell::play_spell(&mut game, player_a, spell, None)
            .unwrap();
        game.compute_commands().unwrap();

        // d) Assert max_mana unchanged, only absolute_max_mana increased
        assert_eq!(game.players.get(&player_a).unwrap().max_mana, 5);
        assert_eq!(
            game.players.get(&player_a).unwrap().absolute_max_mana,
            15
        );
    }

    #[test]
    fn test_does_not_affect_current_mana() {
        // a) Initialize
        let mut game = create_test_game();
        let player_a = game.player_id_a;

        // b) Modify state: set player with partial mana
        game.players.get_mut(&player_a).unwrap().mana = 3;
        game.players.get_mut(&player_a).unwrap().max_mana = 7;
        game.players.get_mut(&player_a).unwrap().absolute_max_mana = 10;

        // c) Test as user: increase absolute_max_mana
        let spell = create_test_spell(
            &mut game,
            player_a,
            vec![Effect::IncreaseAbsoluteMaxMana {
                initiator: 0,
                player: PlayerTarget::Player,
                amount: 5,
            }],
        );
        crate::game::user_actions::play_spell::play_spell(&mut game, player_a, spell, None)
            .unwrap();
        game.compute_commands().unwrap();

        // d) Assert current mana unchanged
        assert_eq!(game.players.get(&player_a).unwrap().mana, 3);
        assert_eq!(game.players.get(&player_a).unwrap().max_mana, 7);
        assert_eq!(
            game.players.get(&player_a).unwrap().absolute_max_mana,
            15
        );
    }

    #[test]
    fn test_target_self() {
        // a) Initialize
        let mut game = create_test_game();
        let player_a = game.player_id_a;
        let player_b = game.player_id_b;

        // b) Modify state: set both players' absolute_max_mana to 10
        game.players.get_mut(&player_a).unwrap().absolute_max_mana = 10;
        game.players.get_mut(&player_b).unwrap().absolute_max_mana = 10;

        // c) Test as user: player A increases their own absolute_max_mana
        let spell = create_test_spell(
            &mut game,
            player_a,
            vec![Effect::IncreaseAbsoluteMaxMana {
                initiator: 0,
                player: PlayerTarget::Player,
                amount: 3,
            }],
        );
        crate::game::user_actions::play_spell::play_spell(&mut game, player_a, spell, None)
            .unwrap();
        game.compute_commands().unwrap();

        // d) Assert only player A's absolute_max_mana increased
        assert_eq!(
            game.players.get(&player_a).unwrap().absolute_max_mana,
            13
        );
        assert_eq!(
            game.players.get(&player_b).unwrap().absolute_max_mana,
            10
        );
    }

    #[test]
    fn test_target_opponent() {
        // a) Initialize
        let mut game = create_test_game();
        let player_a = game.player_id_a;
        let player_b = game.player_id_b;

        // b) Modify state: set both players' absolute_max_mana to 10
        game.players.get_mut(&player_a).unwrap().absolute_max_mana = 10;
        game.players.get_mut(&player_b).unwrap().absolute_max_mana = 10;

        // c) Test as user: player A increases opponent's absolute_max_mana
        let spell = create_test_spell(
            &mut game,
            player_a,
            vec![Effect::IncreaseAbsoluteMaxMana {
                initiator: 0,
                player: PlayerTarget::EnnemyPlayer,
                amount: 3,
            }],
        );
        crate::game::user_actions::play_spell::play_spell(&mut game, player_a, spell, None)
            .unwrap();
        game.compute_commands().unwrap();

        // d) Assert only player B's absolute_max_mana increased
        assert_eq!(
            game.players.get(&player_a).unwrap().absolute_max_mana,
            10
        );
        assert_eq!(
            game.players.get(&player_b).unwrap().absolute_max_mana,
            13
        );
    }

    #[test]
    fn test_target_both_players() {
        // a) Initialize
        let mut game = create_test_game();
        let player_a = game.player_id_a;
        let player_b = game.player_id_b;

        // b) Modify state: set both players' absolute_max_mana to 10
        game.players.get_mut(&player_a).unwrap().absolute_max_mana = 10;
        game.players.get_mut(&player_b).unwrap().absolute_max_mana = 10;

        // c) Test as user: increase absolute_max_mana for both players
        let spell = create_test_spell(
            &mut game,
            player_a,
            vec![Effect::IncreaseAbsoluteMaxMana {
                initiator: 0,
                player: PlayerTarget::BothPlayers,
                amount: 5,
            }],
        );
        crate::game::user_actions::play_spell::play_spell(&mut game, player_a, spell, None)
            .unwrap();
        game.compute_commands().unwrap();

        // d) Assert both players' absolute_max_mana increased
        assert_eq!(
            game.players.get(&player_a).unwrap().absolute_max_mana,
            15
        );
        assert_eq!(
            game.players.get(&player_b).unwrap().absolute_max_mana,
            15
        );
    }

    #[test]
    fn test_multiple_increases_stack() {
        // a) Initialize
        let mut game = create_test_game();
        let player_a = game.player_id_a;

        // b) Modify state: set player A's absolute_max_mana to 10
        game.players.get_mut(&player_a).unwrap().absolute_max_mana = 10;

        // c) Test as user: apply two separate increases
        let spell1 = create_test_spell(
            &mut game,
            player_a,
            vec![Effect::IncreaseAbsoluteMaxMana {
                initiator: 0,
                player: PlayerTarget::Player,
                amount: 3,
            }],
        );
        crate::game::user_actions::play_spell::play_spell(&mut game, player_a, spell1, None)
            .unwrap();
        game.compute_commands().unwrap();

        let spell2 = create_test_spell(
            &mut game,
            player_a,
            vec![Effect::IncreaseAbsoluteMaxMana {
                initiator: 0,
                player: PlayerTarget::Player,
                amount: 2,
            }],
        );
        crate::game::user_actions::play_spell::play_spell(&mut game, player_a, spell2, None)
            .unwrap();
        game.compute_commands().unwrap();

        // d) Assert increases stacked (10 + 3 + 2 = 15)
        assert_eq!(
            game.players.get(&player_a).unwrap().absolute_max_mana,
            15
        );
    }

    #[test]
    fn test_affects_end_turn_max_mana_cap() {
        // a) Initialize
        let mut game = create_test_game();
        game.vs_ia = false; // Disable AI to prevent auto-play
        let player_a = game.player_id_a;
        let player_b = game.player_id_b;

        // b) Modify state: set player B at their absolute_max_mana cap
        game.players.get_mut(&player_b).unwrap().max_mana = 10;
        game.players.get_mut(&player_b).unwrap().absolute_max_mana = 10;
        game.players.get_mut(&player_b).unwrap().mana = 10;

        // Add card for auto-draw
        crate::game::tests::test_utils::add_card_to_deck(&mut game, player_b);

        // Verify player B is at cap - end turn should not increase max_mana
        crate::game::user_actions::end_turn::end_turn(&mut game, player_a).unwrap();
        assert_eq!(game.players.get(&player_b).unwrap().max_mana, 10);

        // c) Test as user: increase absolute_max_mana for player B
        game.current_player = player_a; // Switch back to player A
        let spell = create_test_spell(
            &mut game,
            player_a,
            vec![Effect::IncreaseAbsoluteMaxMana {
                initiator: 0,
                player: PlayerTarget::EnnemyPlayer,
                amount: 5,
            }],
        );
        crate::game::user_actions::play_spell::play_spell(&mut game, player_a, spell, None)
            .unwrap();
        game.compute_commands().unwrap();

        // Add card for next auto-draw
        crate::game::tests::test_utils::add_card_to_deck(&mut game, player_b);

        // End turn again - now max_mana should increase
        crate::game::user_actions::end_turn::end_turn(&mut game, player_a).unwrap();

        // d) Assert max_mana can now increase beyond previous cap
        assert_eq!(game.players.get(&player_b).unwrap().max_mana, 11);
        assert_eq!(
            game.players.get(&player_b).unwrap().absolute_max_mana,
            15
        );
    }
}
