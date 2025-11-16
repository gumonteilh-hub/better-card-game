use crate::error::Result;
use crate::game::action::Action;
use crate::game::types::PlayerId;

pub fn compute(context: &mut crate::Game, player_id: &PlayerId) -> Result<Vec<Action>> {
    context.winner_id = Some(*player_id);
    Ok(vec![Action::Win(*player_id)])
}
