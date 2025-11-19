use crate::game::action::Action;
use crate::game::card::{CardInstance, CardTypeInstance, MonsterInstance};
use crate::game::effects::{Effect, Target};
use crate::game::tests::test_utils::{create_test_game, create_test_monster};
use crate::game::types::Location;
use crate::collection::Class;
use crate::Race;

#[cfg(test)]
mod test_on_turn_end {
    use super::*;

    #[test]
    fn test_on_turn_end_triggers_for_all_monsters_of_ending_player() {
        let mut game = create_test_game();
        let player_a = game.player_id_a;
        let player_b = game.player_id_b;

        game.current_player = player_a;

        let monster1_id = game.entities.len() + 100;
        let monster2_id = game.entities.len() + 101;

        let monster1 = CardInstance {
            id: monster1_id,
            name: "End Turn Monster 1".to_string(),
            description: "Has on_turn_end effect".to_string(),
            template_id: 9999,
            race: Race::COMMON,
            class: Class::COMMON,
            cost: 3,
            owner: player_a,
            location: Location::Field(0),
            card_type: CardTypeInstance::Monster(MonsterInstance {
                attack: 3,
                hp: 5,
                max_hp: 5,
                asleep: false,
                attack_count: 0,
                keywords: vec![],
                on_play: vec![],
                on_attack: vec![],
                on_defense: vec![],
                on_kill: vec![],
                on_turn_end: vec![Effect::Boost {
                    initiator: monster1_id,
                    target: Target::ItSelf,
                    attack: 1,
                    hp: 1,
                }],
                on_turn_start: vec![],
                on_damaged: vec![],
                on_death: vec![],
                on_surrounded: vec![],
                on_alone: vec![],
            }),
            play_target: None,
        };
        game.entities.insert(monster1_id, monster1);

        let monster2 = CardInstance {
            id: monster2_id,
            name: "End Turn Monster 2".to_string(),
            description: "Has on_turn_end effect".to_string(),
            template_id: 9998,
            race: Race::COMMON,
            class: Class::COMMON,
            cost: 3,
            owner: player_a,
            location: Location::Field(1),
            card_type: CardTypeInstance::Monster(MonsterInstance {
                attack: 2,
                hp: 4,
                max_hp: 4,
                asleep: false,
                attack_count: 0,
                keywords: vec![],
                on_play: vec![],
                on_attack: vec![],
                on_defense: vec![],
                on_kill: vec![],
                on_turn_end: vec![Effect::Boost {
                    initiator: monster2_id,
                    target: Target::ItSelf,
                    attack: 2,
                    hp: 0,
                }],
                on_turn_start: vec![],
                on_damaged: vec![],
                on_death: vec![],
                on_surrounded: vec![],
                on_alone: vec![],
            }),
            play_target: None,
        };
        game.entities.insert(monster2_id, monster2);

        let actions = crate::game::user_actions::end_turn::end_turn(&mut game, player_a).unwrap();

        assert!(
            actions.iter().any(|a| matches!(a, Action::TriggerOnTurnEnd(id) if *id == monster1_id)),
            "Should trigger on_turn_end for monster 1"
        );
        assert!(
            actions.iter().any(|a| matches!(a, Action::TriggerOnTurnEnd(id) if *id == monster2_id)),
            "Should trigger on_turn_end for monster 2"
        );

        let m1 = game.entities.get(&monster1_id).unwrap();
        if let CardTypeInstance::Monster(m) = &m1.card_type {
            assert_eq!(m.attack, 4, "Monster 1 should have +1 attack");
            assert_eq!(m.hp, 6, "Monster 1 should have +1 hp");
        }

        let m2 = game.entities.get(&monster2_id).unwrap();
        if let CardTypeInstance::Monster(m) = &m2.card_type {
            assert_eq!(m.attack, 4, "Monster 2 should have +2 attack");
            assert_eq!(m.hp, 4, "Monster 2 should have same hp");
        }
    }

