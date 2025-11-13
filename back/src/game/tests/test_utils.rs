use crate::{
    Race,
    collection::{Archetype, Class},
    game::{
        Game,
        card::{CardInstance, CardTypeInstance, MonsterInstance, SpellInstance},
        effects::Effect,
        events::EventManager,
        player::PlayerInstance,
        types::Location,
    },
};
use std::collections::{HashMap, VecDeque};

pub fn create_test_game() -> Game {
    let player_id_a = 0;
    let player_id_b = 1;

    let mut players = HashMap::new();
    players.insert(
        player_id_a,
        PlayerInstance::new(player_id_a, 1, Archetype::Race(Race::COMMON)),
    );
    players.insert(
        player_id_b,
        PlayerInstance::new(player_id_b, 0, Archetype::Race(Race::COMMON)),
    );

    Game {
        game_id: uuid::Uuid::new_v4(),
        player_id_a,
        player_id_b,
        entities: HashMap::new(),
        effect_queue: VecDeque::new(),
        players,
        turn: 1,
        current_player: player_id_a,
        event_manager: EventManager::new(),
        winner_id: None,
        vs_ia: true,
    }
}

pub fn create_test_spell(game: &mut Game, owner: usize, mut effects: Vec<Effect>) -> usize {
    let spell_id = game.entities.len() + 2;

    effects = effects
        .into_iter()
        .map(|effect| update_effect_initiator(effect, spell_id))
        .collect();

    let spell = CardInstance {
        id: spell_id,
        name: "Test Spell".to_string(),
        description: "Test spell".to_string(),
        template_id: 9999,
        race: Race::COMMON,
        class: Class::COMMON,
        cost: 0,
        play_target: None,
        owner,
        location: Location::Hand,
        card_type: CardTypeInstance::Spell(SpellInstance { effect: effects }),
    };
    game.entities.insert(spell_id, spell);
    spell_id
}

pub fn update_effect_initiator(effect: Effect, initiator_id: usize) -> Effect {
    match effect {
        Effect::Heal { target, amount, .. } => Effect::Heal {
            initiator: initiator_id,
            target,
            amount,
        },
        Effect::DealDamage { target, amount, .. } => Effect::DealDamage {
            initiator: initiator_id,
            target,
            amount,
        },
        Effect::Boost {
            target, attack, hp, ..
        } => Effect::Boost {
            initiator: initiator_id,
            target,
            attack,
            hp,
        },
        Effect::Destroy { target, .. } => Effect::Destroy {
            initiator: initiator_id,
            target,
        },
        Effect::MakeDraw { player, amount, .. } => Effect::MakeDraw {
            initiator: initiator_id,
            player,
            amount,
        },
        Effect::IncreaseMaxMana { player, amount, .. } => Effect::IncreaseMaxMana {
            initiator: initiator_id,
            player,
            amount,
        },
        Effect::RefreshMana { player, amount, .. } => Effect::RefreshMana {
            initiator: initiator_id,
            player,
            amount,
        },
        Effect::Attack { initiator, target } => Effect::Attack {
            initiator: initiator_id,
            target,
        },
        Effect::AutoDraw { player, amount } => Effect::AutoDraw { player, amount },
        Effect::Win(player) => Effect::Win(player),
        Effect::Summon {
            initiator,
            side,
            target,
        } => Effect::Summon {
            initiator: initiator_id,
            side,
            target,
        },
    }
}

pub fn create_test_monster(
    game: &mut Game,
    owner: usize,
    position: usize,
    hp: usize,
    max_hp: usize,
) -> usize {
    let monster_id = game.entities.len() + 2;
    let monster = CardInstance {
        id: monster_id,
        name: "Test Monster".to_string(),
        description: "Test".to_string(),
        template_id: 9998,
        race: Race::COMMON,
        class: Class::COMMON,
        cost: 0,
        owner,
        play_target: None,
        location: Location::Field(position),
        card_type: CardTypeInstance::Monster(MonsterInstance {
            attack: 2,
            hp,
            max_hp,
            asleep: false,
            attack_count: 0,
            keywords: vec![],
            on_play: vec![],
            on_attack: vec![],
            on_death: vec![],
        }),
    };
    game.entities.insert(monster_id, monster);
    monster_id
}

pub fn create_test_monster_in_hand(
    game: &mut Game,
    owner: usize,
    attack: usize,
    hp: usize,
    keywords: Vec<crate::game::card::Keyword>,
    on_play: Vec<Effect>,
) -> usize {
    let monster_id = game.entities.len() + 2;
    let monster = CardInstance {
        id: monster_id,
        name: "Test Monster in Hand".to_string(),
        description: "Test".to_string(),
        template_id: 9997,
        race: Race::COMMON,
        class: Class::COMMON,
        cost: 0,
        play_target: None,
        owner,
        location: Location::Hand,
        card_type: CardTypeInstance::Monster(MonsterInstance {
            attack,
            hp,
            max_hp: hp,
            asleep: true,
            attack_count: 0,
            keywords,
            on_play,
            on_attack: vec![],
            on_death: vec![],
        }),
    };
    game.entities.insert(monster_id, monster);
    monster_id
}

