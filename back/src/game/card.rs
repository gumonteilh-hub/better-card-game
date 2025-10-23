use crate::collection::Faction;
use crate::template::TemplateEffect;

use super::effects::TriggeredEffect;
use super::types::{EntityId, Location, PlayerId, TemplateId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Keyword {
    Charge,
    Windfury
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CardTemplate {
    pub id: TemplateId,
    pub cost: usize,
    pub name: String,
    pub description: String,
    pub attack: usize,
    pub hp: usize,
    pub keywords: Vec<Keyword>,
    pub faction: Faction,
    #[serde(default)]
    pub on_play: Vec<TemplateEffect>,
    #[serde(default)]
    pub on_attack: Vec<TemplateEffect>,
    #[serde(default)]
    pub triggered_effects: Vec<TriggeredEffect>,
    #[serde(default)]
    pub on_death: Vec<TemplateEffect>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CardInstance {
    pub id: EntityId,
    pub template: CardTemplate,
    pub owner: PlayerId,
    pub location: Location,
    pub attack: usize,
    pub hp: usize,
    pub asleep: bool,
    pub attack_count: usize,
}
impl CardInstance {
    pub fn new(entity_id: usize, player_id: usize, card: &CardTemplate) -> Self {
        Self {
            id: entity_id,
            template: card.clone(),
            owner: player_id,
            location: Location::Deck,
            attack: card.attack,
            hp: card.hp,
            asleep: !card.keywords.contains(&Keyword::Charge),
            attack_count: 0,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct SpellInfo {
    // Ajoutez ici les propriétés spécifiques aux sorts
}
