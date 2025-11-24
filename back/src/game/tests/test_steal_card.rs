// FUNCTIONAL RULES: StealCard Effect
//
// 1. StealCard steals one random card from opponent's specified location (Deck, Hand, Graveyard, or Field)
// 2. The stolen card is moved to the thief's hand
// 3. If thief's hand is full (10 cards), the stolen card goes to the thief's graveyard instead
// 4. If the target location is empty, the effect does nothing
// 5. The card's owner and location are changed to the thief
// 6. Works for all locations: Deck, Hand, Graveyard, Field(_)

#[cfg(test)]
mod tests {
    use super::super::test_utils::{
        add_card_to_deck, add_card_to_hand, create_test_game, create_test_monster,
        create_test_spell,
    };
    use crate::game::effects::Effect;
    use crate::game::types::Location;

    #[test]
    fn test_steal_card_from_deck_moves_to_thief_hand() {
        // a) Initialize with minimal setup
        let mut game = create_test_game();
        let player_a = game.player_id_a;
        let player_b = game.player_id_b;

        // b) Modify state: add cards to opponent's deck
        let card_in_deck = add_card_to_deck(&mut game, player_b);
        add_card_to_deck(&mut game, player_b);
        add_card_to_deck(&mut game, player_b);

        // Give player A enough mana
        game.players.get_mut(&player_a).unwrap().mana = 5;

        // Verify initial state
        assert_eq!(game.entities.get(&card_in_deck).unwrap().owner, player_b);
        assert_eq!(
            game.entities.get(&card_in_deck).unwrap().location,
            Location::Deck
        );
        assert_eq!(game.get_hand(player_a).len(), 0);

        // c) Test as user would: play a spell that steals from deck
        let steal_spell = create_test_spell(
            &mut game,
            player_a,
            vec![Effect::StealCard {
                initiator: 0,
                location: Location::Deck,
            }],
        );
        crate::game::user_actions::play_spell::play_spell(&mut game, player_a, steal_spell, None)
            .unwrap();
        game.compute_commands().unwrap();

        // d) Assert: a card was stolen and moved to player A's hand
        assert_eq!(game.get_hand(player_a).len(), 1);

        // Check that one of the cards now belongs to player A
        let stolen_cards: Vec<_> = game
            .entities
            .values()
            .filter(|e| e.owner == player_a && e.location == Location::Hand)
            .collect();
        assert_eq!(stolen_cards.len(), 1);

        // Check opponent's deck decreased
        let opponent_deck_count = game
            .entities
            .values()
            .filter(|e| e.owner == player_b && e.location == Location::Deck)
            .count();
        assert_eq!(opponent_deck_count, 2);
    }

    #[test]
    fn test_steal_card_from_hand_moves_to_thief_hand() {
        // a) Initialize
        let mut game = create_test_game();
        let player_a = game.player_id_a;
        let player_b = game.player_id_b;

        // b) Modify state: add cards to opponent's hand
        add_card_to_hand(&mut game, player_b);
        add_card_to_hand(&mut game, player_b);
        add_card_to_hand(&mut game, player_b);

        game.players.get_mut(&player_a).unwrap().mana = 5;

        // Verify initial state
        assert_eq!(game.get_hand(player_b).len(), 3);
        assert_eq!(game.get_hand(player_a).len(), 0);

        // c) Test: play a spell that steals from hand
        let steal_spell = create_test_spell(
            &mut game,
            player_a,
            vec![Effect::StealCard {
                initiator: 0,
                location: Location::Hand,
            }],
        );
        crate::game::user_actions::play_spell::play_spell(&mut game, player_a, steal_spell, None)
            .unwrap();
        game.compute_commands().unwrap();

        // d) Assert: card stolen from opponent's hand to player A's hand
        assert_eq!(game.get_hand(player_a).len(), 1);
        assert_eq!(game.get_hand(player_b).len(), 2);
    }

