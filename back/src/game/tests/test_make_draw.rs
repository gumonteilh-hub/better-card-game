// FUNCTIONAL RULES: MakeDraw Effect
//
// 1. Draw moves cards from deck to hand
// 2. Hand has a maximum of 10 cards
// 3. Drawing with full hand burns cards to graveyard
// 4. Drawing from empty deck has no effect
// 5. Can target self or opponent player

#[cfg(test)]
mod tests {
    use super::super::test_utils::{add_card_to_deck, create_test_game, create_test_spell};
    use crate::collection::Class;
    use crate::game::effects::{Effect, PlayerTarget};
    use crate::game::types::Location;
    use crate::{
        Race,
        game::card::{CardInstance, CardTypeInstance, MonsterInstance},
    };

    #[test]
    fn test_draw_moves_card_from_deck_to_hand() {
        let mut game = create_test_game();
        let player_a = game.player_id_a;

        add_card_to_deck(&mut game, player_a);
        add_card_to_deck(&mut game, player_a);
        add_card_to_deck(&mut game, player_a);
        add_card_to_deck(&mut game, player_a);
        game.players.get_mut(&player_a).unwrap().mana = 5;

        let deck_count = game
            .entities
            .values()
            .filter(|e| e.owner == player_a && e.location == Location::Deck)
            .count();
        assert_eq!(deck_count, 4);

        let draw_spell = create_test_spell(
            &mut game,
            player_a,
            vec![Effect::MakeDraw {
                initiator: 0,
                player: PlayerTarget::Player,
                amount: 2,
            }],
        );
        game.play_spell(player_a, draw_spell).unwrap();
        game.compute_commands().unwrap();

        assert_eq!(game.get_hand(player_a).len(), 2);
        let deck_count = game
            .entities
            .values()
            .filter(|e| e.owner == player_a && e.location == Location::Deck)
            .count();
        assert_eq!(deck_count, 2);
    }

    #[test]
    fn test_hand_has_maximum_of_10_cards() {
        let mut game = create_test_game();
        let player_a = game.player_id_a;

        for _ in 0..15 {
            add_card_to_deck(&mut game, player_a);
        }
        game.players.get_mut(&player_a).unwrap().mana = 5;

        let draw_spell = create_test_spell(
            &mut game,
            player_a,
            vec![Effect::MakeDraw {
                initiator: 0,
                player: PlayerTarget::Player,
                amount: 12,
            }],
        );
        game.play_spell(player_a, draw_spell).unwrap();
        game.compute_commands().unwrap();

        assert_eq!(game.get_hand(player_a).len(), 10);

        for (id, entity) in game.entities.iter() {
            if entity.owner == player_a {
                println!("ID: {}, location: {:?}", id, entity.location);
            }
        }

        let graveyard_count = game
            .entities
            .values()
            .filter(|e| e.owner == player_a && e.location == Location::Graveyard)
            .count();
        assert_eq!(graveyard_count, 3); // 2 burned cards + 1 played spell card

        let deck_count = game
            .entities
            .values()
            .filter(|e| e.owner == player_a && e.location == Location::Deck)
            .count();
        assert_eq!(deck_count, 3);
    }

