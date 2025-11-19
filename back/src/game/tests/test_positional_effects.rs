// FUNCTIONAL RULES: Positional Effects (ON_ALONE and ON_SURROUNDED)
//
// ON_ALONE (Solitaire):
// 1. A monster with on_alone triggers its effects when it is alone (no allies on linked positions)
// 2. The on_alone trigger activates:
//    - After playing/summoning a monster (if condition is met)
//    - After moving a monster (if condition becomes true)
//    - After destroying an adjacent allied monster (making a monster alone)
// 3. The on_alone effects trigger ONCE then are consumed (one-shot behavior)
// 4. A monster is alone = NONE of its linked positions are occupied by allies
// 5. Linked positions depend on the position:
//    - Positions 0, 1, 6, 7: 2 possible neighbors
//    - Positions 3, 4: 3 possible neighbors
//    - Positions 2, 5: 4 possible neighbors
//
// ON_SURROUNDED (Entouré):
// 1. A monster with on_surrounded triggers its effects when surrounded (all neighbors occupied by allies)
// 2. The on_surrounded trigger activates:
//    - After playing/summoning a monster (if condition is met)
//    - After moving a monster (if condition becomes true)
//    - After summoning another monster that completes the surrounding
// 3. The on_surrounded effects trigger ONCE then are consumed (one-shot behavior)
// 4. A monster is surrounded = ALL linked positions are occupied by allies
// 5. To be surrounded:
//    - Position 0: must have 2 allies at [1, 2]
//    - Position 2: must have 4 allies at [0, 1, 3, 4]
//    - Position 5: must have 4 allies at [3, 4, 6, 7]
//    - etc.
//
// GENERAL BEHAVIOR:
// - Positional effects only check ALLIES (same owner), not enemies
// - Effects trigger AFTER on_play effects of a card
// - Effects DO NOT trigger on turn change
// - Effects only trigger when board state changes (summon, destroy, move)

#[cfg(test)]
mod tests {
    use super::super::test_utils::{create_test_game, create_test_monster};
    use crate::game::action::Action;
    use crate::game::card::{CardInstance, CardTypeInstance, MonsterInstance};
    use crate::game::effects::{Effect, Target};
    use crate::game::types::Location;
    use crate::{Race, collection::Class};

    // ===== HELPER FUNCTIONS =====

    fn has_trigger_on_alone(actions: &[Action], monster_id: usize) -> bool {
        actions
            .iter()
            .any(|action| matches!(action, Action::TriggerOnAlone(id) if *id == monster_id))
    }

    fn has_trigger_on_surrounded(actions: &[Action], monster_id: usize) -> bool {
        actions
            .iter()
            .any(|action| matches!(action, Action::TriggerOnSurrounded(id) if *id == monster_id))
    }

    fn create_monster_with_on_alone(
        game: &mut crate::Game,
        owner: usize,
        position: usize,
        attack: usize,
        hp: usize,
    ) -> usize {
        let monster_id = game.entities.len() + 1;
        let monster = CardInstance {
            id: monster_id,
            name: "Alone Monster".to_string(),
            description: "Triggers when alone".to_string(),
            template_id: 1011,
            race: Race::HUMAN,
            class: Class::COMMON,
            cost: 2,
            owner,
            location: Location::Field(position),
            card_type: CardTypeInstance::Monster(MonsterInstance {
                attack,
                hp,
                max_hp: hp,
                asleep: false,
                attack_count: 0,
                keywords: vec![],
                on_play: vec![],
                on_attack: vec![],
                on_defense: vec![],
                on_kill: vec![],
                on_turn_end: vec![],
                on_turn_start: vec![],
                on_damaged: vec![],
                on_death: vec![],
                on_surrounded: vec![],
                on_alone: vec![Effect::Boost {
                    initiator: monster_id,
                    target: Target::ItSelf,
                    attack: 3,
                    hp: 3,
                }],
            }),
            play_target: None,
        };
        game.entities.insert(monster_id, monster);
        monster_id
    }