    #[test]
    fn test_steal_card_from_graveyard_moves_to_thief_hand() {
        // a) Initialize
        let mut game = create_test_game();
        let player_a = game.player_id_a;
        let player_b = game.player_id_b;

        // b) Modify state: add cards to opponent's graveyard
        let card_in_graveyard = add_card_to_deck(&mut game, player_b);
        game.entities
            .get_mut(&card_in_graveyard)
            .unwrap()
            .location = Location::Graveyard;

        add_card_to_deck(&mut game, player_b);
        let card_2 = game.entities.len() + 1;
        game.entities.get_mut(&card_2).unwrap().location = Location::Graveyard;

        game.players.get_mut(&player_a).unwrap().mana = 5;

        // Verify initial state
        let graveyard_count = game
            .entities
            .values()
            .filter(|e| e.owner == player_b && e.location == Location::Graveyard)
            .count();
        assert_eq!(graveyard_count, 2);

        // c) Test: play a spell that steals from graveyard
        let steal_spell = create_test_spell(
            &mut game,
            player_a,
            vec![Effect::StealCard {
                initiator: 0,
                location: Location::Graveyard,
            }],
        );
        crate::game::user_actions::play_spell::play_spell(&mut game, player_a, steal_spell, None)
            .unwrap();
        game.compute_commands().unwrap();

        // d) Assert: card stolen from opponent's graveyard to player A's hand
        assert_eq!(game.get_hand(player_a).len(), 1);

        let opponent_graveyard_count = game
            .entities
            .values()
            .filter(|e| e.owner == player_b && e.location == Location::Graveyard)
            .count();
        assert_eq!(opponent_graveyard_count, 1);
    }

    #[test]
    fn test_steal_card_from_field_moves_to_thief_hand() {
        // a) Initialize
        let mut game = create_test_game();
        let player_a = game.player_id_a;
        let player_b = game.player_id_b;

        // b) Modify state: add monsters to opponent's field
        create_test_monster(&mut game, player_b, 0, 5, 5);
        create_test_monster(&mut game, player_b, 2, 3, 3);

        game.players.get_mut(&player_a).unwrap().mana = 5;

        // Verify initial state
        assert_eq!(game.get_field(player_b).len(), 2);
        assert_eq!(game.get_hand(player_a).len(), 0);

        // c) Test: play a spell that steals from field
        let steal_spell = create_test_spell(
            &mut game,
            player_a,
            vec![Effect::StealCard {
                initiator: 0,
                location: Location::Field(0), // Any field position
            }],
        );
        crate::game::user_actions::play_spell::play_spell(&mut game, player_a, steal_spell, None)
            .unwrap();
        game.compute_commands().unwrap();

        // d) Assert: card stolen from opponent's field to player A's hand
        assert_eq!(game.get_hand(player_a).len(), 1);
        assert_eq!(game.get_field(player_b).len(), 1);

        // Verify the stolen card is now in hand
        let hand = game.get_hand(player_a);
        let stolen_card = hand.values().next().unwrap();
        assert_eq!(stolen_card.owner, player_a);
        assert_eq!(stolen_card.location, Location::Hand);
    }

    #[test]
    fn test_steal_card_with_full_hand_goes_to_graveyard() {
        // a) Initialize
        let mut game = create_test_game();
        let player_a = game.player_id_a;
        let player_b = game.player_id_b;

        // b) Modify state: fill player A's hand with 10 cards
        for _ in 0..10 {
            add_card_to_hand(&mut game, player_a);
        }

        // Add a card to opponent's deck to steal
        add_card_to_deck(&mut game, player_b);

        game.players.get_mut(&player_a).unwrap().mana = 5;

        // Verify initial state
        assert_eq!(game.get_hand(player_a).len(), 10);

        let initial_graveyard_count = game
            .entities
            .values()
            .filter(|e| e.owner == player_a && e.location == Location::Graveyard)
            .count();

        // c) Test: play a spell that steals from deck (hand is full)
        let steal_spell = create_test_spell(
            &mut game,
            player_a,
            vec![Effect::StealCard {
                initiator: 0,
                location: Location::Deck,
            }],
        );
        crate::game::user_actions::play_spell::play_spell(&mut game, player_a, steal_spell, None)
            .unwrap();
        game.compute_commands().unwrap();

        // d) Assert: hand still at max (10), stolen card went to graveyard
        assert_eq!(game.get_hand(player_a).len(), 10);

        let new_graveyard_count = game
            .entities
            .values()
            .filter(|e| e.owner == player_a && e.location == Location::Graveyard)
            .count();

        // Should have increased by 2: the played spell + the stolen card
        assert_eq!(new_graveyard_count, initial_graveyard_count + 2);

        // Verify opponent's deck is now empty
        let opponent_deck_count = game
            .entities
            .values()
            .filter(|e| e.owner == player_b && e.location == Location::Deck)
            .count();
        assert_eq!(opponent_deck_count, 0);
    }

