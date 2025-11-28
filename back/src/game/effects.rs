use crate::{
    collection::types::{CardTemplate, TargetMatcherTemplate},
    game::types::{InstanceId, PlayerId},
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Clone)]
pub enum Target {
    EnnemyPlayer,
    Player,
    BothPlayers,
    ItSelf,
    Allies,
    Id(InstanceId),
    Ennemies,
    AllMonsters,
    All,
    Ids(Vec<InstanceId>),
    Matching(TargetMatcherTemplate),
    And(Box<Target>, Box<Target>),
    Or(Box<Target>, Box<Target>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum PlayerTarget {
    EnnemyPlayer,
    Player,
    BothPlayers,
    Id(PlayerId),
}

#[derive(Debug, Serialize, Clone)]
#[serde(tag = "type", content = "value")]
pub enum Effect {
    // Condition {
    //     initiator: InstanceId,
    //     effect: Box<Effect>,
    //     condition:
    // },
    IncreaseMaxMana {
        initiator: InstanceId,
        player: PlayerTarget,
        amount: usize,
    },
    DecreaseCurrentMana {
        initiator: InstanceId,
        player: PlayerTarget,
        amount: usize,
    },
    RefreshMana {
        initiator: InstanceId,
        player: PlayerTarget,
        amount: usize,
    },
    IncreaseAbsoluteMaxMana {
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
    Summon {
        initiator: InstanceId,
        side: PlayerTarget,
        target: CardTemplate,
    },
    Win(PlayerId),
}
