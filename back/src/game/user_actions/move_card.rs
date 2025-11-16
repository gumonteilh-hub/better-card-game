use crate::{
    Game,
    error::{Error, Result},
    game::types::{InstanceId, Location, PlayerId},
};

fn get_linked_positions(position: usize) -> Result<Vec<usize>> {
    match position {
        0 => Ok(vec![1, 2]),
        1 => Ok(vec![0, 2]),
        2 => Ok(vec![0, 1, 3, 4]),
        3 => Ok(vec![2, 4, 5]),
        4 => Ok(vec![2, 3, 5]),
        5 => Ok(vec![3, 4, 6, 7]),
        6 => Ok(vec![5, 7]),
        7 => Ok(vec![5, 6]),
        _ => Err(Error::Game("Invalid starting position".into())),
    }
}

pub fn move_card(
    context: &mut Game,
    player: PlayerId,
    card_id: InstanceId,
    position: usize,
) -> Result<()> {
    let card = context
        .entities
        .get(&card_id)
        .ok_or_else(|| Error::Game(format!("Card with id {} not found", card_id)))?;

    if card.owner != player {
        return Err(Error::Game("You can only move your monsters".into()));
    }

    let starting_position = match card.location {
        Location::Field(pos) => pos,
        _ => return Err(Error::Game("Card must be on the field".into())),
    };

    if !get_linked_positions(starting_position)?.contains(&position) {
        return Err(Error::Game("Target position is not valid".into()));
    }

    if context
        .get_field_with_position(card.owner)
        .contains_key(&position)
    {
        return Err(Error::Game("You can't move to a position not empty".into()));
    }

    if context.get_player(context.current_player)?.move_count == 0 {
        return Err(Error::Game("You don't have any move left".into()));
    }

    let owner = context.get_mut_player(context.current_player)?;
    owner.move_count -= 1;

    let card = context
        .entities
        .get_mut(&card_id)
        .ok_or_else(|| Error::Game(format!("Card with id {} not found", card_id)))?;

    card.location = Location::Field(position);

    Ok(())
}