    #[test]
    fn test_on_turn_end_does_not_trigger_for_opponent_monsters() {
        let mut game = create_test_game();
        let player_a = game.player_id_a;
        let player_b = game.player_id_b;

        game.current_player = player_a;
        game.vs_ia = false;

        let monster_player_a = game.entities.len() + 100;
        let monster_player_b = game.entities.len() + 101;

        let monster_a = CardInstance {
            id: monster_player_a,
            name: "Player A Monster".to_string(),
            description: "Has on_turn_end effect".to_string(),
            template_id: 9999,
            race: Race::COMMON,
            class: Class::COMMON,
            cost: 3,
            owner: player_a,
            location: Location::Field(0),
            card_type: CardTypeInstance::Monster(MonsterInstance {
                attack: 3,
                hp: 5,
                max_hp: 5,
                asleep: false,
                attack_count: 0,
                keywords: vec![],
                on_play: vec![],
                on_attack: vec![],
                on_defense: vec![],
                on_kill: vec![],
                on_turn_end: vec![Effect::Boost {
                    initiator: monster_player_a,
                    target: Target::ItSelf,
                    attack: 1,
                    hp: 0,
                }],
                on_turn_start: vec![],
                on_damaged: vec![],
                on_death: vec![],
                on_surrounded: vec![],
                on_alone: vec![],
            }),
            play_target: None,
        };
        game.entities.insert(monster_player_a, monster_a);

        let monster_b = CardInstance {
            id: monster_player_b,
            name: "Player B Monster".to_string(),
            description: "Has on_turn_end effect".to_string(),
            template_id: 9998,
            race: Race::COMMON,
            class: Class::COMMON,
            cost: 3,
            owner: player_b,
            location: Location::Field(1),
            card_type: CardTypeInstance::Monster(MonsterInstance {
                attack: 3,
                hp: 5,
                max_hp: 5,
                asleep: false,
                attack_count: 0,
                keywords: vec![],
                on_play: vec![],
                on_attack: vec![],
                on_defense: vec![],
                on_kill: vec![],
                on_turn_end: vec![Effect::Boost {
                    initiator: monster_player_b,
                    target: Target::ItSelf,
                    attack: 1,
                    hp: 0,
                }],
                on_turn_start: vec![],
                on_damaged: vec![],
                on_death: vec![],
                on_surrounded: vec![],
                on_alone: vec![],
            }),
            play_target: None,
        };
        game.entities.insert(monster_player_b, monster_b);

        let actions = crate::game::user_actions::end_turn::end_turn(&mut game, player_a).unwrap();

        assert!(
            actions.iter().any(|a| matches!(a, Action::TriggerOnTurnEnd(id) if *id == monster_player_a)),
            "Should trigger on_turn_end for player A's monster"
        );
        assert!(
            !actions.iter().any(|a| matches!(a, Action::TriggerOnTurnEnd(id) if *id == monster_player_b)),
            "Should NOT trigger on_turn_end for player B's monster"
        );

        let m_a = game.entities.get(&monster_player_a).unwrap();
        if let CardTypeInstance::Monster(m) = &m_a.card_type {
            assert_eq!(m.attack, 4, "Player A monster should have +1 attack");
        }

        let m_b = game.entities.get(&monster_player_b).unwrap();
        if let CardTypeInstance::Monster(m) = &m_b.card_type {
            assert_eq!(
                m.attack, 3,
                "Player B monster should NOT have gained attack"
            );
        }
    }

    #[test]
    fn test_on_turn_end_triggers_before_end_turn_action() {
        let mut game = create_test_game();
        let player_a = game.player_id_a;

        game.current_player = player_a;

        let monster_id = game.entities.len() + 100;
        let monster = CardInstance {
            id: monster_id,
            name: "Turn End Monster".to_string(),
            description: "Has on_turn_end effect".to_string(),
            template_id: 9999,
            race: Race::COMMON,
            class: Class::COMMON,
            cost: 3,
            owner: player_a,
            location: Location::Field(0),
            card_type: CardTypeInstance::Monster(MonsterInstance {
                attack: 3,
                hp: 5,
                max_hp: 5,
                asleep: false,
                attack_count: 0,
                keywords: vec![],
                on_play: vec![],
                on_attack: vec![],
                on_defense: vec![],
                on_kill: vec![],
                on_turn_end: vec![Effect::Boost {
                    initiator: monster_id,
                    target: Target::ItSelf,
                    attack: 1,
                    hp: 0,
                }],
                on_turn_start: vec![],
                on_damaged: vec![],
                on_death: vec![],
                on_surrounded: vec![],
                on_alone: vec![],
            }),
            play_target: None,
        };
        game.entities.insert(monster_id, monster);

        let actions = crate::game::user_actions::end_turn::end_turn(&mut game, player_a).unwrap();

        let trigger_index = actions
            .iter()
            .position(|a| matches!(a, Action::TriggerOnTurnEnd(_)));
        let end_turn_index = actions
            .iter()
            .position(|a| matches!(a, Action::EndTurn(_)));

        assert!(
            trigger_index.is_some(),
            "Should have TriggerOnTurnEnd action"
        );
        assert!(end_turn_index.is_some(), "Should have EndTurn action");

        if let (Some(t), Some(e)) = (trigger_index, end_turn_index) {
            assert!(
                t < e,
                "TriggerOnTurnEnd should happen before EndTurn action"
            );
        }
    }

    #[test]
    fn test_on_turn_end_does_not_trigger_when_no_monsters_on_field() {
        let mut game = create_test_game();
        let player_a = game.player_id_a;

        game.current_player = player_a;

        let actions = crate::game::user_actions::end_turn::end_turn(&mut game, player_a).unwrap();

        let has_trigger = actions
            .iter()
            .any(|a| matches!(a, Action::TriggerOnTurnEnd(_)));
        assert!(
            !has_trigger,
            "Should not have any TriggerOnTurnEnd when field is empty"
        );
    }

    #[test]
    fn test_on_turn_end_does_not_trigger_when_monster_has_no_effects() {
        let mut game = create_test_game();
        let player_a = game.player_id_a;

        game.current_player = player_a;

        let monster_id = create_test_monster(&mut game, player_a, 0, 5, 5);

        let actions = crate::game::user_actions::end_turn::end_turn(&mut game, player_a).unwrap();

        assert!(
            !actions.iter().any(|a| matches!(a, Action::TriggerOnTurnEnd(id) if *id == monster_id)),
            "Should NOT emit TriggerOnTurnEnd action when monster has no on_turn_end effects"
        );
    }
}
