use serde::Serialize;
use crate::collection::Faction;

use super::types::PlayerId;

#[derive(Debug, Clone, Serialize)]
pub struct PlayerInstance {
    pub player_id: PlayerId,
    pub hp: usize,
    pub mana: usize,
    pub base_mana: usize,
    pub faction: Faction,
}
impl PlayerInstance {
    pub fn new(id: usize, base_mana: usize, faction: Faction) -> Self {
        Self {
            player_id: id,
            hp: 30,
            mana: base_mana,
            base_mana,
            faction
        }
    }
}
