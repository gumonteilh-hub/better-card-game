# Classe WARRIOR - Propositions de Game Design

## 🎯 Identité Thématique

Les Warriors représentent **le combat au corps-à-corps, l'endurance, la protection et la montée en puissance**. Ils excellent dans les combats prolongés, accumulent de l'armure, et deviennent plus forts à travers l'adversité.

**Note importante** : Ces mécaniques doivent fonctionner pour **Human Warrior, Dragon Warrior ET Demon Warrior** !

---

## 💡 Proposition 1 : "ARMURE ET ENDURANCE"

### Concept Principal
Les Warriors accumulent de l'**Armure** qui bloque les dégâts, et deviennent plus forts plus ils restent en jeu.

### Nouvelles Mécaniques

#### **1. Armure (nouveau compteur)**
**Description :** Compteur qui absorbe les dégâts avant les HP
- Exemple : Monstre avec 5 HP et 3 Armure → prend 4 dégâts → perd 3 Armure et 1 HP
- Certains Warriors génèrent de l'Armure à chaque tour
- L'Armure peut être partagée ou transférée
- Visuel : bouclier doré sur la créature

#### **2. Fortification (gain d'armure)**
**Description :** "Gagne X Armure"
- Exemple : "Début de tour : Fortification 2"
- Différentes sources : tours passés, attaques bloquées, alliés, etc.
- Synergie : certaines cartes ont "Si vous avez 5+ Armure..."

#### **3. Mur de Chair (protection d'équipe)**
**Description :** "Tant que ce monstre est en défense, les alliés adjacents ont +X Armure"
- Crée des "tanks" qui protègent l'équipe
- Encourage le positionnement tactique
- Synergie avec les positions de défense du board

#### **4. Robustesse (anti-removal)**
**Description :** "Ne peut pas être détruit si vous avez X+ Armure"
- Exemple : "Robustesse (5) : Tant que vous avez 5+ Armure, ne peut pas être détruit"
- Rend les tanks très difficiles à éliminer
- Encourage l'accumulation d'Armure

### Exemples de Cartes

```
⚔️ GARDE VAILLANT (2 mana, 2/4)
[Race] Warrior
"Fortification 1 (gagne 1 Armure par tour)"
"Mur de Chair : Les alliés adjacents ont +1 Armure"

⚔️ CHEVALIER CUIRASSÉ (4 mana, 3/5)
[Race] Warrior
"Apparition : Fortification 3"
"Robustesse (5)"
"Si vous avez 10+ Armure : +3/+3 et Téméraire"

⚔️ FORTERESSE VIVANTE (6 mana, 4/8)
[Race] Warrior
"Fortification 2 par tour"
"Mur de Chair : +3 Armure aux alliés adjacents"
"Toutes les Armures que vous avez sont doublées"

📜 BASTION (3 mana)
Sort [Race] Warrior
"Tous vos alliés gagnent Fortification 2"
"Si vous avez 15+ Armure : Vos alliés ne peuvent pas être détruits ce tour"

⚔️ IMMORTEL BLINDÉ (7 mana, 5/10)
[Race] Warrior
"Apparition : Fortification 10"
"Robustesse (5) ET Régénération (2 HP par tour)"
"Peut attaquer avec son Armure (dégâts = Armure actuelle)"

⚔️ TITAN D'ACIER (8 mana, 6/6)
[Race] Warrior
"Gagne +1/+1 permanent par Armure que vous avez"
"Quand vous prenez des dégâts : Gagnez cette quantité en Armure"
"Robustesse (15)"
```

### Synergies Race-Spécifiques
- **Human Warrior** : Armure partagée dans les formations
- **Dragon Warrior** : Armure massive sur un seul dragon
- **Demon Warrior** : Armure sacrifiée pour des effets

---

## 💡 Proposition 2 : "MONTÉE EN PUISSANCE"

### Concept Principal
Les Warriors **grandissent au combat**. Plus ils survivent et combattent, plus ils deviennent puissants.

### Nouvelles Mécaniques

#### **1. Vétéran (déjà mentionné dans race-human)**
**Description :** "Gagne +1/+1 permanent après avoir tué un ennemi"
- Fonctionne pour toutes les races
- Crée des "snowball units"
- Récompense l'agressivité et la survie

