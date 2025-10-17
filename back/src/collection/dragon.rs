use crate::{
    collection::{card, heal, Faction},
    game::card::CardTemplate,
    template::TemplateTarget,
};
use once_cell::sync::Lazy;

pub fn get_collection() -> Vec<CardTemplate> {
    vec![FEERIQUE.clone()]
}

static FEERIQUE: Lazy<CardTemplate> = Lazy::new(|| {
    card(
        2001,
        5,
        "Féérique",
        "Attaque: soigne tout les alliés de 5 hp",
        7,
        7,
        Faction::DRAGON,
    )
    .on_attack(vec![heal(TemplateTarget::Allies, 5)])
    .build()
});
