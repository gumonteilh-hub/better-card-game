use crate::{
    UserDeck,
    collection::{
        Archetype, Class, Race, boost, deal_damage, draw, heal, monster, spell,
        types::{CardTemplate, PlayerTemplateTarget, TemplateTarget},
    },
    game::card::Keyword,
};
use once_cell::sync::Lazy;

pub fn get_collection() -> Vec<CardTemplate> {
    vec![
        RECRUE.clone(),
        GUERRIER.clone(),
        ECLAIREUR.clone(),
        CHEVALIER.clone(),
        CAVALIER.clone(),
        CHAMPION.clone(),
        BERSERKER.clone(),
        GARDIEN.clone(),
        GEANT.clone(),
        DRAGON.clone(),
        APPRENTI_MAGE.clone(),
        CLERC.clone(),
        PYROMANCIEN.clone(),
        ARCANISTE.clone(),
        ANGE_GARDIEN.clone(),
        KAMIKAZE.clone(),
        NECROMANCIEN.clone(),
        SORCIERE.clone(),
        VAMPIRE.clone(),
        PALADIN.clone(),
        BOOSTER_TEST.clone(),
        HEALER_TEST.clone(),
    ]
}

pub fn get_ia_deck() -> UserDeck {
    UserDeck {
        archetype: Archetype::Race(Race::DEMON),
        cards: vec![
            RECRUE.clone().id,
            GUERRIER.clone().id,
            ECLAIREUR.clone().id,
            CHEVALIER.clone().id,
            CAVALIER.clone().id,
            CHAMPION.clone().id,
            BERSERKER.clone().id,
            GARDIEN.clone().id,
            GEANT.clone().id,
            DRAGON.clone().id,
            RECRUE.clone().id,
            GUERRIER.clone().id,
            ECLAIREUR.clone().id,
            CHEVALIER.clone().id,
            CAVALIER.clone().id,
            CHAMPION.clone().id,
            BERSERKER.clone().id,
            GARDIEN.clone().id,
            GEANT.clone().id,
            DRAGON.clone().id,
            APPRENTI_MAGE.clone().id,
            CLERC.clone().id,
            PYROMANCIEN.clone().id,
            ARCANISTE.clone().id,
            ANGE_GARDIEN.clone().id,
            KAMIKAZE.clone().id,
            NECROMANCIEN.clone().id,
            SORCIERE.clone().id,
            VAMPIRE.clone().id,
            PALADIN.clone().id,
        ],
    }
}

static RECRUE: Lazy<CardTemplate> = Lazy::new(|| {
    monster(
        1,
        1,
        "Recrue",
        "Une simple recrue",
        1,
        1,
        Race::COMMON,
        Class::COMMON,
    )
    .build()
});

static BOOSTER_TEST: Lazy<CardTemplate> = Lazy::new(|| {
    monster(
        2,
        1,
        "Booster Test",
        "On play : boost the cards on the field +1/+1",
        1,
        1,
        Race::COMMON,
        Class::COMMON,
    )
    .on_play(vec![boost(TemplateTarget::Allies, 1, 1)])
    .build()
});

static HEALER_TEST: Lazy<CardTemplate> = Lazy::new(|| {
    spell(
        3,
        1,
        "Healer Test",
        "Heal all your cards 5 hp",
        Race::COMMON,
        Class::COMMON,
    )
    .effect(vec![heal(TemplateTarget::Allies, 5)])
    .build()
});

static GUERRIER: Lazy<CardTemplate> = Lazy::new(|| {
    monster(
        4,
        2,
        "Guerrier",
        "Un guerrier courageux",
        2,
        2,
        Race::COMMON,
        Class::COMMON,
    )
    .build()
});

static ECLAIREUR: Lazy<CardTemplate> = Lazy::new(|| {
    monster(
        5,
        1,
        "Éclaireur",
        "Un éclaireur agile",
        1,
        1,
        Race::COMMON,
        Class::COMMON,
    )
    .keywords(vec![Keyword::Charge])
    .build()
});

static CHEVALIER: Lazy<CardTemplate> = Lazy::new(|| {
    monster(
        6,
        3,
        "Chevalier",
        "Un noble chevalier",
        2,
        4,
        Race::COMMON,
        Class::COMMON,
    )
    .keywords(vec![])
    .build()
});

static CAVALIER: Lazy<CardTemplate> = Lazy::new(|| {
    monster(
        7,
        3,
        "Cavalier",
        "Un cavalier rapide",
        3,
        2,
        Race::COMMON,
        Class::COMMON,
    )
    .keywords(vec![Keyword::Charge])
    .build()
});

static CHAMPION: Lazy<CardTemplate> = Lazy::new(|| {
    monster(
        8,
        4,
        "Champion",
        "Un champion puissant",
        4,
        4,
        Race::COMMON,
        Class::COMMON,
    )
    .build()
});

