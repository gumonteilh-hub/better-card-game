// FUNCTIONAL RULES: End Turn
//
// 1. The active player (current_player) switches to the opponent
// 2. The new active player automatically draws 1 card
// 3. The new player's base_mana increases by 1 (capped at 10)
// 4. The available mana is refreshed to base_mana
// 5. Movement points are reset to 3
// 6. Attack counters (attack_count) for all monsters of the new player are reset to 0
// 7. All monsters of the new player wake up (asleep = false)

#[cfg(test)]
mod tests {
    use super::super::test_utils::{add_card_to_deck, create_test_game, create_test_monster};
    use crate::game::card::CardTypeInstance;
    use crate::game::types::Location;

    #[test]
    fn test_end_turn_switches_active_player() {
        // a) Initialize
        let mut game = create_test_game();
        game.vs_ia = false; // Disable AI to prevent auto-play
        let player_a = game.player_id_a;
        let player_b = game.player_id_b;

        // Verify initial state
        assert_eq!(game.current_player, player_a);

        // b) Modify state: ensure player A has cards to draw (AutoDraw needs a card)
        add_card_to_deck(&mut game, player_b);

        // c) Test as user would: end player A's turn
        game.end_turn(player_a).unwrap();

        // d) Assert the current player is now player B
        assert_eq!(game.current_player, player_b);
    }

    #[test]
    fn test_end_turn_new_player_draws_one_card() {
        // a) Initialize
        let mut game = create_test_game();
        game.vs_ia = false; // Disable AI to prevent auto-play
        let player_a = game.player_id_a;
        let player_b = game.player_id_b;

        // b) Modify state: add a card to player B's deck
        let card_id = add_card_to_deck(&mut game, player_b);

        // Verify card is in deck
        assert_eq!(
            game.entities.get(&card_id).unwrap().location,
            Location::Deck
        );

        // c) Test: end turn
        game.end_turn(player_a).unwrap();

        // d) Assert the card moved to hand
        assert_eq!(
            game.entities.get(&card_id).unwrap().location,
            Location::Hand
        );
    }

    #[test]
    fn test_end_turn_increases_base_mana_by_one() {
        // a) Initialize
        let mut game = create_test_game();
        game.vs_ia = false; // Disable AI to prevent auto-play
        let player_a = game.player_id_a;
        let player_b = game.player_id_b;

        // b) Modify state: set player B's base_mana to 3
        game.players.get_mut(&player_b).unwrap().base_mana = 3;
        add_card_to_deck(&mut game, player_b); // For AutoDraw

        // c) Test: end turn
        game.end_turn(player_a).unwrap();

        // d) Assert base_mana increased to 4
        assert_eq!(game.players.get(&player_b).unwrap().base_mana, 4);
    }

    #[test]
    fn test_end_turn_base_mana_capped_at_ten() {
        // a) Initialize
        let mut game = create_test_game();
        game.vs_ia = false; // Disable AI to prevent auto-play
        let player_a = game.player_id_a;
        let player_b = game.player_id_b;

        // b) Modify state: set player B's base_mana to 10 (max)
        game.players.get_mut(&player_b).unwrap().base_mana = 10;
        add_card_to_deck(&mut game, player_b); // For AutoDraw

        // c) Test: end turn
        game.end_turn(player_a).unwrap();

        // d) Assert base_mana stayed at 10 (not 11)
        assert_eq!(game.players.get(&player_b).unwrap().base_mana, 10);
    }

    #[test]
    fn test_end_turn_refreshes_mana_to_base_mana() {
        // a) Initialize
        let mut game = create_test_game();
        game.vs_ia = false; // Disable AI to prevent auto-play
        let player_a = game.player_id_a;
        let player_b = game.player_id_b;

        // b) Modify state: player B has base_mana 5 but only 2 available
        game.players.get_mut(&player_b).unwrap().base_mana = 5;
        game.players.get_mut(&player_b).unwrap().mana = 2;
        add_card_to_deck(&mut game, player_b); // For AutoDraw

        // Verify before
        assert_eq!(game.players.get(&player_b).unwrap().mana, 2);

        // c) Test: end turn
        game.end_turn(player_a).unwrap();

        // d) Assert mana refreshed to base_mana (which increased to 6)
        assert_eq!(game.players.get(&player_b).unwrap().base_mana, 6);
        assert_eq!(game.players.get(&player_b).unwrap().mana, 6);
    }

