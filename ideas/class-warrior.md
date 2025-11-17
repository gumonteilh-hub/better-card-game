# Classe WARRIOR - Design Final

## 🎯 Identité Thématique

Les Warriors représentent **le combat au corps-à-corps, la résilience et l'évolution à travers le combat**.

**Principe de design clé** : Chaque race de Warrior a une **courbe de puissance unique** basée sur des **triggers de combat** différents.

---

## ⚔️ Trinité Warriors - Courbes de Puissance

| | **Human Warrior** | **Dragon Warrior** | **Demon Warrior** |
|---|---|---|---|
| **Courbe** | Linéaire ↗️ | Lente puis explosive ↗️↗️ | Pic puis chute ↘️ |
| **Timing fort** | Mid-game | Late-game | Early/Tempo |
| **Scaling** | +1/+1 par combat | Rage on_damage + Regen | Stats initiales élevées |
| **Survie** | Groupe solidaire | Tank régénérant | Autodestruction |
| **Triggers clés** | On combat (attack/defend) | On damage (rage/heal) | On death (dernière volonté) |
| **Playstyle** | Armée disciplinée | Titan solitaire | Berserker suicide |

---

## 🔥 HUMAN WARRIOR : L'ARMÉE DISCIPLINÉE

### Identité
Formations militaires organisées qui s'aguerrissent au combat et se protègent mutuellement.

### Mécaniques Clés
- **Vétéran** : Gagne +1/+1 permanent après chaque combat (attaque ou défense)
- **Formation** : Bonus quand adjacents à des alliés
- **Synergie de groupe** : Partagent buffs/armure entre eux
- **Scaling linéaire** : Croissance constante et prévisible

### Courbe de Puissance
**Croissance linéaire** ↗️ - L'armée devient progressivement plus forte, fiable en mid-game

### Triggers de Combat
- **On Attack** : Gagne Vétéran (+1/+1)
- **On Defense** : Gagne Vétéran (+1/+1)
- **On Ally Summon** : Ralliement (buffs partagés)

### Exemples de Cartes

```
⚔️ LÉGIONNAIRE ROYAL (3 mana, 2/4)
Human Warrior
"Vétéran : Gagne +1/+1 permanent après chaque combat"
"Formation 1/1"
"Mur de Chair : Les alliés adjacents gagnent +2 Armure"

⚔️ CENTURION VÉTÉRAN (5 mana, 4/5)
Human Warrior
"Vétéran"
"Formation 2/2"
"Ralliement : Quand un allié apparaît, ce monstre gagne +1/+1 permanent"
"Si vous contrôlez 5+ alliés : Robustesse (10)"

⚔️ PHALANGE D'ACIER (6 mana, 3/6)
Human Warrior
"Apparition : Invoquez 2 Légionnaires 2/3 avec Vétéran"
"Formation 2/2"
"Tous vos Human Warriors partagent leurs bonus de Vétéran"

📜 DISCIPLINE MILITAIRE (4 mana)
Sort Human Warrior
"Tous vos Warriors en Formation gagnent +2/+2 permanent"
"Vétéran : La prochaine attaque de chaque Warrior lui donne +2/+2 supplémentaire"
```

### Stratégie
Construire une formation compacte de tanks qui accumulent de l'expérience de combat et partagent leurs buffs. Se renforce progressivement tour après tour.

### Points Forts
- Croissance prévisible et constante
- Synergie de groupe forte
- Excellent contrôle du board mid-game

### Points Faibles
- Lent à démarrer
- Vulnérable au removal de masse (AOE)
- Dépendant du positionnement

---

## 🐉 DRAGON WARRIOR : LE TITAN SOLITAIRE

### Identité
Dragon-tank solitaire qui se régénère, enrage quand blessé, et devient inarrêtable.

