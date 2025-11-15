// FUNCTIONAL RULES: play_spell()
//
// 1. The card must be in hand (Location::Hand)
// 2. The player must have enough mana (player.mana >= card.cost)
// 3. Mana is consumed (player.mana -= card.cost)
// 4. The card must be a spell (not a monster)
// 5. The spell's effects are added to the effect queue
// 6. The spell goes to the graveyard after being played

#[cfg(test)]
mod tests {
    use super::super::test_utils::create_test_spell;
    use crate::game::effects::{Effect, Target};
    use crate::game::types::Location;

    #[test]
    fn test_play_spell_card_must_be_in_hand() {
        // a) Initialize
        let mut game = super::super::test_utils::create_test_game();
        let player_a = game.player_id_a;

        // b) Modify state: create a spell in deck (not in hand)
        use crate::game::card::{CardInstance, CardTypeInstance, SpellInstance};
        use crate::{Race, collection::Class};

        let spell_id = 100;
        let spell = CardInstance {
            id: spell_id,
            name: "Test Spell".to_string(),
            description: "Test".to_string(),
            template_id: 9999,
            race: Race::COMMON,
            class: Class::COMMON,
            cost: 2,
            owner: player_a,
            location: Location::Deck, // In deck, not hand
            card_type: CardTypeInstance::Spell(SpellInstance {
                effect: vec![Effect::DealDamage {
                    initiator: spell_id,
                    target: Target::EnnemyPlayer,
                    amount: 3,
                }],
            }),
            play_target: None,
        };
        game.entities.insert(spell_id, spell);
        game.players.get_mut(&player_a).unwrap().mana = 5;

        // c) Test: try to play spell from deck
        let result = game.play_spell(player_a, spell_id, None);

        // d) Assert the play failed
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Game Logic Error: This card must be in your hand to play it"
        );
    }

    #[test]
    fn test_play_spell_requires_enough_mana() {
        // a) Initialize
        let mut game = super::super::test_utils::create_test_game();
        let player_a = game.player_id_a;

        // b) Modify state: create a 5-cost spell, but player has only 3 mana
        use crate::game::card::{CardInstance, CardTypeInstance, SpellInstance};
        use crate::{Race, collection::Class};

        let spell_id = 100;
        let spell = CardInstance {
            id: spell_id,
            name: "Expensive Spell".to_string(),
            description: "Test".to_string(),
            template_id: 9999,
            race: Race::COMMON,
            class: Class::COMMON,
            cost: 5, // Costs 5 mana
            owner: player_a,
            location: Location::Hand,
            card_type: CardTypeInstance::Spell(SpellInstance {
                effect: vec![Effect::DealDamage {
                    initiator: spell_id,
                    target: Target::EnnemyPlayer,
                    amount: 5,
                }],
            }),
            play_target: None,
        };
        game.entities.insert(spell_id, spell);
        game.players.get_mut(&player_a).unwrap().mana = 3; // Only 3 mana

        // c) Test: try to play expensive spell without enough mana
        let result = game.play_spell(player_a, spell_id, None);

        // d) Assert the play failed
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Game Logic Error: You don't have enough mana to play this card"
        );
    }

    #[test]
    fn test_play_spell_consumes_mana() {
        // a) Initialize
        let mut game = super::super::test_utils::create_test_game();
        let player_a = game.player_id_a;

        // b) Modify state: create a 3-cost spell, player has 5 mana
        let spell_id = create_test_spell(
            &mut game,
            player_a,
            vec![Effect::DealDamage {
                initiator: 0,
                target: Target::EnnemyPlayer,
                amount: 2,
            }],
        );

        // Set spell cost and player mana
        game.entities.get_mut(&spell_id).unwrap().cost = 3;
        game.players.get_mut(&player_a).unwrap().mana = 5;

        // Verify mana before
        assert_eq!(game.players.get(&player_a).unwrap().mana, 5);

        // c) Test: play the spell
        game.play_spell(player_a, spell_id, None).unwrap();

        // d) Assert mana was consumed (5 - 3 = 2)
        assert_eq!(game.players.get(&player_a).unwrap().mana, 2);
    }

    #[test]
    fn test_play_spell_cannot_cast_monster() {
        // a) Initialize
        let mut game = super::super::test_utils::create_test_game();
        let player_a = game.player_id_a;

        // b) Modify state: create a monster in hand (not a spell)
        use super::super::test_utils::create_test_monster_in_hand;

        let monster_id = create_test_monster_in_hand(&mut game, player_a, 3, 5, vec![], vec![]);
        game.players.get_mut(&player_a).unwrap().mana = 5;

        // c) Test: try to cast monster as spell
        let result = game.play_spell(player_a, monster_id, None);

        // d) Assert the play failed
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Game Logic Error: You can not cast a monster, only a spell"
        );
    }

    #[test]
    fn test_play_spell_queues_effects() {
        // a) Initialize
        let mut game = super::super::test_utils::create_test_game();
        let player_a = game.player_id_a;

        // b) Modify state: create a spell with multiple effects
        let spell_id = create_test_spell(
            &mut game,
            player_a,
            vec![
                Effect::DealDamage {
                    initiator: 0,
                    target: Target::EnnemyPlayer,
                    amount: 3,
                },
                Effect::Heal {
                    initiator: 0,
                    target: Target::Player,
                    amount: 2,
                },
            ],
        );

        game.players.get_mut(&player_a).unwrap().mana = 5;

        // Verify effect queue is empty
        assert_eq!(game.effect_queue.len(), 0);

        // c) Test: play the spell
        game.play_spell(player_a, spell_id, None).unwrap();

        // d) Assert effects were queued (2 effects)
        assert_eq!(game.effect_queue.len(), 2);

        // Verify the effects are correct
        let effects: Vec<&Effect> = game.effect_queue.iter().collect();
        assert!(matches!(
            effects[0],
            Effect::DealDamage {
                target: Target::EnnemyPlayer,
                amount: 3,
                ..
            }
        ));
        assert!(matches!(
            effects[1],
            Effect::Heal {
                target: Target::Player,
                amount: 2,
                ..
            }
        ));
    }

    #[test]
    fn test_play_spell_goes_to_graveyard() {
        // a) Initialize
        let mut game = super::super::test_utils::create_test_game();
        let player_a = game.player_id_a;

        // b) Modify state: create a spell in hand
        let spell_id = create_test_spell(
            &mut game,
            player_a,
            vec![Effect::DealDamage {
                initiator: 0,
                target: Target::EnnemyPlayer,
                amount: 3,
            }],
        );

        game.players.get_mut(&player_a).unwrap().mana = 5;

        // Verify spell is in hand
        assert_eq!(
            game.entities.get(&spell_id).unwrap().location,
            Location::Hand
        );

        // c) Test: play the spell
        game.play_spell(player_a, spell_id, None).unwrap();

        // d) Assert spell went to graveyard
        assert_eq!(
            game.entities.get(&spell_id).unwrap().location,
            Location::Graveyard
        );
    }

    #[test]
    fn test_play_spell_with_play_target_and_targets_provided() {
        // a) Initialize
        let mut game = super::super::test_utils::create_test_game();
        let player_a = game.player_id_a;
        let player_b = game.player_id_b;

        // b) Modify state: create enemy monsters to target and a spell with play_target
        use super::super::test_utils::create_test_monster;
        use crate::collection::types::{PlayTarget, TargetMatcher};
        use crate::game::card::{CardInstance, CardTypeInstance, SpellInstance};
        use crate::{Race, collection::Class};

        let enemy_monster_1 = create_test_monster(&mut game, player_b, 0, 3, 5);
        let enemy_monster_2 = create_test_monster(&mut game, player_b, 1, 3, 5);

        // Create a spell that can target 2 enemy monsters and deal damage to them
        let spell_id = game.entities.len() + 100;

        let spell = CardInstance {
            id: spell_id,
            name: "Multi-Target Spell".to_string(),
            description: "Deals 3 damage to selected enemies".to_string(),
            template_id: 9999,
            race: Race::COMMON,
            class: Class::COMMON,
            cost: 4,
            owner: player_a,
            location: Location::Hand,
            card_type: CardTypeInstance::Spell(SpellInstance {
                effect: vec![Effect::DealDamage {
                    initiator: spell_id,
                    target: Target::Ids(vec![]),
                    amount: 3,
                }],
            }),
            play_target: Some(PlayTarget {
                strict: false,
                amount: 2,
                matcher: TargetMatcher::Owner(player_b),
            }),
        };
        game.entities.insert(spell_id, spell);
        game.players.get_mut(&player_a).unwrap().mana = 5;

        // Verify effect queue is empty
        assert_eq!(game.effect_queue.len(), 0);

        // c) Test: play the spell with targets
        game.play_spell(
            player_a,
            spell_id,
            Some(vec![enemy_monster_1, enemy_monster_2]),
        )
        .unwrap();

        // d) Assert spell went to graveyard
        assert_eq!(
            game.entities.get(&spell_id).unwrap().location,
            Location::Graveyard
        );

        // Verify effect was queued with the selected targets
        assert_eq!(game.effect_queue.len(), 1);
        if let Some(Effect::DealDamage { target, amount, .. }) = game.effect_queue.front() {
            assert_eq!(*amount, 3);
            assert!(
                matches!(target, Target::Ids(ids) if ids.len() == 2 && ids.contains(&enemy_monster_1) && ids.contains(&enemy_monster_2))
            );
        } else {
            panic!("Expected DealDamage effect in queue");
        }
    }

    #[test]
    fn test_play_spell_with_play_target_but_no_targets_provided() {
        // a) Initialize
        let mut game = super::super::test_utils::create_test_game();
        let player_a = game.player_id_a;
        let player_b = game.player_id_b;

        // b) Modify state: create a spell with play_target
        use crate::collection::types::{PlayTarget, TargetMatcher};
        use crate::game::card::{CardInstance, CardTypeInstance, SpellInstance};
        use crate::{Race, collection::Class};

        let spell_id = game.entities.len() + 100;

        let spell = CardInstance {
            id: spell_id,
            name: "Targeted Spell".to_string(),
            description: "Requires target".to_string(),
            template_id: 9999,
            race: Race::COMMON,
            class: Class::COMMON,
            cost: 3,
            owner: player_a,
            location: Location::Hand,
            card_type: CardTypeInstance::Spell(SpellInstance {
                effect: vec![Effect::DealDamage {
                    initiator: spell_id,
                    target: Target::Ids(vec![]),
                    amount: 3,
                }],
            }),
            play_target: Some(PlayTarget {
                strict: false,
                amount: 1,
                matcher: TargetMatcher::Owner(player_b),
            }),
        };
        game.entities.insert(spell_id, spell);
        game.players.get_mut(&player_a).unwrap().mana = 5;

        // Verify effect queue is empty
        assert_eq!(game.effect_queue.len(), 0);

        // c) Test: play the spell without providing targets
        game.play_spell(player_a, spell_id, None).unwrap();

        // d) Assert spell went to graveyard
        assert_eq!(
            game.entities.get(&spell_id).unwrap().location,
            Location::Graveyard
        );

        // Verify effect was queued but with empty target list
        assert_eq!(game.effect_queue.len(), 1);
        if let Some(Effect::DealDamage { target, amount, .. }) = game.effect_queue.front() {
            assert_eq!(*amount, 3);
            assert!(matches!(target, Target::Ids(ids) if ids.is_empty()));
        } else {
            panic!("Expected DealDamage effect in queue");
        }
    }

    #[test]
    fn test_play_spell_without_play_target_but_targets_provided() {
        // a) Initialize
        let mut game = super::super::test_utils::create_test_game();
        let player_a = game.player_id_a;
        let player_b = game.player_id_b;

        // b) Modify state: create a regular spell (no play_target)
        use super::super::test_utils::create_test_monster;

        let spell_id = create_test_spell(
            &mut game,
            player_a,
            vec![Effect::DealDamage {
                initiator: 0,
                target: Target::EnnemyPlayer,
                amount: 3,
            }],
        );
        game.players.get_mut(&player_a).unwrap().mana = 5;

        let enemy_monster = create_test_monster(&mut game, player_b, 0, 3, 5);

        // c) Test: try to play the spell with targets (should fail or ignore)
        let result = game.play_spell(player_a, spell_id, Some(vec![enemy_monster]));

        // d) Assert: This should either fail with an error or succeed by ignoring the targets
        // Based on the code, it should succeed and ignore the targets
        assert!(result.is_ok());
        assert_eq!(
            game.entities.get(&spell_id).unwrap().location,
            Location::Graveyard
        );
    }

    #[test]
    fn test_play_spell_with_invalid_targets() {
        // a) Initialize
        let mut game = super::super::test_utils::create_test_game();
        let player_a = game.player_id_a;
        let player_b = game.player_id_b;

        // b) Modify state: create a spell with play_target that requires enemy targets
        use super::super::test_utils::create_test_monster;
        use crate::collection::types::{PlayTarget, TargetMatcher};
        use crate::game::card::{CardInstance, CardTypeInstance, SpellInstance};
        use crate::{Race, collection::Class};

        // Create a friendly monster and an enemy monster
        let friendly_monster = create_test_monster(&mut game, player_a, 0, 3, 5);
        let enemy_monster = create_test_monster(&mut game, player_b, 0, 3, 5);

        let spell_id = game.entities.len() + 100;

        let spell = CardInstance {
            id: spell_id,
            name: "Enemy-Only Spell".to_string(),
            description: "Can only target enemies".to_string(),
            template_id: 9999,
            race: Race::COMMON,
            class: Class::COMMON,
            cost: 3,
            owner: player_a,
            location: Location::Hand,
            card_type: CardTypeInstance::Spell(SpellInstance {
                effect: vec![Effect::DealDamage {
                    initiator: spell_id,
                    target: Target::Ids(vec![]),
                    amount: 5,
                }],
            }),
            play_target: Some(PlayTarget {
                strict: false,
                amount: 1,
                matcher: TargetMatcher::Owner(player_b), // Only enemy targets
            }),
        };
        game.entities.insert(spell_id, spell);
        game.players.get_mut(&player_a).unwrap().mana = 5;

        // c) Test: try to play the spell targeting a friendly monster (should fail)
        let result = game.play_spell(player_a, spell_id, Some(vec![friendly_monster]));

        // d) Assert the play failed
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Game Logic Error: You selected a target that doesn't match the card conditions"
        );
    }

    #[test]
    fn test_play_spell_with_strict_target_exact_amount() {
        // a) Initialize
        let mut game = super::super::test_utils::create_test_game();
        let player_a = game.player_id_a;
        let player_b = game.player_id_b;

        // b) Modify state: create enemy monsters to target and a spell with strict play_target
        use super::super::test_utils::create_test_monster;
        use crate::collection::types::{PlayTarget, TargetMatcher};
        use crate::game::card::{CardInstance, CardTypeInstance, SpellInstance};
        use crate::{Race, collection::Class};

        let enemy_monster_1 = create_test_monster(&mut game, player_b, 0, 3, 5);
        let enemy_monster_2 = create_test_monster(&mut game, player_b, 1, 3, 5);

        // Create a spell that REQUIRES exactly 2 enemy targets (strict mode)
        let spell_id = game.entities.len() + 100;

        let spell = CardInstance {
            id: spell_id,
            name: "Strict Target Spell".to_string(),
            description: "Requires exactly 2 targets".to_string(),
            template_id: 9999,
            race: Race::COMMON,
            class: Class::COMMON,
            cost: 4,
            owner: player_a,
            location: Location::Hand,
            card_type: CardTypeInstance::Spell(SpellInstance {
                effect: vec![Effect::DealDamage {
                    initiator: spell_id,
                    target: Target::Ids(vec![]),
                    amount: 3,
                }],
            }),
            play_target: Some(PlayTarget {
                strict: true,
                amount: 2,
                matcher: TargetMatcher::Owner(player_b),
            }),
        };
        game.entities.insert(spell_id, spell);
        game.players.get_mut(&player_a).unwrap().mana = 5;

        // c) Test: play the spell with exactly 2 targets (strict requirement)
        let result = game.play_spell(
            player_a,
            spell_id,
            Some(vec![enemy_monster_1, enemy_monster_2]),
        );

        // d) Assert the play succeeded
        assert!(result.is_ok());
        assert_eq!(
            game.entities.get(&spell_id).unwrap().location,
            Location::Graveyard
        );

        // Verify effect was queued with the selected targets
        assert_eq!(game.effect_queue.len(), 1);
        if let Some(Effect::DealDamage { target, amount, .. }) = game.effect_queue.front() {
            assert_eq!(*amount, 3);
            assert!(
                matches!(target, Target::Ids(ids) if ids.len() == 2 && ids.contains(&enemy_monster_1) && ids.contains(&enemy_monster_2))
            );
        } else {
            panic!("Expected DealDamage effect in queue");
        }
    }

    #[test]
    fn test_play_spell_with_strict_target_wrong_amount() {
        // a) Initialize
        let mut game = super::super::test_utils::create_test_game();
        let player_a = game.player_id_a;
        let player_b = game.player_id_b;

        // b) Modify state: create enemy monsters and a spell with strict play_target
        use super::super::test_utils::create_test_monster;
        use crate::collection::types::{PlayTarget, TargetMatcher};
        use crate::game::card::{CardInstance, CardTypeInstance, SpellInstance};
        use crate::{Race, collection::Class};

        let enemy_monster_1 = create_test_monster(&mut game, player_b, 0, 3, 5);
        let _enemy_monster_2 = create_test_monster(&mut game, player_b, 1, 3, 5);

        // Create a spell that REQUIRES exactly 2 enemy targets (strict mode)
        let spell_id = game.entities.len() + 100;

        let spell = CardInstance {
            id: spell_id,
            name: "Strict Target Spell".to_string(),
            description: "Requires exactly 2 targets".to_string(),
            template_id: 9999,
            race: Race::COMMON,
            class: Class::COMMON,
            cost: 4,
            owner: player_a,
            location: Location::Hand,
            card_type: CardTypeInstance::Spell(SpellInstance {
                effect: vec![Effect::DealDamage {
                    initiator: spell_id,
                    target: Target::Ids(vec![]),
                    amount: 3,
                }],
            }),
            play_target: Some(PlayTarget {
                strict: true,
                amount: 2,
                matcher: TargetMatcher::Owner(player_b),
            }),
        };
        game.entities.insert(spell_id, spell);
        game.players.get_mut(&player_a).unwrap().mana = 5;

        // c) Test: play the spell with only 1 target (should fail because strict requires exactly 2)
        let result = game.play_spell(player_a, spell_id, Some(vec![enemy_monster_1]));

        // d) Assert the play failed with the correct error message
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Game Logic Error: Wrong quantity of targets selected (required: 2)"
        );
    }
}
