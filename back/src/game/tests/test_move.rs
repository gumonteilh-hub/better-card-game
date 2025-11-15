// FUNCTIONAL RULES: Movement System
//
// 1. A monster on the field can move to a linked adjacent position
// 2. Movement consumes 1 movement point (player has 3 by default)
// 3. The target position must be empty (not occupied)
// 4. The monster must belong to the player performing the movement
// 5. The monster must be on the field (not in hand or graveyard)
// 6. A player cannot move if they have no movement points left

#[cfg(test)]
mod tests {
    use super::super::test_utils::{create_test_game, create_test_monster};
    use crate::game::types::Location;

    #[test]
    fn test_monster_can_move_to_linked_adjacent_position() {
        // a) Initialize with minimal setup
        let mut game = create_test_game();
        let player_a = game.player_id_a;

        // b) Modify state: place a monster on position 0
        let monster_id = create_test_monster(&mut game, player_a, 0, 5, 5);

        // Verify initial position
        assert_eq!(
            game.entities.get(&monster_id).unwrap().location,
            Location::Field(0)
        );

        // c) Test as user would: move to linked position 1
        game.move_card(player_a, monster_id, 1).unwrap();

        // d) Assert the monster moved to position 1
        assert_eq!(
            game.entities.get(&monster_id).unwrap().location,
            Location::Field(1)
        );
    }

    #[test]
    fn test_movement_consumes_one_movement_point() {
        // a) Initialize
        let mut game = create_test_game();
        let player_a = game.player_id_a;

        // b) Modify state: place a monster on position 0
        let monster_id = create_test_monster(&mut game, player_a, 0, 5, 5);

        // Verify player has 3 movement points initially
        assert_eq!(game.players.get(&player_a).unwrap().move_count, 3);

        // c) Test: move the monster
        game.move_card(player_a, monster_id, 1).unwrap();

        // d) Assert movement points decreased by 1
        assert_eq!(game.players.get(&player_a).unwrap().move_count, 2);
    }

    #[test]
    fn test_cannot_move_to_occupied_position() {
        // a) Initialize
        let mut game = create_test_game();
        let player_a = game.player_id_a;

        // b) Modify state: place two monsters on positions 0 and 1
        let monster_a = create_test_monster(&mut game, player_a, 0, 5, 5);
        let monster_b = create_test_monster(&mut game, player_a, 1, 5, 5);

        // c) Test: try to move monster_a to position 1 (occupied by monster_b)
        let result = game.move_card(player_a, monster_a, 1);

        // d) Assert the move failed
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Game Logic Error: You can't move to a position not empty"
        );
        // Verify monster_a didn't move
        assert_eq!(
            game.entities.get(&monster_a).unwrap().location,
            Location::Field(0)
        );
    }

    #[test]
    fn test_only_owner_can_move_their_monsters() {
        // a) Initialize
        let mut game = create_test_game();
        let player_a = game.player_id_a;
        let player_b = game.player_id_b;

        // b) Modify state: player A has a monster on position 0
        let monster_a = create_test_monster(&mut game, player_a, 0, 5, 5);

        // c) Test: player B tries to move player A's monster
        let result = game.move_card(player_b, monster_a, 1);

        // d) Assert the move failed
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Game Logic Error: You can only move your monsters"
        );
        // Verify monster didn't move
        assert_eq!(
            game.entities.get(&monster_a).unwrap().location,
            Location::Field(0)
        );
    }

    #[test]
    fn test_monster_must_be_on_field_to_move() {
        // a) Initialize
        let mut game = create_test_game();
        let player_a = game.player_id_a;

        // b) Modify state: create a monster in hand (not on field)
        use crate::game::card::{CardInstance, CardTypeInstance, MonsterInstance};
        use crate::{Race, collection::Class};

        let monster_id = 100;
        let monster = CardInstance {
            id: monster_id,
            name: "Test Monster".to_string(),
            description: "Test".to_string(),
            template_id: 9999,
            race: Race::COMMON,
            class: Class::COMMON,
            cost: 0,
            owner: player_a,
            location: Location::Hand,
            card_type: CardTypeInstance::Monster(MonsterInstance {
                attack: 2,
                hp: 5,
                max_hp: 5,
                asleep: false,
                attack_count: 0,
                keywords: vec![],
                on_play: vec![],
                on_attack: vec![],
                on_death: vec![],
            }),
            play_target: None
        };
        game.entities.insert(monster_id, monster);

        // c) Test: try to move a monster in hand
        let result = game.move_card(player_a, monster_id, 1);

        // d) Assert the move failed
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Game Logic Error: Card must be on the field"
        );
    }

    #[test]
    fn test_cannot_move_without_movement_points() {
        // a) Initialize
        let mut game = create_test_game();
        let player_a = game.player_id_a;

        // b) Modify state: place a monster and set movement points to 0
        let monster_id = create_test_monster(&mut game, player_a, 0, 5, 5);
        game.players.get_mut(&player_a).unwrap().move_count = 0;

        // c) Test: try to move without movement points
        let result = game.move_card(player_a, monster_id, 1);

        // d) Assert the move failed
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Game Logic Error: You don't have any move left"
        );
        // Verify monster didn't move
        assert_eq!(
            game.entities.get(&monster_id).unwrap().location,
            Location::Field(0)
        );
    }
}
