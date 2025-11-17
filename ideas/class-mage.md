# Classe MAGE - Identité de Game Design

## 🎯 Identité Thématique

Les Mages représentent **la magie, le contrôle et l'accumulation de pouvoir arcanique**. Ils génèrent et dépensent de l'Arcane pour amplifier leurs sorts.

---

## 💡 Concept Central : L'ARCANE

Les Mages **accumulent de l'Arcane** (ressource magique) qu'ils peuvent **dépenser pour booster leurs sorts**. La façon dont ils génèrent l'Arcane dépend de leur race.

---

## 🎭 Les 3 Identités Race × Mage

### **👥 HUMAN MAGE : La Confrérie de Mages**

**Génération d'Arcane :** En étant regroupé avec d'autres alliés

**Synergies Race :**
- Plus d'alliés autour → Plus d'Arcane générée
- Confrérie/Formation : Mages qui se soutiennent mutuellement
- Synergise avec Nombre Critique, Formation, Ralliement

**Plan de jeu :**
- Développer un groupe de mages
- Chaque mage génère de l'Arcane pour le groupe
- Accumulation collective de puissance magique
- Sorts de zone boostés pour affecter tous les alliés/ennemis

**Génération d'Arcane :**
- `on_play` : Génère Arcane si alliés présents
- `on_surrounded` : Génère plus d'Arcane (formation magique)
- Début de tour : +X Arcane (X = nombre d'alliés)

---

### **🐉 DRAGON MAGE : Le Sorcier Solitaire**

**Génération d'Arcane :** En étant seul (méditation) ou à partir du mana

**Synergies Race :**
- Solitude → Méditation → Arcane massive
- Convertit le mana excédentaire en Arcane
- Plus puissant seul (identité Dragon : Solitaire)

**Plan de jeu :**
- Un dragon mage isolé qui médite
- Accumule de l'Arcane rapidement en solo
- Cast des sorts dévastateurs avec l'Arcane accumulée
- Convertit ressources (mana, trésors) en Arcane

**Génération d'Arcane :**
- `on_alone` : Génère beaucoup d'Arcane (méditation)
- `on_play` : Convertit X mana en Arcane
- Début de tour : Génère Arcane si seul

---

### **😈 DEMON MAGE : Le Nécromancien Sacrificateur**

**Génération d'Arcane :** En sacrifiant HP ou alliés

**Synergies Race :**
- Sacrifie des HP → Génère Arcane (Prix du Sang)
- Sacrifie des alliés → Génère Arcane (Rituel)
- Fournit un outlet pour les mécaniques Demon (sacrifice)

**Plan de jeu :**
- Génère des tokens/créatures faibles
- Sacrifie tout pour Arcane
- Cast des sorts massifs avec l'Arcane générée
- Cycle : Invocation → Sacrifice → Arcane → Sorts puissants

**Génération d'Arcane :**
- `on_play` : Sacrifie X HP pour générer Arcane
- `on_death` (alliés) : Génère Arcane quand un allié meurt
- Action manuelle : Sacrifie un allié pour Arcane

---

## 📊 Tableau Récapitulatif

| | **Human Mage** | **Dragon Mage** | **Demon Mage** |
|---|---|---|---|
| **Génère Arcane via** | Regroupement | Solitude / Mana | Sacrifice HP/Alliés |
| **Style** | Confrérie (collectif) | Solitaire (méditation) | Nécromancien (sacrifice) |
| **Sorts boostés** | AoE / Buffs de groupe | Dégâts massifs / Single target | Invocations / Drains |
| **Synergie Race** | Nombre/Formation | Solitaire/Thésaurisation | Sacrifice/Rituels |

---

## ⚡ Utilisation de l'Arcane

L'Arcane sert à **booster les sorts** de plusieurs manières :

### **Surcharge Arcanique**
"Si vous avez X+ Arcane, ce sort a un effet bonus"
```
📜 BOULE DE FEU (3 mana)
"Inflige 3 dégâts"
"Surcharge (5 Arcane) : Inflige 6 dégâts à tous les ennemis"
```

### **Dépense d'Arcane**
"Dépensez X Arcane pour amplifier ce sort"
```
📜 ÉCLAIR ARCANIQUE (2 mana)
"Inflige 2 dégâts"
"Dépensez X Arcane : Inflige +X dégâts supplémentaires"
```

### **Catalyseur**
"Consommez de l'Arcane pour des effets puissants"
```
📜 MÉTÉORE (6 mana)
"Dépensez 10 Arcane : Détruisez tous les ennemis"
```

---

## 🔄 Intégration avec les Triggers Existants

Les mécaniques d'Arcane s'intègrent naturellement avec les triggers existants :

### Human Mage
- `on_play` : Génère Arcane si X alliés présents
- `on_surrounded` : Génère Arcane bonus (formation de mages)
- Début de tour : Génère Arcane selon nombre d'alliés

### Dragon Mage
- `on_alone` : Génère beaucoup d'Arcane (méditation solitaire)
- `on_play` : Convertit mana → Arcane
- Sorts joués : Génèrent Arcane si seul

### Demon Mage
- `on_play` : Prix du Sang (HP → Arcane)
- `on_death` (alliés) : Mort d'allié → Arcane
- Rituel : Sacrifie plusieurs alliés pour Arcane massive

---

## ✅ Cohérence de Classe

**Ce qui unit tous les Mages :**
- Arcane comme ressource centrale
- Accumulation puis dépense pour sorts puissants
- Amplification magique

**Ce qui les différencie :**
- **Human** : Arcane collective (confrérie) → Sorts de groupe
- **Dragon** : Arcane solitaire (méditation) → Sorts massifs single target
- **Demon** : Arcane sacrificielle (sang/vie) → Sorts de nécromancie

---

## 🎯 Exemples de Cartes (Grandes Lignes)

### Human Mage
```
✨ MAGE DE CONFRÉRIE (coût faible)
"Génère +1 Arcane par allié adjacent"
"Surcharge (3) : Tous les alliés gagnent +1/+1"
```

### Dragon Mage
```
🐉 DRAGON ARCHIMAGE (coût élevé)
"Solitaire : Génère +3 Arcane par tour"
"Dépense 10 Arcane : Cast un sort dévastateur"
```

### Demon Mage
```
😈 NÉCROMANCIEN (coût moyen)
"Apparition : Sacrifie 5 HP → Génère 5 Arcane"
"Sacrifie un allié → Génère 3 Arcane"
"Dépense 8 Arcane : Invoque 3 démons"
```

---

## 🎮 Design Philosophy

Cette approche crée une identité de classe **cohérente** (tous accumulent/dépensent Arcane) mais **flexible** (chacun génère différemment) qui :

✅ S'intègre naturellement dans le plan de jeu de chaque race
✅ Utilise les triggers et mécaniques existants
✅ Crée des synergies cross-race (Mages ensemble) ET intra-race (Mage + sa race)
✅ Offre un gameplay rythmé : Accumulation → Dépense explosive
✅ Reste simple à comprendre : "Les Mages accumulent Arcane via X, dépensent pour booster sorts"

---

## 🎯 Cycles de Jeu par Race

### Human Mage
**Early :** Développe plusieurs mages
**Mid :** Accumule Arcane collectivement
**Late :** Sorts de masse boostés qui affectent tout le board

### Dragon Mage
**Early :** Survie en solo
**Mid :** Méditation → Arcane massive
**Late :** Un sort qui finit la partie (Arcane accumulée)

### Demon Mage
**Early :** Invoque des tokens
**Mid :** Sacrifie pour Arcane
**Late :** Sorts nécromantiques puissants (résurrections, drains massifs)

---

## 🚀 Implémentation Suggérée

**Phase 1** : Génération Arcane basique
- Human Mage : +1 Arcane par allié adjacent (via `on_play`)
- Dragon Mage : +2 Arcane par tour si seul (via `on_alone`)
- Demon Mage : Sacrifie HP → Arcane (via `on_play`)

**Phase 2** : Utilisation Arcane
- Surcharge basique : "Si 5+ Arcane, effet bonus"
- Dépense simple : "Dépense X Arcane pour effet"

**Phase 3** : Synergies complètes
- Sorts complexes avec multiples paliers de Surcharge
- Cartes qui manipulent l'Arcane (vol, partage, conversion)
- Combos entre génération et dépense

---

## 💡 Mécaniques Secondaires Possibles

### Sorts Signature par Race

**Human Mage :**
- Sorts de zone (AoE)
- Buffs de groupe
- Redéploiement tactique (Portail)

**Dragon Mage :**
- Sorts élémentaires massifs (Souffle magique)
- Transmutation
- Sorts à coût Arcane très élevé

**Demon Mage :**
- Invocations démoniaques (coût en Arcane)
- Drains de vie (HP → Arcane → Sorts)
- Résurrections (dépense Arcane pour ramener les morts)

### Arcane Avancé

**Partage d'Arcane** (Human Mage)
"Transférez X Arcane à un autre mage allié"

**Conversion** (Dragon Mage)
"Convertissez Mana/Trésors en Arcane"

**Vol d'Arcane** (potentiellement Rogue Mage ?)
"Volez X Arcane à l'adversaire"

