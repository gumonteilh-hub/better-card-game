// FUNCTIONAL RULES: play_monster()
//
// 1. The card must be in hand (Location::Hand)
// 2. The player must have enough mana (player.mana >= card.cost)
// 3. Mana is consumed (player.mana -= card.cost)
// 4. The target position must be empty
// 5. The board must not be full (< 8 cards on field, positions 0-7)
// 6. The monster is placed directly on the field at the specified position
// 7. If the monster has play_target and targets are provided, on_play effects are queued with selected targets
// 8. If the monster has play_target but no targets are provided, on_play effects are queued with empty target list (Target::Ids([]))

#[cfg(test)]
mod tests {
    use super::super::test_utils::{
        create_test_game, create_test_monster, create_test_monster_in_hand,
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
        use crate::{Race, collection::Class};

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
            play_target: None,
        };
        game.entities.insert(monster_id, monster);
        game.players.get_mut(&player_a).unwrap().mana = 5;

        // c) Test: try to play monster from deck
        let result = game.play_monster(player_a, monster_id, 0, None);

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
        use crate::{Race, collection::Class};

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
            play_target: None,
        };
        game.entities.insert(monster_id, monster);
        game.players.get_mut(&player_a).unwrap().mana = 3; // Only 3 mana

        // c) Test: try to play expensive monster without enough mana
        let result = game.play_monster(player_a, monster_id, 0, None);

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
        game.play_monster(player_a, monster_id, 0, None).unwrap();

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
        let result = game.play_monster(player_a, new_monster, 0, None);

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

        // b) Modify state: fill the board with 8 monsters (all positions 0-7)
        for pos in 0..8 {
            create_test_monster(&mut game, player_a, pos, 5, 5);
        }

        // Create a monster in hand to try to play
        let new_monster = create_test_monster_in_hand(&mut game, player_a, 2, 5, vec![], vec![]);
        game.players.get_mut(&player_a).unwrap().mana = 5;

        // Verify board is full with 8 monsters
        assert_eq!(game.get_field(player_a).len(), 8);

        // c) Test: try to play 9th monster (no position available)
        let result = game.play_monster(player_a, new_monster, 0, None);

        // d) Assert the play failed
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Game Logic Error: Your board is already full"
        );
    }

    #[test]
    fn test_play_monster_places_monster_on_field() {
        // a) Initialize
        let mut game = create_test_game();
        let player_a = game.player_id_a;

        // b) Modify state: create a monster in hand
        let monster_id = create_test_monster_in_hand(&mut game, player_a, 3, 5, vec![], vec![]);
        game.players.get_mut(&player_a).unwrap().mana = 5;

        // Verify effect queue is empty
        assert_eq!(game.effect_queue.len(), 0);

        // c) Test: play the monster
        game.play_monster(player_a, monster_id, 0, None).unwrap();

        // d) Assert monster is on field at position 0
        let monster = game.entities.get(&monster_id).unwrap();
        assert_eq!(monster.location, Location::Field(0));

        // Verify monster is asleep (just summoned)
        if let CardTypeInstance::Monster(monster_instance) = &monster.card_type {
            assert_eq!(monster_instance.asleep, true);
        } else {
            panic!("Expected monster card type");
        }

        // Verify effect queue is still empty (no effects queued)
        assert_eq!(game.effect_queue.len(), 0);
    }

    #[test]
    fn test_play_monster_with_play_target_and_targets_provided() {
        // a) Initialize
        let mut game = create_test_game();
        let player_a = game.player_id_a;
        let player_b = game.player_id_b;

        // b) Modify state: create enemy monsters to target and a monster with play_target
        use crate::collection::types::{PlayTarget, TargetMatcher};
        use crate::game::card::{CardInstance, MonsterInstance};
        use crate::game::effects::{Effect, Target};
        use crate::{Race, collection::Class};

        let enemy_monster_1 = create_test_monster(&mut game, player_b, 0, 3, 5);
        let enemy_monster_2 = create_test_monster(&mut game, player_b, 1, 3, 5);

        // Create a monster that can target 2 enemy monsters and deal damage to them
        let monster_id = game.entities.len() + 100;

        let monster = CardInstance {
            id: monster_id,
            name: "Targeted Monster".to_string(),
            description: "Deals 2 damage to selected enemies".to_string(),
            template_id: 9999,
            race: Race::COMMON,
            class: Class::COMMON,
            cost: 3,
            owner: player_a,
            location: Location::Hand,
            card_type: CardTypeInstance::Monster(MonsterInstance {
                attack: 2,
                hp: 3,
                max_hp: 3,
                asleep: true,
                attack_count: 0,
                keywords: vec![],
                on_play: vec![Effect::DealDamage {
                    initiator: monster_id,
                    target: Target::Ids(vec![]),
                    amount: 2,
                }],
                on_attack: vec![],
                on_death: vec![],
            }),
            play_target: Some(PlayTarget {
                strict: false,
                amount: 2,
                matcher: TargetMatcher::Owner(player_b),
            }),
        };
        game.entities.insert(monster_id, monster);
        game.players.get_mut(&player_a).unwrap().mana = 5;

        // Verify effect queue is empty
        assert_eq!(game.effect_queue.len(), 0);

        // c) Test: play the monster with targets
        game.play_monster(
            player_a,
            monster_id,
            0,
            Some(vec![enemy_monster_1, enemy_monster_2]),
        )
        .unwrap();

        // d) Assert monster is on field
        assert_eq!(
            game.entities.get(&monster_id).unwrap().location,
            Location::Field(0)
        );

        // Verify effect was queued with the selected targets
        assert_eq!(game.effect_queue.len(), 1);
        if let Some(Effect::DealDamage { target, amount, .. }) = game.effect_queue.front() {
            assert_eq!(*amount, 2);
            assert!(
                matches!(target, Target::Ids(ids) if ids.len() == 2 && ids.contains(&enemy_monster_1) && ids.contains(&enemy_monster_2))
            );
        } else {
            panic!("Expected DealDamage effect in queue");
        }
    }

    #[test]
    fn test_play_monster_with_play_target_but_no_targets_provided() {
        // a) Initialize
        let mut game = create_test_game();
        let player_a = game.player_id_a;
        let player_b = game.player_id_b;

        // b) Modify state: create a monster with play_target
        use crate::collection::types::{PlayTarget, TargetMatcher};
        use crate::game::card::{CardInstance, MonsterInstance};
        use crate::game::effects::{Effect, Target};
        use crate::{Race, collection::Class};

        let monster_id = game.entities.len() + 100;

        let monster = CardInstance {
            id: monster_id,
            name: "Targeted Monster".to_string(),
            description: "Requires target".to_string(),
            template_id: 9999,
            race: Race::COMMON,
            class: Class::COMMON,
            cost: 3,
            owner: player_a,
            location: Location::Hand,
            card_type: CardTypeInstance::Monster(MonsterInstance {
                attack: 2,
                hp: 3,
                max_hp: 3,
                asleep: true,
                attack_count: 0,
                keywords: vec![],
                on_play: vec![Effect::DealDamage {
                    initiator: monster_id,
                    target: Target::Ids(vec![]),
                    amount: 2,
                }],
                on_attack: vec![],
                on_death: vec![],
            }),
            play_target: Some(PlayTarget {
                strict: false,
                amount: 1,
                matcher: TargetMatcher::Owner(player_b),
            }),
        };
        game.entities.insert(monster_id, monster);
        game.players.get_mut(&player_a).unwrap().mana = 5;

        // Verify effect queue is empty
        assert_eq!(game.effect_queue.len(), 0);

        // c) Test: play the monster without providing targets
        game.play_monster(player_a, monster_id, 0, None).unwrap();

        // d) Assert monster is on field
        assert_eq!(
            game.entities.get(&monster_id).unwrap().location,
            Location::Field(0)
        );

        // Verify effect was queued but with empty target list
        assert_eq!(game.effect_queue.len(), 1);
        if let Some(Effect::DealDamage { target, amount, .. }) = game.effect_queue.front() {
            assert_eq!(*amount, 2);
            assert!(matches!(target, Target::Ids(ids) if ids.is_empty()));
        } else {
            panic!("Expected DealDamage effect in queue");
        }
    }

    #[test]
    fn test_play_monster_without_play_target_but_targets_provided() {
        // a) Initialize
        let mut game = create_test_game();
        let player_a = game.player_id_a;
        let player_b = game.player_id_b;

        // b) Modify state: create a regular monster (no play_target)
        let monster_id = create_test_monster_in_hand(&mut game, player_a, 3, 5, vec![], vec![]);
        game.players.get_mut(&player_a).unwrap().mana = 5;

        let enemy_monster = create_test_monster(&mut game, player_b, 0, 3, 5);

        // c) Test: try to play the monster with targets (should fail or ignore)
        let result = game.play_monster(player_a, monster_id, 0, Some(vec![enemy_monster]));

        // d) Assert: This should either fail with an error or succeed by ignoring the targets
        // Based on the code, it should succeed and ignore the targets
        assert!(result.is_ok());
        assert_eq!(
            game.entities.get(&monster_id).unwrap().location,
            Location::Field(0)
        );
    }

    #[test]
    fn test_play_monster_with_invalid_targets() {
        // a) Initialize
        let mut game = create_test_game();
        let player_a = game.player_id_a;
        let player_b = game.player_id_b;

        // b) Modify state: create a monster with play_target that requires enemy targets
        use crate::collection::types::{PlayTarget, TargetMatcher};
        use crate::game::card::{CardInstance, MonsterInstance};
        use crate::game::effects::{Effect, Target};
        use crate::{Race, collection::Class};

        // Create a friendly monster and an enemy monster
        let friendly_monster = create_test_monster(&mut game, player_a, 2, 3, 5);
        let enemy_monster = create_test_monster(&mut game, player_b, 0, 3, 5);

        let monster_id = game.entities.len() + 100;

        let monster = CardInstance {
            id: monster_id,
            name: "Enemy Targeter".to_string(),
            description: "Can only target enemies".to_string(),
            template_id: 9999,
            race: Race::COMMON,
            class: Class::COMMON,
            cost: 3,
            owner: player_a,
            location: Location::Hand,
            card_type: CardTypeInstance::Monster(MonsterInstance {
                attack: 2,
                hp: 3,
                max_hp: 3,
                asleep: true,
                attack_count: 0,
                keywords: vec![],
                on_play: vec![Effect::DealDamage {
                    initiator: monster_id,
                    target: Target::Ids(vec![]),
                    amount: 2,
                }],
                on_attack: vec![],
                on_death: vec![],
            }),
            play_target: Some(PlayTarget {
                strict: false,
                amount: 1,
                matcher: TargetMatcher::Owner(player_b), // Only enemy targets
            }),
        };
        game.entities.insert(monster_id, monster);
        game.players.get_mut(&player_a).unwrap().mana = 5;

        // c) Test: try to play the monster targeting a friendly monster (should fail)
        let result = game.play_monster(player_a, monster_id, 0, Some(vec![friendly_monster]));

        // d) Assert the play failed
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Game Logic Error: You selected a target that doesn't match the card conditions"
        );
    }

    #[test]
    fn test_play_monster_with_strict_target_exact_amount() {
        // a) Initialize
        let mut game = create_test_game();
        let player_a = game.player_id_a;
        let player_b = game.player_id_b;

        // b) Modify state: create enemy monsters to target and a monster with strict play_target
        use crate::collection::types::{PlayTarget, TargetMatcher};
        use crate::game::card::{CardInstance, MonsterInstance};
        use crate::game::effects::{Effect, Target};
        use crate::{Race, collection::Class};

        let enemy_monster_1 = create_test_monster(&mut game, player_b, 0, 3, 5);
        let enemy_monster_2 = create_test_monster(&mut game, player_b, 1, 3, 5);

        // Create a monster that REQUIRES exactly 2 enemy targets (strict mode)
        let monster_id = game.entities.len() + 100;

        let monster = CardInstance {
            id: monster_id,
            name: "Strict Targeter".to_string(),
            description: "Requires exactly 2 targets".to_string(),
            template_id: 9999,
            race: Race::COMMON,
            class: Class::COMMON,
            cost: 3,
            owner: player_a,
            location: Location::Hand,
            card_type: CardTypeInstance::Monster(MonsterInstance {
                attack: 2,
                hp: 3,
                max_hp: 3,
                asleep: true,
                attack_count: 0,
                keywords: vec![],
                on_play: vec![Effect::DealDamage {
                    initiator: monster_id,
                    target: Target::Ids(vec![]),
                    amount: 2,
                }],
                on_attack: vec![],
                on_death: vec![],
            }),
            play_target: Some(PlayTarget {
                strict: true,
                amount: 2,
                matcher: TargetMatcher::Owner(player_b),
            }),
        };
        game.entities.insert(monster_id, monster);
        game.players.get_mut(&player_a).unwrap().mana = 5;

        // c) Test: play the monster with exactly 2 targets (strict requirement)
        let result = game.play_monster(
            player_a,
            monster_id,
            0,
            Some(vec![enemy_monster_1, enemy_monster_2]),
        );

        // d) Assert the play succeeded
        assert!(result.is_ok());
        assert_eq!(
            game.entities.get(&monster_id).unwrap().location,
            Location::Field(0)
        );

        // Verify effect was queued with the selected targets
        assert_eq!(game.effect_queue.len(), 1);
        if let Some(Effect::DealDamage { target, amount, .. }) = game.effect_queue.front() {
            assert_eq!(*amount, 2);
            assert!(
                matches!(target, Target::Ids(ids) if ids.len() == 2 && ids.contains(&enemy_monster_1) && ids.contains(&enemy_monster_2))
            );
        } else {
            panic!("Expected DealDamage effect in queue");
        }
    }

    #[test]
    fn test_play_monster_with_strict_target_wrong_amount() {
        // a) Initialize
        let mut game = create_test_game();
        let player_a = game.player_id_a;
        let player_b = game.player_id_b;

        // b) Modify state: create enemy monsters and a monster with strict play_target
        use crate::collection::types::{PlayTarget, TargetMatcher};
        use crate::game::card::{CardInstance, MonsterInstance};
        use crate::game::effects::{Effect, Target};
        use crate::{Race, collection::Class};

        let enemy_monster_1 = create_test_monster(&mut game, player_b, 0, 3, 5);
        let _enemy_monster_2 = create_test_monster(&mut game, player_b, 1, 3, 5);

        // Create a monster that REQUIRES exactly 2 enemy targets (strict mode)
        let monster_id = game.entities.len() + 100;

        let monster = CardInstance {
            id: monster_id,
            name: "Strict Targeter".to_string(),
            description: "Requires exactly 2 targets".to_string(),
            template_id: 9999,
            race: Race::COMMON,
            class: Class::COMMON,
            cost: 3,
            owner: player_a,
            location: Location::Hand,
            card_type: CardTypeInstance::Monster(MonsterInstance {
                attack: 2,
                hp: 3,
                max_hp: 3,
                asleep: true,
                attack_count: 0,
                keywords: vec![],
                on_play: vec![Effect::DealDamage {
                    initiator: monster_id,
                    target: Target::Ids(vec![]),
                    amount: 2,
                }],
                on_attack: vec![],
                on_death: vec![],
            }),
            play_target: Some(PlayTarget {
                strict: true,
                amount: 2,
                matcher: TargetMatcher::Owner(player_b),
            }),
        };
        game.entities.insert(monster_id, monster);
        game.players.get_mut(&player_a).unwrap().mana = 5;

        // c) Test: play the monster with only 1 target (should fail because strict requires exactly 2)
        let result = game.play_monster(player_a, monster_id, 0, Some(vec![enemy_monster_1]));

        // d) Assert the play failed with the correct error message
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Game Logic Error: Wrong quantity of targets selected (required: 2)"
        );
    }
}