    fn create_monster_with_on_surrounded(
        game: &mut crate::Game,
        owner: usize,
        position: usize,
        attack: usize,
        hp: usize,
    ) -> usize {
        let monster_id = game.entities.len() + 1;
        let monster = CardInstance {
            id: monster_id,
            name: "Surrounded Monster".to_string(),
            description: "Triggers when surrounded".to_string(),
            template_id: 1010,
            race: Race::HUMAN,
            class: Class::COMMON,
            cost: 3,
            owner,
            location: Location::Field(position),
            card_type: CardTypeInstance::Monster(MonsterInstance {
                attack,
                hp,
                max_hp: hp,
                asleep: false,
                attack_count: 0,
                keywords: vec![],
                on_play: vec![],
                on_attack: vec![],
                on_defense: vec![],
                on_kill: vec![],
                on_turn_end: vec![],
                on_turn_start: vec![],
                on_damaged: vec![],
                on_death: vec![],
                on_surrounded: vec![Effect::Boost {
                    initiator: monster_id,
                    target: Target::ItSelf,
                    attack: 3,
                    hp: 3,
                }],
                on_alone: vec![],
            }),
            play_target: None,
        };
        game.entities.insert(monster_id, monster);
        monster_id
    }

    // ===== ON_ALONE TESTS =====

    #[test]
    fn test_alone_triggers_when_summoned_with_no_neighbors() {
        // a) Initialize
        let mut game = create_test_game();
        let player_a = game.player_id_a;

        // b) Modify state: create a monster with on_alone at position 0 (no neighbors yet)
        let alone_monster = create_monster_with_on_alone(&mut game, player_a, 0, 4, 4);

        // Verify initial stats
        if let CardTypeInstance::Monster(monster) =
            &game.entities.get(&alone_monster).unwrap().card_type
        {
            assert_eq!(monster.attack, 4);
            assert_eq!(monster.hp, 4);
            assert!(!monster.on_alone.is_empty()); // Effect not triggered yet
        }

        // c) Test: trigger positional effects (simulates what happens after summon)
        let actions = crate::game::logic::trigger_positional_effects(&mut game).unwrap();

        // d) Assert on_alone triggered
        assert!(has_trigger_on_alone(&actions, alone_monster));

        // Verify boost was applied
        game.compute_commands().unwrap();

        if let CardTypeInstance::Monster(monster) =
            &game.entities.get(&alone_monster).unwrap().card_type
        {
            assert_eq!(monster.attack, 7); // 4 + 3
            assert_eq!(monster.hp, 7); // 4 + 3
            assert!(monster.on_alone.is_empty()); // Effect consumed
        }
    }

    #[test]
    fn test_alone_does_not_trigger_when_has_neighbors() {
        // a) Initialize
        let mut game = create_test_game();
        let player_a = game.player_id_a;

        // b) Modify state: create a monster with on_alone and a neighbor
        let alone_monster = create_monster_with_on_alone(&mut game, player_a, 0, 4, 4);
        // Position 0 has neighbors [1, 2], add ally at position 1
        create_test_monster(&mut game, player_a, 1, 2, 2);

        // c) Test: trigger positional effects
        let actions = crate::game::logic::trigger_positional_effects(&mut game).unwrap();

        // d) Assert on_alone did NOT trigger
        assert!(!has_trigger_on_alone(&actions, alone_monster));

        // Verify no boost was applied
        if let CardTypeInstance::Monster(monster) =
            &game.entities.get(&alone_monster).unwrap().card_type
        {
            assert_eq!(monster.attack, 4); // unchanged
            assert_eq!(monster.hp, 4); // unchanged
            assert!(!monster.on_alone.is_empty()); // Effect still available
        }
    }