### Mécaniques Clés
- **Solitaire** : Buffs massifs quand seul ou avec peu d'alliés
- **Rage Draconique** : Gagne +X/+0 permanent quand subit des dégâts (on_damage)
- **Régénération** : Se heal automatiquement (fin de tour ou on_damage)
- **Génération de mana** : Certains dragons produisent du mana
- **Robustesse** : Résiste aux petits dégâts

### Courbe de Puissance
**Croissance lente puis explosive** ↗️↗️ - Faible early, inarrêtable late-game

### Triggers de Combat
- **On Damage** : Rage (+X/+0 permanent) ET/OU Régénération (heal)
- **End of Turn** : Régénération si a combattu
- **Start of Turn** : Génération de mana (certaines cartes)

### Exemples de Cartes

```
🐉 DRAGON CUIRASSÉ (7 mana, 5/8)
Dragon Warrior
"Solitaire : +3/+3 et Robustesse (5)"
"On Damage : Gagne +2/+0 permanent (Rage Draconique)"
"Fin de Tour : Si a combattu ce tour, heal 3 HP"

🐉 WYRM RÉGÉNÉRANT (9 mana, 6/10)
Dragon Warrior
"Solitaire : +4/+4 et Robustesse (10)"
"On Damage : Heal X HP (X = dégâts subis / 2)"
"On Damage : Gagne +3/+0 permanent"
"Génère 1 mana au début de votre tour"

🐉 JEUNE DRAGON GUERRIER (5 mana, 4/6)
Dragon Warrior
"Solitaire : +2/+2"
"On Damage : Gagne +1/+0 permanent"
"À 10+ ATK : Gagne Régénération (5) et Robustesse (5)"

⚔️ FUREUR DRACONIQUE (4 mana)
Sort Dragon Warrior
"Un dragon ciblé gagne : 'On Damage : +5/+0 et Heal 5 HP'"
"Solitaire : L'effet dure toute la partie"
```

### Stratégie
Jouer un seul dragon massif qui encaisse les dégâts, se régénère, et devient exponentiellement plus dangereux à mesure qu'il est blessé.

### Points Forts
- Scaling exponentiel (Rage cumulative)
- Sustain incroyable (Régénération)
- Extrêmement difficile à tuer late-game

### Points Faibles
- Très vulnérable early game
- Dépendant d'une seule créature (removal = GG)
- Lent à setup
- Coût en mana élevé

---

## 😈 DEMON WARRIOR : LE BERSERKER MAUDIT

### Identité
Guerrier puissant mais maudit qui dépérit rapidement et explose à la mort.

### Mécaniques Clés
- **Stats initiales élevées** : Très fort dès l'invocation pour son coût
- **Dépérissement** : Perd HP à la fin du tour, ou quand attaque
- **On Damage triggers** : Effets quand subit des dégâts (buffs temporaires, AOE)
- **Dernière Volonté (on_death)** : Effets puissants à la mort
- **Pactes/Sacrifices** : Peut sacrifier HP pour des effets

### Courbe de Puissance
**Pic immédiat puis chute** ↘️ - Très fort early/mid, s'effondre ensuite ou meurt glorieusement

### Triggers de Combat
- **On Death** : Dernière Volonté (effets massifs)
- **On Damage** : Buffs temporaires OU dégâts AOE
- **End of Turn** : Perd HP (dépérissement)
- **On Attack** : Perd HP (certaines cartes)

### Exemples de Cartes

