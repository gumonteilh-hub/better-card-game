#[cfg(test)]
pub mod tests {
    use crate::game::{
        tests::test_utils::{
            add_card_to_deck, add_card_to_hand, create_test_game, create_test_monster,
        },
        types::Location,
        view::PublicGameState,
    };

    #[test]
    fn test_public_game_state_basic() {
        // Create a game with minimal setup
        let game = create_test_game();

        // Create public view for player A
        let public_view = PublicGameState::new(&game, 0).unwrap();

        // Verify basic game info
        assert_eq!(public_view.game_id, game.game_id);
        assert_eq!(public_view.player_id, 0);
        assert_eq!(public_view.turn, 1);
        assert_eq!(public_view.winner_id, None);

        // Verify player info
        assert_eq!(public_view.player.hero.id, 0);
        assert_eq!(public_view.player.hero.hp, 30);
        assert_eq!(public_view.player.max_mana, 1);
        assert_eq!(public_view.player.current_mana, 1);
        assert_eq!(public_view.player.hand.len(), 0);
        assert_eq!(public_view.player.field.len(), 0);
        assert_eq!(public_view.player.deck_size, 0);

        // Verify enemy info
        assert_eq!(public_view.enemy.hero.id, 1);
        assert_eq!(public_view.enemy.hero.hp, 30);
        assert_eq!(public_view.enemy.max_mana, 0);
        assert_eq!(public_view.enemy.current_mana, 0);
        assert_eq!(public_view.enemy.hand, 0);
        assert_eq!(public_view.enemy.field.len(), 0);
        assert_eq!(public_view.enemy.deck_size, 0);
    }

    #[test]
    fn test_public_game_state_field_separation() {
        // Create a game and add monsters to both players' fields
        let mut game = create_test_game();

        // Player A has monsters at positions 0 and 2
        let monster_a1 = create_test_monster(&mut game, 0, 0, 5, 5);
        let monster_a2 = create_test_monster(&mut game, 0, 2, 3, 4);

        // Player B has monsters at positions 1 and 3
        let monster_b1 = create_test_monster(&mut game, 1, 1, 7, 7);
        let monster_b2 = create_test_monster(&mut game, 1, 3, 2, 2);

        // Create public view for player A
        let public_view_a = PublicGameState::new(&game, 0).unwrap();

        // Verify player A sees their own monsters
        assert_eq!(public_view_a.player.field.len(), 2);
        assert!(public_view_a.player.field.contains_key(&0));
        assert!(public_view_a.player.field.contains_key(&2));
        assert_eq!(public_view_a.player.field.get(&0).unwrap().id, monster_a1);
        assert_eq!(public_view_a.player.field.get(&2).unwrap().id, monster_a2);

        // Verify player A sees enemy monsters
        assert_eq!(public_view_a.enemy.field.len(), 2);
        assert!(public_view_a.enemy.field.contains_key(&1));
        assert!(public_view_a.enemy.field.contains_key(&3));
        assert_eq!(public_view_a.enemy.field.get(&1).unwrap().id, monster_b1);
        assert_eq!(public_view_a.enemy.field.get(&3).unwrap().id, monster_b2);

        // Create public view for player B - verify perspective switch
        let public_view_b = PublicGameState::new(&game, 1).unwrap();

        // Player B should see the opposite
        assert_eq!(public_view_b.player.field.len(), 2);
        assert!(public_view_b.player.field.contains_key(&1));
        assert!(public_view_b.player.field.contains_key(&3));

        assert_eq!(public_view_b.enemy.field.len(), 2);
        assert!(public_view_b.enemy.field.contains_key(&0));
        assert!(public_view_b.enemy.field.contains_key(&2));
    }

    #[test]
    fn test_public_game_state_hand_privacy() {
        // Create a game and add cards to both players' hands
        let mut game = create_test_game();

        // Player A has 3 cards in hand
        let card_a1 = add_card_to_hand(&mut game, 0);
        let card_a2 = add_card_to_hand(&mut game, 0);
        let card_a3 = add_card_to_hand(&mut game, 0);

        // Player B has 2 cards in hand
        add_card_to_hand(&mut game, 1);
        add_card_to_hand(&mut game, 1);

        // Create public view for player A
        let public_view_a = PublicGameState::new(&game, 0).unwrap();

        // Verify player A can see their own hand (full card details)
        assert_eq!(public_view_a.player.hand.len(), 3);

        // Verify all expected card IDs are present (order is not guaranteed from HashMap)
        let hand_ids: Vec<usize> = public_view_a.player.hand.iter().map(|c| c.id).collect();
        assert!(hand_ids.contains(&card_a1));
        assert!(hand_ids.contains(&card_a2));
        assert!(hand_ids.contains(&card_a3));

        // Verify player A only sees enemy hand size (not card details)
        assert_eq!(public_view_a.enemy.hand, 2);

        // Create public view for player B - verify opposite
        let public_view_b = PublicGameState::new(&game, 1).unwrap();

        assert_eq!(public_view_b.player.hand.len(), 2);
        assert_eq!(public_view_b.enemy.hand, 3);
    }