#### **2. Bataille Prolongée (compteur de tours)**
**Description :** "Pour chaque tour où ce monstre a survécu..."
- Exemple : "Bataille Prolongée : +1/+1 par tour survécu (max 5)"
- Récompense de garder le tank en vie
- Visuel : compteur de tours de présence

#### **3. Cicatrices de Guerre (mémoire de dégâts)**
**Description :** "Pour chaque X dégâts subis (lifetime), gagne un bonus permanent"
- Exemple : "Cicatrices de Guerre : +1/+0 permanent par 5 dégâts subis"
- Transforme les dégâts reçus en force
- Très thématique pour les guerriers battle-hardened

#### **4. Second Souffle (résilience)**
**Description :** "Quand ce monstre tombe sous 50% HP, effet bonus"
- Exemple : "Second Souffle : Gagne +3/+3 et Téméraire"
- Crée des moments épiques de comeback
- Une seule fois par monstre

### Exemples de Cartes

```
⚔️ RECRUE ASPIRANTE (1 mana, 1/3)
[Race] Warrior
"Vétéran (gagne +1/+1 permanent après chaque kill)"

⚔️ VÉTÉRAN AGUERRI (3 mana, 2/3)
[Race] Warrior
"Vétéran"
"Bataille Prolongée : +1/+1 par tour survécu"
"Second Souffle : +2/+2 et Fortification 5"

⚔️ SURVIVANT LÉGENDAIRE (5 mana, 3/6)
[Race] Warrior
"Cicatrices de Guerre : +1/+0 permanent par 3 dégâts subis"
"Bataille Prolongée : +2/+2 par tour survécu (pas de limite)"

⚔️ CHAMPION IMMORTEL (6 mana, 4/5)
[Race] Warrior
"Vétéran"
"Second Souffle : Soigne complètement et gagne +5/+5 (permanent)"
"Cicatrices de Guerre : Chaque cicatrice donne aussi +1 Armure"

📜 ENTRAÎNEMENT SPARTIATE (2 mana)
Sort [Race] Warrior
"Infligez 5 dégâts à tous vos alliés"
"Tous vos Warriors survivants gagnent Vétéran et +2/+2 (permanent)"

⚔️ MAÎTRE DE GUERRE (7 mana, 5/7)
[Race] Warrior
"Apparition : Tous vos Warriors gagnent Vétéran"
"Bataille Prolongée : Tous les Warriors gagnent +1/+1 par tour que CE monstre a survécu"
"Si cette carte a 5+ Cicatrices : Tous les Warriors ont Windfury"
```

### Synergies Race-Spécifiques
- **Human Warrior** : Vétérans en masse (armée qui grandit)
- **Dragon Warrior** : Un dragon qui devient légendaire avec le temps
- **Demon Warrior** : Cicatrices transformées en puissance démoniaque

---

## 💡 Proposition 3 : "RIPOSTE ET CONTRE-ATTAQUE"

### Concept Principal
Les Warriors **punissent ceux qui les attaquent**. Ils ont des effets défensifs qui retournent les dégâts ou activent lors des attaques reçues.

### Nouvelles Mécaniques

#### **1. Riposte (trigger défensif)**
**Description :** "Quand ce monstre est attaqué, avant les dégâts..."
- Exemple : "Riposte : Inflige 3 dégâts à l'attaquant"
- Peut réduire ou annuler l'attaque
- Différent de "Défense" (trigger après dégâts)

#### **2. Épines (dégâts de retour passifs)**
**Description :** "Quand ce monstre subit des dégâts, inflige X dégâts à la source"
- Exemple : "Épines (3) : Renvoie 3 dégâts"
- Passif permanent, fonctionne sur toutes sources
- Encourage l'adversaire à éviter d'attaquer

#### **3. Posture Défensive (stance)**
**Description :** "Mode spécial : Ne peut pas attaquer, mais effets défensifs doublés"
- Exemple : "Posture Défensive : Ne peut pas attaquer. Riposte et Épines doublés. +0/+5"
- Peut être toggle on/off
- Crée des choix tactiques

#### **4. Mur Infranchissable (blocage obligatoire)**
**Description :** "Les ennemis doivent attaquer ce monstre en priorité"
- Version Warrior du Taunt
- Exemple : "Tant que ce monstre est en défense, les ennemis ne peuvent attaquer que lui"
- Force l'adversaire à focus le tank

