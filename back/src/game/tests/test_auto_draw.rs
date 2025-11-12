// FUNCTIONAL RULES: AutoDraw Effect
//
// 1. AutoDraw is triggered at the start of each turn
// 2. AutoDraw respects the 10-card hand limit (burns excess cards)
// 3. Drawing from an empty deck has no effect

#[cfg(test)]
mod tests {
    use super::super::test_utils::{
        add_card_to_deck, add_card_to_hand, create_test_game, create_test_monster_in_hand,
    };
    use crate::game::effects::Effect;
    use crate::game::types::Location;

    #[test]
    fn test_auto_draw_triggered_at_start_of_turn() {
        let mut game = create_test_game();
        let player_a = game.player_id_a;

        add_card_to_deck(&mut game, player_a);

        let deck_count = game
            .entities
            .values()
            .filter(|e| e.owner == player_a && e.location == Location::Deck)
            .count();
        assert_eq!(deck_count, 1);
        assert_eq!(game.get_hand(player_a).len(), 0);

        game.end_turn(player_a).unwrap();
        game.compute_commands().unwrap();

        assert_eq!(game.get_hand(player_a).len(), 1);
        let deck_count = game
            .entities
            .values()
            .filter(|e| e.owner == player_a && e.location == Location::Deck)
            .count();
        assert_eq!(deck_count, 0);
    }

    #[test]
    fn test_auto_draw_respects_hand_limit() {
        let mut game = create_test_game();
        let player_a = game.player_id_a;

        for _ in 0..9 {
            add_card_to_hand(&mut game, player_a);
        }

        add_card_to_deck(&mut game, player_a);
        add_card_to_deck(&mut game, player_a);
        add_card_to_deck(&mut game, player_a);

        assert_eq!(game.get_hand(player_a).len(), 9);
        let deck_count = game
            .entities
            .values()
            .filter(|e| e.owner == player_a && e.location == Location::Deck)
            .count();
        assert_eq!(deck_count, 3);

        let monster = create_test_monster_in_hand(
            &mut game,
            player_a,
            2,
            2,
            vec![],
            vec![Effect::AutoDraw {
                player: player_a,
                amount: 3,
            }],
        );
        game.players.get_mut(&player_a).unwrap().mana = 5;
        game.play_monster(player_a, monster, 0).unwrap();
        game.compute_commands().unwrap();

        assert_eq!(game.get_hand(player_a).len(), 10);

        let deck_count = game
            .entities
            .values()
            .filter(|e| e.owner == player_a && e.location == Location::Deck)
            .count();
        assert_eq!(deck_count, 0);

        let graveyard_count = game
            .entities
            .values()
            .filter(|e| e.owner == player_a && e.location == Location::Graveyard)
            .count();
        assert_eq!(graveyard_count, 2);
    }

    #[test]
    fn test_drawing_from_empty_deck_has_no_effect() {
        let mut game = create_test_game();
        let player_a = game.player_id_a;

        let deck_size = game
            .entities
            .values()
            .filter(|e| e.owner == player_a && e.location == Location::Deck)
            .count();
        assert_eq!(deck_size, 0);

        let hand_size_before = game.get_hand(player_a).len();
        assert_eq!(hand_size_before, 0);

        let result = game.end_turn(player_a);

        assert!(result.is_ok());
        game.compute_commands().unwrap();

        let hand_size_after = game.get_hand(player_a).len();
        assert_eq!(hand_size_after, 0);
        assert_eq!(hand_size_before, hand_size_after);

        assert!(game.winner_id.is_none());
    }

    #[test]
    fn test_auto_draw_multiple_cards_from_partial_deck() {
        let mut game = create_test_game();
        let player_a = game.player_id_a;

        add_card_to_deck(&mut game, player_a);
        add_card_to_deck(&mut game, player_a);

        let deck_count = game
            .entities
            .values()
            .filter(|e| e.owner == player_a && e.location == Location::Deck)
            .count();
        assert_eq!(deck_count, 2);
        assert_eq!(game.get_hand(player_a).len(), 0);

        let monster = create_test_monster_in_hand(
            &mut game,
            player_a,
            2,
            2,
            vec![],
            vec![Effect::AutoDraw {
                player: player_a,
                amount: 5,
            }],
        );
        game.players.get_mut(&player_a).unwrap().mana = 5;
        game.play_monster(player_a, monster, 0).unwrap();
        game.compute_commands().unwrap();

        let hand_count = game.get_hand(player_a).len();
        assert_eq!(hand_count, 2);

        let deck_count = game
            .entities
            .values()
            .filter(|e| e.owner == player_a && e.location == Location::Deck)
            .count();
        assert_eq!(deck_count, 0);
    }

    #[test]
    fn test_auto_draw_with_full_hand_burns_all_cards() {
        let mut game = create_test_game();
        let player_a = game.player_id_a;

        for _ in 0..9 {
            add_card_to_hand(&mut game, player_a);
        }

        add_card_to_deck(&mut game, player_a);
        add_card_to_deck(&mut game, player_a);
        add_card_to_deck(&mut game, player_a);

        let monster = create_test_monster_in_hand(
            &mut game,
            player_a,
            2,
            2,
            vec![],
            vec![Effect::AutoDraw {
                player: player_a,
                amount: 3,
            }],
        );

        assert_eq!(game.get_hand(player_a).len(), 10);
        game.players.get_mut(&player_a).unwrap().mana = 5;
        game.play_monster(player_a, monster, 0).unwrap();
        game.compute_commands().unwrap();

        assert_eq!(game.get_hand(player_a).len(), 10);

        let graveyard_count = game
            .entities
            .values()
            .filter(|e| e.owner == player_a && e.location == Location::Graveyard)
            .count();
        assert_eq!(graveyard_count, 2);

        let deck_count = game
            .entities
            .values()
            .filter(|e| e.owner == player_a && e.location == Location::Deck)
            .count();
        assert_eq!(deck_count, 0);
    }
}
