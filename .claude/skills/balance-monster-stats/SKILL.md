---
name: balance-monster-stats
description: Helper skill for balancing monster card statistics based on cost, faction, and class. Use when creating or reviewing monster cards to ensure stats follow game design guidelines.
---

# Monster Stats Balancing

This skill provides the formula and guidelines for calculating balanced monster statistics based on cost, faction, and class.

## Core Formula

**Base Power Level** = `(cost × 4) - 2`

This represents the total stat budget (attack + health) for a monster of a given cost.

### Examples
- Cost 1 → 2 stats → 1/1
- Cost 2 → 6 stats → 3/3
- Cost 3 → 10 stats → 5/5
- Cost 4 → 14 stats → 7/7
- Cost 5 → 18 stats → 9/9
- Cost 6 → 22 stats → 11/11
- Cost 7 → 26 stats → 13/13

## Stat Distribution by Faction

Apply these distribution patterns to the base power level:

### Human (Balanced)
Equal distribution between attack and health.
- Example: 9/9, 7/7, 5/5

### Demon (Aggressive)
Favor attack over health.
**Imbalance Constraint:** |Attack - Health| ≤ Card Cost
- Cost 3 (10 stats): 6/4 (diff=2 ≤ 3) ✓
- Cost 4 (14 stats): 9/5 (diff=4 ≤ 4) ✓
- Cost 5 (18 stats): 11/7 (diff=4 ≤ 5) ✓

### Dragon (Defensive)
Favor health over attack.
**Imbalance Constraint:** |Attack - Health| ≤ Card Cost
- Cost 3 (10 stats): 4/6 (diff=2 ≤ 3) ✓
- Cost 4 (14 stats): 5/9 (diff=4 ≤ 4) ✓
- Cost 5 (18 stats): 7/11 (diff=4 ≤ 5) ✓

### Common (No specific faction)
Apply a **-1/-1 penalty** (systematic).
- Cost 5 → 18 stats → 9/9 - 1/-1 → 8/8
- Cost 4 → 14 stats → 7/7 - 1/-1 → 6/6
- Distribution: Balanced (like Human but with -1/-1)

## Class Bonus

Add these bonuses AFTER applying faction distribution:

- **Warrior**: +1/+1
- **Rogue**: +2/+0
- **Mage**: +0/+2
- **Common** (no class): +0/+0

## Complete Calculation Process

1. Calculate base power level: `(cost × 4) - 2`
2. Distribute stats according to faction (respecting |ATK - HP| ≤ cost):
   - Human: balanced (e.g., 9/9)
   - Demon: aggressive (e.g., 11/7)
   - Dragon: defensive (e.g., 7/11)
   - Common: balanced with -1/-1 penalty (e.g., 8/8)
3. Add class bonus (Warrior/Rogue/Mage/Common)
4. Adjust for card effects and game design needs

## Important Notes

**These are guidelines, not absolute rules.**

- Stats should be balanced against the card's effects and abilities
- Powerful effects may require stat reduction
- Weak/situational effects may allow stat increases
- Use these formulas as a baseline reference point for consistency
- Game design and balance take priority over strict formula adherence

## Example Calculations

**Human Warrior, Cost 5:**
1. Base: (5 × 4) - 2 = 18 stats
2. Human distribution: 9/9
3. Warrior bonus (+1/+1): **10/10**

**Demon Rogue, Cost 3:**
1. Base: (3 × 4) - 2 = 10 stats
2. Demon distribution: 6/4 (diff=2 ≤ 3)
3. Rogue bonus (+2/+0): **8/4**

**Common (no faction) Mage, Cost 4:**
1. Base: (4 × 4) - 2 = 14 stats
2. Common distribution: 7/7 - 1/-1 = 6/6
3. Mage bonus (+0/+2): **6/8**

**Dragon Warrior, Cost 6:**
1. Base: (6 × 4) - 2 = 22 stats
2. Dragon distribution: 9/13 (diff=4 ≤ 6)
3. Warrior bonus (+1/+1): **10/14**

**Demon (no class), Cost 4:**
1. Base: (4 × 4) - 2 = 14 stats
2. Demon distribution: 9/5 (diff=4 ≤ 4)
3. No class bonus: **9/5**

## When to Deviate

Deviate from these formulas when:
- Card has very powerful effects (reduce stats)
- Card has restrictive conditions (increase stats)
- Card has keywords like Téméraire, Entouré, Solitaire, etc. (evaluate impact)
- Playtesting reveals imbalance
- Game design needs trump mathematical balance
