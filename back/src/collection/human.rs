use crate::collection::{
    Race, boost, monster, spell,
    types::{CardTemplate, PlayerTemplateTarget, TemplateEffect, TemplateTarget},
};
use once_cell::sync::Lazy;

pub fn get_collection() -> Vec<CardTemplate> {
    vec![
        BRAS_DROIT.clone(),
        CHEVALIER.clone(),
        FANFARE.clone(),
        ECUYER.clone(),
    ]
}

static FANFARE: Lazy<CardTemplate> = Lazy::new(|| {
    spell(
        1001,
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
        1002,
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

static ECUYER: Lazy<CardTemplate> = Lazy::new(|| {
    monster(
        1003,
        1,
        "Ecuyer",
        "Une jeune recrue",
        1,
        1,
        Race::HUMAN,
        super::Class::COMMON,
    )
    .build()
});

static CHEVALIER: Lazy<CardTemplate> = Lazy::new(|| {
    monster(
        1004,
        3,
        "Chevalier",
        "Invoque 1 ecuyer",
        3,
        3,
        Race::HUMAN,
        super::Class::COMMON,
    )
    .on_play(vec![TemplateEffect::Summon {
        side: PlayerTemplateTarget::Player,
        target: ECUYER.clone(),
    }])
    .build()
});

static COMMANDANT: Lazy<CardTemplate> = Lazy::new(|| {
    monster(
        1006,
        3,
        "Commandant",
        "tout les alliés ont +2/+2",
        3,
        3,
        Race::HUMAN,
        super::Class::COMMON,
    )
    .passiv_effect(vec![TemplateEffect::Boost {
        target: TemplateTarget::Allies,
        attack: 2,
        hp: 2,
    }])
    .build()
});
