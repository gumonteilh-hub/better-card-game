use crate::collection::{
    Faction, boost, draw, monster, spell,
    types::{CardTemplate, PlayerTemplateTarget},
};
use once_cell::sync::Lazy;

pub fn get_collection() -> Vec<CardTemplate> {
    vec![BRAS_DROIT.clone(), CHEVALIER.clone(), FANFARE.clone()]
}

static FANFARE: Lazy<CardTemplate> = Lazy::new(|| {
    spell(45641211, 2, "Fanfare", "+2/+2 a tout vos monstres", Faction::HUMAN)
        .effect(vec![boost(
            crate::collection::types::TemplateTarget::Allies,
            2,
            2,
        )])
        .build()
});

static BRAS_DROIT: Lazy<CardTemplate> = Lazy::new(|| {
    monster(
        1,
        2,
        "Bras droit",
        "Le bras droit du roi",
        5,
        5,
        Faction::HUMAN,
    )
    .build()
});

static CHEVALIER: Lazy<CardTemplate> = Lazy::new(|| {
    monster(
        2,
        3,
        "Chevalier",
        "Apparition: vous piochez une carte",
        3,
        4,
        Faction::HUMAN,
    )
    .on_play(vec![draw(PlayerTemplateTarget::Player, 1)])
    .build()
});
