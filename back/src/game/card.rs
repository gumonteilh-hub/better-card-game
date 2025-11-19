use crate::{
    Race,
    collection::{
        Class,
        types::{CardTemplate, CardTypeTemplate, PlayTarget, TemplateId},
    },
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
    pub race: Race,
    pub class: Class,
    pub cost: usize,
    pub owner: PlayerId,
    pub location: Location,
    pub card_type: CardTypeInstance,
    pub play_target: Option<PlayTarget>,
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
    pub on_play: Vec<Effect>,
    pub on_attack: Vec<Effect>,
    pub on_defense: Vec<Effect>,
    pub on_kill: Vec<Effect>,
    pub on_turn_end: Vec<Effect>,
    pub on_turn_start: Vec<Effect>,
    pub on_damaged: Vec<Effect>,
    pub on_death: Vec<Effect>,
    pub on_surrounded: Vec<Effect>,
    pub on_alone: Vec<Effect>,
}

impl CardInstance {
    pub fn new(
        entity_id: usize,
        player_id: PlayerId,
        template: &CardTemplate,
        oponent_id: PlayerId,
    ) -> Self {
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
                        .iter()
                        .map(|e| e.clone().convert(entity_id))
                        .collect(),
                    on_attack: monster_template
                        .on_attack
                        .iter()
                        .map(|e| e.clone().convert(entity_id))
                        .collect(),
                    on_defense: monster_template
                        .on_defend
                        .iter()
                        .map(|e| e.clone().convert(entity_id))
                        .collect(),
                    on_death: monster_template
                        .on_death
                        .iter()
                        .map(|e| e.clone().convert(entity_id))
                        .collect(),
                    on_surrounded: monster_template
                        .on_surrounded
                        .iter()
                        .map(|e| e.clone().convert(entity_id))
                        .collect(),
                    on_alone: monster_template
                        .on_alone
                        .iter()
                        .map(|e| e.clone().convert(entity_id))
                        .collect(),
                    on_damaged: monster_template
                        .on_damaged
                        .iter()
                        .map(|e| e.clone().convert(entity_id))
                        .collect(),
                    on_kill: monster_template
                        .on_kill
                        .iter()
                        .map(|e| e.clone().convert(entity_id))
                        .collect(),

                    on_turn_end: monster_template
                        .on_turn_end
                        .iter()
                        .map(|e| e.clone().convert(entity_id))
                        .collect(),

                    on_turn_start: monster_template
                        .on_turn_start
                        .iter()
                        .map(|e| e.clone().convert(entity_id))
                        .collect(),
                })
            }
            CardTypeTemplate::Spell(spell_template) => CardTypeInstance::Spell(SpellInstance {
                effect: spell_template
                    .effect
                    .iter()
                    .map(|e| e.clone().convert(entity_id))
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
            race: template.race,
            class: template.class,
            play_target: template
                .play_target
                .map(|t| t.convert(player_id, oponent_id)),
            card_type,
        }
    }
}
