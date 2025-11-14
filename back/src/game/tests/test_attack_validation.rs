// FUNCTIONAL RULES: attack() Validation
//
// 1. A monster must be on the field (Location::Field) to attack
// 2. A monster must be on an attack position [0, 2, 3, 5, 6] to attack
// 3. A monster cannot attack if it is asleep (asleep = true)
// 4. A regular monster can only attack once per turn (attack_count <= 0)
// 5. A monster with Windfury can attack twice per turn (attack_count <= 1)
// 6. A player cannot attack their own player
// 7. A player cannot attack their own monsters
// 8. A player cannot attack the enemy player if the enemy has monsters in defense positions [1, 2, 4, 5, 7]
// 9. A spell cannot attack

#[cfg(test)]
mod tests {
    use super::super::test_utils::{create_test_game, create_test_monster, create_test_monster_with_attack};
    use crate::game::card::{CardInstance, CardTypeInstance, Keyword, MonsterInstance};
    use crate::game::types::Location;
    use crate::{collection::Class, Race};

    #[test]
    fn test_monster_must_be_on_field_to_attack() {
        // a) Initialize
        let mut game = create_test_game();
        let player_a = game.player_id_a;
        let player_b = game.player_id_b;

        // b) Modify state: create a monster in hand (not on field)
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
            location: Location::Hand, // In hand, not on field
            card_type: CardTypeInstance::Monster(MonsterInstance {
                attack: 5,
                hp: 5,
                max_hp: 5,
                asleep: false,
                attack_count: 0,
                keywords: vec![],
                on_play: vec![],
                on_attack: vec![],
                on_death: vec![],
            }),
        };
        game.entities.insert(monster_id, monster);

        let target = create_test_monster(&mut game, player_b, 1, 5, 5);

        // c) Test: try to attack with monster in hand
        let result = game.attack(player_a, monster_id, target);

