// FUNCTIONAL RULES: Boost Effect
//
// 1. Positive boost increases monster's attack stat
// 2. Positive boost increases monster's HP and max_hp

use crate::game::{
    card::CardTypeInstance,
    effects::{Effect, Target},
};

use super::test_utils::{create_test_game, create_test_monster, create_test_spell};

#[test]
fn test_boost_increases_attack_hp_and_max_hp() {
    let mut game = create_test_game();
    let player_a = 0;
    let player_b = 1;

    let monster_id = create_test_monster(&mut game, player_b, 3, 2, 5);

    let monster = game.entities.get(&monster_id).unwrap();
    match &monster.card_type {
        CardTypeInstance::Monster(monster_instance) => {
            assert_eq!(monster_instance.attack, 2);
            assert_eq!(monster_instance.hp, 2);
            assert_eq!(monster_instance.max_hp, 5);
        }
        _ => panic!("Expected monster"),
    }

    let spell_id = create_test_spell(
        &mut game,
        player_a,
        vec![Effect::Boost {
            initiator: 0,
            attack: 3,
            hp: 3,
            target: Target::Id(monster_id),
        }],
    );

    let result = game.play_spell(player_a, spell_id);
    assert!(result.is_ok());
    game.compute_commands().unwrap();

    let monster = game.entities.get(&monster_id).unwrap();
    match &monster.card_type {
        CardTypeInstance::Monster(monster_instance) => {
            assert_eq!(monster_instance.attack, 5);
            assert_eq!(monster_instance.hp, 5);
            assert_eq!(monster_instance.max_hp, 8);
        }
        _ => panic!("Expected monster"),
    }
}
