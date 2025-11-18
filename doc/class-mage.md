# Classe MAGE - Design Final

## 🎯 Identité Thématique

Les Mages représentent **la magie et l'accumulation de pouvoir arcanique**.

**Principe de design clé** : Les Mages **accumulent Arcane** puis **dépensent pour booster sorts**, mais génèrent différemment selon leur race :
- **Human Mage** : Génère Arcane via **regroupement** (on_surrounded)
- **Dragon Mage** : Génère Arcane via **solitude** (on_alone) + conversion mana
- **Demon Mage** : Génère Arcane via **sacrifice** HP/alliés

---

## 👥 HUMAN MAGE : LA CONFRÉRIE

### Identité
Groupe de mages qui **génèrent Arcane collectivement** quand entourés.

### Mécaniques Clés
- **Génération collective** : Plus d'alliés → Plus d'Arcane
- **On Surrounded** : Génère Arcane bonus
- **Sorts de zone** : AoE et buffs de groupe
- **Surcharge** : Sorts boostés avec Arcane

### Courbe
Accumulation collective ↗️ puis burst - Mid-game fort

---

## 🐉 DRAGON MAGE : LE SORCIER SOLITAIRE

### Identité
Dragon qui **médite seul** pour Arcane massive et convertit mana → Arcane.

### Mécaniques Clés
- **Méditation solitaire** : Génère Arcane quand seul (on_alone)
- **Conversion Mana → Arcane** : Transforme ressources
- **Génération de mana** : Accumule puis convertit
- **Sorts massifs** : Single target dévastateur

### Courbe
Accumulation lente puis burst explosif ↗️↗️ - Late-game apocalyptique

---

## 😈 DEMON MAGE : LE NÉCROMANCIEN

### Identité
Sorcier qui **sacrifie HP/alliés** pour Arcane et sorts surpuissants.

### Mécaniques Clés
- **Rituel** : Sacrifie alliés → Arcane
- **Prix du Sang** : Sacrifie HP → Arcane
- **Renaissance** : Ressuscite les sacrifiés
- **Sorts nécromantiques** : Invocations, drains

### Courbe
Génère tokens → Sacrifie → Burst ↗️ - Mid-game rituel

---

## 📊 Trinité Mages - Comparaison

| | **Human** | **Dragon** | **Demon** |
|---|---|---|---|
| **Génère via** | Regroupement | Solitude + Mana | Sacrifice |
| **Style** | Collectif | Méditation | Rituel |
| **Sorts** | AoE/Buffs groupe | Massifs single | Nécromancie |
| **Timing** | Mid-game | Late-game | Variable |

---

## 🎯 Objectif de Jeu

**Human Mage** : Remplir board → Génération Arcane collective → Sorts AoE massifs
**Dragon Mage** : Seul + Accumule mana → Convertit en Arcane → Sort qui finit partie
**Demon Mage** : Génère tokens → Sacrifie pour Arcane → Invocations/Resurrections massives
