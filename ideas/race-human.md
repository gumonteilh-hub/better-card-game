# Race HUMAN - Propositions de Game Design

## 🎯 Identité Thématique

Les Humains représentent **l'armée organisée, la force du nombre et la coordination tactique**. Faibles individuellement mais redoutables en groupe, ils excellent dans les formations militaires et le soutien mutuel.

---

## 💡 Proposition 1 : "L'ARMÉE COORDONNÉE"

### Concept Principal
Les Humains gagnent en puissance selon le **nombre d'alliés sur le terrain**. Plus vous contrôlez de créatures, plus vos effets sont puissants.

### Nouvelles Mécaniques

#### **1. Formation (nouveau keyword)**
**Description :** "Ce monstre gagne +X/+X pour chaque allié adjacent"
- Exemple : Un Capitaine avec "Formation 1/1" gagne +1/+1 par allié lié
- Synergie directe avec le positionnement tactique
- Encourage à créer des lignes de défense compactes

#### **2. Ralliement (nouveau trigger)**
**Description :** "S'active quand un autre allié entre sur le terrain"
- Exemple : "Ralliement : +1/+1"
- Crée des chaînes de value quand vous invoquez plusieurs créatures
- Synergie parfaite avec les effets Summon

#### **3. Nombre Critique (condition)**
**Description :** "Si vous contrôlez X alliés ou plus..."
- Exemple : "Nombre Critique (4) : +3/+3 et Téméraire"
- Récompense les decks swarm
- Crée des moments de "masse critique" où l'armée devient inarrêtable

### Exemples de Cartes

```
🗡️ RECRUE DU ROYAUME (1 mana, 1/2)
Humain Warrior
"Ralliement : +1/+0"

🗡️ CAPITAINE DE GARDE (3 mana, 2/3)
Humain Warrior
"Formation 1/1"
"Entouré : Les alliés adjacents gagnent +1/+1"

⚔️ MARÉCHAL ROYAL (5 mana, 3/4)
Humain Warrior
"Apparition : Invoque 2 Recrues du Royaume"
"Nombre Critique (5) : Tous les alliés gagnent Téméraire"

📜 CRI DE GUERRE (2 mana)
Sort Humain
"Donnez +2/+2 à tous vos alliés"
"Nombre Critique (4) : À la place, donnez +4/+4"

🗡️ PHALANGE ROYALE (4 mana, 2/5)
Humain Warrior
"Formation 0/2"
"Tant que vous contrôlez 3 alliés ou plus, cette carte ne peut pas être détruite par des effets"
```

### Synergies Inter-Classes
- **Human Mage** : Invoque des tokens magiques, buffs de zone
- **Human Rogue** : Infiltrateurs qui bénéficient de la distraction de l'armée

---

## 💡 Proposition 2 : "TACTIQUE MILITAIRE"

### Concept Principal
Les Humains manipulent le **positionnement** pour créer des formations tactiques avantageuses.

### Nouvelles Mécaniques

#### **1. Ligne de Front (nouveau keyword)**
**Description :** "Si ce monstre est en position d'attaque ET qu'un allié est derrière lui en défense..."
- Crée des synergies position avant/arrière
- Le monstre de front protège celui de l'arrière
- Exemple : "Ligne de Front : +2/+2 et les alliés en défense derrière gagnent +1/+1"

#### **2. Redéploiement (nouvelle action)**
**Description :** Permet de déplacer un allié GRATUITEMENT (sans coût de mouvement)
- Certaines cartes ont "Apparition : Redéploiement (déplacez un allié gratuitement)"
- Permet de réorganiser les formations tactiquement
- Synergie avec Formation et Ligne de Front

#### **3. Flanc (condition de position)**
**Description :** "Si ce monstre est sur une position latérale (0, 1, 6, ou 7)..."
- Récompense le positionnement sur les côtés
- Exemple : "Flanc : Gagne Agile"

### Exemples de Cartes

