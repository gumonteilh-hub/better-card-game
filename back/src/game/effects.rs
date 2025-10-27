use crate::game::types::{EntityId, PlayerId};

use super::events::EventType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Target {
    EnnemyPlayer,
    Player,
    BothPlayers,
    ItSelf,
    Allies,
    Id(EntityId),
    //OneAlly,
    //OneEnnemy,
    Ennemies,
    AllMonsters,
    All,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum PlayerTarget {
    EnnemyPlayer,
    Player,
    BothPlayers,
    Id(PlayerId),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type", content = "value")]
pub enum Effect {
    IncreaseMaxMana {
        initiator: EntityId,
        player: PlayerTarget,
        amount: usize,
    },
    RefreshMana {
        initiator: EntityId,
        player: PlayerTarget,
        amount: usize,
    },
    MakeDraw {
        initiator: EntityId,
        player: PlayerTarget,
        amount: usize,
    },
    AutoDraw {
        player: PlayerId,
        amount: usize,
    },
    Heal {
        initiator: EntityId,
        target: Target,
        amount: usize,
    },
    Destroy {
        initiator: EntityId,
        target: Target,
    },
    DealDamage {
        initiator: EntityId,
        target: Target,
        amount: usize,
    },
    SummonFromHand {
        entity_id: EntityId,
        position: usize,
    },
    Attack {
        initiator: EntityId,
        target: Target,
    },
    Boost {
        initiator: EntityId,
        attack: usize,
        hp: usize,
        target: Target,
    },
    Win(PlayerId),
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TriggeredEffect {
    pub trigger: EventType,
    pub effects: Vec<Effect>,
}
