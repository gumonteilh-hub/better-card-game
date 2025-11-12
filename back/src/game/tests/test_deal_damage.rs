// FUNCTIONAL RULES: DealDamage Effect
//
// 1. Damage reduces HP of player targets
// 2. Damage reduces HP of monster targets
// 3. Monster with 0 HP is destroyed and moved to Graveyard
// 4. Player with 0 HP loses the game (opponent wins)

use crate::game::{
    card::CardTypeInstance,
    effects::{Effect, Target},
    types::Location,
};

use super::test_utils::{create_test_game, create_test_monster, create_test_spell};

#[test]
fn test_deal_damage_to_player() {
    let mut game = create_test_game();
    let player_a = 0;
    let player_b = 1;

    let initial_hp = game.players.get(&player_b).unwrap().hp;
    assert_eq!(initial_hp, 30);

    let spell_id = create_test_spell(
        &mut game,
        player_a,
        vec![Effect::DealDamage {
            initiator: 0,
            target: Target::EnnemyPlayer,
            amount: 3,
        }],
    );

    let result = game.play_spell(player_a, spell_id);
    assert!(result.is_ok());
    game.compute_commands().unwrap();

    let final_hp = game.players.get(&player_b).unwrap().hp;
    assert_eq!(final_hp, 27);
}

#[test]
fn test_deal_damage_to_monster() {
    let mut game = create_test_game();
    let player_a = 0;
    let player_b = 1;

    let monster_id = create_test_monster(&mut game, player_b, 3, 5, 5);

    let monster = game.entities.get(&monster_id).unwrap();
    match &monster.card_type {
        CardTypeInstance::Monster(monster_instance) => {
            assert_eq!(monster_instance.hp, 5);
        }
        _ => panic!("Expected monster"),
    }

    let spell_id = create_test_spell(
        &mut game,
        player_a,
        vec![Effect::DealDamage {
            initiator: 0,
            target: Target::Id(monster_id),
            amount: 3,
        }],
    );

    let result = game.play_spell(player_a, spell_id);
    assert!(result.is_ok());
    game.compute_commands().unwrap();

    let monster = game.entities.get(&monster_id).unwrap();
    match &monster.card_type {
        CardTypeInstance::Monster(monster_instance) => {
            assert_eq!(monster_instance.hp, 2);
        }
        _ => panic!("Expected monster"),
    }
}

#[test]
fn test_monster_destroyed_when_hp_zero() {
    let mut game = create_test_game();
    let player_a = 0;
    let player_b = 1;

    let monster_id = create_test_monster(&mut game, player_b, 3, 3, 5);

    let initial_location = &game.entities.get(&monster_id).unwrap().location;
    assert!(matches!(initial_location, Location::Field(_)));

    let spell_id = create_test_spell(
        &mut game,
        player_a,
        vec![Effect::DealDamage {
            initiator: 0,
            target: Target::Id(monster_id),
            amount: 3,
        }],
    );

    let result = game.play_spell(player_a, spell_id);
    assert!(result.is_ok());

    game.compute_commands().unwrap();

    let final_location = &game.entities.get(&monster_id).unwrap().location;
    assert_eq!(final_location, &Location::Graveyard);
}

#[test]
fn test_player_loses_when_hp_zero() {
    let mut game = create_test_game();
    let player_a = 0;
    let player_b = 1;

    game.players.get_mut(&player_b).unwrap().hp = 5;

    assert!(game.winner_id.is_none());

    let spell_id = create_test_spell(
        &mut game,
        player_a,
        vec![Effect::DealDamage {
            initiator: 0,
            target: Target::EnnemyPlayer,
            amount: 5,
        }],
    );

    let result = game.play_spell(player_a, spell_id);
    assert!(result.is_ok());

    game.compute_commands().unwrap();

    assert_eq!(game.winner_id, Some(player_a));
}
