use crate::{
    collection::{
        Class, Race, boost, monster, spell,
        types::{
            CardTemplate, PlayTargetTemplate, PlayerTemplateTarget, Side, TargetMatcherTemplate,
            TemplateEffect, TemplateTarget,
        },
    },
    game::card::Keyword,
};
use once_cell::sync::Lazy;

pub fn get_collection() -> Vec<CardTemplate> {
    vec![
        BRAS_DROIT.clone(),
        CHEVALIER.clone(),
        FANFARE.clone(),
        ECUYER.clone(),
        ARCHER.clone(),
        SACRIFIEUR.clone(),
        ACCELERATEUR.clone(),
        SUPER_ACCELERATEUR.clone(),
        RAFRAICHISSEUR.clone(),
        ALONE.clone(),
        SURROUNDED.clone(),
        BRISEUR_DE_LIMITE.clone(),
        GUERRIER_MOTIVE.clone(),
        BLOQUEUR.clone(),
        ASSASSIN_RAPIDE.clone(),
        MASOCHISTE.clone(),
        DESACTIVATEUR_DE_MANA.clone(),
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

static ARCHER: Lazy<CardTemplate> = Lazy::new(|| {
    monster(
        1005,
        2,
        "Archer",
        "On play: (optionel) Choisie un monstre adverse : le detruit",
        4,
        4,
        Race::HUMAN,
        Class::COMMON,
    )
    .on_play_with_target_choice(
        vec![TemplateEffect::Destroy {
            target: TemplateTarget::Choose,
        }],
        PlayTargetTemplate {
            strict: false,
            amount: 1,
            matcher: TargetMatcherTemplate::Side(Side::Enemy),
        },
    )
    .build()
});

static SACRIFIEUR: Lazy<CardTemplate> = Lazy::new(|| {
    monster(
        1006,
        2,
        "SACRIFIEUR",
        "On play: Choisie un monstre allier a sacrifier",
        4,
        4,
        Race::HUMAN,
        Class::COMMON,
    )
    .on_play_with_target_choice(
        vec![TemplateEffect::Destroy {
            target: TemplateTarget::Choose,
        }],
        PlayTargetTemplate {
            strict: true,
            amount: 1,
            matcher: TargetMatcherTemplate::Side(Side::Player),
        },
    )
    .build()
});

static RAFRAICHISSEUR: Lazy<CardTemplate> = Lazy::new(|| {
    monster(
        1007,
        2,
        "Rafraichisseur de mana",
        "On play: rafraichis 2 cristaux de mana",
        4,
        4,
        Race::HUMAN,
        Class::COMMON,
    )
    .on_play(vec![TemplateEffect::RefreshMana {
        player: PlayerTemplateTarget::Player,
        amount: 2,
    }])
    .build()
});

static ACCELERATEUR: Lazy<CardTemplate> = Lazy::new(|| {
    monster(
        1008,
        2,
        "accelerateur de mana",
        "On play: Gagne 2 cristaux de mana vide",
        4,
        4,
        Race::HUMAN,
        Class::COMMON,
    )
    .on_play(vec![TemplateEffect::IncreaseMaxMana {
        player: PlayerTemplateTarget::Player,
        amount: 2,
    }])
    .build()
});

static SUPER_ACCELERATEUR: Lazy<CardTemplate> = Lazy::new(|| {
    monster(
        1009,
        2,
        "SUPER accelerateur de mana",
        "On play: Gagne 2 cristaux de mana pleins",
        4,
        4,
        Race::HUMAN,
        Class::COMMON,
    )
    .on_play(vec![
        TemplateEffect::IncreaseMaxMana {
            player: PlayerTemplateTarget::Player,
            amount: 2,
        },
        TemplateEffect::RefreshMana {
            player: PlayerTemplateTarget::Player,
            amount: 2,
        },
    ])
    .build()
});

static ALONE: Lazy<CardTemplate> = Lazy::new(|| {
    monster(
        1010,
        2,
        "Solitaire",
        "seul: Gagne 2 cristaux de mana pleins",
        4,
        4,
        Race::HUMAN,
        Class::COMMON,
    )
    .on_alone(vec![boost(TemplateTarget::ItSelf, 3, 3)])
    .build()
});

static SURROUNDED: Lazy<CardTemplate> = Lazy::new(|| {
    monster(
        1011,
        3,
        "Entouré",
        "entouré: +3/+3",
        3,
        3,
        Race::HUMAN,
        Class::COMMON,
    )
    .on_surrounded(vec![boost(TemplateTarget::ItSelf, 3, 3)])
    .build()
});

static BRISEUR_DE_LIMITE: Lazy<CardTemplate> = Lazy::new(|| {
    monster(
        1012,
        3,
        "Briseur de limite",
        "Augmente la limite de Mana de 2",
        3,
        3,
        Race::HUMAN,
        Class::WARRIOR,
    )
    .on_play(vec![TemplateEffect::IncreaseAbsoluteMaxMana {
        player: PlayerTemplateTarget::Player,
        amount: 2,
    }])
    .build()
});

static GUERRIER_MOTIVE: Lazy<CardTemplate> = Lazy::new(|| {
    monster(
        1013,
        3,
        "Guerrier motivé",
        "Au début du tour gagne +2/+0, a la fin du tour gagne +0/+2",
        3,
        3,
        Race::HUMAN,
        Class::WARRIOR,
    )
    .on_turn_start(vec![TemplateEffect::Boost {
        target: TemplateTarget::ItSelf,
        attack: 2,
        hp: 0,
    }])
    .on_turn_end(vec![TemplateEffect::Boost {
        target: TemplateTarget::ItSelf,
        attack: 0,
        hp: 2,
    }])
    .build()
});

static MASOCHISTE: Lazy<CardTemplate> = Lazy::new(|| {
    monster(
        1014,
        3,
        "Masochiste",
        "Souffrance: gagne 2 d'attaque ",
        0,
        12,
        Race::HUMAN,
        Class::WARRIOR,
    )
    .on_damaged(vec![TemplateEffect::Boost {
        target: TemplateTarget::ItSelf,
        attack: 2,
        hp: 0,
    }])
    .build()
});

static BLOQUEUR: Lazy<CardTemplate> = Lazy::new(|| {
    monster(
        1015,
        5,
        "Bloqueur",
        "En defense: pioche une carte",
        4,
        9,
        Race::HUMAN,
        Class::WARRIOR,
    )
    .on_defend(vec![TemplateEffect::MakeDraw {
        player: PlayerTemplateTarget::Player,
        amount: 1,
    }])
    .build()
});

static ASSASSIN_RAPIDE: Lazy<CardTemplate> = Lazy::new(|| {
    monster(
        1016,
        3,
        "Assassin rapide",
        "Charge, En tuant: invoque 1 bloqueur",
        7,
        1,
        Race::HUMAN,
        Class::WARRIOR,
    )
    .keywords(vec![Keyword::Charge])
    .on_kill(vec![TemplateEffect::Summon {
        side: PlayerTemplateTarget::Player,
        target: BLOQUEUR.clone(),
    }])
    .build()
});

static DESACTIVATEUR_DE_MANA: Lazy<CardTemplate> = Lazy::new(|| {
    spell(
        1017,
        2,
        "court-circuit",
        "Reduit de 3 la mana de l'adversaire pour son prochain tour",
        Race::HUMAN,
        Class::MAGE,
    )
    .effect(vec![TemplateEffect::DecreaseCurrentMana {
        player: PlayerTemplateTarget::EnnemyPlayer,
        amount: 3,
    }])
    .build()
});
