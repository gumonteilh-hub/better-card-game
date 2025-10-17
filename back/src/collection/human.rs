use crate::{
    collection::{card, draw, Faction},
    game::card::CardTemplate,
    template::PlayerTemplateTarget,
};
use once_cell::sync::Lazy;

pub fn get_collection() -> Vec<CardTemplate> {
    vec![BRAS_DROIT.clone(), CHEVALIER.clone()]
}

static BRAS_DROIT: Lazy<CardTemplate> =
    Lazy::new(|| card(1, 2, "Bras droit", "Le bras droit du roi", 5, 5, Faction::HUMAN).build());

static CHEVALIER: Lazy<CardTemplate> = Lazy::new(|| {
    card(
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