    #[test]
    fn test_alone_triggers_after_neighbor_destroyed() {
        // a) Initialize
        let mut game = create_test_game();
        let player_a = game.player_id_a;

        // b) Modify state: create monster with on_alone and a neighbor
        let alone_monster = create_monster_with_on_alone(&mut game, player_a, 0, 4, 4);
        let neighbor = create_test_monster(&mut game, player_a, 1, 2, 2);

        // Verify on_alone doesn't trigger yet (has neighbor)
        let actions = crate::game::logic::trigger_positional_effects(&mut game).unwrap();
        assert!(!has_trigger_on_alone(&actions, alone_monster));

        // c) Test as user: destroy the neighbor
        // Simulate destruction
        game.entities.get_mut(&neighbor).unwrap().location = Location::Graveyard;
        game.entities.remove(&neighbor);

        // Trigger positional effects (happens after destroy logic)
        let actions = crate::game::logic::trigger_positional_effects(&mut game).unwrap();

        // d) Assert on_alone triggered after neighbor removed
        assert!(has_trigger_on_alone(&actions, alone_monster));

        game.compute_commands().unwrap();

        if let CardTypeInstance::Monster(monster) =
            &game.entities.get(&alone_monster).unwrap().card_type
        {
            assert_eq!(monster.attack, 7); // Boosted
            assert_eq!(monster.hp, 7);
            assert!(monster.on_alone.is_empty()); // Consumed
        }
    }

    #[test]
    fn test_alone_one_shot_does_not_retrigger() {
        // a) Initialize
        let mut game = create_test_game();
        let player_a = game.player_id_a;

        // b) Modify state: create alone monster and trigger effect
        let alone_monster = create_monster_with_on_alone(&mut game, player_a, 0, 4, 4);

        // First trigger
        let actions = crate::game::logic::trigger_positional_effects(&mut game).unwrap();
        assert!(has_trigger_on_alone(&actions, alone_monster));
        game.compute_commands().unwrap();

        // Verify effect was consumed
        if let CardTypeInstance::Monster(monster) =
            &game.entities.get(&alone_monster).unwrap().card_type
        {
            assert_eq!(monster.attack, 7);
            assert!(monster.on_alone.is_empty());
        }

        // c) Test: add then remove a neighbor (monster becomes alone again)
        let neighbor = create_test_monster(&mut game, player_a, 1, 2, 2);
        game.entities.remove(&neighbor);

        // Try to trigger again
        let actions = crate::game::logic::trigger_positional_effects(&mut game).unwrap();

        // d) Assert on_alone does NOT trigger again (one-shot behavior)
        assert!(!has_trigger_on_alone(&actions, alone_monster));

        // Stats unchanged from first boost
        if let CardTypeInstance::Monster(monster) =
            &game.entities.get(&alone_monster).unwrap().card_type
        {
            assert_eq!(monster.attack, 7); // Still 7, not 10
            assert_eq!(monster.hp, 7);
        }
    }

    #[test]
    fn test_alone_triggers_at_position_with_4_neighbors() {
        // a) Initialize
        let mut game = create_test_game();
        let player_a = game.player_id_a;

        // b) Modify state: position 2 has 4 linked positions [0, 1, 3, 4]
        // Create monster at position 2 with no neighbors
        let alone_monster = create_monster_with_on_alone(&mut game, player_a, 2, 4, 4);

        // c) Test: trigger positional effects
        let actions = crate::game::logic::trigger_positional_effects(&mut game).unwrap();

        // d) Assert on_alone triggered (all 4 neighbor slots are empty)
        assert!(has_trigger_on_alone(&actions, alone_monster));

        game.compute_commands().unwrap();

        if let CardTypeInstance::Monster(monster) =
            &game.entities.get(&alone_monster).unwrap().card_type
        {
            assert_eq!(monster.attack, 7);
            assert_eq!(monster.hp, 7);
        }
    }

