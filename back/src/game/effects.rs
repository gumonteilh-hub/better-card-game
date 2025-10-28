use crate::game::types::{InstanceId, PlayerId};

use super::events::EventType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Target {
    EnnemyPlayer,
    Player,
    BothPlayers,
    ItSelf,
    Allies,
    Id(InstanceId),
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
        initiator: InstanceId,
        player: PlayerTarget,
        amount: usize,
    },
    RefreshMana {
        initiator: InstanceId,
        player: PlayerTarget,
        amount: usize,
    },
    MakeDraw {
        initiator: InstanceId,
        player: PlayerTarget,
        amount: usize,
    },
    AutoDraw {
        player: PlayerId,
        amount: usize,
    },
    Heal {
        initiator: InstanceId,
        target: Target,
        amount: usize,
    },
    Destroy {
        initiator: InstanceId,
        target: Target,
    },
    DealDamage {
        initiator: InstanceId,
        target: Target,
        amount: usize,
    },
    SummonFromHand {
        entity_id: InstanceId,
        position: usize,
    },
    Attack {
        initiator: InstanceId,
        target: Target,
    },
    Boost {
        initiator: InstanceId,
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
