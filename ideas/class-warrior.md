# Classe WARRIOR - Design Final

## 🎯 Identité Thématique

Les Warriors représentent **le combat au corps-à-corps et l'évolution à travers le combat**.

**Principe de design clé** : Les Warriors se **boostent** à travers le combat, mais différemment selon leur race :
- **Human Warrior** : Booste ses **alliés**
- **Dragon Warrior** : Booste **lui-même**
- **Demon Warrior** : Se **dé-booste** au fil du temps (malédiction)

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
Formations militaires qui **boostent leurs alliés** à travers le combat.

### Mécaniques Clés
- **Vétéran** : Gagne +1/+1 permanent après chaque combat (attaque ou défense)
- **Boost d'alliés** : Donne des buffs permanents aux alliés adjacents ou à toute l'armée
- **Formation** : Bonus quand adjacents à des alliés
- **Scaling collectif** : L'armée entière devient plus forte ensemble

### Courbe de Puissance
**Croissance linéaire collective** ↗️ - L'armée devient progressivement plus forte, fiable en mid-game

### Triggers de Combat
- **On Attack** : Gagne Vétéran (+1/+1) ET/OU booste un allié
- **On Defense** : Gagne Vétéran (+1/+1) ET/OU booste un allié

### Exemples de Cartes

```
⚔️ LÉGIONNAIRE ROYAL (3 mana, 2/4)
Human Warrior
"Vétéran : Gagne +1/+1 permanent après chaque combat"
"Quand attaque : Un allié adjacent gagne +1/+1 permanent"

⚔️ CENTURION VÉTÉRAN (5 mana, 4/5)
Human Warrior
"Vétéran"
"Formation 2/2"
"Quand ce monstre combat : Tous vos alliés gagnent +1/+0 permanent"

⚔️ PHALANGE D'ACIER (6 mana, 3/6)
Human Warrior
"Apparition : Invoquez 2 Légionnaires 2/3 avec Vétéran"
"Formation 2/2"
"Quand un allié combat : Tous vos Human Warriors gagnent +1/+1 permanent"

📜 DISCIPLINE MILITAIRE (4 mana)
Sort Human Warrior
"Tous vos Warriors gagnent +2/+2 permanent"
"Tous vos Warriors gagnent Vétéran jusqu'à la fin du tour"
```

### Stratégie
Construire une armée qui se booste collectivement à chaque combat. Chaque warrior rend les autres plus forts.

### Points Forts
- Croissance prévisible et constante
- Synergie de groupe forte (se boostent mutuellement)
- Excellent contrôle du board mid-game
- Snowball collectif

### Points Faibles
- Lent à démarrer
- Vulnérable au removal de masse (AOE)
- Nécessite plusieurs créatures pour être efficace

---

## 🐉 DRAGON WARRIOR : LE TITAN SOLITAIRE

### Identité
Dragon solitaire qui **se booste lui-même** à travers le combat et la douleur.

### Mécaniques Clés
- **Solitaire** : Buffs massifs quand seul ou avec peu d'alliés
- **Rage Draconique** : Gagne +X/+0 permanent quand subit des dégâts (on_damage)
- **Self-heal** : Se régénère après avoir combattu
- **Self-boost au combat** : Gagne stats permanentes quand attaque ou défend

### Courbe de Puissance
**Croissance lente puis explosive** ↗️↗️ - Faible early, inarrêtable late-game

### Triggers de Combat
- **On Damage** : Gagne +X/+0 permanent (Rage)
- **On Attack** : Gagne +1/+1 permanent (Vétéran dragon)
- **On Defense** : Gagne +0/+1 permanent ou heal

### Exemples de Cartes

```
🐉 DRAGON CUIRASSÉ (7 mana, 5/8)
Dragon Warrior
"Solitaire : +3/+3"
"On Damage : Gagne +2/+0 permanent (Rage Draconique)"
"Quand attaque : Gagne +1/+1 permanent"

🐉 WYRM RÉGÉNÉRANT (9 mana, 6/10)
Dragon Warrior
"Solitaire : +4/+4"
"On Damage : Gagne +3/+0 permanent ET heal 3 HP"
"Quand combat : Gagne +2/+2 permanent"

🐉 JEUNE DRAGON GUERRIER (5 mana, 4/6)
Dragon Warrior
"Solitaire : +2/+2"
"On Damage : Gagne +1/+0 permanent"
"Quand attaque : Heal 2 HP"

⚔️ FUREUR DRACONIQUE (4 mana)
Sort Dragon Warrior
"Un dragon ciblé gagne +5/+5 permanent"
"Ce dragon gagne : 'On Damage : +3/+0 permanent' jusqu'à la fin de la partie"
```

### Stratégie
Jouer un seul dragon massif qui se booste exponentiellement à chaque combat et à chaque dégât reçu.