    #[test]
    fn test_public_game_state_deck_counting() {
        // Create a game and add cards to both players' decks
        let mut game = create_test_game();

        // Player A has 5 cards in deck
        for _ in 0..5 {
            add_card_to_deck(&mut game, 0);
        }

        // Player B has 3 cards in deck
        for _ in 0..3 {
            add_card_to_deck(&mut game, 1);
        }

        // Create public view for player A
        let public_view_a = PublicGameState::new(&game, 0).unwrap();

        // Verify deck sizes
        assert_eq!(public_view_a.player.deck_size, 5);
        assert_eq!(public_view_a.enemy.deck_size, 3);

        // Create public view for player B - verify opposite
        let public_view_b = PublicGameState::new(&game, 1).unwrap();

        assert_eq!(public_view_b.player.deck_size, 3);
        assert_eq!(public_view_b.enemy.deck_size, 5);
    }

    #[test]
    fn test_public_game_state_mana_tracking() {
        // Create a game
        let mut game = create_test_game();

        // Modify player mana
        game.players.get_mut(&0).unwrap().base_mana = 5;
        game.players.get_mut(&0).unwrap().mana = 3; // Player A has spent 2 mana

        game.players.get_mut(&1).unwrap().base_mana = 7;
        game.players.get_mut(&1).unwrap().mana = 7; // Player B has full mana

        // Create public view for player A
        let public_view_a = PublicGameState::new(&game, 0).unwrap();

        // Verify player mana
        assert_eq!(public_view_a.player.max_mana, 5);
        assert_eq!(public_view_a.player.current_mana, 3);

        // Verify enemy mana
        assert_eq!(public_view_a.enemy.max_mana, 7);
        assert_eq!(public_view_a.enemy.current_mana, 7);
    }

    #[test]
    fn test_public_game_state_movement_points() {
        // Create a game
        let mut game = create_test_game();

        // Set movement points
        game.players.get_mut(&0).unwrap().max_move = 3;
        game.players.get_mut(&0).unwrap().move_count = 1; // Used 1 movement

        // Create public view for player A
        let public_view = PublicGameState::new(&game, 0).unwrap();

        // Verify movement tracking
        assert_eq!(public_view.player.max_move, 3);
        assert_eq!(public_view.player.move_count, 1);
    }

    #[test]
    fn test_public_game_state_hp_tracking() {
        // Create a game
        let mut game = create_test_game();

        // Damage the players
        game.players.get_mut(&0).unwrap().hp = 25; // Player A took 5 damage
        game.players.get_mut(&1).unwrap().hp = 10; // Player B took 20 damage

        // Create public view for player A
        let public_view_a = PublicGameState::new(&game, 0).unwrap();

        // Verify HP tracking
        assert_eq!(public_view_a.player.hero.hp, 25);
        assert_eq!(public_view_a.enemy.hero.hp, 10);

        // Create public view for player B - verify opposite
        let public_view_b = PublicGameState::new(&game, 1).unwrap();

        assert_eq!(public_view_b.player.hero.hp, 10);
        assert_eq!(public_view_b.enemy.hero.hp, 25);
    }

    #[test]
    fn test_public_game_state_mixed_locations() {
        // Create a game with cards in all locations
        let mut game = create_test_game();

        // Player A cards
        let monster_a_field = create_test_monster(&mut game, 0, 0, 5, 5);
        let card_a_hand = add_card_to_hand(&mut game, 0);
        add_card_to_deck(&mut game, 0);
        add_card_to_deck(&mut game, 0);

        // Add a card to graveyard (simulate destroyed card)
        let graveyard_card = create_test_monster(&mut game, 0, 5, 1, 1);
        game.entities.get_mut(&graveyard_card).unwrap().location = Location::Graveyard;

        // Player B cards
        let monster_b_field = create_test_monster(&mut game, 1, 3, 3, 3);
        add_card_to_hand(&mut game, 1);
        add_card_to_hand(&mut game, 1);
        add_card_to_hand(&mut game, 1);
        add_card_to_deck(&mut game, 1);

        // Create public view for player A
        let public_view = PublicGameState::new(&game, 0).unwrap();

        // Verify field
        assert_eq!(public_view.player.field.len(), 1);
        assert_eq!(public_view.player.field.get(&0).unwrap().id, monster_a_field);
        assert_eq!(public_view.enemy.field.len(), 1);
        assert_eq!(public_view.enemy.field.get(&3).unwrap().id, monster_b_field);

        // Verify hand
        assert_eq!(public_view.player.hand.len(), 1);
        assert_eq!(public_view.player.hand[0].id, card_a_hand);
        assert_eq!(public_view.enemy.hand, 3);

        // Verify deck
        assert_eq!(public_view.player.deck_size, 2);
        assert_eq!(public_view.enemy.deck_size, 1);

        // Verify graveyard cards are not counted anywhere
        let total_player_cards = public_view.player.field.len()
            + public_view.player.hand.len()
            + public_view.player.deck_size;
        assert_eq!(total_player_cards, 4); // Should not include graveyard card
    }

