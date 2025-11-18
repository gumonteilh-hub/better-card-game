use crate::collection::Archetype;
use serde::Serialize;

use super::types::PlayerId;

#[derive(Debug, Clone, Serialize)]
pub struct PlayerInstance {
    pub player_id: PlayerId,
    pub hp: usize,
    pub mana: usize,
    pub max_mana: usize,
    pub absolute_max_mana: usize,
    pub move_count: usize,
    pub max_move: usize,
    pub archetype: Archetype,
}
impl PlayerInstance {
    pub fn new(id: usize, max_mana: usize, archetype: Archetype) -> Self {
        Self {
            player_id: id,
            hp: 30,
            mana: max_mana,
            max_mana,
            absolute_max_mana: 10,
            archetype,
            max_move: 3,
            move_count: 3,
        }
    }
}
