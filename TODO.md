# TODO - Game Design Implementation

## 📋 HOOKS (Triggers)

### ✅ Existants
- `on_play`, `on_attack`, `on_death`, `on_alone`, `on_surrounded`

### ❌ À implémenter Priority 1
- `on_defense` / `on_attacked` - Quand subit une attaque
- `on_damage` - Quand subit dégâts (Dragon Warrior rage, Demon Warrior sursaut)
- `on_kill` - Quand tue une créature (Dragon Rogue vol stats, Vétéran)
- `at_end_of_turn` - Fin de tour (Demon Warrior dépérissement)
- `at_start_of_turn` - Début de tour (génération ressources)

---

## 📦 EFFETS

### ✅ Existants
- `Boost`, `MakeDraw`, `Heal`, `Destroy`, `DealDamage`, `Attack`, `Summon`, `IncreaseMaxMana`, `RefreshMana`

### ❌ À implémenter Priority 1 (effets atomiques)
- `DecreaseMana` - Réduit mana
- `GenerateArcane` - Génère Arcane (Mages)
- `SpendArcane` - Dépense Arcane pour effet amplifié
- `StealCard` - Pioche deck adverse (Human Rogue)
- `ChangeOwner` - Change propriétaire créature (Demon Rogue mind control)
- `SummonFromGraveyard` - Invoque du cimetière (Demon Mage Renaissance)
- Modifier `Boost` pour accepter valeurs négatives (Demon Warrior dépérissement)

### ✅ Effets composables (gratuits - combo d'atomiques)
- `StealMana` = DecreaseMana(enemy) + RefreshMana(self)
- `ConvertManaToArcane` = DecreaseMana(self) + GenerateArcane
- `DrainLife` = DealDamage + Heal
- `Sacrifice` = Destroy(chosen ally)
- `StealStats` = Boost negatif(enemy) + Boost(self)
- `GenerateMana` = IncreaseMaxMana + RefreshMana

### ❌ Priority 2
- `Evolve` - Dragon (dragonet → mature)

---

## 🎯 CIBLAGE (Target)

### ✅ Existants
- `EnnemyPlayer`, `Player`, `BothPlayers`, `ItSelf`, `Allies`, `Ennemies`, `AllMonsters`, `All`, `Choose`, `Matching`, `And`, `Or`

### ❌ À implémenter Priority 1
- `AdjacentAllies` - Alliés adjacents (Human Warrior Formation)
- `Random` - Cible aléatoire
- `InPool`:
    - Location
    - Side
    - Class
    - Race
    - Keyword

---

## 🔋 RESSOURCES

### ✅ Existant
- Mana, HP, Cartes (deck/hand/graveyard)

### ❌ À implémenter Priority 1
- **Arcane** - Ressource Mage (ajouter au PlayerInstance + display front)

---

## 🐛 BUGS
- Card miniature overview pop when trying to move to the left
- Windfury broken in front

---

## 🚀 FEATURES FUTURES
- Register/Login
- Collection management
- Card rarities