    #[test]
    fn test_public_game_state_winner_tracking() {
        // Create a game
        let mut game = create_test_game();

        // Set winner
        game.winner_id = Some(0);

        // Create public view
        let public_view = PublicGameState::new(&game, 0).unwrap();

        // Verify winner is tracked
        assert_eq!(public_view.winner_id, Some(0));
    }

    #[test]
    fn test_public_game_state_turn_tracking() {
        // Create a game
        let mut game = create_test_game();

        // Advance turn
        game.turn = 5;

        // Create public view
        let public_view = PublicGameState::new(&game, 0).unwrap();

        // Verify turn is tracked
        assert_eq!(public_view.turn, 5);
    }

    #[test]
    fn test_public_game_state_full_board() {
        // Create a game with a full board (8 positions)
        let mut game = create_test_game();

        // Player A has 4 monsters
        create_test_monster(&mut game, 0, 0, 1, 1);
        create_test_monster(&mut game, 0, 2, 2, 2);
        create_test_monster(&mut game, 0, 4, 3, 3);
        create_test_monster(&mut game, 0, 6, 4, 4);

        // Player B has 4 monsters
        create_test_monster(&mut game, 1, 1, 5, 5);
        create_test_monster(&mut game, 1, 3, 6, 6);
        create_test_monster(&mut game, 1, 5, 7, 7);
        create_test_monster(&mut game, 1, 7, 8, 8);

        // Create public view for player A
        let public_view = PublicGameState::new(&game, 0).unwrap();

        // Verify both players have 4 monsters
        assert_eq!(public_view.player.field.len(), 4);
        assert_eq!(public_view.enemy.field.len(), 4);

        // Verify positions are correct
        assert!(public_view.player.field.contains_key(&0));
        assert!(public_view.player.field.contains_key(&2));
        assert!(public_view.player.field.contains_key(&4));
        assert!(public_view.player.field.contains_key(&6));

        assert!(public_view.enemy.field.contains_key(&1));
        assert!(public_view.enemy.field.contains_key(&3));
        assert!(public_view.enemy.field.contains_key(&5));
        assert!(public_view.enemy.field.contains_key(&7));
    }

    #[test]
    fn test_public_game_state_empty_game() {
        // Create a completely empty game (no cards anywhere)
        let game = create_test_game();

        // Create public view
        let public_view = PublicGameState::new(&game, 0).unwrap();

        // Verify everything is empty
        assert_eq!(public_view.player.field.len(), 0);
        assert_eq!(public_view.player.hand.len(), 0);
        assert_eq!(public_view.player.deck_size, 0);
        assert_eq!(public_view.enemy.field.len(), 0);
        assert_eq!(public_view.enemy.hand, 0);
        assert_eq!(public_view.enemy.deck_size, 0);
    }

    #[test]
    fn test_public_game_state_consistency_both_perspectives() {
        // Create a complex game state
        let mut game = create_test_game();

        // Add various cards
        create_test_monster(&mut game, 0, 0, 5, 5);
        create_test_monster(&mut game, 1, 3, 3, 3);
        add_card_to_hand(&mut game, 0);
        add_card_to_hand(&mut game, 0);
        add_card_to_hand(&mut game, 1);
        add_card_to_deck(&mut game, 0);
        add_card_to_deck(&mut game, 1);
        add_card_to_deck(&mut game, 1);

        // Create public views for both players
        let view_a = PublicGameState::new(&game, 0).unwrap();
        let view_b = PublicGameState::new(&game, 1).unwrap();

        // Verify consistency: what A sees as their cards should be what B sees as enemy cards
        assert_eq!(view_a.player.field.len(), view_b.enemy.field.len());
        assert_eq!(view_a.player.hand.len(), view_b.enemy.hand);
        assert_eq!(view_a.player.deck_size, view_b.enemy.deck_size);

        // And vice versa
        assert_eq!(view_a.enemy.field.len(), view_b.player.field.len());
        assert_eq!(view_a.enemy.hand, view_b.player.hand.len());
        assert_eq!(view_a.enemy.deck_size, view_b.player.deck_size);

        // Verify specific numbers
        assert_eq!(view_a.player.field.len(), 1);
        assert_eq!(view_a.enemy.field.len(), 1);
        assert_eq!(view_a.player.hand.len(), 2);
        assert_eq!(view_a.enemy.hand, 1);
        assert_eq!(view_a.player.deck_size, 1);
        assert_eq!(view_a.enemy.deck_size, 2);
    }
}
