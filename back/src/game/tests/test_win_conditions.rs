// FUNCTIONAL RULES: Win Conditions
//
// 1. When a player reaches 0 HP from damage, Effect::Win is triggered for the opponent
// 2. Effect::Win sets the winner_id in the game state
// 3. Action::Win is sent to the frontend

#[cfg(test)]
mod tests {
    use super::super::test_utils::create_test_spell;
    use crate::game::effects::{Effect, Target};

    #[test]
    fn test_player_at_zero_hp_triggers_win_for_opponent() {
        // a) Initialize
        let mut game = super::super::test_utils::create_test_game();
        let player_a = game.player_id_a;
        let player_b = game.player_id_b;

        // b) Modify state: set player B to 1 HP
        game.players.get_mut(&player_b).unwrap().hp = 1;

        // Create a spell that deals 1 damage to enemy player
        let damage_spell = create_test_spell(
            &mut game,
            player_a,
            vec![Effect::DealDamage {
                initiator: 0,
                target: Target::EnnemyPlayer,
                amount: 1,
            }],
        );

        // c) Test as user would: play the spell to kill opponent
        game.play_spell(player_a, damage_spell, None).unwrap();
        game.compute_commands().unwrap();

        // d) Assert player B is at 0 HP and player A won
        assert_eq!(game.players.get(&player_b).unwrap().hp, 0);
        assert_eq!(game.winner_id, Some(player_a));
    }

    #[test]
    fn test_win_effect_sets_winner_id() {
        // a) Initialize
        let mut game = super::super::test_utils::create_test_game();
        let player_a = game.player_id_a;
        let player_b = game.player_id_b;

        // Verify no winner initially
        assert_eq!(game.winner_id, None);

        // b) Modify state: directly queue a Win effect
        game.effect_queue.push_back(Effect::Win(player_a));

        // c) Test: execute the effect
        game.compute_commands().unwrap();

        // d) Assert winner_id is set
        assert_eq!(game.winner_id, Some(player_a));
    }

    #[test]
    fn test_win_sends_action_to_frontend() {
        // a) Initialize
        let mut game = super::super::test_utils::create_test_game();
        let player_a = game.player_id_a;

        // b) Modify state: queue Win effect
        game.effect_queue.push_back(Effect::Win(player_a));

        // c) Test: execute and get actions
        let actions = game.compute_commands().unwrap();

        // d) Assert Action::Win is in the actions
        use crate::game::action::Action;
        assert!(
            actions
                .iter()
                .any(|a| matches!(a, Action::Win(winner_id) if *winner_id == player_a))
        );
    }

    #[test]
    fn test_dealing_exact_lethal_damage_triggers_win() {
        // a) Initialize
        let mut game = super::super::test_utils::create_test_game();
        let player_a = game.player_id_a;
        let player_b = game.player_id_b;

        // b) Modify state: player B has 5 HP, create spell that deals exactly 5
        game.players.get_mut(&player_b).unwrap().hp = 5;

        let lethal_spell = create_test_spell(
            &mut game,
            player_a,
            vec![Effect::DealDamage {
                initiator: 0,
                target: Target::EnnemyPlayer,
                amount: 5,
            }],
        );

        // c) Test: deal exact lethal damage
        game.play_spell(player_a, lethal_spell, None).unwrap();
        game.compute_commands().unwrap();

        // d) Assert player B is dead and player A won
        assert_eq!(game.players.get(&player_b).unwrap().hp, 0);
        assert_eq!(game.winner_id, Some(player_a));
    }

    #[test]
    fn test_overkill_damage_also_triggers_win() {
        // a) Initialize
        let mut game = super::super::test_utils::create_test_game();
        let player_a = game.player_id_a;
        let player_b = game.player_id_b;

        // b) Modify state: player B has 3 HP, create spell that deals 10 (overkill)
        game.players.get_mut(&player_b).unwrap().hp = 3;

        let overkill_spell = create_test_spell(
            &mut game,
            player_a,
            vec![Effect::DealDamage {
                initiator: 0,
                target: Target::EnnemyPlayer,
                amount: 10,
            }],
        );

        // c) Test: deal overkill damage
        game.play_spell(player_a, overkill_spell, None).unwrap();
        game.compute_commands().unwrap();

        // d) Assert player B is at 0 HP (saturating_sub prevents negative) and player A won
        assert_eq!(game.players.get(&player_b).unwrap().hp, 0);
        assert_eq!(game.winner_id, Some(player_a));
    }

    #[test]
    fn test_non_lethal_damage_does_not_trigger_win() {
        // a) Initialize
        let mut game = super::super::test_utils::create_test_game();
        let player_a = game.player_id_a;
        let player_b = game.player_id_b;

        // b) Modify state: player B has 10 HP, create spell that deals 5 (non-lethal)
        game.players.get_mut(&player_b).unwrap().hp = 10;

        let damage_spell = create_test_spell(
            &mut game,
            player_a,
            vec![Effect::DealDamage {
                initiator: 0,
                target: Target::EnnemyPlayer,
                amount: 5,
            }],
        );

        // c) Test: deal non-lethal damage
        game.play_spell(player_a, damage_spell, None).unwrap();
        game.compute_commands().unwrap();

        // d) Assert player B is still alive and no winner yet
        assert_eq!(game.players.get(&player_b).unwrap().hp, 5);
        assert_eq!(game.winner_id, None);
    }
}
