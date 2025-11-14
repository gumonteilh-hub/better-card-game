// FUNCTIONAL RULES: Keywords
//
// CHARGE:
// 1. A monster with Charge is not asleep (asleep = false) when summoned
// 2. A monster with Charge can attack on the turn it is summoned
//
// WINDFURY:
// 1. A monster with Windfury can attack twice per turn
// 2. A monster with Windfury cannot attack more than twice per turn
// 3. The attack counter (attack_count) increments with each attack

#[cfg(test)]
mod tests {
    use super::super::test_utils::{create_test_game, create_test_monster_in_hand};
    use crate::game::card::{CardTypeInstance, Keyword};
    use crate::game::types::Location;

    // ===== CHARGE KEYWORD TESTS =====

    #[test]
    fn test_charge_monster_not_asleep_when_summoned() {
        // a) Initialize
        let mut game = create_test_game();
        let player_a = game.player_id_a;

        // b) Modify state: create a monster with Charge in hand
        let monster_id =
            create_test_monster_in_hand(&mut game, player_a, 3, 4, vec![Keyword::Charge], vec![]);

        // Give player enough mana
        game.players.get_mut(&player_a).unwrap().mana = 5;

        // c) Test as user would: summon the monster with Charge
        game.play_monster(player_a, monster_id, 0).unwrap();
        game.compute_commands().unwrap();

        // d) Assert the monster is NOT asleep
        let monster_instance = game.entities.get(&monster_id).unwrap();
        if let CardTypeInstance::Monster(monster) = &monster_instance.card_type {
            assert_eq!(monster.asleep, false);
            assert!(monster.keywords.contains(&Keyword::Charge));
        } else {
            panic!("Expected monster card type");
        }
    }

    #[test]
    fn test_charge_monster_can_attack_immediately() {
        // a) Initialize
        let mut game = create_test_game();
        let player_a = game.player_id_a;
        let player_b = game.player_id_b;

        // b) Modify state: create monster with Charge and an enemy target
        let monster_with_charge =
            create_test_monster_in_hand(&mut game, player_a, 3, 4, vec![Keyword::Charge], vec![]);

        // Create enemy target
        use super::super::test_utils::create_test_monster_with_attack;
        let enemy_monster = create_test_monster_with_attack(&mut game, player_b, 1, 2, 5, 5);

        // Give player enough mana and summon
        game.players.get_mut(&player_a).unwrap().mana = 5;
        game.play_monster(player_a, monster_with_charge, 0).unwrap();
        game.compute_commands().unwrap();

        // Verify monster is on field and not asleep
        assert_eq!(
            game.entities.get(&monster_with_charge).unwrap().location,
            Location::Field(0)
        );

        // c) Test: attack immediately with the just-summoned monster
        let result = game.attack(player_a, monster_with_charge, enemy_monster);

        // d) Assert the attack succeeded (no error)
        assert!(result.is_ok());
    }

    // ===== WINDFURY KEYWORD TESTS =====

    #[test]
    fn test_windfury_monster_can_attack_twice() {
        // a) Initialize
        let mut game = create_test_game();
        let player_a = game.player_id_a;
        let player_b = game.player_id_b;

        // b) Modify state: create monster with Windfury on field
        use crate::game::card::{CardInstance, CardTypeInstance, MonsterInstance};
        use crate::{Race, collection::Class};

        let monster_id = 100;
        let monster = CardInstance {
            id: monster_id,
            name: "Windfury Monster".to_string(),
            description: "Test".to_string(),
            template_id: 9999,
            race: Race::COMMON,
            class: Class::COMMON,
            cost: 0,
            owner: player_a,
            location: Location::Field(0),
            card_type: CardTypeInstance::Monster(MonsterInstance {
                attack: 3,
                hp: 5,
                max_hp: 5,
                asleep: false,
                attack_count: 0,
                keywords: vec![Keyword::Windfury],
                on_play: vec![],
                on_attack: vec![],
                on_death: vec![],
            }),
        };
        game.entities.insert(monster_id, monster);

        // Create two enemy targets
        use super::super::test_utils::create_test_monster_with_attack;
        let enemy_1 = create_test_monster_with_attack(&mut game, player_b, 1, 2, 10, 10);
        let enemy_2 = create_test_monster_with_attack(&mut game, player_b, 4, 2, 10, 10);

        // Verify initial attack count
        if let CardTypeInstance::Monster(monster) =
            &game.entities.get(&monster_id).unwrap().card_type
        {
            assert_eq!(monster.attack_count, 0);
        }

        // c) Test: attack twice with Windfury monster
        // First attack
        game.attack(player_a, monster_id, enemy_1).unwrap();
        game.compute_commands().unwrap();

        // Second attack
        let result = game.attack(player_a, monster_id, enemy_2);

        // d) Assert both attacks succeeded
        assert!(result.is_ok());

        // Verify attack count is 2
        if let CardTypeInstance::Monster(monster) =
            &game.entities.get(&monster_id).unwrap().card_type
        {
            assert_eq!(monster.attack_count, 2);
            assert!(monster.keywords.contains(&Keyword::Windfury));
        }
    }

