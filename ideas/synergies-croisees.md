# Synergies Croisées Race × Classe

## 🎯 Vue d'Ensemble

Ce document présente comment les mécaniques des **Races** (Human, Dragon, Demon) et des **Classes** (Warrior, Mage, Rogue) se combinent pour créer des identités uniques pour chaque combinaison.

Le système est conçu pour que :
- ✅ Les cartes d'une **Race** synergisent entre elles (ex: tous les Demons bénéficient de la Peur)
- ✅ Les cartes d'une **Classe** synergisent entre elles (ex: tous les Warriors accumulent de l'Armure)
- ✅ Chaque **combinaison Race+Classe** a une identité unique

---

## 📊 Matrice des Synergies

| | **Warrior** | **Mage** | **Rogue** |
|---|---|---|---|
| **Human** | Armée Disciplinée | Mages de Guerre | Guilde d'Assassins |
| **Dragon** | Forteresse Volante | Sorcier Ancien | Prédateur Furtif |
| **Demon** | Berserker Maudit | Nécromancien | Ombre Corrompue |

---

## 🔥 HUMAN + WARRIOR : L'ARMÉE DISCIPLINÉE

### Identité
Formations militaires organisées qui s'aguerrissent au combat et se protègent mutuellement.

### Mécaniques Clés
- **Vétéran** : Gagne +1/+1 permanent après chaque combat (attaque ou défense)
- **Formation** (Human) : Bonus quand adjacents à des alliés
- **Synergie de groupe** : Partagent buffs/armure entre eux
- **Scaling linéaire** : Croissance constante et prévisible, excellent mid-game

### Courbe de Puissance
Croissance **linéaire** ↗️ - L'armée devient progressivement plus forte, fiable en mid-game

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
```

### Stratégie
Construire une formation compacte de tanks qui accumulent de l'expérience de combat et partagent leurs buffs. Se renforce progressivement tour après tour.

---

## ✨ HUMAN + MAGE : LES MAGES DE GUERRE

### Identité
Tacticiens magiques qui manipulent le positionnement et créent des illusions d'armées.

### Mécaniques Clés
- **Redéploiement** (Mage) + **Formation** (Human) = Optimisation tactique
- **Illusion** (Mage) + **Nombre Critique** (Human) = Armée fantôme
- **Arcane** (Mage) + **Ralliement** (Human) = Magie collective

### Exemples de Cartes

```
✨ TACTICIEN ARCANIQUE (4 mana, 2/4)
Human Mage
"Apparition : Redéploiement x2"
"Vos alliés en Formation gagnent +1 Arcane par tour"
"Surcharge (5) : Tous les alliés gagnent +2/+2"

✨ ILLUSIONNISTE DE GUERRE (5 mana, 3/4)
Human Mage
"Apparition : Créez des Illusions de tous vos Human"
"Nombre Critique (4) : Les Illusions deviennent réelles (stats pleines)"

📜 STRATÉGIE PARFAITE (3 mana)
Sort Human Mage
"Redéploiement de tous vos alliés"
"Formez une Escouade avec jusqu'à 4 alliés"
"Gagne +3 Arcane. Surcharge (10) : Doublez les stats de l'Escouade"
```

### Stratégie
Créer une armée d'illusions puis les transformer en vraies créatures. Optimiser le board via Redéploiement.

---

## 🗡️ HUMAN + ROGUE : LA GUILDE D'ASSASSINS

### Identité
Escouade d'assassins coordonnés qui combinent leurs attaques et utilisent le Flanc.

### Mécaniques Clés
- **Combo** (Rogue) + **Nombre Critique** (Human) = Combos de masse
- **Embuscade** (Rogue) + **Flanc** (Human) = Attaques positionnelles
- **Momentum** (Rogue) + **Ralliement** (Human) = Vitesse collective

### Exemples de Cartes

```
🗡️ ASSASSIN DE LA GUILDE (2 mana, 3/1)
Human Rogue
"Combo : +2/+2"
"Flanc : Gagne Embuscade"
"Ralliement : Gagne +1 Momentum"

🗡️ MAÎTRE DE LA GUILDE (5 mana, 4/3)
Human Rogue
"Apparition : Invoquez 2 Assassins 2/2"
"Tous vos Human Rogue partagent leur Momentum"
"Nombre Critique (4) : Tous les alliés gagnent Vélocité (2)"