static BERSERKER: Lazy<CardTemplate> = Lazy::new(|| {
    monster(
        9,
        4,
        "Berserker",
        "Un berserker féroce",
        3,
        3,
        Race::COMMON,
        Class::COMMON,
    )
    .keywords(vec![Keyword::Windfury])
    .build()
});

static GARDIEN: Lazy<CardTemplate> = Lazy::new(|| {
    monster(
        10,
        5,
        "Gardien",
        "Un gardien robuste",
        3,
        7,
        Race::COMMON,
        Class::COMMON,
    )
    .keywords(vec![])
    .build()
});

static GEANT: Lazy<CardTemplate> = Lazy::new(|| {
    monster(
        11,
        6,
        "Géant",
        "Un géant imposant",
        6,
        6,
        Race::COMMON,
        Class::COMMON,
    )
    .build()
});

static DRAGON: Lazy<CardTemplate> = Lazy::new(|| {
    monster(
        12,
        7,
        "Dragon",
        "Un dragon majestueux",
        7,
        7,
        Race::COMMON,
        Class::COMMON,
    )
    .keywords(vec![Keyword::Charge])
    .build()
});

static APPRENTI_MAGE: Lazy<CardTemplate> = Lazy::new(|| {
    monster(
        13,
        2,
        "Apprenti Mage",
        "Apparition: piochez une carte",
        1,
        2,
        Race::COMMON,
        Class::COMMON,
    )
    .on_play(vec![draw(PlayerTemplateTarget::Player, 1)])
    .build()
});

static CLERC: Lazy<CardTemplate> = Lazy::new(|| {
    monster(
        14,
        3,
        "Clerc",
        "Apparition: rend 3 PV à vos alliés",
        2,
        3,
        Race::COMMON,
        Class::COMMON,
    )
    .on_play(vec![heal(TemplateTarget::Allies, 3)])
    .build()
});

static PYROMANCIEN: Lazy<CardTemplate> = Lazy::new(|| {
    monster(
        15,
        4,
        "Pyromancien",
        "Apparition: inflige 2 dégâts à tous les ennemis",
        3,
        2,
        Race::COMMON,
        Class::COMMON,
    )
    .on_play(vec![deal_damage(TemplateTarget::Ennemies, 2)])
    .build()
});

static ARCANISTE: Lazy<CardTemplate> = Lazy::new(|| {
    monster(
        16,
        5,
        "Arcaniste",
        "Apparition: les deux joueurs piochent 2 cartes",
        4,
        4,
        Race::COMMON,
        Class::COMMON,
    )
    .on_play(vec![draw(PlayerTemplateTarget::BothPlayers, 2)])
    .build()
});

static ANGE_GARDIEN: Lazy<CardTemplate> = Lazy::new(|| {
    monster(
        17,
        4,
        "Ange Gardien",
        "Apparition: rend 5 PV au héros allié",
        3,
        4,
        Race::COMMON,
        Class::COMMON,
    )
    .keywords(vec![])
    .on_play(vec![heal(TemplateTarget::Player, 5)])
    .build()
});

static KAMIKAZE: Lazy<CardTemplate> = Lazy::new(|| {
    monster(
        18,
        1,
        "Kamikaze",
        "Mort: inflige 3 dégâts à tous les personnages",
        1,
        1,
        Race::COMMON,
        Class::COMMON,
    )
    .on_death(vec![deal_damage(TemplateTarget::All, 3)])
    .build()
});

static NECROMANCIEN: Lazy<CardTemplate> = Lazy::new(|| {
    monster(
        19,
        5,
        "Nécromancien",
        "Mort: inflige 4 dégâts au héros ennemi",
        4,
        4,
        Race::COMMON,
        Class::COMMON,
    )
    .on_death(vec![deal_damage(TemplateTarget::EnnemyPlayer, 4)])
    .build()
});

static SORCIERE: Lazy<CardTemplate> = Lazy::new(|| {
    monster(
        20,
        3,
        "Sorcière",
        "Attaque: inflige 1 dégât au héros ennemi",
        2,
        3,
        Race::COMMON,
        Class::COMMON,
    )
    .on_attack(vec![deal_damage(TemplateTarget::EnnemyPlayer, 1)])
    .build()
});

static VAMPIRE: Lazy<CardTemplate> = Lazy::new(|| {
    monster(
        21,
        4,
        "Vampire",
        "Attaque: rend 2 PV au héros allié",
        3,
        3,
        Race::COMMON,
        Class::COMMON,
    )
    .on_attack(vec![heal(TemplateTarget::Player, 2)])
    .build()
});

static PALADIN: Lazy<CardTemplate> = Lazy::new(|| {
    monster(
        22,
        6,
        "Paladin",
        "Apparition: rend 4 PV à tous les alliés. Provocation",
        4,
        6,
        Race::COMMON,
        Class::COMMON,
    )
    .keywords(vec![])
    .on_play(vec![heal(TemplateTarget::Allies, 4)])
    .build()
});