### Exemples de Cartes

```
⚔️ GARDE DU CORPS (2 mana, 1/5)
[Race] Warrior
"Mur Infranchissable"
"Riposte : Inflige 2 dégâts à l'attaquant"

⚔️ CHEVALIER D'ÉPINES (3 mana, 3/4)
[Race] Warrior
"Épines (2)"
"Fortification 1 par tour"

⚔️ DÉFENSEUR ULTIME (5 mana, 2/8)
[Race] Warrior
"Posture Défensive (toggle)"
"Riposte : Inflige dégâts = à sa défense actuelle"
"Mur Infranchissable"

⚔️ MAÎTRE ÉPÉISTE (4 mana, 4/3)
[Race] Warrior
"Riposte : Attaque l'attaquant (combat simultané)"
"Vétéran"

📜 FORMATION DÉFENSIVE (3 mana)
Sort [Race] Warrior
"Tous vos Warriors gagnent Épines (2) et Mur Infranchissable ce tour"

⚔️ COLOSSE VENGEUR (7 mana, 5/9)
[Race] Warrior
"Épines (5)"
"Riposte : +3/+3 permanent"
"Mur Infranchissable"
"Pour chaque attaque reçue : Gagne +2 Armure"
```

### Synergies Race-Spécifiques
- **Human Warrior** : Murs humains protégeant les alliés
- **Dragon Warrior** : Dragon impossible à tuer (riposte massive)
- **Demon Warrior** : Épines démoniaques qui corrompent l'attaquant

---

## 💡 Proposition 4 : "ARMES ET ÉQUIPEMENT"

### Concept Principal
Les Warriors utilisent des **Armes** (équipements) qui leur donnent des bonus et peuvent être améliorés.

### Nouvelles Mécaniques

#### **1. Arme (équipement attaché)**
**Description :** Certains Warriors invoquent/équipent des Armes
- Arme = objet attaché visuellement au monstre
- Donne des bonus de stats et effets
- Peut être transférée, améliorée, détruite
- Exemples : Épée (+2 ATK), Bouclier (+3 DEF), Hache (Windfury)

#### **2. Forger (création/amélioration d'arme)**
**Description :** "Créez ou améliorez une Arme"
- Exemple : "Apparition : Forgez une Épée Longue (+3 ATK)"
- Certaines cartes améliorent les armes existantes
- Thématique artisan/forgeron

#### **3. Maîtrise d'Arme (bonus si équipé)**
**Description :** "Si ce monstre possède une Arme..."
- Exemple : "Maîtrise d'Arme : +2/+2 et Vétéran"
- Encourage à équiper les Warriors
- Différents types de maîtrise (épée, hache, lance, etc.)

#### **4. Arme Brisée (sacrifice d'arme)**
**Description :** "Détruisez une Arme pour un effet puissant"
- Exemple : "Brisez votre Arme : Inflige X dégâts (X = bonus ATK de l'arme)"
- Risk/reward : sacrifier l'équipement pour un burst
- Thématique "lame brisée héroïque"

### Exemples de Cartes

```
⚔️ GUERRIER NOVICE (2 mana, 2/3)
[Race] Warrior
"Apparition : Forgez une Épée Courte (+1 ATK)"
"Maîtrise d'Arme : +1/+1"

⚔️ MAÎTRE FORGERON (3 mana, 2/4)
[Race] Warrior
"Apparition : Forgez une Arme de votre choix (Épée, Hache, Lance, Bouclier)"
"Début de tour : Améliorez toutes vos Armes (+1 bonus)"

⚔️ BRISE-FER (4 mana, 4/3)
[Race] Warrior
"Attaque : Détruisez l'Arme de la cible (si elle en a)"
"Quand vous brisez une Arme : Gagne ses bonus (permanent)"

⚔️ CHAMPION ARMÉ (5 mana, 3/5)
[Race] Warrior
"Apparition : Forgez une Arme Légendaire (+3 ATK, +2 DEF, Vétéran)"
"Maîtrise d'Arme : Double tous les bonus de l'Arme"

📜 LAME DU DESTIN (4 mana)
Sort [Race] Warrior
"Brisez une Arme : Détruisez tous les ennemis avec ATK inférieure au bonus de l'Arme"

⚔️ SEIGNEUR DE GUERRE (7 mana, 6/6)
[Race] Warrior
"Apparition : Forgez 3 Armes différentes et distribuez-les à vos alliés"
"Tous vos Warriors avec Arme ont +3/+3 et Vétéran"
"Maîtrise d'Arme Ultime : Vos Armes ne peuvent pas être détruites"

📜 FORGE ANCESTRALE (6 mana)
Sort [Race] Warrior
"Fusionnez toutes vos Armes en une Arme Ultime"
"Arme Ultime = somme de tous les bonus + Immunité + Vétéran + Windfury"
```

