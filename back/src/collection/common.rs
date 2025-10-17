use crate::{
    collection::{card, deal_damage, draw, heal, Faction},
    game::card::{CardTemplate, Keyword},
    template::{PlayerTemplateTarget, TemplateTarget}, UserDeck,
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
    ]
}

pub fn get_ia_deck() -> UserDeck {
    UserDeck {
        faction: Faction::DEMON,
        cards: vec![
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
        ],
    }
}

static RECRUE: Lazy<CardTemplate> =
    Lazy::new(|| card(100, 1, "Recrue", "Une simple recrue", 1, 1, Faction::COMMON).build());

static GUERRIER: Lazy<CardTemplate> = Lazy::new(|| {
    card(
        101,
        2,
        "Guerrier",
        "Un guerrier courageux",
        2,
        2,
        Faction::COMMON,
    )
    .build()
});

static ECLAIREUR: Lazy<CardTemplate> = Lazy::new(|| {
    card(
        102,
        1,
        "Éclaireur",
        "Un éclaireur agile",
        1,
        1,
        Faction::COMMON,
    )
    .keywords(vec![Keyword::Charge])
    .build()
});

static CHEVALIER: Lazy<CardTemplate> = Lazy::new(|| {
    card(
        103,
        3,
        "Chevalier",
        "Un noble chevalier",
        2,
        4,
        Faction::COMMON,
    )
    .keywords(vec![Keyword::Taunt])
    .build()
});

static CAVALIER: Lazy<CardTemplate> = Lazy::new(|| {
    card(
        104,
        3,
        "Cavalier",
        "Un cavalier rapide",
        3,
        2,
        Faction::COMMON,
    )
    .keywords(vec![Keyword::Charge])
    .build()
});

static CHAMPION: Lazy<CardTemplate> = Lazy::new(|| {
    card(
        105,
        4,
        "Champion",
        "Un champion puissant",
        4,
        4,
        Faction::COMMON,
    )
    .build()
});

static BERSERKER: Lazy<CardTemplate> = Lazy::new(|| {
    card(
        106,
        4,
        "Berserker",
        "Un berserker féroce",
        3,
        3,
        Faction::COMMON,
    )
    .keywords(vec![Keyword::Windfury])
    .build()
});

static GARDIEN: Lazy<CardTemplate> = Lazy::new(|| {
    card(
        107,
        5,
        "Gardien",
        "Un gardien robuste",
        3,
        7,
        Faction::COMMON,
    )
    .keywords(vec![Keyword::Taunt])
    .build()
});

static GEANT: Lazy<CardTemplate> =
    Lazy::new(|| card(108, 6, "Géant", "Un géant imposant", 6, 6, Faction::COMMON).build());

static DRAGON: Lazy<CardTemplate> = Lazy::new(|| {
    card(
        109,
        7,
        "Dragon",
        "Un dragon majestueux",
        7,
        7,
        Faction::COMMON,
    )
    .keywords(vec![Keyword::Charge])
    .build()
});

static APPRENTI_MAGE: Lazy<CardTemplate> = Lazy::new(|| {
    card(
        110,
        2,
        "Apprenti Mage",
        "Apparition: piochez une carte",
        1,
        2,
        Faction::COMMON,
    )
    .on_play(vec![draw(PlayerTemplateTarget::Player, 1)])
    .build()
});

static CLERC: Lazy<CardTemplate> = Lazy::new(|| {
    card(
        111,
        3,
        "Clerc",
        "Apparition: rend 3 PV à vos alliés",
        2,
        3,
        Faction::COMMON,
    )
    .on_play(vec![heal(TemplateTarget::Allies, 3)])
    .build()
});

static PYROMANCIEN: Lazy<CardTemplate> = Lazy::new(|| {
    card(
        112,
        4,
        "Pyromancien",
        "Apparition: inflige 2 dégâts à tous les ennemis",
        3,
        2,
        Faction::COMMON,
    )
    .on_play(vec![deal_damage(TemplateTarget::Ennemies, 2)])
    .build()
});

static ARCANISTE: Lazy<CardTemplate> = Lazy::new(|| {
    card(
        113,
        5,
        "Arcaniste",
        "Apparition: les deux joueurs piochent 2 cartes",
        4,
        4,
        Faction::COMMON,
    )
    .on_play(vec![draw(PlayerTemplateTarget::BothPlayers, 2)])
    .build()
});

static ANGE_GARDIEN: Lazy<CardTemplate> = Lazy::new(|| {
    card(
        114,
        4,
        "Ange Gardien",
        "Apparition: rend 5 PV au héros allié",
        3,
        4,
        Faction::COMMON,
    )
    .keywords(vec![Keyword::Taunt])
    .on_play(vec![heal(TemplateTarget::Player, 5)])
    .build()
});

static KAMIKAZE: Lazy<CardTemplate> = Lazy::new(|| {
    card(
        115,
        1,
        "Kamikaze",
        "Mort: inflige 3 dégâts à tous les personnages",
        1,
        1,
        Faction::COMMON,
    )
    .on_death(vec![deal_damage(TemplateTarget::All, 3)])
    .build()
});

static NECROMANCIEN: Lazy<CardTemplate> = Lazy::new(|| {
    card(
        116,
        5,
        "Nécromancien",
        "Mort: inflige 4 dégâts au héros ennemi",
        4,
        4,
        Faction::COMMON,
    )
    .on_death(vec![deal_damage(TemplateTarget::EnnemyPlayer, 4)])
    .build()
});

static SORCIERE: Lazy<CardTemplate> = Lazy::new(|| {
    card(
        117,
        3,
        "Sorcière",
        "Attaque: inflige 1 dégât au héros ennemi",
        2,
        3,
        Faction::COMMON,
    )
    .on_attack(vec![deal_damage(TemplateTarget::EnnemyPlayer, 1)])
    .build()
});

static VAMPIRE: Lazy<CardTemplate> = Lazy::new(|| {
    card(
        118,
        4,
        "Vampire",
        "Attaque: rend 2 PV au héros allié",
        3,
        3,
        Faction::COMMON,
    )
    .on_attack(vec![heal(TemplateTarget::Player, 2)])
    .build()
});

static PALADIN: Lazy<CardTemplate> = Lazy::new(|| {
    card(
        119,
        6,
        "Paladin",
        "Apparition: rend 4 PV à tous les alliés. Provocation",
        4,
        6,
        Faction::COMMON,
    )
    .keywords(vec![Keyword::Taunt])
    .on_play(vec![heal(TemplateTarget::Allies, 4)])
    .build()
});
