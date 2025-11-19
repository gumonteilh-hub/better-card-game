use crate::game::action::Action;
use crate::game::card::{CardInstance, CardTypeInstance, MonsterInstance};
use crate::game::effects::{Effect, Target};
use crate::game::tests::test_utils::{add_card_to_deck, create_test_game, create_test_monster};
use crate::game::types::Location;
use crate::collection::Class;
use crate::Race;

#[cfg(test)]
mod test_on_kill {
    use super::*;

    #[test]
    fn test_on_kill_triggers_when_monster_destroys_target() {
        let mut game = create_test_game();
        let player_a = game.player_id_a;
        let player_b = game.player_id_b;

        let killer_id = game.entities.len() + 100;
        let target_id = create_test_monster(&mut game, player_b, 1, 3, 3);

        let killer = CardInstance {
            id: killer_id,
            name: "Killer Monster".to_string(),
            description: "Has on_kill effect".to_string(),
            template_id: 9999,
            race: Race::COMMON,
            class: Class::COMMON,
            cost: 3,
            owner: player_a,
            location: Location::Field(0),
            card_type: CardTypeInstance::Monster(MonsterInstance {
                attack: 5,
                hp: 5,
                max_hp: 5,
                asleep: false,
                attack_count: 0,
                keywords: vec![],
                on_play: vec![],
                on_attack: vec![],
                on_defense: vec![],
                on_kill: vec![Effect::MakeDraw {
                    initiator: killer_id,
                    player: crate::game::effects::PlayerTarget::Player,
                    amount: 1,
                }],
                on_turn_end: vec![],
                on_turn_start: vec![],
                on_damaged: vec![],
                on_death: vec![],
                on_surrounded: vec![],
                on_alone: vec![],
            }),
            play_target: None,
        };
        game.entities.insert(killer_id, killer);

        add_card_to_deck(&mut game, player_a);

        let initial_hand_size = game.get_hand(player_a).len();

        game.effect_queue.push_back(Effect::Destroy {
            initiator: killer_id,
            target: Target::Id(target_id),
        });

        let actions = game.compute_commands().unwrap();

        assert!(
            actions.iter().any(|a| matches!(a, Action::TriggerOnKill(id) if *id == killer_id)),
            "Should trigger on_kill for killer"
        );

        let final_hand_size = game.get_hand(player_a).len();
        assert_eq!(
            final_hand_size,
            initial_hand_size + 1,
            "Player A should have drawn 1 card from on_kill"
        );

        let target = game.entities.get(&target_id).unwrap();
        assert_eq!(
            target.location,
            Location::Graveyard,
            "Target should be destroyed"
        );
    }


    #[test]
    fn test_on_kill_does_not_trigger_when_monster_has_no_on_kill_effects() {
        let mut game = create_test_game();
        let player_a = game.player_id_a;
        let player_b = game.player_id_b;

        let killer_id = create_test_monster(&mut game, player_a, 0, 5, 5);
        let target_id = create_test_monster(&mut game, player_b, 1, 3, 3);

        game.effect_queue.push_back(Effect::Destroy {
            initiator: killer_id,
            target: Target::Id(target_id),
        });

        let actions = game.compute_commands().unwrap();

        assert!(
            !actions.iter().any(|a| matches!(a, Action::TriggerOnKill(id) if *id == killer_id)),
            "Should not trigger on_kill when monster has no on_kill effects"
        );
    }

    #[test]
    fn test_on_kill_triggers_multiple_times_for_multiple_kills() {
        let mut game = create_test_game();
        let player_a = game.player_id_a;
        let player_b = game.player_id_b;

        let killer_id = game.entities.len() + 100;
        let target1_id = create_test_monster(&mut game, player_b, 1, 3, 3);
        let target2_id = create_test_monster(&mut game, player_b, 2, 3, 3);

        let killer = CardInstance {
            id: killer_id,
            name: "Killer Monster".to_string(),
            description: "Has on_kill effect".to_string(),
            template_id: 9999,
            race: Race::COMMON,
            class: Class::COMMON,
            cost: 3,
            owner: player_a,
            location: Location::Field(0),
            card_type: CardTypeInstance::Monster(MonsterInstance {
                attack: 5,
                hp: 5,
                max_hp: 5,
                asleep: false,
                attack_count: 0,
                keywords: vec![],
                on_play: vec![],
                on_attack: vec![],
                on_defense: vec![],
                on_kill: vec![Effect::MakeDraw {
                    initiator: killer_id,
                    player: crate::game::effects::PlayerTarget::Player,
                    amount: 1,
                }],
                on_turn_end: vec![],
                on_turn_start: vec![],
                on_damaged: vec![],
                on_death: vec![],
                on_surrounded: vec![],
                on_alone: vec![],
            }),
            play_target: None,
        };
        game.entities.insert(killer_id, killer);

        add_card_to_deck(&mut game, player_a);
        add_card_to_deck(&mut game, player_a);

        let initial_hand_size = game.get_hand(player_a).len();

        game.effect_queue.push_back(Effect::Destroy {
            initiator: killer_id,
            target: Target::Ids(vec![target1_id, target2_id]),
        });

        let actions = game.compute_commands().unwrap();

        let on_kill_count = actions
            .iter()
            .filter(|a| matches!(a, Action::TriggerOnKill(id) if *id == killer_id))
            .count();
        assert_eq!(
            on_kill_count, 2,
            "Should trigger on_kill twice for two kills"
        );

        let final_hand_size = game.get_hand(player_a).len();
        assert_eq!(
            final_hand_size,
            initial_hand_size + 2,
            "Player A should have drawn 2 cards from 2 kills"
        );
    }
}
