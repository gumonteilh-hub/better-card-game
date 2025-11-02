use crate::{
    Faction,
    collection::types::{CardTemplate, CardTypeTemplate, TemplateId, convert_to_effect},
    game::effects::Effect,
};

use super::types::{InstanceId, Location, PlayerId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Keyword {
    Charge,
    Windfury,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CardInstance {
    pub id: InstanceId,
    pub name: String,
    pub description: String,
    pub template_id: TemplateId,
    pub faction: Faction,
    pub cost: usize,
    pub owner: PlayerId,
    pub location: Location,
    pub card_type: CardTypeInstance,
}

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "camelCase")]
pub enum CardTypeInstance {
    Monster(MonsterInstance),
    Spell(SpellInstance),
}

#[derive(Debug, Clone, Serialize)]
pub struct SpellInstance {
    pub effect: Vec<Effect>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MonsterInstance {
    pub attack: usize,
    pub hp: usize,
    pub max_hp: usize,
    pub asleep: bool,
    pub attack_count: usize,
    pub keywords: Vec<Keyword>,
    #[serde(default)]
    pub on_play: Vec<Effect>,
    #[serde(default)]
    pub on_attack: Vec<Effect>,
    #[serde(default)]
    pub on_death: Vec<Effect>,
}

impl CardInstance {
    pub fn new(entity_id: usize, player_id: usize, template: &CardTemplate) -> Self {
        let card_type = match &template.card_type {
            CardTypeTemplate::Monster(monster_template) => {
                CardTypeInstance::Monster(MonsterInstance {
                    attack: monster_template.attack,
                    hp: monster_template.hp,
                    max_hp: monster_template.hp,
                    asleep: true,
                    attack_count: 0,
                    keywords: monster_template.keywords.clone(),
                    on_play: monster_template
                        .on_play
                        .clone()
                        .iter()
                        .map(|e| convert_to_effect(e, entity_id))
                        .collect(),
                    on_attack: monster_template
                        .on_attack
                        .clone()
                        .iter()
                        .map(|e| convert_to_effect(e, entity_id))
                        .collect(),
                    on_death: monster_template
                        .on_death
                        .clone()
                        .iter()
                        .map(|e| convert_to_effect(e, entity_id))
                        .collect(),
                })
            }
            CardTypeTemplate::Spell(spell_template) => CardTypeInstance::Spell(SpellInstance {
                effect: spell_template
                    .effect
                    .clone()
                    .iter()
                    .map(|e| convert_to_effect(e, entity_id))
                    .collect(),
            }),
        };
        Self {
            id: entity_id,
            template_id: template.id,
            owner: player_id,
            location: Location::Deck,
            cost: template.cost,
            name: template.name.clone(),
            description: template.description.clone(),
            faction: template.faction,
            card_type,
        }
    }
}
