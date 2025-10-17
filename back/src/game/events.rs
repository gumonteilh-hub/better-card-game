use crate::error::Result;
use crate::game::Game;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use super::effects::Effect;
use super::logic::execute_effect;

#[derive(Debug, Clone, Serialize, PartialEq, Eq, Hash, Deserialize)]
pub enum EventType {
    MinionDies,
    MinionSummoned,
    TurnStarts,
    TurnEnds,
}

#[derive(Debug, Clone)]
pub struct Subscription {
    pub id: Uuid,
    pub source_card_instance_id: Uuid,
    pub effects_to_trigger: Vec<Effect>,
}

#[derive(Default, Debug, Clone)]
pub struct EventManager {
    subscriptions: HashMap<EventType, Vec<Subscription>>,
}

impl EventManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, event: EventType, sub: Subscription) {
        self.subscriptions.entry(event).or_default().push(sub);
    }

    pub fn notify(&self, event: &EventType, context: &mut Game) -> Result<()> {
        if let Some(subs) = self.subscriptions.get(event) {
            for sub in subs {
                for effect in &sub.effects_to_trigger {
                    execute_effect(effect, context)?;
                }
            }
        }
        Ok(())
    }

    pub fn unregister_all_from_source(&mut self, source_id: Uuid) {
        for subs in self.subscriptions.values_mut() {
            subs.retain(|sub| sub.source_card_instance_id != source_id);
        }
    }
}
