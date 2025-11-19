use crate::game::action::Action;
use crate::game::card::{CardInstance, CardTypeInstance, MonsterInstance};
use crate::game::effects::{Effect, Target};
use crate::game::tests::test_utils::{create_test_game, create_test_monster};
use crate::game::types::Location;
use crate::collection::Class;
use crate::Race;

#[cfg(test)]
mod test_on_damaged {
    use super::*;

    #[test]
    fn test_on_damaged_triggers_when_monster_survives_damage() {
        let mut game = create_test_game();
        let player_a = game.player_id_a;
        let player_b = game.player_id_b;

        let target_id = game.entities.len() + 100;
        let initiator_id = create_test_monster(&mut game, player_a, 0, 3, 3);

        let target = CardInstance {
            id: target_id,
            name: "Damaged Monster".to_string(),
            description: "Has on_damaged effect".to_string(),
            template_id: 9999,
            race: Race::COMMON,
            class: Class::COMMON,
            cost: 3,
            owner: player_b,
            location: Location::Field(1),
            card_type: CardTypeInstance::Monster(MonsterInstance {
                attack: 3,
                hp: 10,
                max_hp: 10,
                asleep: false,
                attack_count: 0,
                keywords: vec![],
                on_play: vec![],
                on_attack: vec![],
                on_defense: vec![],
                on_kill: vec![],
                on_turn_end: vec![],
                on_turn_start: vec![],
                on_damaged: vec![Effect::Boost {
                    initiator: target_id,
                    target: Target::ItSelf,
                    attack: 2,
                    hp: 0,
                }],
                on_death: vec![],
                on_surrounded: vec![],
                on_alone: vec![],
            }),
            play_target: None,
        };
        game.entities.insert(target_id, target);

        game.effect_queue.push_back(Effect::DealDamage {
            initiator: initiator_id,
            target: Target::Id(target_id),
            amount: 3,
        });

        let actions = game.compute_commands().unwrap();

        assert!(
            actions.iter().any(|a| matches!(a, Action::TriggerOnDamaged(id) if *id == target_id)),
            "Should trigger on_damaged when monster survives damage"
        );

        let damaged_monster = game.entities.get(&target_id).unwrap();
        if let CardTypeInstance::Monster(m) = &damaged_monster.card_type {
            assert_eq!(m.hp, 7, "Monster should have 7 HP after taking 3 damage");
            assert_eq!(
                m.attack, 5,
                "Monster should have 5 attack (3 base + 2 from on_damaged)"
            );
        } else {
            panic!("Entity should be a monster");
        }
    }

    #[test]
    fn test_on_damaged_does_not_trigger_on_lethal_damage() {
        let mut game = create_test_game();
        let player_a = game.player_id_a;
        let player_b = game.player_id_b;

        let target_id = game.entities.len() + 100;
        let initiator_id = create_test_monster(&mut game, player_a, 0, 3, 3);

        let target = CardInstance {
            id: target_id,
            name: "Dying Monster".to_string(),
            description: "Has on_damaged effect but will die".to_string(),
            template_id: 9999,
            race: Race::COMMON,
            class: Class::COMMON,
            cost: 3,
            owner: player_b,
            location: Location::Field(1),
            card_type: CardTypeInstance::Monster(MonsterInstance {
                attack: 3,
                hp: 3,
                max_hp: 3,
                asleep: false,
                attack_count: 0,
                keywords: vec![],
                on_play: vec![],
                on_attack: vec![],
                on_defense: vec![],
                on_kill: vec![],
                on_turn_end: vec![],
                on_turn_start: vec![],
                on_damaged: vec![Effect::MakeDraw {
                    initiator: target_id,
                    player: crate::game::effects::PlayerTarget::Player,
                    amount: 1,
                }],
                on_death: vec![],
                on_surrounded: vec![],
                on_alone: vec![],
            }),
            play_target: None,
        };
        game.entities.insert(target_id, target);

        let initial_hand_size = game.get_hand(player_b).len();

        game.effect_queue.push_back(Effect::DealDamage {
            initiator: initiator_id,
            target: Target::Id(target_id),
            amount: 3,
        });

        let actions = game.compute_commands().unwrap();

        assert!(
            !actions.iter().any(|a| matches!(a, Action::TriggerOnDamaged(id) if *id == target_id)),
            "Should NOT trigger on_damaged on lethal damage"
        );

        let final_hand_size = game.get_hand(player_b).len();
        assert_eq!(
            final_hand_size, initial_hand_size,
            "Player B should not have drawn (on_damaged didn't trigger)"
        );

        let dead_monster = game.entities.get(&target_id).unwrap();
        assert_eq!(
            dead_monster.location,
            Location::Graveyard,
            "Monster should be dead"
        );
    }

