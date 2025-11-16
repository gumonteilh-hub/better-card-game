# Game Design Tracking - Effets et Mécaniques Implémentés

Ce document répertorie tous les effets, triggers, et éléments de game design actuellement implémentés dans la codebase.

## 1. Effets de Cartes (TemplateEffect)

Définis dans `back/src/collection/types.rs:141-170`

- **Boost**: Augmente l'attaque et les HP d'une ou plusieurs cibles
- **MakeDraw**: Fait piocher des cartes à un ou plusieurs joueurs
- **Heal**: Restaure des HP à une ou plusieurs cibles
- **Destroy**: Détruit une ou plusieurs cibles
- **DealDamage**: Inflige des dégâts à une ou plusieurs cibles
- **Attack**: Force une cible à attaquer une autre cible
- **Summon**: Invoque une carte sur le terrain
- **IncreaseMaxMana**: Augmente le mana maximum d'un joueur
- **RefreshMana**: Rafraîchit le mana d'un joueur
- **Win**: Déclare un joueur gagnant (effet système)

## 3. Triggers de Cartes

Définis dans `back/src/collection/types.rs:104-114`

- **on_play**: Quand le monstre est invoquée par le joueur
- **on_attack**: Quand le monstre effectue une attaque
- **on_death**: Quand le monstre meurt
- **effect**: Quand le sort est joué

## 5. Systèmes de Ciblage

### 5.1 TemplateTarget (Cibles d'Effets)

Définis dans `back/src/collection/types.rs:117-130`

- **EnnemyPlayer**: Le joueur adverse
- **Player**: Le joueur propriétaire de la carte
- **BothPlayers**: Les deux joueurs
- **ItSelf**: La carte elle-même
- **Allies**: Tous les monstres alliés
- **Ennemies**: Tous les monstres ennemis
- **AllMonsters**: Tous les monstres (alliés et ennemis)
- **All**: Tous les personnages (joueurs + monstres)
- **Choose**: Cible choisie par le joueur au moment du jeu (s'applique a on_play pour un monstre ou effect pour un sort)
- **Matching(TargetMatcherTemplate)**: Cibles correspondant à des critères
- **And(Box<TemplateTarget>, Box<TemplateTarget>)**: Intersection de deux ensembles de cibles
- **Or(Box<TemplateTarget>, Box<TemplateTarget>)**: Union de deux ensembles de cibles

### 5.2 PlayTarget (Sélection Interactive)

Défini dans `back/src/collection/types.rs:33-39`

Permet de demander au joueur de sélectionner des cibles au moment de jouer la carte.

**Paramètres**:
- **strict**: `bool` - Si `true`, la sélection est obligatoire ; si `false`, optionnelle
- **amount**: `usize` - Nombre de cibles à sélectionner
- **matcher**: `TargetMatcherTemplate` - Critères de sélection des cibles

### 5.3 TargetMatcherTemplate (Critères de Filtrage)

Défini dans `back/src/collection/types.rs:67-80`

- **Race(Race)**: Filtre par race (COMMON, HUMAN, DRAGON, DEMON)
- **Class(Class)**: Filtre par classe (COMMON, etc.)
- **Side(Side)**: Filtre par camp (Player ou Enemy)

## 6. Keywords

Définis dans `back/src/game/card.rs:14-17`

### 6.1 Charge
- **Description**: Le monstre peut attaquer immédiatement le tour où il est invoqué (ignore `asleep`)

### 6.2 Windfury
- **Description**: Le monstre peut attaquer deux fois par tour
## 8. Historique et Évolution