📜 FRAPPE COORDONNÉE (4 mana)
Sort Human Rogue
"Tous vos alliés attaquent la même cible"
"Combo : Gagnent Embuscade pour cette attaque"
"Nombre Critique (5) : La cible est détruite même si elle survit"
```

### Stratégie
Développer rapidement une équipe d'assassins, puis lancer un tour explosif avec Combo + Momentum partagé.

---

## 🐉 DRAGON + WARRIOR : LE TITAN SOLITAIRE

### Identité
Dragon-tank solitaire qui se régénère, enrage quand blessé, et devient inarrêtable.

### Mécaniques Clés
- **Solitaire** (Dragon) : Buffs massifs quand seul ou avec peu d'alliés
- **Rage Draconique** : Gagne +X/+0 permanent quand subit des dégâts (on_damage)
- **Régénération** : Se heal automatiquement (fin de tour ou on_damage)
- **Génération de mana** : Certains dragons produisent du mana
- **Robustesse** : Résiste aux petits dégâts

### Courbe de Puissance
Croissance **lente puis explosive** ↗️↗️ - Faible early, inarrêtable late-game

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

⚔️ FUREUR DRACONIQUE (4 mana)
Sort Dragon Warrior
"Un dragon ciblé gagne : 'On Damage : +5/+0 et Heal 5 HP'"
"Solitaire : L'effet dure toute la partie"
```

### Stratégie
Jouer un seul dragon massif qui encaisse les dégâts, se régénère, et devient exponentiellement plus dangereux à mesure qu'il est blessé.

---

## 🐉 DRAGON + MAGE : LE SORCIER ANCIEN

### Identité
Dragon élémental qui maîtrise la magie pure et cast des sorts dévastateurs.

### Mécaniques Clés
- **Souffle Élémentaire** (Dragon) + **Arcane** (Mage) = Magie draconique
- **Solitaire** (Dragon) + **Surcharge** (Mage) = Sorts massifs
- **Évolution** (Dragon) + **Invocation** (Mage) = Transformation magique

### Exemples de Cartes

```
🐉 DRAGON ARCHIMAGE (8 mana, 6/7)
Dragon Mage
"Solitaire : Gagne +5 Arcane par tour"
"Souffle Élémentaire : Active selon votre Arcane (Feu si <5, Glace si 5-10, Foudre si 10+)"
"Surcharge (15) : Vos sorts coûtent (0) et ont Écho Magique"

🐉 WYRM ÉLÉMENTAIRE (6 mana, 5/6)
Dragon Mage
"Apparition : Invoquez 3 Élémentaux X/X (X = votre Arcane)"
"Les Élémentaux ont le même élément que ce dragon"
"Solitaire II : Double l'Arcane gagnée par vos sorts"

📜 CATACLYSME DRACONIQUE (10 mana)
Sort Dragon Mage
"Tous vos dragons activent leur Souffle"
"Surcharge (20) : Répétez 3 fois"
"Solitaire : Inflige des dégâts = à votre Arcane totale à tous les ennemis"
```

### Stratégie
Un dragon mage ultra-puissant qui accumule Arcane en solo et lance des sorts apocalyptiques.

---

## 🐉 DRAGON + ROGUE : LE PRÉDATEUR FURTIF

### Identité
Dragon chasseur rapide, furtif, avec Embuscade dévastatrice.

### Mécaniques Clés
- **Solitaire** (Dragon) + **Furtif** (Rogue) = Invisible et puissant
- **Vélocité** (Rogue) + **Souffle** (Dragon) = Multi-attaques élémentaires
- **Embuscade** (Rogue) + **Domination** (Dragon) = One-shot

### Exemples de Cartes

```
🐉 DRAGON D'OMBRE (6 mana, 7/5)
Dragon Rogue
"Furtif permanent"
"Solitaire : +4/+0 et Embuscade (triple dégâts)"
"Vélocité (2)"

🐉 WYRM CHASSEUR (8 mana, 8/6)
Dragon Rogue
"Agile Suprême"
"Vélocité (X = cases parcourues ce tour)"
"Embuscade : Si dégâts tuent, regagne Furtif"
"Souffle Foudre : Attaque tous les ennemis adjacents"

🗡️ CHASSE DRACONIQUE (5 mana)
Sort Dragon Rogue
"Un dragon ciblé gagne Furtif et Embuscade jusqu'à la fin du tour"
"Solitaire : Gagne aussi Vélocité (5) et +5/+0"
"Finisher : Si tue une cible, peut attaquer à nouveau"
```

### Stratégie
Dragon assassin qui reste Furtif, puis frappe avec Embuscade pour des dégâts massifs.

---

## 😈 DEMON + WARRIOR : LE BERSERKER MAUDIT

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

