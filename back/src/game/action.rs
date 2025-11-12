use ::serde::Serialize;

use crate::{
    PublicGameState,
    game::{
        card::CardInstance,
        types::{InstanceId, Location, PlayerId},
    },
};

#[derive(Debug, Serialize, Clone)]
#[serde(tag = "type", content = "value")]
pub enum Action {
    Boost {
        target: InstanceId,
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
        card: InstanceId,
    },
    Draw {
        player: PlayerId,
        card: CardInstance,
    },
    EnemyDraw {
        player: PlayerId,
    },
    Heal {
        target: InstanceId, // or PLayerId, todo update logic to make EntityId and PlayerId the same
        amount: usize,
    },
    Destroy {
        target: InstanceId,
    },
    ReceiveDamage {
        target: InstanceId,
        amount: usize,
    },
    Summon {
        source: Location,
        destination: usize,
        target: CardInstance,
        owner: PlayerId,
    },
    Attack {
        initiator: InstanceId,
        target: InstanceId,
    },
    TriggerOnDeath(InstanceId),
    TriggerOnPlay(InstanceId),
    TriggerOnAttack(InstanceId),
    Win(PlayerId),
    UpdateGameView {
        player: PlayerId,
        game: PublicGameState,
    },
    StartTurn(PlayerId),
}