    #[test]
    fn test_on_damaged_does_not_trigger_on_overkill() {
        let mut game = create_test_game();
        let player_a = game.player_id_a;
        let player_b = game.player_id_b;

        let target_id = game.entities.len() + 100;
        let initiator_id = create_test_monster(&mut game, player_a, 0, 3, 3);

        let target = CardInstance {
            id: target_id,
            name: "Overkilled Monster".to_string(),
            description: "Has on_damaged effect but takes overkill damage".to_string(),
            template_id: 9999,
            race: Race::COMMON,
            class: Class::COMMON,
            cost: 3,
            owner: player_b,
            location: Location::Field(1),
            card_type: CardTypeInstance::Monster(MonsterInstance {
                attack: 3,
                hp: 3,
                max_hp: 3,
                asleep: false,
                attack_count: 0,
                keywords: vec![],
                on_play: vec![],
                on_attack: vec![],
                on_defense: vec![],
                on_kill: vec![],
                on_turn_end: vec![],
                on_turn_start: vec![],
                on_damaged: vec![Effect::MakeDraw {
                    initiator: target_id,
                    player: crate::game::effects::PlayerTarget::Player,
                    amount: 1,
                }],
                on_death: vec![],
                on_surrounded: vec![],
                on_alone: vec![],
            }),
            play_target: None,
        };
        game.entities.insert(target_id, target);

        game.effect_queue.push_back(Effect::DealDamage {
            initiator: initiator_id,
            target: Target::Id(target_id),
            amount: 10,
        });

        let actions = game.compute_commands().unwrap();

        assert!(
            !actions.iter().any(|a| matches!(a, Action::TriggerOnDamaged(id) if *id == target_id)),
            "Should NOT trigger on_damaged on overkill"
        );
    }

    #[test]
    fn test_on_damaged_triggers_multiple_times_on_multiple_hits() {
        let mut game = create_test_game();
        let player_a = game.player_id_a;
        let player_b = game.player_id_b;

        let target_id = game.entities.len() + 100;
        let initiator_id = create_test_monster(&mut game, player_a, 0, 3, 3);

        let target = CardInstance {
            id: target_id,
            name: "Tough Monster".to_string(),
            description: "Has on_damaged effect and survives multiple hits".to_string(),
            template_id: 9999,
            race: Race::COMMON,
            class: Class::COMMON,
            cost: 3,
            owner: player_b,
            location: Location::Field(1),
            card_type: CardTypeInstance::Monster(MonsterInstance {
                attack: 1,
                hp: 20,
                max_hp: 20,
                asleep: false,
                attack_count: 0,
                keywords: vec![],
                on_play: vec![],
                on_attack: vec![],
                on_defense: vec![],
                on_kill: vec![],
                on_turn_end: vec![],
                on_turn_start: vec![],
                on_damaged: vec![Effect::Boost {
                    initiator: target_id,
                    target: Target::ItSelf,
                    attack: 1,
                    hp: 0,
                }],
                on_death: vec![],
                on_surrounded: vec![],
                on_alone: vec![],
            }),
            play_target: None,
        };
        game.entities.insert(target_id, target);

        game.effect_queue.push_back(Effect::DealDamage {
            initiator: initiator_id,
            target: Target::Id(target_id),
            amount: 3,
        });

        let _actions1 = game.compute_commands().unwrap();

        game.effect_queue.push_back(Effect::DealDamage {
            initiator: initiator_id,
            target: Target::Id(target_id),
            amount: 3,
        });

        let _actions2 = game.compute_commands().unwrap();

        let tough_monster = game.entities.get(&target_id).unwrap();
        if let CardTypeInstance::Monster(m) = &tough_monster.card_type {
            assert_eq!(
                m.hp, 14,
                "Monster should have 14 HP after taking 6 total damage"
            );
            assert_eq!(
                m.attack, 3,
                "Monster should have 3 attack (1 base + 2 from two on_damaged triggers)"
            );
        } else {
            panic!("Entity should be a monster");
        }
    }

    #[test]
    fn test_on_damaged_does_not_trigger_when_no_on_damaged_effects() {
        let mut game = create_test_game();
        let player_a = game.player_id_a;
        let player_b = game.player_id_b;

        let target_id = create_test_monster(&mut game, player_b, 1, 10, 10);
        let initiator_id = create_test_monster(&mut game, player_a, 0, 3, 3);

        game.effect_queue.push_back(Effect::DealDamage {
            initiator: initiator_id,
            target: Target::Id(target_id),
            amount: 3,
        });

        let actions = game.compute_commands().unwrap();

        assert!(
            !actions.iter().any(|a| matches!(a, Action::TriggerOnDamaged(id) if *id == target_id)),
            "Should not trigger on_damaged when monster has no on_damaged effects"
        );
    }
}