    #[test]
    fn test_steal_card_from_empty_location_does_nothing() {
        // a) Initialize
        let mut game = create_test_game();
        let player_a = game.player_id_a;
        let player_b = game.player_id_b;

        // b) Modify state: ensure opponent's deck is empty
        game.players.get_mut(&player_a).unwrap().mana = 5;

        // Verify initial state
        let opponent_deck_count = game
            .entities
            .values()
            .filter(|e| e.owner == player_b && e.location == Location::Deck)
            .count();
        assert_eq!(opponent_deck_count, 0);
        assert_eq!(game.get_hand(player_a).len(), 0);

        // c) Test: play a spell that tries to steal from empty deck
        let steal_spell = create_test_spell(
            &mut game,
            player_a,
            vec![Effect::StealCard {
                initiator: 0,
                location: Location::Deck,
            }],
        );
        crate::game::user_actions::play_spell::play_spell(&mut game, player_a, steal_spell, None)
            .unwrap();
        game.compute_commands().unwrap();

        // d) Assert: nothing was stolen, hand remains empty
        assert_eq!(game.get_hand(player_a).len(), 0);
    }

    #[test]
    fn test_stolen_card_changes_owner() {
        // a) Initialize
        let mut game = create_test_game();
        let player_a = game.player_id_a;
        let player_b = game.player_id_b;

        // b) Modify state: add a specific card to opponent's deck
        let card_id = add_card_to_deck(&mut game, player_b);

        game.players.get_mut(&player_a).unwrap().mana = 5;

        // Verify initial state
        assert_eq!(game.entities.get(&card_id).unwrap().owner, player_b);
        assert_eq!(
            game.entities.get(&card_id).unwrap().location,
            Location::Deck
        );

        // c) Test: play a spell that steals the card
        let steal_spell = create_test_spell(
            &mut game,
            player_a,
            vec![Effect::StealCard {
                initiator: 0,
                location: Location::Deck,
            }],
        );
        crate::game::user_actions::play_spell::play_spell(&mut game, player_a, steal_spell, None)
            .unwrap();
        game.compute_commands().unwrap();

        // d) Assert: card now belongs to player A and is in their hand
        assert_eq!(game.entities.get(&card_id).unwrap().owner, player_a);
        assert_eq!(
            game.entities.get(&card_id).unwrap().location,
            Location::Hand
        );
    }

    #[test]
    fn test_steal_card_from_empty_hand_does_nothing() {
        // a) Initialize
        let mut game = create_test_game();
        let player_a = game.player_id_a;
        let player_b = game.player_id_b;

        // b) Modify state: ensure opponent's hand is empty
        game.players.get_mut(&player_a).unwrap().mana = 5;

        assert_eq!(game.get_hand(player_b).len(), 0);

        // c) Test: play a spell that tries to steal from empty hand
        let steal_spell = create_test_spell(
            &mut game,
            player_a,
            vec![Effect::StealCard {
                initiator: 0,
                location: Location::Hand,
            }],
        );
        crate::game::user_actions::play_spell::play_spell(&mut game, player_a, steal_spell, None)
            .unwrap();
        game.compute_commands().unwrap();

        // d) Assert: nothing was stolen
        assert_eq!(game.get_hand(player_a).len(), 0);
    }

    #[test]
    fn test_steal_card_from_empty_field_does_nothing() {
        // a) Initialize
        let mut game = create_test_game();
        let player_a = game.player_id_a;
        let player_b = game.player_id_b;

        // b) Modify state: ensure opponent's field is empty
        game.players.get_mut(&player_a).unwrap().mana = 5;

        assert_eq!(game.get_field(player_b).len(), 0);

        // c) Test: play a spell that tries to steal from empty field
        let steal_spell = create_test_spell(
            &mut game,
            player_a,
            vec![Effect::StealCard {
                initiator: 0,
                location: Location::Field(0),
            }],
        );
        crate::game::user_actions::play_spell::play_spell(&mut game, player_a, steal_spell, None)
            .unwrap();
        game.compute_commands().unwrap();

        // d) Assert: nothing was stolen
        assert_eq!(game.get_hand(player_a).len(), 0);
    }
}