### Armes Exemples
```
⚔️ ÉPÉE COURTE (+1 ATK)
⚔️ ÉPÉE LONGUE (+3 ATK)
🪓 HACHE DE GUERRE (+2 ATK, peut attaquer 2 fois)
🛡️ BOUCLIER LOURD (+0 ATK, +3 DEF, +3 Armure)
🗡️ LAME DOUBLE (+2 ATK, Windfury)
🔨 MARTEAU DE TITAN (+5 ATK, Riposte (3))
```

### Synergies Race-Spécifiques
- **Human Warrior** : Armement de masse (armée équipée)
- **Dragon Warrior** : Armes légendaires uniques (Excalibur style)
- **Demon Warrior** : Armes maudites (Arme Brisée = Pacte)

---

## 🎲 Synergies Croisées Classe/Race

### Human + Warrior
- **Thème** : Armée militaire disciplinée
- **Mécaniques combinées** : Formation + Armure partagée, Vétérans en masse, Murs de Chair multiples
- **Exemple** : Phalange de guerriers qui grandissent ensemble et se protègent

### Dragon + Warrior
- **Thème** : Dragon-chevalier indestructible
- **Mécaniques combinées** : Solitaire + Armure massive, Vétéran + Thésaurisation, Arme Légendaire unique
- **Exemple** : Un dragon avec 50 Armure, une épée légendaire, qui grandit à chaque combat

### Demon + Warrior
- **Thème** : Berserker maudit
- **Mécaniques combinées** : Pacte + Cicatrices de Guerre, Armure sacrifiée pour Renaissance, Armes maudites
- **Exemple** : Guerrier qui sacrifie son Armure pour des Rituels, revient plus fort

---

## 🔄 Compatibilité avec les Autres Classes

### Warrior vs Mage
- Warrior = contact, endurance → Mage = distance, burst
- Warrior résiste aux sorts (Armure, Robustesse)
- Match-up : Tank vs Artillerie

### Warrior vs Rogue
- Warrior = lent, tanky → Rogue = rapide, fragile
- Warrior counter via Épines et Riposte
- Match-up : Forteresse vs Assassin

### Warrior + Warrior (même classe)
- Synergie forte : partagent Armure, Armes, buffs
- Armée de tanks ou équipe coordonnée

---

## 📊 Équilibrage Suggéré

### Points Forts
- Extrêmement résilient (Armure, HP élevés)
- Scale bien dans le temps (Vétéran, Bataille Prolongée)
- Excellente défense et contrôle du board

### Points Faibles
- Lent à setup (besoin de temps pour accumuler)
- Faible burst damage initial
- Vulnérable aux effets qui ignorent l'Armure (Destroy, Possession)

### Courbe de Puissance
```
Early (tours 1-3) : ★★☆☆☆ (Setup, accumulation)
Mid (tours 4-6)   : ★★★★☆ (Peak tank)
Late (tours 7+)   : ★★★★★ (Unkillable, stats massives)
```

---

## 🎨 Variantes et Extensions Futures

### Sous-Archétypes
1. **Tank Fortress** : Focus Armure et défense
2. **Vétéran Army** : Focus growth et snowball
3. **Weapon Master** : Focus Armes et équipement
4. **Counter-Strike** : Focus Riposte et punish

### Nouvelles Mécaniques Futures
- **Bannière de Guerre** : Buff d'équipe persistant
- **Cri de Guerre** : AOE buff lors de l'invocation
- **Duels** : Combats 1v1 forcés
- **Régénération** : Heal over time pour Warriors
- **Enragé** : Bonus quand bas HP (inverse de Second Souffle)

