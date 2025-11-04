use crate::collection::{
    Race, deal_damage, monster,
    types::{CardTemplate, TemplateTarget},
};
use once_cell::sync::Lazy;

pub fn get_collection() -> Vec<CardTemplate> {
    vec![INDICIBLE.clone()]
}

static INDICIBLE: Lazy<CardTemplate> = Lazy::new(|| {
    monster(
        3,
        1,
        "Indicible",
        "Mort: inflige 3 degats a tout les adversaires",
        1,
        1,
        Race::DEMON,
        super::Class::COMMON,
    )
    .on_death(vec![deal_damage(TemplateTarget::Ennemies, 3)])
    .build()
});