    #[test]
    fn test_end_turn_resets_movement_points_to_three() {
        // a) Initialize
        let mut game = create_test_game();
        game.vs_ia = false; // Disable AI to prevent auto-play
        let player_a = game.player_id_a;
        let player_b = game.player_id_b;

        // b) Modify state: player B has 0 movement points
        game.players.get_mut(&player_b).unwrap().move_count = 0;
        add_card_to_deck(&mut game, player_b); // For AutoDraw

        // Verify before
        assert_eq!(game.players.get(&player_b).unwrap().move_count, 0);

        // c) Test: end turn
        game.end_turn(player_a).unwrap();

        // d) Assert movement points reset to 3
        assert_eq!(game.players.get(&player_b).unwrap().move_count, 3);
    }

    #[test]
    fn test_end_turn_resets_monster_attack_counts() {
        // a) Initialize
        let mut game = create_test_game();
        game.vs_ia = false; // Disable AI to prevent auto-play
        let player_a = game.player_id_a;
        let player_b = game.player_id_b;

        // b) Modify state: create monsters for player B with attack_count > 0
        let monster_1 = create_test_monster(&mut game, player_b, 0, 5, 5);
        let monster_2 = create_test_monster(&mut game, player_b, 1, 5, 5);

        // Set attack_count to non-zero
        if let CardTypeInstance::Monster(m) =
            &mut game.entities.get_mut(&monster_1).unwrap().card_type
        {
            m.attack_count = 1;
        }
        if let CardTypeInstance::Monster(m) =
            &mut game.entities.get_mut(&monster_2).unwrap().card_type
        {
            m.attack_count = 2;
        }

        add_card_to_deck(&mut game, player_b); // For AutoDraw

        // c) Test: end turn
        game.end_turn(player_a).unwrap();

        // d) Assert all attack counts are reset to 0
        if let CardTypeInstance::Monster(m) = &game.entities.get(&monster_1).unwrap().card_type {
            assert_eq!(m.attack_count, 0);
        }
        if let CardTypeInstance::Monster(m) = &game.entities.get(&monster_2).unwrap().card_type {
            assert_eq!(m.attack_count, 0);
        }
    }

    #[test]
    fn test_end_turn_wakes_up_all_monsters() {
        // a) Initialize
        let mut game = create_test_game();
        game.vs_ia = false; // Disable AI to prevent auto-play
        let player_a = game.player_id_a;
        let player_b = game.player_id_b;

        // b) Modify state: create asleep monsters for player B
        let monster_1 = create_test_monster(&mut game, player_b, 0, 5, 5);
        let monster_2 = create_test_monster(&mut game, player_b, 1, 5, 5);

        // Set monsters as asleep
        if let CardTypeInstance::Monster(m) =
            &mut game.entities.get_mut(&monster_1).unwrap().card_type
        {
            m.asleep = true;
        }
        if let CardTypeInstance::Monster(m) =
            &mut game.entities.get_mut(&monster_2).unwrap().card_type
        {
            m.asleep = true;
        }

        add_card_to_deck(&mut game, player_b); // For AutoDraw

        // c) Test: end turn
        game.end_turn(player_a).unwrap();

        // d) Assert all monsters are now awake (asleep = false)
        if let CardTypeInstance::Monster(m) = &game.entities.get(&monster_1).unwrap().card_type {
            assert_eq!(m.asleep, false);
        }
        if let CardTypeInstance::Monster(m) = &game.entities.get(&monster_2).unwrap().card_type {
            assert_eq!(m.asleep, false);
        }
    }
}
