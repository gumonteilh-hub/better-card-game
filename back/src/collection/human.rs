use crate::collection::{
    Race, boost, draw, monster, spell,
    types::{CardTemplate, PlayerTemplateTarget},
};
use once_cell::sync::Lazy;

pub fn get_collection() -> Vec<CardTemplate> {
    vec![BRAS_DROIT.clone(), CHEVALIER.clone(), FANFARE.clone()]
}

static FANFARE: Lazy<CardTemplate> = Lazy::new(|| {
    spell(
        45641211,
        2,
        "Fanfare",
        "+2/+2 a tout vos monstres",
        Race::HUMAN,
        super::Class::COMMON,
    )
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
        Race::HUMAN,
        super::Class::COMMON,
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
        Race::HUMAN,
        super::Class::COMMON,
    )
    .on_play(vec![draw(PlayerTemplateTarget::Player, 1)])
    .build()
});
