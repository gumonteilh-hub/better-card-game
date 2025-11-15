use crate::collection::types::CardTemplate;
use crate::error::Result;
use crate::game::action::Action;
use crate::game::card::CardInstance;
use crate::game::types::{InstanceId, Location};

const SPAWN_POSITIONS: [usize; 8] = [3, 4, 2, 5, 0, 1, 6, 7];

pub fn compute(
    context: &mut crate::Game,
    initiator: &InstanceId,
    side: &crate::game::effects::PlayerTarget,
    template: &CardTemplate,
) -> Result<Vec<Action>> {
    let mut actions = Vec::new();

    let targets = super::resolve_player_target(*initiator, side, context)?;

    for target in targets {
        if context.get_field(target).len() == 8 {
            continue;
        }

        for position in SPAWN_POSITIONS {
            if !context
                .get_field_with_position(target)
                .contains_key(&position)
            {
                let new_instance_id = 10000 + context.entities.len();
                let oponent = context.get_opponent(&target)?;
                let mut new_card =
                    CardInstance::new(new_instance_id, target, template, oponent.player_id);
                new_card.location = Location::Field(position);
                context.entities.insert(new_instance_id, new_card.clone());
                actions.push(Action::Summon {
                    source: Location::Deck,
                    destination: position,
                    target: new_card,
                    owner: target,
                });
                break;
            }
        }
    }
    Ok(actions)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Race;
    use crate::collection::Class;
    use crate::collection::types::{CardTypeTemplate, MonsterTemplate};
    use crate::game::action::Action;
    use crate::game::effects::PlayerTarget;
    use crate::game::tests::test_utils::{create_test_game, create_test_monster};

    fn create_test_template() -> CardTemplate {
        CardTemplate {
            id: 1,
            cost: 1,
            name: "Test Summon".to_string(),
            description: "Test".to_string(),
            race: Race::COMMON,
            class: Class::COMMON,
            play_target: None,
            card_type: CardTypeTemplate::Monster(MonsterTemplate {
                attack: 1,
                hp: 1,
                keywords: vec![],
                on_play: vec![],
                on_attack: vec![],
                on_death: vec![],
            }),
        }
    }

    #[test]
    fn test_spawn_position_priority() {
        let mut game = create_test_game();
        let player_id = game.player_id_a;

        create_test_monster(&mut game, player_id, 3, 1, 1);

        let template = create_test_template();
        let initiator = player_id;

        let actions = compute(&mut game, &initiator, &PlayerTarget::Player, &template).unwrap();

        assert_eq!(actions.len(), 1);
        if let Action::Summon { destination, .. } = actions[0] {
            assert_eq!(destination, 4);
        } else {
            panic!("Expected Action::Summon");
        }
    }

    #[test]
    fn test_action_fields() {
        let mut game = create_test_game();
        let player_id = game.player_id_a;

        let template = create_test_template();
        let initiator = player_id;

        let actions = compute(&mut game, &initiator, &PlayerTarget::Player, &template).unwrap();

        assert_eq!(actions.len(), 1);
        if let Action::Summon {
            source,
            destination,
            target,
            owner,
        } = &actions[0]
        {
            assert_eq!(*source, Location::Deck);
            assert_eq!(*destination, 3);
            assert_eq!(*owner, player_id);
            assert_eq!(target.name, "Test Summon");
            assert_eq!(target.location, Location::Field(3));
        } else {
            panic!("Expected Action::Summon");
        }
    }

    #[test]
    fn test_multiple_player_targets() {
        let mut game = create_test_game();
        let initiator = game.player_id_a;

        let template = create_test_template();

        let actions =
            compute(&mut game, &initiator, &PlayerTarget::BothPlayers, &template).unwrap();

        assert_eq!(actions.len(), 2);

        let owners: Vec<usize> = actions
            .iter()
            .filter_map(|a| {
                if let Action::Summon { owner, .. } = a {
                    Some(*owner)
                } else {
                    None
                }
            })
            .collect();

        assert!(owners.contains(&game.player_id_a));
        assert!(owners.contains(&game.player_id_b));
    }

    #[test]
    fn test_full_field() {
        let mut game = create_test_game();
        let player_id = game.player_id_a;

        for position in 0..8 {
            create_test_monster(&mut game, player_id, position, 1, 1);
        }

        let template = create_test_template();
        let initiator = player_id;

        let actions = compute(&mut game, &initiator, &PlayerTarget::Player, &template).unwrap();

        assert_eq!(actions.len(), 0);
    }
}
