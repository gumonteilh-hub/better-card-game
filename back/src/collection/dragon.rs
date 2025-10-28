use crate::collection::{
    Faction, heal, monster,
    types::{CardTemplate, TemplateTarget},
};
use once_cell::sync::Lazy;

pub fn get_collection() -> Vec<CardTemplate> {
    vec![FEERIQUE.clone()]
}

static FEERIQUE: Lazy<CardTemplate> = Lazy::new(|| {
    monster(
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
