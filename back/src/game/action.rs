use ::serde::Serialize;

use crate::game::{
    card::CardInstance,
    types::{EntityId, Location, PlayerId},
};

#[derive(Debug, Serialize, Clone)]
#[serde(tag = "type", content = "value")]
pub enum Action {
    Boost {
        target: EntityId,
        attack: usize,
        hp: usize,
    },
    IncreaseMaxMana {
        player: PlayerId,
        amount: usize,
    },
    RefreshMana {
        player: PlayerId,
        amount: usize,
    },
    BurnCard {
        player: PlayerId,
        card: EntityId,
    },
    Draw {
        player: PlayerId,
        card: CardInstance,
    },
    Heal {
        target: EntityId, // or PLayerId, todo update logic to make EntityId and PlayerId the same
        amount: usize,
    },
    Destroy {
        target: EntityId,
    },
    ReceiveDamage {
        target: EntityId,
        amount: usize,
    },
    Summon {
        source: Location,
        destination: usize,
        target: CardInstance,
        owner: PlayerId,
    },
    Attack {
        initiator: EntityId,
        target: EntityId,
    },
    TriggerOnDeath(EntityId),
    TriggerOnPlay(EntityId),
    TriggerOnAttack(EntityId),
    Win(PlayerId),
}