        // d) Assert the attack failed
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Game Logic Error: This monster must be on the field to attack"
        );
    }

    #[test]
    fn test_monster_must_be_on_attack_position() {
        // a) Initialize
        let mut game = create_test_game();
        let player_a = game.player_id_a;
        let player_b = game.player_id_b;

        // b) Modify state: create a monster on defense-only position 1
        let monster_a = create_test_monster_with_attack(&mut game, player_a, 1, 5, 10, 10); // Position 1 is defense-only
        let monster_b = create_test_monster(&mut game, player_b, 2, 5, 5);

        // Make monster not asleep
        if let CardTypeInstance::Monster(ref mut m) = game.entities.get_mut(&monster_a).unwrap().card_type {
            m.asleep = false;
        }

        // c) Test: try to attack from defense-only position
        let result = game.attack(player_a, monster_a, monster_b);

        // d) Assert the attack failed
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Game Logic Error: This monster must be on an attack slot to attack"
        );
    }

    #[test]
    fn test_asleep_monster_cannot_attack() {
        // a) Initialize
        let mut game = create_test_game();
        let player_a = game.player_id_a;
        let player_b = game.player_id_b;

        // b) Modify state: create an asleep monster on attack position
        let monster_a = create_test_monster_with_attack(&mut game, player_a, 0, 5, 10, 10);
        let monster_b = create_test_monster(&mut game, player_b, 1, 5, 5);

        // Manually set monster to asleep (helper creates them awake)
        if let CardTypeInstance::Monster(ref mut m) = game.entities.get_mut(&monster_a).unwrap().card_type {
            m.asleep = true;
        }

        // Verify monster is asleep
        if let CardTypeInstance::Monster(ref m) = game.entities.get(&monster_a).unwrap().card_type {
            assert_eq!(m.asleep, true);
        }

        // c) Test: try to attack with asleep monster
        let result = game.attack(player_a, monster_a, monster_b);

        // d) Assert the attack failed
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Game Logic Error: This monster can't attack on his first turn"
        );
    }

    #[test]
    fn test_regular_monster_can_only_attack_once() {
        // a) Initialize
        let mut game = create_test_game();
        let player_a = game.player_id_a;
        let player_b = game.player_id_b;

        // b) Modify state: create monsters and make one attack
        let monster_a = create_test_monster_with_attack(&mut game, player_a, 0, 5, 10, 10);
        let monster_b = create_test_monster(&mut game, player_b, 1, 5, 5);
        let monster_c = create_test_monster(&mut game, player_b, 2, 5, 5);

        // Make monster not asleep
        if let CardTypeInstance::Monster(ref mut m) = game.entities.get_mut(&monster_a).unwrap().card_type {
            m.asleep = false;
        }

        // First attack succeeds
        game.attack(player_a, monster_a, monster_b).unwrap();

        // c) Test: try to attack again
        let result = game.attack(player_a, monster_a, monster_c);

        // d) Assert the second attack failed
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Game Logic Error: This monster has already attacked this turn"
        );
    }

    #[test]
    fn test_windfury_monster_can_attack_twice() {
        // a) Initialize
        let mut game = create_test_game();
        let player_a = game.player_id_a;
        let player_b = game.player_id_b;

        // b) Modify state: create a monster with Windfury
        let monster_id = 100;
        let monster = CardInstance {
            id: monster_id,
            name: "Windfury Monster".to_string(),
            description: "Test".to_string(),
            template_id: 9999,
            race: Race::COMMON,
            class: Class::COMMON,
            cost: 3,
            owner: player_a,
            location: Location::Field(0), // Attack position
            card_type: CardTypeInstance::Monster(MonsterInstance {
                attack: 5,
                hp: 10,
                max_hp: 10,
                asleep: false,
                attack_count: 0,
                keywords: vec![Keyword::Windfury],
                on_play: vec![],
                on_attack: vec![],
                on_death: vec![],
            }),
        };
        game.entities.insert(monster_id, monster);

        let target1 = create_test_monster(&mut game, player_b, 1, 5, 5);
        let target2 = create_test_monster(&mut game, player_b, 2, 5, 5);

        // c) Test: attack twice with Windfury monster
        let result1 = game.attack(player_a, monster_id, target1);
        let result2 = game.attack(player_a, monster_id, target2);

        // d) Assert both attacks succeeded
        assert!(result1.is_ok());
        assert!(result2.is_ok());
    }

    #[test]
    fn test_windfury_monster_cannot_attack_three_times() {
        // a) Initialize
        let mut game = create_test_game();
        let player_a = game.player_id_a;
        let player_b = game.player_id_b;

        // b) Modify state: create a monster with Windfury and make it attack twice
        let monster_id = 100;
        let monster = CardInstance {
            id: monster_id,
            name: "Windfury Monster".to_string(),
            description: "Test".to_string(),
            template_id: 9999,
            race: Race::COMMON,
            class: Class::COMMON,
            cost: 3,
            owner: player_a,
            location: Location::Field(0), // Attack position
            card_type: CardTypeInstance::Monster(MonsterInstance {
                attack: 5,
                hp: 20,
                max_hp: 20,
                asleep: false,
                attack_count: 0,
                keywords: vec![Keyword::Windfury],
                on_play: vec![],
                on_attack: vec![],
                on_death: vec![],
            }),
        };
        game.entities.insert(monster_id, monster);

        let target1 = create_test_monster(&mut game, player_b, 1, 5, 5);
        let target2 = create_test_monster(&mut game, player_b, 2, 5, 5);
        let target3 = create_test_monster(&mut game, player_b, 4, 5, 5);

        // Attack twice (should succeed)
        game.attack(player_a, monster_id, target1).unwrap();
        game.attack(player_a, monster_id, target2).unwrap();

        // c) Test: try to attack a third time
        let result = game.attack(player_a, monster_id, target3);

        // d) Assert the third attack failed
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Game Logic Error: This monster has already attacked this turn"
        );
    }

    #[test]
    fn test_cannot_attack_own_player() {
        // a) Initialize
        let mut game = create_test_game();
        let player_a = game.player_id_a;

        // b) Modify state: create a monster for player A
        let monster_a = create_test_monster_with_attack(&mut game, player_a, 0, 5, 10, 10);

        // Make monster not asleep
        if let CardTypeInstance::Monster(ref mut m) = game.entities.get_mut(&monster_a).unwrap().card_type {
            m.asleep = false;
        }

        // c) Test: try to attack own player
        let result = game.attack(player_a, monster_a, player_a);

        // d) Assert the attack failed
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Game Logic Error: You can't attack your own player"
        );
    }

    #[test]
    fn test_cannot_attack_own_monster() {
        // a) Initialize
        let mut game = create_test_game();
        let player_a = game.player_id_a;

        // b) Modify state: create two monsters for player A
        let monster_a = create_test_monster_with_attack(&mut game, player_a, 0, 5, 10, 10);
        let monster_b = create_test_monster(&mut game, player_a, 1, 5, 5);

        // Make monster not asleep
        if let CardTypeInstance::Monster(ref mut m) = game.entities.get_mut(&monster_a).unwrap().card_type {
            m.asleep = false;
        }

        // c) Test: try to attack own monster
        let result = game.attack(player_a, monster_a, monster_b);

        // d) Assert the attack failed
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Game Logic Error: You can't attack your own monster"
        );
    }

    #[test]
    fn test_cannot_attack_player_when_enemy_has_defense_monsters() {
        // a) Initialize
        let mut game = create_test_game();
        let player_a = game.player_id_a;
        let player_b = game.player_id_b;

        // b) Modify state: create attacking monster and defender
        let monster_a = create_test_monster_with_attack(&mut game, player_a, 0, 5, 10, 10);
        let _defender = create_test_monster(&mut game, player_b, 1, 5, 5); // Position 1 is a defense position

        // Make monster not asleep
        if let CardTypeInstance::Monster(ref mut m) = game.entities.get_mut(&monster_a).unwrap().card_type {
            m.asleep = false;
        }

        // c) Test: try to attack enemy player while they have defense
        let result = game.attack(player_a, monster_a, player_b);

        // d) Assert the attack failed
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Game Logic Error: You can't attack the enemy player if he has a monster in defense"
        );
    }

    #[test]
    fn test_can_attack_player_when_no_defense_monsters() {
        // a) Initialize
        let mut game = create_test_game();
        let player_a = game.player_id_a;
        let player_b = game.player_id_b;

        // b) Modify state: create attacking monster, enemy has no defense
        let monster_a = create_test_monster_with_attack(&mut game, player_a, 0, 5, 10, 10);

        // Make monster not asleep
        if let CardTypeInstance::Monster(ref mut m) = game.entities.get_mut(&monster_a).unwrap().card_type {
            m.asleep = false;
        }

        // c) Test: attack enemy player (should succeed)
        let result = game.attack(player_a, monster_a, player_b);

        // d) Assert the attack succeeded
        assert!(result.is_ok());
    }

    #[test]
    fn test_can_attack_player_when_only_attack_position_monsters() {
        // a) Initialize
        let mut game = create_test_game();
        let player_a = game.player_id_a;
        let player_b = game.player_id_b;

        // b) Modify state: create monsters, enemy has only attack-only position (0 is attack-only, not defense)
        let monster_a = create_test_monster_with_attack(&mut game, player_a, 3, 5, 10, 10);
        let _enemy_monster = create_test_monster(&mut game, player_b, 0, 5, 5); // Position 0 is attack-only, not defense

        // Make monster not asleep
        if let CardTypeInstance::Monster(ref mut m) = game.entities.get_mut(&monster_a).unwrap().card_type {
            m.asleep = false;
        }

        // c) Test: attack enemy player (should succeed because enemy has no defense)
        let result = game.attack(player_a, monster_a, player_b);

        // d) Assert the attack succeeded
        assert!(result.is_ok());
    }

    #[test]
    fn test_spell_cannot_attack() {
        // a) Initialize
        let mut game = create_test_game();
        let player_a = game.player_id_a;
        let player_b = game.player_id_b;

        // b) Modify state: create a spell on the field (artificial scenario)
        use crate::game::card::SpellInstance;

        let spell_id = 100;
        let spell = CardInstance {
            id: spell_id,
            name: "Test Spell".to_string(),
            description: "Test".to_string(),
            template_id: 9999,
            race: Race::COMMON,
            class: Class::COMMON,
            cost: 3,
            owner: player_a,
            location: Location::Field(0), // Artificially on field
            card_type: CardTypeInstance::Spell(SpellInstance {
                effect: vec![],
            }),
        };
        game.entities.insert(spell_id, spell);

        let target = create_test_monster(&mut game, player_b, 1, 5, 5);

        // c) Test: try to attack with spell
        let result = game.attack(player_a, spell_id, target);

        // d) Assert the attack failed
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Game Logic Error: A spell can not attack"
        );
    }
}