pub fn create_test_monster_with_on_death(
    game: &mut Game,
    owner: usize,
    position: usize,
    hp: usize,
    max_hp: usize,
    mut on_death: Vec<Effect>,
) -> usize {
    let monster_id = game.entities.len() + 2;

    on_death = on_death
        .into_iter()
        .map(|effect| update_effect_initiator(effect, monster_id))
        .collect();

    let monster = CardInstance {
        id: monster_id,
        name: "Test Monster with On Death".to_string(),
        description: "Test".to_string(),
        template_id: 9996,
        race: Race::COMMON,
        class: Class::COMMON,
        cost: 0,
        play_target: None,
        owner,
        location: Location::Field(position),
        card_type: CardTypeInstance::Monster(MonsterInstance {
            attack: 2,
            hp,
            max_hp,
            asleep: false,
            attack_count: 0,
            keywords: vec![],
            on_play: vec![],
            on_attack: vec![],
            on_death,
        }),
    };
    game.entities.insert(monster_id, monster);
    monster_id
}

pub fn create_test_monster_with_attack(
    game: &mut Game,
    owner: usize,
    position: usize,
    attack: usize,
    hp: usize,
    max_hp: usize,
) -> usize {
    let monster_id = game.entities.len() + 2;
    let monster = CardInstance {
        id: monster_id,
        name: "Test Monster".to_string(),
        description: "Test".to_string(),
        template_id: 9998,
        race: Race::COMMON,
        play_target: None,
        class: Class::COMMON,
        cost: 0,
        owner,
        location: Location::Field(position),
        card_type: CardTypeInstance::Monster(MonsterInstance {
            attack,
            hp,
            max_hp,
            asleep: false,
            attack_count: 0,
            keywords: vec![],
            on_play: vec![],
            on_attack: vec![],
            on_death: vec![],
        }),
    };
    game.entities.insert(monster_id, monster);
    monster_id
}

pub fn create_test_monster_with_on_attack(
    game: &mut Game,
    owner: usize,
    position: usize,
    attack: usize,
    hp: usize,
    mut on_attack: Vec<Effect>,
) -> usize {
    let monster_id = game.entities.len() + 2;

    on_attack = on_attack
        .into_iter()
        .map(|effect| update_effect_initiator(effect, monster_id))
        .collect();

    let monster = CardInstance {
        id: monster_id,
        name: "Test Monster with On Attack".to_string(),
        play_target: None,
        description: "Test".to_string(),
        template_id: 9997,
        race: Race::COMMON,
        class: Class::COMMON,
        cost: 0,
        owner,
        location: Location::Field(position),
        card_type: CardTypeInstance::Monster(MonsterInstance {
            attack,
            hp,
            max_hp: hp,
            asleep: false,
            attack_count: 0,
            keywords: vec![],
            on_play: vec![],
            on_attack,
            on_death: vec![],
        }),
    };
    game.entities.insert(monster_id, monster);
    monster_id
}

pub fn add_card_to_deck(game: &mut Game, owner: usize) -> usize {
    let card_id = game.entities.len() + 2;
    let card = CardInstance {
        id: card_id,
        name: "Test Card in Deck".to_string(),
        description: "Test".to_string(),
        template_id: 9995,
        race: Race::COMMON,
        class: Class::COMMON,
        cost: 0,
        owner,
        play_target: None,
        location: Location::Deck,
        card_type: CardTypeInstance::Monster(MonsterInstance {
            attack: 1,
            hp: 1,
            max_hp: 1,
            asleep: true,
            attack_count: 0,
            keywords: vec![],
            on_play: vec![],
            on_attack: vec![],
            on_death: vec![],
        }),
    };
    game.entities.insert(card_id, card);
    card_id
}

pub fn add_card_to_hand(game: &mut Game, owner: usize) -> usize {
    let card_id = game.entities.len() + 2;
    let card = CardInstance {
        id: card_id,
        name: "Test Card in Hand".to_string(),
        description: "Test".to_string(),
        template_id: 9994,
        race: Race::COMMON,
        class: Class::COMMON,
        cost: 0,
        play_target: None,
        owner,
        location: Location::Hand,
        card_type: CardTypeInstance::Monster(MonsterInstance {
            attack: 1,
            hp: 1,
            max_hp: 1,
            asleep: true,
            attack_count: 0,
            keywords: vec![],
            on_play: vec![],
            on_attack: vec![],
            on_death: vec![],
        }),
    };
    game.entities.insert(card_id, card);
    card_id
}