    #[test]
    fn test_alone_does_not_trigger_with_one_of_four_neighbors() {
        // a) Initialize
        let mut game = create_test_game();
        let player_a = game.player_id_a;

        // b) Modify state: position 2 has 4 linked positions [0, 1, 3, 4]
        let alone_monster = create_monster_with_on_alone(&mut game, player_a, 2, 4, 4);
        // Add just one neighbor
        create_test_monster(&mut game, player_a, 0, 2, 2);

        // c) Test: trigger positional effects
        let actions = crate::game::logic::trigger_positional_effects(&mut game).unwrap();

        // d) Assert on_alone did NOT trigger (has at least one neighbor)
        assert!(!has_trigger_on_alone(&actions, alone_monster));

        if let CardTypeInstance::Monster(monster) =
            &game.entities.get(&alone_monster).unwrap().card_type
        {
            assert_eq!(monster.attack, 4); // No boost
            assert_eq!(monster.hp, 4);
        }
    }

    // ===== ON_SURROUNDED TESTS =====

    #[test]
    fn test_surrounded_triggers_when_all_neighbors_present() {
        // a) Initialize
        let mut game = create_test_game();
        let player_a = game.player_id_a;

        // b) Modify state: position 0 needs neighbors at [1, 2] to be surrounded
        let surrounded_monster = create_monster_with_on_surrounded(&mut game, player_a, 0, 3, 3);
        create_test_monster(&mut game, player_a, 1, 2, 2);
        create_test_monster(&mut game, player_a, 2, 2, 2);

        // Verify initial stats
        if let CardTypeInstance::Monster(monster) =
            &game.entities.get(&surrounded_monster).unwrap().card_type
        {
            assert_eq!(monster.attack, 3);
            assert_eq!(monster.hp, 3);
            assert!(!monster.on_surrounded.is_empty());
        }

        // c) Test: trigger positional effects
        let actions = crate::game::logic::trigger_positional_effects(&mut game).unwrap();

        // d) Assert on_surrounded triggered
        assert!(has_trigger_on_surrounded(&actions, surrounded_monster));

        game.compute_commands().unwrap();

        if let CardTypeInstance::Monster(monster) =
            &game.entities.get(&surrounded_monster).unwrap().card_type
        {
            assert_eq!(monster.attack, 6); // 3 + 3
            assert_eq!(monster.hp, 6); // 3 + 3
            assert!(monster.on_surrounded.is_empty()); // Consumed
        }
    }

    #[test]
    fn test_surrounded_does_not_trigger_with_missing_neighbor() {
        // a) Initialize
        let mut game = create_test_game();
        let player_a = game.player_id_a;

        // b) Modify state: position 0 needs [1, 2], only add 1
        let surrounded_monster = create_monster_with_on_surrounded(&mut game, player_a, 0, 3, 3);
        create_test_monster(&mut game, player_a, 1, 2, 2);
        // Missing neighbor at position 2

        // c) Test: trigger positional effects
        let actions = crate::game::logic::trigger_positional_effects(&mut game).unwrap();

        // d) Assert on_surrounded did NOT trigger
        assert!(!has_trigger_on_surrounded(&actions, surrounded_monster));

        if let CardTypeInstance::Monster(monster) =
            &game.entities.get(&surrounded_monster).unwrap().card_type
        {
            assert_eq!(monster.attack, 3); // No boost
            assert_eq!(monster.hp, 3);
            assert!(!monster.on_surrounded.is_empty()); // Still available
        }
    }