📜 RAGE SUICIDAIRE (3 mana)
Sort Demon Warrior
"Tous vos Demon Warriors gagnent +5/+5"
"Ils perdent 4 HP supplémentaires à la fin du tour"
"Dernière Volonté amplifiée : Double les effets de mort"
```

### Stratégie
Rush agressif avec des créatures surpuissantes qui dépérissent. Transformer leur mort inévitable en avantage tactique via Dernière Volonté.

---

## 😈 DEMON + MAGE : LE NÉCROMANCIEN

### Identité
Sorcier corrompu qui fait des Rituels, manipule la Corruption, contrôle la mort.

### Mécaniques Clés
- **Rituel** (Demon) + **Invocation** (Mage) = Sacrifices pour invoquer
- **Corruption** (Demon) + **Transmutation** (Mage) = Transformation corrompue
- **Arcane** (Mage) + **Damnation** (Demon) = Puissance vs Risque

### Exemples de Cartes

```
😈 NÉCROMANCIEN MAUDIT (5 mana, 3/5)
Demon Mage
"Rituel (2) : Sacrifiez 2 alliés : Gagne +5 Arcane"
"Renaissance : Ressuscitez un allié sacrifié avec +3/+3 et Démoniaque"
"Surcharge (10) : Tous vos Rituels coûtent 1 sacrifice de moins"

😈 SEIGNEUR DE LA CORRUPTION (7 mana, 4/6)
Demon Mage
"Apparition : Inflige 3 Corruption à tous les ennemis"
"Transmutation Corrompue : Les ennemis transformés deviennent des démons sous votre contrôle"
"Gagne +1 Arcane par ennemi corrompu"

📜 RITUEL ARCANIQUE INTERDIT (8 mana)
Sort Demon Mage
"Rituel (5) : Sacrifiez 5 alliés"
"Invoque un Avatar du Chaos X/X (X = Arcane totale)"
"Avatar : Démoniaque, Écho Magique, Corruption (5)"
"Damnation +3"
```

### Stratégie
Générer des tokens, les sacrifier pour Arcane, invoquer des horreurs, corrompre le board adverse.

---

## 😈 DEMON + ROGUE : L'OMBRE CORROMPUE

### Identité
Assassin démoniaque ultra-rapide qui empoisonne, sacrifie pour Combos, spread la Peur.

### Mécaniques Clés
- **Combo** (Rogue) + **Rituel** (Demon) = Sacrifices pour cartes gratuites → Mega Combos
- **Poison** (Rogue) + **Corruption** (Demon) = Debuffs stackés
- **Furtif** (Rogue) + **Peur** (Demon) = Invisible et terrifiant

### Exemples de Cartes

```
😈 ASSASSIN DE L'ABÎME (3 mana, 4/2)
Demon Rogue
"Furtif"
"Combo : +2/+2 et inflige 2 Peur à la cible"
"Attaque : Poison (2) et Corruption (1)"
"Démoniaque"

😈 MAÎTRE DES OMBRES MAUDIT (6 mana, 5/4)
Demon Rogue
"Rituel (1) : Sacrifiez un allié : Cette carte coûte (0)"
"Créez 3 Ombres démoniaques (Furtif, Poison 3, Corruption 2)"
"Combo : Les Ombres ont Embuscade"
"Tous les ennemis avec Peur ou Poison ont -2/-2"

