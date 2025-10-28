use std::collections::HashSet;

use crate::{
    error::{Result},
    game::{
        action::Action,
        types::{InstanceId, Location, PlayerId},
        Game,
    },
};

pub fn summon_max_cards(game_state: &mut Game, player_id: PlayerId) -> Result<Vec<Action>> {
    let field_size = game_state.get_field(player_id).len();
    if field_size < 7 {
        let current_mana = game_state.get_mut_player(player_id)?.mana;
        let cards_to_play = maximize_mana_spend(
            game_state
                .get_hand(player_id)
                .iter()
                .map(|(id, entity)| Ok((entity.cost, **id)))
                .collect::<Result<Vec<(usize, InstanceId)>>>()?,
            current_mana,
            7 - field_size,
        );

        let occupied: HashSet<usize> = game_state
            .get_field(player_id)
            .iter()
            .filter_map(|(_, card)| match card.location {
                Location::Field(pos) => Some(pos),
                _ => None,
            })
            .collect();

        let mut free_positions: Vec<usize> = (0..7).filter(|pos| !occupied.contains(pos)).collect();
        free_positions.sort_by_key(|&pos| (pos as i32 - 3).abs());

        for (card_id, &position) in cards_to_play.iter().zip(&free_positions) {
            game_state.play_card(*card_id, position)?;
        }
    }

    let summon_actions = game_state.compute_commands()?;
    Ok(summon_actions)
}

fn maximize_mana_spend(
    options: Vec<(usize, InstanceId)>,
    mana_max: usize,
    place_available: usize,
) -> Vec<InstanceId> {
    let n = options.len();

    if n == 0 || mana_max == 0 || place_available == 0 {
        return vec![];
    }

    let mut dp = vec![vec![vec![0; place_available + 1]; mana_max + 1]; n + 1];

    for i in 1..=n {
        let (cost, _) = options[i - 1];

        for m in 0..=mana_max {
            for p in 0..=place_available {
                // Option 1: ne pas prendre l'objet i-1
                dp[i][m][p] = dp[i - 1][m][p];

                // Option 2: prendre l'objet i-1 (si possible)
                if p > 0 && cost <= m {
                    let with_current = dp[i - 1][m - cost][p - 1] + cost;
                    if with_current > dp[i][m][p] {
                        dp[i][m][p] = with_current;
                    }
                }
            }
        }
    }

    let mut result = Vec::new();
    let mut remaining_mana = mana_max;
    let mut remaining_places = place_available;

    for i in (1..=n).rev() {
        let (cost, entity_id) = options[i - 1];

        if remaining_places > 0 && cost <= remaining_mana {
            let without = if i > 0 {
                dp[i - 1][remaining_mana][remaining_places]
            } else {
                0
            };

            let with = if i > 0 && remaining_places > 0 {
                dp[i - 1][remaining_mana - cost][remaining_places - 1] + cost
            } else if remaining_places > 0 && cost <= remaining_mana {
                cost
            } else {
                0
            };

            if with > without {
                result.push(entity_id);
                remaining_mana -= cost;
                remaining_places -= 1;
            }
        }
    }

    result.reverse();
    result
}

mod test {

    #[test]
    fn simple_max() {
        let options = vec![(5, 1), (3, 2), (6, 3)];

        let result = super::maximize_mana_spend(options, 5, 1);

        assert_eq!(result, vec![1]);
    }

    #[test]
    fn simple_max_more_option() {
        let options = vec![(2, 1), (3, 2), (5, 3)];

        let result = super::maximize_mana_spend(options, 5, 2);

        assert_eq!(result, vec![1, 2]);
    }

    #[test]
    fn simple_max_less_option() {
        let options = vec![(5, 1), (3, 2), (2, 3)];

        let result = super::maximize_mana_spend(options, 5, 2);

        assert_eq!(result, vec![1]);
    }

    #[test]
    fn complexe_max_multiple() {
        let options = vec![(1, 1), (1, 2), (2, 3), (5, 4)];

        let result = super::maximize_mana_spend(options, 3, 7);

        assert_eq!(result, vec![1, 3]);
    }
}