    #[test]
    fn test_surrounded_triggers_after_last_neighbor_summoned() {
        // a) Initialize
        let mut game = create_test_game();
        let player_a = game.player_id_a;

        // b) Modify state: position 0 with only 1 neighbor initially
        let surrounded_monster = create_monster_with_on_surrounded(&mut game, player_a, 0, 3, 3);
        create_test_monster(&mut game, player_a, 1, 2, 2);

        // Verify not surrounded yet
        let actions = crate::game::logic::trigger_positional_effects(&mut game).unwrap();
        assert!(!has_trigger_on_surrounded(&actions, surrounded_monster));

        // c) Test as user: summon the missing neighbor
        create_test_monster(&mut game, player_a, 2, 2, 2);

        // Trigger positional effects (happens after summon)
        let actions = crate::game::logic::trigger_positional_effects(&mut game).unwrap();

        // d) Assert on_surrounded triggered
        assert!(has_trigger_on_surrounded(&actions, surrounded_monster));

        game.compute_commands().unwrap();

        if let CardTypeInstance::Monster(monster) =
            &game.entities.get(&surrounded_monster).unwrap().card_type
        {
            assert_eq!(monster.attack, 6);
            assert_eq!(monster.hp, 6);
            assert!(monster.on_surrounded.is_empty());
        }
    }

    #[test]
    fn test_surrounded_one_shot_does_not_retrigger() {
        // a) Initialize
        let mut game = create_test_game();
        let player_a = game.player_id_a;

        // b) Modify state: create surrounded monster and trigger
        let surrounded_monster = create_monster_with_on_surrounded(&mut game, player_a, 0, 3, 3);
        create_test_monster(&mut game, player_a, 1, 2, 2);
        let neighbor_2 = create_test_monster(&mut game, player_a, 2, 2, 2);

        // First trigger
        let actions = crate::game::logic::trigger_positional_effects(&mut game).unwrap();
        assert!(has_trigger_on_surrounded(&actions, surrounded_monster));
        game.compute_commands().unwrap();

        // Verify consumed
        if let CardTypeInstance::Monster(monster) =
            &game.entities.get(&surrounded_monster).unwrap().card_type
        {
            assert_eq!(monster.attack, 6);
            assert!(monster.on_surrounded.is_empty());
        }

        // c) Test: remove then re-add neighbor (becomes surrounded again)
        game.entities.remove(&neighbor_2);
        create_test_monster(&mut game, player_a, 2, 2, 2);

        let actions = crate::game::logic::trigger_positional_effects(&mut game).unwrap();

        // d) Assert does NOT trigger again (one-shot)
        assert!(!has_trigger_on_surrounded(&actions, surrounded_monster));

        if let CardTypeInstance::Monster(monster) =
            &game.entities.get(&surrounded_monster).unwrap().card_type
        {
            assert_eq!(monster.attack, 6); // Still 6, not 9
            assert_eq!(monster.hp, 6);
        }
    }

    #[test]
    fn test_surrounded_at_position_with_4_neighbors() {
        // a) Initialize
        let mut game = create_test_game();
        let player_a = game.player_id_a;

        // b) Modify state: position 2 needs all 4 neighbors [0, 1, 3, 4]
        let surrounded_monster = create_monster_with_on_surrounded(&mut game, player_a, 2, 3, 3);
        create_test_monster(&mut game, player_a, 0, 2, 2);
        create_test_monster(&mut game, player_a, 1, 2, 2);
        create_test_monster(&mut game, player_a, 3, 2, 2);
        create_test_monster(&mut game, player_a, 4, 2, 2);

        // c) Test: trigger positional effects
        let actions = crate::game::logic::trigger_positional_effects(&mut game).unwrap();

        // d) Assert on_surrounded triggered (all 4 neighbors present)
        assert!(has_trigger_on_surrounded(&actions, surrounded_monster));

        game.compute_commands().unwrap();

        if let CardTypeInstance::Monster(monster) =
            &game.entities.get(&surrounded_monster).unwrap().card_type
        {
            assert_eq!(monster.attack, 6);
            assert_eq!(monster.hp, 6);
        }
    }

