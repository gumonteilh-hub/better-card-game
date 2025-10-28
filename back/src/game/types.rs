use serde::Serialize;


pub type InstanceId = usize;
pub type PlayerId = usize;

#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub enum Event {
    Death,
    EnterPlay,
    Attack,
    IsAttacked,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(tag = "type", content = "value")]
pub enum Location {
    Deck,
    Hand,
    Field(usize),
    Graveyard,
}