```
😈 BERSERKER DAMNÉ (4 mana, 6/6)
Demon Warrior
"Fin de Tour : Perd 2 HP"
"On Damage : Gagne +3/+0 jusqu'à la fin du tour"
"Dernière Volonté : Inflige 6 dégâts à tous les ennemis"

😈 CHAMPION DE L'ABÎME (6 mana, 8/8)
Demon Warrior
"Quand attaque : Perd 3 HP"
"On Damage : Inflige 2 dégâts à tous les ennemis"
"Dernière Volonté : Tous vos alliés gagnent +4/+4 permanent"
"Prix du Sang : Sacrifiez 5 HP pour que ce monstre attaque immédiatement"

😈 GLADIATEUR MAUDIT (3 mana, 5/4)
Demon Warrior
"Fin de Tour : Perd 1 HP"
"On Damage : Tous les ennemis perdent 1 HP"
"Dernière Volonté : Invoquez une copie de cette carte avec 1 HP"

📜 RAGE SUICIDAIRE (3 mana)
Sort Demon Warrior
"Tous vos Demon Warriors gagnent +5/+5"
"Ils perdent 4 HP supplémentaires à la fin du tour"
"Dernière Volonté amplifiée : Double les effets de mort"
```

### Stratégie
Rush agressif avec des créatures surpuissantes qui dépérissent. Transformer leur mort inévitable en avantage tactique via Dernière Volonté.

### Points Forts
- Pression early game énorme
- Stats très efficaces pour le coût
- Effets de mort dévastateurs
- Excellent pour le tempo

### Points Faibles
- Auto-destruction inévitable
- Perd de la valeur en late-game
- Difficile de contrôler le timing de mort
- Vulnérable au silence/exile

---

## 🎲 Mécaniques Partagées (Toutes Races)

### Armure
- Compteur qui absorbe les dégâts avant les HP
- Peut être générée par cartes spécifiques
- **Human** : Armure partagée (Mur de Chair)
- **Dragon** : Armure massive individuelle
- **Demon** : Armure sacrifiée pour effets

### Robustesse (X)
- "Immune aux sources de dégâts de X ou moins"
- Exemple : Robustesse (5) = immune aux dégâts de 1-5
- Rend les tanks très difficiles à éliminer avec petits dégâts

### Fortification
- "Gagne X Armure"
- Peut être passive (chaque tour) ou conditionnelle

### Riposte
- "Quand attaqué, inflige des dégâts en retour"
- Variantes : dégâts fixes, dégâts = stats, contre-attaque complète

---

## 🔄 Comparaison avec Autres Classes

### Warrior vs Mage
- **Warrior** : Contact, endurance, scaling → **Mage** : Distance, burst, arcane
- Warrior résiste avec Armure/Robustesse
- Match-up : Tank vs Artillerie

### Warrior vs Rogue
- **Warrior** : Lent, tanky → **Rogue** : Rapide, fragile
- Warrior counter via Riposte et HP élevés
- Match-up : Forteresse vs Assassin

---

## 📊 Résumé Design

### Philosophie Classe
**"Évolution à travers le combat"** - Chaque race évolue différemment :
- Human : Croissance collective stable
- Dragon : Transformation en monstre inarrêtable
- Demon : Explosion de puissance puis sacrifice glorieux

### Mécaniques Centrales
1. **Triggers de combat** (on_attack, on_defend, on_damage, on_death)
2. **Scaling** (différent par race)
3. **Résilience** (Armure, Robustesse, Régénération)

### Balance Général
- **Early** : Demon > Human > Dragon
- **Mid** : Human > Demon > Dragon
- **Late** : Dragon > Human > Demon

### Counters
- **Removal direct** : Effective contre Dragon (1 seule créature)
- **AOE** : Effective contre Human (nombreux petits)
- **Stall/Control** : Effective contre Demon (laisse mourir)

---

## 🎨 Variantes et Extensions Futures

### Sous-Archétypes Possibles
1. **Human Tank Wall** : Focus défense + Formation
2. **Dragon Rage Scaling** : Focus on_damage + ATK massif
3. **Demon Suicide Aggro** : Focus Dernière Volonté + tempo
4. **Hybrid Regeneration** : Mix de toutes races avec heal

### Nouvelles Mécaniques Futures
- **Bannière de Guerre** : Buff d'équipe persistant
- **Stance (Offensive/Défensive)** : Toggle entre modes
- **Cicatrices de Guerre** : Compteur permanent de dégâts subis
- **Armes** : Équipement attaché (si système développé)