    #[test]
    fn test_surrounded_does_not_trigger_with_3_of_4_neighbors() {
        // a) Initialize
        let mut game = create_test_game();
        let player_a = game.player_id_a;

        // b) Modify state: position 2 needs [0, 1, 3, 4], only add 3
        let surrounded_monster = create_monster_with_on_surrounded(&mut game, player_a, 2, 3, 3);
        create_test_monster(&mut game, player_a, 0, 2, 2);
        create_test_monster(&mut game, player_a, 1, 2, 2);
        create_test_monster(&mut game, player_a, 3, 2, 2);
        // Missing position 4

        // c) Test: trigger positional effects
        let actions = crate::game::logic::trigger_positional_effects(&mut game).unwrap();

        // d) Assert on_surrounded did NOT trigger
        assert!(!has_trigger_on_surrounded(&actions, surrounded_monster));

        if let CardTypeInstance::Monster(monster) =
            &game.entities.get(&surrounded_monster).unwrap().card_type
        {
            assert_eq!(monster.attack, 3); // No boost
            assert_eq!(monster.hp, 3);
        }
    }

    // ===== MIXED SCENARIOS =====

    #[test]
    fn test_enemies_do_not_count_for_positional_effects() {
        // a) Initialize
        let mut game = create_test_game();
        let player_a = game.player_id_a;
        let player_b = game.player_id_b;

        // b) Modify state: position 0 with enemy neighbors
        let alone_monster = create_monster_with_on_alone(&mut game, player_a, 0, 4, 4);
        // Add enemies at positions 1 and 2 (should not prevent alone trigger)
        create_test_monster(&mut game, player_b, 1, 2, 2);
        create_test_monster(&mut game, player_b, 2, 2, 2);

        // c) Test: trigger positional effects
        let actions = crate::game::logic::trigger_positional_effects(&mut game).unwrap();

        // d) Assert on_alone triggers (enemies don't count as neighbors)
        assert!(has_trigger_on_alone(&actions, alone_monster));

        game.compute_commands().unwrap();

        if let CardTypeInstance::Monster(monster) =
            &game.entities.get(&alone_monster).unwrap().card_type
        {
            assert_eq!(monster.attack, 7); // Boosted
        }
    }

    #[test]
    fn test_multiple_monsters_can_trigger_simultaneously() {
        // a) Initialize
        let mut game = create_test_game();
        let player_a = game.player_id_a;

        // b) Modify state: two alone monsters at different positions
        let alone_monster_1 = create_monster_with_on_alone(&mut game, player_a, 0, 4, 4);
        let alone_monster_2 = create_monster_with_on_alone(&mut game, player_a, 6, 4, 4);
        // Position 0 neighbors: [1, 2] - both empty
        // Position 6 neighbors: [5, 7] - both empty

        // c) Test: trigger positional effects
        let actions = crate::game::logic::trigger_positional_effects(&mut game).unwrap();

        // d) Assert both trigger
        assert!(has_trigger_on_alone(&actions, alone_monster_1));
        assert!(has_trigger_on_alone(&actions, alone_monster_2));

        game.compute_commands().unwrap();

        // Both boosted
        if let CardTypeInstance::Monster(monster) =
            &game.entities.get(&alone_monster_1).unwrap().card_type
        {
            assert_eq!(monster.attack, 7);
        }
        if let CardTypeInstance::Monster(monster) =
            &game.entities.get(&alone_monster_2).unwrap().card_type
        {
            assert_eq!(monster.attack, 7);
        }
    }

    #[test]
    fn test_monster_without_positional_effects_does_not_trigger() {
        // a) Initialize
        let mut game = create_test_game();
        let player_a = game.player_id_a;

        // b) Modify state: regular monster with no positional effects
        let regular_monster = create_test_monster(&mut game, player_a, 0, 4, 4);

        // c) Test: trigger positional effects
        let actions = crate::game::logic::trigger_positional_effects(&mut game).unwrap();

        // d) Assert no positional actions triggered
        assert!(actions.is_empty());
    }
}