```
🗡️ CHEVALIER D'AVANT-GARDE (3 mana, 3/4)
Humain Warrior
"Ligne de Front : +2/+0 et peut attaquer deux fois"

🏹 ARCHER ROYAL (2 mana, 2/2)
Humain Rogue
"Flanc : Gagne +2 attaque"
"Ne peut pas être placé sur une position centrale (2, 3, 4, 5)"

🗡️ TACTICIEN (4 mana, 2/5)
Humain Warrior
"Apparition : Redéploiement x2"
"Vos monstres en Formation gagnent +1/+1 supplémentaire"

✨ MAGE STRATÈGE (5 mana, 3/4)
Humain Mage
"Chaque tour, vous pouvez dépenser 1 mana : Redéploiement"
"Ligne de Front : Pioche 1 carte"

📜 MANŒUVRE TACTIQUE (1 mana)
Sort Humain
"Redéploiement x3"
"Si vous avez au moins une Ligne de Front active, pioche 1 carte"
```

### Synergies Inter-Classes
- **Human Warrior** : Tient la ligne de front
- **Human Rogue** : Attaque depuis les flancs
- **Human Mage** : Contrôle depuis l'arrière

---

## 💡 Proposition 3 : "L'UNITÉ FAIT LA FORCE"

### Concept Principal
Les Humains ont des cartes **faibles seules mais qui se combinent** pour créer des effets puissants.

### Nouvelles Mécaniques

#### **1. Jumelé (nouveau keyword)**
**Description :** "Si vous contrôlez au moins un autre allié avec Jumelé..."
- Les cartes Jumelées ont un effet de base faible
- Mais deviennent TRÈS fortes quand plusieurs sont sur le terrain
- Exemple : Soldat Jumelé (1/1) devient (3/3) si vous avez un autre Jumelé