### Points Forts
- Scaling exponentiel (se booste lui-même en continu)
- Devient un monstre inarrêtable late-game
- Self-heal permet de survivre longtemps

### Points Faibles
- Très vulnérable early game
- Dépendant d'une seule créature (removal direct = perte)
- Lent à démarrer
- Coût en mana élevé

---

## 😈 DEMON WARRIOR : LE BERSERKER MAUDIT

### Identité
Guerrier puissant mais maudit qui **se dé-booste** au fil du temps (malédiction).

### Mécaniques Clés
- **Stats initiales élevées** : Très fort dès l'invocation pour son coût (overcost)
- **Dépérissement** : Perd HP à la fin du tour, ou quand attaque
- **Malédiction progressive** : Perd des stats au fil du temps
- **On Death triggers** : Effets puissants à la mort pour compenser
- **Pactes/Sacrifices** : Peut sacrifier HP pour ralentir le dépérissement

### Courbe de Puissance
**Pic immédiat puis chute** ↘️ - Très fort early/mid, s'effondre ensuite ou meurt glorieusement

### Triggers de Dépérissement
- **End of Turn** : Perd HP (dépérissement)
- **On Attack** : Perd HP ou stats (certaines cartes)
- **On Damage** : Buffs temporaires (sursaut avant la mort)
- **On Death** : Dernière Volonté (effets massifs)

### Exemples de Cartes

```
😈 BERSERKER DAMNÉ (4 mana, 6/6)
Demon Warrior
"Fin de Tour : Perd 2 HP"
"On Damage : Gagne +3/+0 jusqu'à la fin du tour (sursaut)"
"Dernière Volonté : Inflige 6 dégâts à tous les ennemis"

😈 CHAMPION DE L'ABÎME (6 mana, 8/8)
Demon Warrior
"Quand attaque : Perd 3 HP"
"Fin de Tour : Perd 1/1 permanent (malédiction)"
"Dernière Volonté : Tous vos alliés gagnent +4/+4 permanent"

😈 GLADIATEUR MAUDIT (3 mana, 5/4)
Demon Warrior
"Fin de Tour : Perd 1 HP"
"Dernière Volonté : Invoquez une copie de cette carte avec 1 HP"

📜 RAGE SUICIDAIRE (3 mana)
Sort Demon Warrior
"Tous vos Demon Warriors gagnent +5/+5 permanent"
"Ils perdent 3 HP supplémentaires à la fin du tour (malédiction amplifiée)"
```

### Stratégie
Rush agressif avec des créatures surpuissantes qui dépérissent. Transformer leur mort inévitable en avantage tactique via Dernière Volonté.

### Points Forts
- Pression early game énorme (stats overcost)
- Stats très efficaces pour le coût initial
- Effets de mort dévastateurs
- Excellent pour le tempo et l'aggro

### Points Faibles
- Auto-destruction inévitable (malédiction)
- Perd de la valeur en late-game
- Difficile de contrôler le timing de mort
- Se dé-booste constamment

---

## 🎲 Mécaniques Warrior

### Vétéran
- "Gagne +1/+1 permanent après chaque combat (attaque ou défense)"
- Utilisé par toutes les races mais différemment :
  - **Human** : Vétéran + booste les alliés
  - **Dragon** : Vétéran + self-boost massif
  - **Demon** : Commence fort mais se dé-booste

### Triggers de Combat
- **On Attack** : Se déclenche quand la créature attaque
- **On Defense** : Se déclenche quand la créature défend/bloque
- **On Damage** : Se déclenche quand la créature subit des dégâts
- **On Death** : Se déclenche quand la créature meurt

### Solitaire
- Keyword Dragon : Buffs quand seul ou avec peu d'alliés
- Encourage le gameplay mono-créature

### Formation
- Keyword Human : Bonus quand adjacent à des alliés
- Encourage le gameplay multi-créatures

---

## 📊 Résumé Design

### Philosophie Classe
**"Boost à travers le combat"** - Chaque race booste différemment :
- **Human** : Booste ses **alliés** (croissance collective)
- **Dragon** : Booste **lui-même** (croissance individuelle explosive)
- **Demon** : Se **dé-booste** (malédiction, déclin)

### Mécaniques Centrales
1. **Triggers de combat** (on_attack, on_defend, on_damage, on_death)
2. **Vétéran** : Utilisé par toutes les races
3. **Boost/Dé-boost** : Direction différente selon la race

### Balance Général
- **Early** : Demon > Human > Dragon
- **Mid** : Human > Demon > Dragon
- **Late** : Dragon > Human > Demon

### Différenciation Boost
- **Human** : Boost d'équipe, linéaire, partagé
- **Dragon** : Boost personnel, exponentiel, concentré
- **Demon** : Dé-boost progressif, compensé par on_death