📜 COMBO INTERDIT (5 mana)
Sort Demon Rogue
"Prix du Sang : 5 mana OU 10 HP"
"Sacrifiez tous vos alliés. Gagnez +3 Momentum par allié sacrifié"
"Tous vos Rogues attaquent immédiatement avec Embuscade"
"Inflige Peur = Momentum total à tous les ennemis"
```

### Stratégie
Générer des petites créatures, les sacrifier pour jouer gratuitement, lancer un tour ultra-combo avec Momentum massif.

---

## 🎲 Tableau Récapitulatif des Mécaniques

### ⚔️ Trinité Warriors - Courbes de Puissance

| | **Human Warrior** | **Dragon Warrior** | **Demon Warrior** |
|---|---|---|---|
| **Courbe** | Linéaire ↗️ | Lente puis explosive ↗️↗️ | Pic puis chute ↘️ |
| **Timing fort** | Mid-game | Late-game | Early/Tempo |
| **Scaling** | +1/+1 par combat | Rage on_damage + Regen | Stats initiales élevées |
| **Survie** | Groupe solidaire | Tank régénérant | Autodestruction |
| **Triggers clés** | On combat (attack/defend) | On damage (rage/heal) | On death (dernière volonté) |
| **Playstyle** | Armée disciplinée | Titan solitaire | Berserker suicide |

---

### Mécaniques par Race

| **Human** | **Dragon** | **Demon** |
|---|---|---|
| Formation | Solitaire | Peur |
| Ralliement | Élément | Pacte |
| Nombre Critique | Souffle | Corruption |
| Escouade | Thésaurisation | Rituel |
| Vétéran (partagé) | Évolution | Prix du Sang |
| | Domination | Damnation |
| | Maturité | Renaissance |
| | Sagesse Ancienne | Contagion |

### Mécaniques par Classe

| **Warrior** | **Mage** | **Rogue** |
|---|---|---|
| Armure | Arcane | Combo |
| Fortification | Surcharge Arcanique | Momentum |
| Vétéran | Catalyseur | Furtif |
| Riposte | Écho Magique | Embuscade |
| Épines | Redéploiement | Vélocité |
| Robustesse | Duplication | Agile |
| Arme | Invocation Élémentaire | Dash |
| Mur de Chair | Stase Temporelle | Esquive |
| Bataille Prolongée | Chaîne de Sorts | Poison |
| Cicatrices de Guerre | Transmutation | Vol |
| | Illusion | Sabotage |
| | Résonance | Finisher |

---

## 💡 Conseils de Game Design

### 1. Équilibrage
- Chaque combinaison doit avoir des **points forts et faibles**
- Human excelle en mid-game, Dragon en late, Demon en risk/reward
- Warrior tank, Mage contrôle, Rogue aggro

### 2. Identité Claire
- Chaque deck Race+Classe doit se jouer **différemment**
- Human Warrior ≠ Dragon Warrior ≠ Demon Warrior
- Les mécaniques se combinent pour créer de nouveaux archétypes

### 3. Synergies Croisées
- Les cartes Warrior doivent marcher dans **tous les decks Warrior**
- Les cartes Dragon doivent marcher dans **tous les decks Dragon**
- Mais les cartes **Dragon Warrior** sont encore meilleures dans un deck Dragon Warrior

### 4. Évolutivité
- Le système permet d'ajouter de nouvelles Races/Classes facilement
- Nouvelles mécaniques peuvent cibler Race OU Classe OU combinaison

---

## 🎨 Exemples de Decks

### Deck "Phalange Immortelle" (Human Warrior)
- Focus : Formation + Armure
- Win condition : Mur indestructible de tanks
- Cartes clés : Légionnaires, Fortification de masse

### Deck "Dragon Solitaire" (Dragon Warrior)
- Focus : Un seul dragon ultra-tanké
- Win condition : Domination totale late game
- Cartes clés : Armure + Trésors sur un dragon

### Deck "Pacte de Sang" (Demon Warrior)
- Focus : Pactes + Renaissance
- Win condition : Meurt et revient en boucle
- Cartes clés : Berserkers maudits, Rituels

### Deck "Illusion Army" (Human Mage)
- Focus : Illusions + Nombre Critique
- Win condition : Armée fantôme devient réelle
- Cartes clés : Illusionniste, Transmutation

### Deck "Arcane Dragon" (Dragon Mage)
- Focus : Arcane + Solitaire
- Win condition : Sorts apocalyptiques
- Cartes clés : Accumulation Arcane, Souffles

### Deck "Corruption Mill" (Demon Mage)
- Focus : Corruption + Possession
- Win condition : Contrôle total du board adverse
- Cartes clés : Contagion, Transmutation corrompue

### Deck "Assassin Strike" (Human Rogue)
- Focus : Combo coordonné
- Win condition : OTK via Momentum partagé
- Cartes clés : Guilde d'assassins, Combos de masse

### Deck "Shadow Dragon" (Dragon Rogue)
- Focus : Furtif + Embuscade
- Win condition : One-shot surprise
- Cartes clés : Dragon d'ombre, Vélocité

### Deck "Poison Chain" (Demon Rogue)
- Focus : Poison + Corruption + Peur
- Win condition : Debuff stack mortel
- Cartes clés : Multi-debuffs, Sacrifice pour Combo

---

## 🔮 Extensions Futures

### Nouvelles Races Possibles
- **Undead** (Morts-vivants) : Résurrection, Drain
- **Elf** (Elfes) : Nature, Archery, Precision
- **Orc** : Rage, Sacrifice, Brutality

### Nouvelles Classes Possibles
- **Priest** (Prêtre) : Heal, Resurrection, Buff
- **Ranger** : Distance, Traps, Pets
- **Paladin** : Hybrid Warrior/Priest

### Cross-Synergies
- Cartes qui profitent de **2 races** (ex: "Si vous contrôlez un Human OU un Dragon...")
- Cartes neutres qui boostent une **combinaison spécifique** (ex: "Human Warriors gagnent +2/+2")