    #[test]
    fn test_drawing_with_full_hand_burns_cards_to_graveyard() {
        let mut game = create_test_game();
        let player_a = game.player_id_a;

        for _ in 0..10 {
            let card_id = game.entities.len() + 2;
            let card = CardInstance {
                id: card_id,
                name: "Test Card in Hand".to_string(),
                description: "Test".to_string(),
                template_id: 9996,
                race: Race::COMMON,
                class: Class::COMMON,
                cost: 0,
                owner: player_a,
                location: Location::Hand,
                card_type: CardTypeInstance::Monster(MonsterInstance {
                    attack: 1,
                    hp: 1,
                    max_hp: 1,
                    asleep: true,
                    attack_count: 0,
                    keywords: vec![],
                    on_play: vec![],
                    on_attack: vec![],
                    on_death: vec![],
                }),
            };
            game.entities.insert(card_id, card);
        }

        add_card_to_deck(&mut game, player_a);
        add_card_to_deck(&mut game, player_a);
        add_card_to_deck(&mut game, player_a);
        add_card_to_deck(&mut game, player_a);
        game.players.get_mut(&player_a).unwrap().mana = 5;

        assert_eq!(game.get_hand(player_a).len(), 10);

        let draw_spell = create_test_spell(
            &mut game,
            player_a,
            vec![Effect::MakeDraw {
                initiator: 0,
                player: PlayerTarget::Player,
                amount: 3,
            }],
        );
        game.play_spell(player_a, draw_spell).unwrap();
        game.compute_commands().unwrap();

        assert_eq!(game.get_hand(player_a).len(), 10);
        let graveyard_count = game
            .entities
            .values()
            .filter(|e| e.owner == player_a && e.location == Location::Graveyard)
            .count();
        assert_eq!(graveyard_count, 4); // 3 burned + 1 played spell

        let deck_count = game
            .entities
            .values()
            .filter(|e| e.owner == player_a && e.location == Location::Deck)
            .count();
        assert_eq!(deck_count, 1);
    }

    #[test]
    fn test_drawing_from_empty_deck_has_no_effect() {
        let mut game = create_test_game();
        let player_a = game.player_id_a;

        game.players.get_mut(&player_a).unwrap().mana = 5;

        let deck_size = game
            .entities
            .values()
            .filter(|e| e.owner == player_a && e.location == Location::Deck)
            .count();
        assert_eq!(deck_size, 0);

        let hand_size_before = game.get_hand(player_a).len();

        let draw_spell = create_test_spell(
            &mut game,
            player_a,
            vec![Effect::MakeDraw {
                initiator: 0,
                player: PlayerTarget::Player,
                amount: 3,
            }],
        );
        game.play_spell(player_a, draw_spell).unwrap();
        game.compute_commands().unwrap();

        let hand_size_after = game.get_hand(player_a).len();
        assert_eq!(hand_size_before, hand_size_after);
    }

    #[test]
    fn test_make_draw_can_target_opponent() {
        let mut game = create_test_game();
        let player_a = game.player_id_a;
        let player_b = game.player_id_b;

        add_card_to_deck(&mut game, player_b);
        add_card_to_deck(&mut game, player_b);
        game.players.get_mut(&player_a).unwrap().mana = 5;

        let deck_count = game
            .entities
            .values()
            .filter(|e| e.owner == player_b && e.location == Location::Deck)
            .count();
        assert_eq!(deck_count, 2);
        assert_eq!(game.get_hand(player_b).len(), 0);

        let draw_spell = create_test_spell(
            &mut game,
            player_a,
            vec![Effect::MakeDraw {
                initiator: 0,
                player: PlayerTarget::EnnemyPlayer,
                amount: 2,
            }],
        );
        game.play_spell(player_a, draw_spell).unwrap();
        game.compute_commands().unwrap();

        assert_eq!(game.get_hand(player_b).len(), 2);
        let deck_count = game
            .entities
            .values()
            .filter(|e| e.owner == player_b && e.location == Location::Deck)
            .count();
        assert_eq!(deck_count, 0);

        assert_eq!(game.get_hand(player_a).len(), 0);
    }

    #[test]
    fn test_multiple_draws_from_partial_deck() {
        let mut game = create_test_game();
        let player_a = game.player_id_a;

        add_card_to_deck(&mut game, player_a);
        add_card_to_deck(&mut game, player_a);
        game.players.get_mut(&player_a).unwrap().mana = 5;

        let deck_count = game
            .entities
            .values()
            .filter(|e| e.owner == player_a && e.location == Location::Deck)
            .count();
        assert_eq!(deck_count, 2);

        let draw_spell = create_test_spell(
            &mut game,
            player_a,
            vec![Effect::MakeDraw {
                initiator: 0,
                player: PlayerTarget::Player,
                amount: 5,
            }],
        );
        game.play_spell(player_a, draw_spell).unwrap();
        game.compute_commands().unwrap();

        assert_eq!(game.get_hand(player_a).len(), 2);
        let deck_count = game
            .entities
            .values()
            .filter(|e| e.owner == player_a && e.location == Location::Deck)
            .count();
        assert_eq!(deck_count, 0);
    }
}