#### **2. Escouade (nouveau type de buff)**
**Description :** Certaines cartes créent des "Escouades" - groupes de créatures qui partagent des buffs
- Exemple : "Formez une Escouade avec jusqu'à 3 alliés adjacents. Tous les membres gagnent +1/+1"
- Mécanisme persistant (l'Escouade reste tant que les créatures vivent)
- Visuel marqueur pour identifier les membres

#### **3. Sacrifice Coordonné (mécanique avancée)**
**Description :** "Sacrifiez X alliés : Effet proportionnel"
- Utilise vos nombreuses petites créatures comme ressource
- Exemple : "Sacrifiez 3 alliés : Invoque un Paladin 6/6"
- Transforme la quantité en qualité

### Exemples de Cartes

```
🗡️ SOLDAT ROYAL (2 mana, 1/1)
Humain Warrior
"Jumelé : +2/+2 et Téméraire"

🗡️ FRÈRES D'ARMES (3 mana, 2/2)
Humain Warrior
"Apparition : Invoque une copie de cette carte"
"Jumelé : Les deux gagnent +3/+3"

👑 SERGENT RECRUTEUR (3 mana, 2/3)
Humain Warrior
"Apparition : Formez une Escouade avec jusqu'à 2 alliés adjacents"
"Les membres de l'Escouade gagnent +1/+1 et Formation 1/0"

✨ PRÊTRE DE BATAILLE (4 mana, 2/5)
Humain Mage
"Fin de tour : Si vous contrôlez 4 alliés ou plus, tous gagnent +1/+1 (permanent)"

📜 POUR LE ROYAUME ! (5 mana)
Sort Humain
"Sacrifiez X alliés : Vos alliés restants gagnent +2X/+2X"
"Si vous sacrifiez 3+ alliés : Invoquez un Dragon Antique 10/10"

🗡️ GARDE D'ÉLITE (4 mana, 3/3)
Humain Warrior
"Jumelé ET Formation 1/1"
"Sacrifice Coordonné (2) : Détruit un monstre ennemi ciblé"
```

### Synergies Inter-Classes
- **Human Warrior** : Créatures Jumelées qui forment la masse
- **Human Mage** : Buffs d'Escouade, sacrifices pour invocations
- **Human Rogue** : Sacrifices pour des dégâts directs ou effets sournois

---

## 💡 Proposition 4 : "HÉRITAGE & TRADITION"

### Concept Principal
Les Humains ont une **mémoire collective** - ils deviennent plus forts en fonction des alliés tombés au combat.

### Nouvelles Mécaniques

#### **1. En Mémoire (condition)**
**Description :** "Pour chaque allié mort ce tour..."
- Les effets s'amplifient selon les pertes
- Transforme les sacrifices en puissance
- Exemple : "En Mémoire : +1/+1 par allié mort"

#### **2. Vétéran (nouveau keyword)**
**Description :** "Gagne +1/+1 permanent après avoir tué un ennemi"
- Progression permanente par le combat
- Crée des "carry units" qui grandissent
- Très thématique pour Warrior

#### **3. Héritage (nouveau trigger)**
**Description :** "Quand cet allié meurt, transmettez ses stats/effets à un autre"
- Exemple : "Héritage : Un allié aléatoire gagne mes stats actuelles"
- La force ne disparaît jamais, elle se transmet
- Mécanisme très tactique

### Exemples de Cartes

```
🗡️ RECRUE VAILLANTE (1 mana, 1/1)
Humain Warrior
"Héritage : +1/+1 à un allié aléatoire"

⚔️ VÉTÉRAN DE GUERRE (3 mana, 2/3)
Humain Warrior
"Vétéran (ce monstre gagne +1/+1 permanent après avoir tué un ennemi)"

👑 COMMANDANT HÉROÏQUE (5 mana, 3/5)
Humain Warrior
"Tous vos alliés ont Vétéran"
"En Mémoire : Tous les alliés gagnent +1/+1"

✨ ORACLE DES ANCÊTRES (4 mana, 2/4)
Humain Mage
"Héritage : Invoque une copie de moi"
"Apparition : Pioche 1 carte par allié mort ce tour"

📜 RITUEL COMMÉMORATIF (3 mana)
Sort Humain
"Ressuscitez un allié mort ce tour avec +2/+2"
"En Mémoire : À la place, ressuscitez tous les alliés morts ce tour"

🗡️ CHAMPION IMMORTEL (6 mana, 5/5)
Humain Warrior
"Vétéran ET Héritage (mes stats à tous les alliés)"
"En Mémoire : Gagne +2/+2 et Téméraire"
```

### Synergies Inter-Classes
- **Human Warrior** : Vétérans qui grandissent au combat
- **Human Mage** : Rituels de résurrection et mémoire
- **Human Rogue** : Héritages d'agilité et esquive

---

## 🎲 Synergies Croisées Race/Classe

### Human + Warrior
- **Thème** : Armée disciplinée, formations militaires
- **Mécanique phare** : Formation, Ligne de Front, Vétéran
- **Exemple** : "Légion Royale" - cartes qui deviennent plus fortes en groupe et survivent au combat

### Human + Mage
- **Thème** : Stratégie militaire, tactique magique
- **Mécanique phare** : Escouade magique, buffs de zone, redéploiement
- **Exemple** : "Mages de Guerre" - invoquent des illusions et boostent les formations

### Human + Rogue
- **Thème** : Infiltration, sabotage, attaques coordonnées
- **Mécanique phare** : Flanc, Jumelé (duo d'assassins), Sacrifice Coordonné
- **Exemple** : "Lames Jumelles" - duo d'assassins qui combinent leurs attaques

---

## 🔄 Compatibilité avec les Autres Races

### Synergie avec COMMON
- Les cartes communes (recrues basiques) alimentent les mécaniques de nombre
- Parfait pour les decks swarm Human

### Contre-jeu avec DRAGON
- Dragons excellent seuls (Solitaire) vs Humains en groupe
- Match-up thématique : Qualité vs Quantité

### Contre-jeu avec DEMON
- Démons ont des clears de zone → punissent le swarm humain
- Humains ont la résilience (Héritage, En Mémoire) pour reconstruire

---

## 📊 Équilibrage Suggéré

### Points Forts
- Excel en mid-game (quand le terrain se remplit)
- Très bonne synergie entre cartes
- Résilient face à la removal ciblée

### Points Faibles
- Faible en early game (cartes individuellement faibles)
- Vulnérable aux clears de zone (AoE)
- Dépend du positionnement (sensible au displacement)

### Courbe de Puissance
```
Early (tours 1-3) : ★★☆☆☆ (Développement)
Mid (tours 4-6)   : ★★★★★ (Pic de puissance)
Late (tours 7+)   : ★★★☆☆ (Dépend du board control)
```

---

## 🎨 Variantes et Extensions Futures

### Sous-Factions Possibles
1. **Royaume du Nord** : Focus sur Formation et défense
2. **Empire du Sud** : Focus sur Nombre Critique et offensive
3. **Confédération de l'Est** : Focus sur Escouade et synergie

### Nouveaux Archétypes
- **Token Swarm** : Masse de petites créatures
- **Formation Tactique** : Positionnement optimal
- **Armée Vétérane** : Croissance à long terme

