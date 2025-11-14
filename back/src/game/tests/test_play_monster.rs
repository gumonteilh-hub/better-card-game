// FUNCTIONAL RULES: play_monster()
//
// 1. The card must be in hand (Location::Hand)
// 2. The player must have enough mana (player.mana >= card.cost)
// 3. Mana is consumed (player.mana -= card.cost)
// 4. The target position must be empty
// 5. The board must not be full (< 8 cards on field, positions 0-7)
// 6. Effect::SummonFromHand is queued for execution

#[cfg(test)]
mod tests {
    use super::super::test_utils::{
        add_card_to_hand, create_test_game, create_test_monster, create_test_monster_in_hand,
    };
    use crate::game::card::CardTypeInstance;
    use crate::game::types::Location;

    #[test]
    fn test_play_monster_card_must_be_in_hand() {
        // a) Initialize
        let mut game = create_test_game();
        let player_a = game.player_id_a;

        // b) Modify state: create a monster in deck (not in hand)
        use crate::game::card::{CardInstance, MonsterInstance};
        use crate::{collection::Class, Race};

        let monster_id = 100;
        let monster = CardInstance {
            id: monster_id,
            name: "Test Monster".to_string(),
            description: "Test".to_string(),
            template_id: 9999,
            race: Race::COMMON,
            class: Class::COMMON,
            cost: 3,
            owner: player_a,
            location: Location::Deck, // In deck, not hand
            card_type: CardTypeInstance::Monster(MonsterInstance {
                attack: 2,
                hp: 5,
                max_hp: 5,
                asleep: true,
                attack_count: 0,
                keywords: vec![],
                on_play: vec![],
                on_attack: vec![],
                on_death: vec![],
            }),
        };
        game.entities.insert(monster_id, monster);
        game.players.get_mut(&player_a).unwrap().mana = 5;

        // c) Test: try to play monster from deck
        let result = game.play_monster(player_a, monster_id, 0);

        // d) Assert the play failed
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Game Logic Error: This card must be in your hand to play it"
        );
    }

    #[test]
    fn test_play_monster_requires_enough_mana() {
        // a) Initialize
        let mut game = create_test_game();
        let player_a = game.player_id_a;

        // b) Modify state: create a 5-cost monster, but player has only 3 mana
        use crate::game::card::{CardInstance, MonsterInstance};
        use crate::{collection::Class, Race};

        let monster_id = 100;
        let monster = CardInstance {
            id: monster_id,
            name: "Expensive Monster".to_string(),
            description: "Test".to_string(),
            template_id: 9999,
            race: Race::COMMON,
            class: Class::COMMON,
            cost: 5, // Costs 5 mana
            owner: player_a,
            location: Location::Hand,
            card_type: CardTypeInstance::Monster(MonsterInstance {
                attack: 5,
                hp: 5,
                max_hp: 5,
                asleep: true,
                attack_count: 0,
                keywords: vec![],
                on_play: vec![],
                on_attack: vec![],
                on_death: vec![],
            }),
        };
        game.entities.insert(monster_id, monster);
        game.players.get_mut(&player_a).unwrap().mana = 3; // Only 3 mana

        // c) Test: try to play expensive monster without enough mana
        let result = game.play_monster(player_a, monster_id, 0);

        // d) Assert the play failed
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Game Logic Error: You don't have enough mana to play this card"
        );
    }

    #[test]
    fn test_play_monster_consumes_mana() {
        // a) Initialize
        let mut game = create_test_game();
        let player_a = game.player_id_a;

        // b) Modify state: create a 3-cost monster, player has 5 mana
        let monster_id = create_test_monster_in_hand(&mut game, player_a, 2, 5, vec![], vec![]);

        // Set monster cost and player mana
        game.entities.get_mut(&monster_id).unwrap().cost = 3;
        game.players.get_mut(&player_a).unwrap().mana = 5;

        // Verify mana before
        assert_eq!(game.players.get(&player_a).unwrap().mana, 5);

        // c) Test: play the monster
        game.play_monster(player_a, monster_id, 0).unwrap();

        // d) Assert mana was consumed (5 - 3 = 2)
        assert_eq!(game.players.get(&player_a).unwrap().mana, 2);
    }

    #[test]
    fn test_play_monster_position_must_be_empty() {
        // a) Initialize
        let mut game = create_test_game();
        let player_a = game.player_id_a;

        // b) Modify state: place a monster at position 0, then try to play another there
        let existing_monster = create_test_monster(&mut game, player_a, 0, 5, 5);

        let new_monster = create_test_monster_in_hand(&mut game, player_a, 2, 5, vec![], vec![]);
        game.players.get_mut(&player_a).unwrap().mana = 5;

        // c) Test: try to play new monster at occupied position 0
        let result = game.play_monster(player_a, new_monster, 0);

        // d) Assert the play failed
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Game Logic Error: This place on the field is not empty"
        );
    }

    #[test]
    fn test_play_monster_board_must_not_be_full() {
        // a) Initialize
        let mut game = create_test_game();
        let player_a = game.player_id_a;

        // b) Modify state: fill the board with 7 monsters (positions 0-6)
        for pos in 0..7 {
            create_test_monster(&mut game, player_a, pos, 5, 5);
        }

        // Create a monster in hand to try to play
        let new_monster = create_test_monster_in_hand(&mut game, player_a, 2, 5, vec![], vec![]);
        game.players.get_mut(&player_a).unwrap().mana = 5;

        // Verify board has 7 monsters (at capacity)
        assert_eq!(game.get_field(player_a).len(), 7);

        // c) Test: try to play 8th monster at empty position 7
        let result = game.play_monster(player_a, new_monster, 7);

        // d) Assert the play failed
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Game Logic Error: Your board is already full"
        );
    }

    #[test]
    fn test_play_monster_queues_summon_effect() {
        // a) Initialize
        let mut game = create_test_game();
        let player_a = game.player_id_a;

        // b) Modify state: create a monster in hand
        let monster_id = create_test_monster_in_hand(&mut game, player_a, 3, 5, vec![], vec![]);
        game.players.get_mut(&player_a).unwrap().mana = 5;

        // Verify effect queue is empty
        assert_eq!(game.effect_queue.len(), 0);

        // c) Test: play the monster
        game.play_monster(player_a, monster_id, 0).unwrap();

        // d) Assert SummonFromHand effect was queued
        assert_eq!(game.effect_queue.len(), 1);

        // Verify it's the right effect
        use crate::game::effects::Effect;
        if let Some(Effect::SummonFromHand {
            entity_id,
            position,
        }) = game.effect_queue.front()
        {
            assert_eq!(*entity_id, monster_id);
            assert_eq!(*position, 0);
        } else {
            panic!("Expected SummonFromHand effect in queue");
        }
    }
}