    #[test]
    fn test_windfury_monster_cannot_attack_three_times() {
        // a) Initialize
        let mut game = create_test_game();
        let player_a = game.player_id_a;
        let player_b = game.player_id_b;

        // b) Modify state: create Windfury monster that already attacked twice
        use crate::game::card::{CardInstance, CardTypeInstance, MonsterInstance};
        use crate::{Race, collection::Class};

        let monster_id = 100;
        let monster = CardInstance {
            id: monster_id,
            name: "Windfury Monster".to_string(),
            description: "Test".to_string(),
            template_id: 9999,
            race: Race::COMMON,
            class: Class::COMMON,
            cost: 0,
            owner: player_a,
            location: Location::Field(0),
            card_type: CardTypeInstance::Monster(MonsterInstance {
                attack: 3,
                hp: 5,
                max_hp: 5,
                asleep: false,
                attack_count: 2, // Already attacked twice
                keywords: vec![Keyword::Windfury],
                on_play: vec![],
                on_attack: vec![],
                on_death: vec![],
            }),
        };
        game.entities.insert(monster_id, monster);

        // Create enemy target
        use super::super::test_utils::create_test_monster_with_attack;
        let enemy = create_test_monster_with_attack(&mut game, player_b, 1, 2, 10, 10);

        // c) Test: try to attack a third time
        let result = game.attack(player_a, monster_id, enemy);

        // d) Assert the third attack failed
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Game Logic Error: This monster has already attacked this turn"
        );
    }

    #[test]
    fn test_windfury_attack_count_increments_with_each_attack() {
        // a) Initialize
        let mut game = create_test_game();
        let player_a = game.player_id_a;
        let player_b = game.player_id_b;

        // b) Modify state: create Windfury monster on field
        use crate::game::card::{CardInstance, CardTypeInstance, MonsterInstance};
        use crate::{Race, collection::Class};

        let monster_id = 100;
        let monster = CardInstance {
            id: monster_id,
            name: "Windfury Monster".to_string(),
            description: "Test".to_string(),
            template_id: 9999,
            race: Race::COMMON,
            class: Class::COMMON,
            cost: 0,
            owner: player_a,
            location: Location::Field(0),
            card_type: CardTypeInstance::Monster(MonsterInstance {
                attack: 3,
                hp: 5,
                max_hp: 5,
                asleep: false,
                attack_count: 0,
                keywords: vec![Keyword::Windfury],
                on_play: vec![],
                on_attack: vec![],
                on_death: vec![],
            }),
        };
        game.entities.insert(monster_id, monster);

        // Create enemy target
        use super::super::test_utils::create_test_monster_with_attack;
        let enemy = create_test_monster_with_attack(&mut game, player_b, 1, 2, 10, 10);

        // c) Test: perform first attack and check counter
        game.attack(player_a, monster_id, enemy).unwrap();

        // d) Assert attack_count incremented to 1
        if let CardTypeInstance::Monster(monster) =
            &game.entities.get(&monster_id).unwrap().card_type
        {
            assert_eq!(monster.attack_count, 1);
        }

        game.compute_commands().unwrap();

        // Perform second attack
        game.attack(player_a, monster_id, enemy).unwrap();

        // Assert attack_count incremented to 2
        if let CardTypeInstance::Monster(monster) =
            &game.entities.get(&monster_id).unwrap().card_type
        {
            assert_eq!(monster.attack_count, 2);
        }
    }
}
