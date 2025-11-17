# Classe ROGUE - Identité de Game Design

## 🎯 Identité Thématique

Les Rogues représentent **le vol, la vitesse et l'opportunisme**. Chaque race vole quelque chose de différent qui alimente sa propre stratégie.

---

## 💡 Concept Central : LE VOL

Les Rogues **volent des ressources** à l'adversaire, mais ce qu'ils volent et comment ils l'utilisent dépend de leur race.

---

## 🎭 Les 3 Identités Race × Rogue

### **👥 HUMAN ROGUE : La Guilde de Voleurs**

**Vol :** Cartes de la main/deck adverse

**Synergies Race :**
- Volent des cartes → Main pleine → Jouent plus de créatures
- Guilde nombreuse (identité Human : swarm, nombre)
- Plus ils sont nombreux, plus ils volent efficacement

**Plan de jeu :**
- Développer une guilde de petits voleurs
- Chaque voleur pique des cartes adverses
- Accumuler des ressources pour submerger l'adversaire
- Synergie avec Formation, Ralliement, Nombre Critique

---

### **🐉 DRAGON ROGUE : Le Dragon Cupide**

**Vol :** Mana, Stats, Keywords

**Synergies Race :**
- Volent du mana → Accélèrent l'apparition d'autres dragons
- Volent des stats/keywords → Deviennent des unités puissantes et croissantes
- Solitaire : Plus efficace seul (identité Dragon)

**Plan de jeu :**
- Dragon chasseur qui vole des ressources en attaquant/tuant
- Le mana volé finance d'autres dragons coûteux
- Accumulation progressive de puissance (stats permanentes)
- Synergie avec Solitaire, Thésaurisation, Domination

---

### **😈 DEMON ROGUE : Le Voleur d'Âmes**

**Vol :** Vie (HP), Monstres ennemis

**Synergies Race :**
- Volent de la vie → Ressource pour Pactes (Prix du Sang)
- Volent des monstres → Ressources à sacrifier pour Rituels
- Fournit ce dont les autres démons ont besoin (HP, créatures sacrifiables)

**Plan de jeu :**
- Drainer la vie adverse pour alimenter les Pactes
- Posséder/voler des créatures ennemies
- Sacrifier les créatures volées pour des Rituels
- Synergie avec Pacte, Rituel, Prix du Sang, Sacrifice

---

## 📊 Tableau Récapitulatif

| | **Human Rogue** | **Dragon Rogue** | **Demon Rogue** |
|---|---|---|---|
| **Vole** | Cartes | Mana/Stats/Keywords | Vie/Monstres |
| **Utilise pour** | Jouer plus | Accélérer/Grossir | Sacrifier/Pactes |
| **Style** | Guilde (nombreux) | Chasseur (solitaire) | Parasite (drain) |
| **Synergie Race** | Swarm Human | Solitaire Dragon | Sacrifice Demon |

---

## 🔄 Intégration avec les Triggers Existants

Les mécaniques de vol s'intègrent naturellement avec les triggers existants :

### Human Rogue
- `on_play` : Vol de cartes à l'invocation
- `on_surrounded` : Vol amplifié en formation

### Dragon Rogue
- `on_attack` : Vol de mana/buffs pendant l'attaque
- `on_kill` : Vol de stats/keywords de la victime (permanent)
- `on_alone` : Vol doublé/triplé quand solitaire

### Demon Rogue
- `on_attack` : Drain de vie (Vol HP + Heal)
- `on_kill` : Possession/vol de la créature tuée
- `on_death` : Sacrifice des créatures volées

---

## ✅ Cohérence de Classe

**Ce qui unit tous les Rogues :**
- Vol comme mécanique centrale
- Opportunisme (profiter des faiblesses)
- Ressources non-conventionnelles (volées vs générées)

**Ce qui les différencie :**
- **Human** : Vol passif/collectif → Main pleine → Swarm
- **Dragon** : Vol actif/agressif → Accélération → Unité puissante
- **Demon** : Vol parasitaire → Conversion → Sacrifices

---

## 🎯 Exemples de Cartes (Grandes Lignes)

### Human Rogue
```
🗡️ VOLEUR DE LA GUILDE (coût faible)
"Apparition : Vol 1 carte adverse"
"Nombre Critique : Vole plus"
```

### Dragon Rogue
```
🐉 DRAGON PILLARD (coût moyen)
"Attaque : Vol mana"
"Kill : Vol stats/keywords (permanent)"
"Solitaire : Effet doublé"
```

### Demon Rogue
```
😈 VOLEUR D'ÂMES (coût moyen)
"Attaque : Drain vie"
"Kill : Possède la victime"
"Peut sacrifier les créatures possédées"
```

---

## 🎮 Design Philosophy

Cette approche crée une identité de classe **cohérente** (tous volent) mais **flexible** (chacun vole différemment) qui :

✅ S'intègre naturellement dans le plan de jeu de chaque race
✅ Utilise les triggers et mécaniques existants
✅ Crée des synergies cross-race (Rogues ensemble) ET intra-race (Rogue + sa race)
✅ Reste simple à comprendre : "Les Rogues volent X"
✅ Laisse de la place pour l'implémentation créative des détails

---

## 🚀 Implémentation Suggérée

**Phase 1** : Vol basique
- Human Rogue : Vol 1 carte via `on_play`
- Dragon Rogue : Vol mana via `on_attack`
- Demon Rogue : Drain vie via `on_attack`

**Phase 2** : Vol avancé
- Human : Nombre Critique affecte le vol
- Dragon : `on_kill` vole stats (permanent)
- Demon : `on_kill` possède la créature

**Phase 3** : Synergies complètes
- Interactions complexes entre les vols
- Cartes légendaires qui amplifient le vol
- Combos inter-classes

