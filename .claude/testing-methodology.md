# Functional Testing Methodology

This document defines the systematic approach for creating functional tests for game mechanics.

## Overview

When creating functional tests for game mechanics, follow this systematic approach to ensure comprehensive and maintainable tests.

## Step-by-Step Process

### 1. Create a test file for the mechanic

Create a new test file in `back/src/game/tests/` for the specific mechanic being tested.

**Naming convention**: `test_<mechanic>.rs` (e.g., `test_heal.rs`, `test_attack.rs`, `test_boost.rs`)

### 2. Document the functional rule

Write the functional rule as comments at the top of the test file. This serves as documentation and specification.

**IMPORTANT**: Before proceeding to step 3, ask the user to verify and correct the functional rule. Do not continue writing tests until the user confirms the rule is accurate.

**Example**:
```rust
// FUNCTIONAL RULE: Heal Effect
// - When a card with Heal effect is played, it should restore HP to the target
// - Healing cannot exceed the target's maximum HP
// - Healing a player restores their hero HP
// - Healing a dead/destroyed target has no effect
```

### 3. Create isolated tests for each rule

Break down the functional rule into individual test cases, with each test focusing on one specific aspect of the rule.

**Guidelines**:
- One test per rule component
- Tests should be independent and not rely on execution order
- Use descriptive test names that explain what is being tested

### 4. Structure each test

For each test case, follow this four-part structure:

#### a) Initialize the game state

Initialize with the **minimum cards needed** for the test.

- You may create custom test-only cards with specific effects
- Keep the setup minimal and focused on the mechanic being tested
- Avoid unnecessary complexity in the initial state

**Example**:
```rust
let mut game = create_test_game();
let player_a = game.player_id_a;
```

#### b) Artificially modify the game state

Manipulate the game state to reach the specific scenario you want to test.

- Modify card locations, HP values, mana, positions, etc.
- Set up the exact preconditions for your test case
- This allows testing edge cases that would be hard to reach naturally

**Example**:
```rust
// Put a damaged monster on the field
let monster_id = add_monster_to_field(&mut game, player_a, position_0);
game.entities.get_mut(&monster_id).unwrap().current_hp = 2; // damaged

// Give player enough mana
game.players.get_mut(&player_a).unwrap().mana = 5;
```

#### c) Test the mechanic as a user would

Simulate real user actions using the **public API methods**.

- Use methods like `play_monster()`, `attack()`, `play_spell()`, `move_card()`, etc.
- Do NOT manipulate internal state directly to trigger effects
- Simulate the actual user flow
- Call `compute_commands()` when needed to process the effect queue

**Example**:
```rust
// Test as user would: play a heal spell card
let heal_card = add_heal_card_to_hand(&mut game, player_a);
game.play_spell(heal_card).unwrap();
game.compute_commands().unwrap();
```

#### d) Assert the expected result

Verify that the game state matches expectations.

- Check the primary effect (e.g., HP was restored)
- Verify side effects (e.g., card moved to graveyard, mana consumed)
- Test both success cases and error cases
- Use clear, specific assertions

**Example**:
```rust
// Assert HP was restored
assert_eq!(game.entities.get(&monster_id).unwrap().current_hp, 5);

// Assert card went to graveyard
assert_eq!(game.entities.get(&heal_card).unwrap().location, Location::Graveyard);

// Assert mana was consumed
assert_eq!(game.players.get(&player_a).unwrap().mana, 2);
```

## Complete Example

```rust
// FUNCTIONAL RULE: Heal Effect
// - When a spell with Heal effect is played, it should restore HP to the target
// - Healing cannot exceed the target's maximum HP

#[test]
fn test_heal_restores_hp_to_damaged_monster() {
    // a) Initialize with minimal cards
    let mut game = create_test_game();
    let player_a = game.player_id_a;

    // b) Modify state: put a damaged monster on field
    let monster_id = add_monster_to_field(&mut game, player_a, 0);
    game.entities.get_mut(&monster_id).unwrap().current_hp = 2; // damaged from 5 max HP
    game.players.get_mut(&player_a).unwrap().mana = 3;

    // c) Test as user would: play a heal card
    let heal_card = add_heal_card_to_hand(&mut game, player_a, 3); // heals for 3
    game.play_spell(heal_card).unwrap();
    game.compute_commands().unwrap();

    // d) Assert result
    assert_eq!(game.entities.get(&monster_id).unwrap().current_hp, 5);
}

#[test]
fn test_heal_cannot_exceed_max_hp() {
    // a) Initialize
    let mut game = create_test_game();
    let player_a = game.player_id_a;

    // b) Modify state: full HP monster
    let monster_id = add_monster_to_field(&mut game, player_a, 0);
    // current_hp already at max (5)
    game.players.get_mut(&player_a).unwrap().mana = 3;

    // c) Test: play heal card on full HP monster
    let heal_card = add_heal_card_to_hand(&mut game, player_a, 3);
    game.play_spell(heal_card).unwrap();
    game.compute_commands().unwrap();

    // d) Assert HP stays at max, doesn't exceed
    assert_eq!(game.entities.get(&monster_id).unwrap().current_hp, 5);
    assert_eq!(game.entities.get(&monster_id).unwrap().max_hp, 5);
}
```

## Best Practices

1. **Ask for rule validation**: Always ask the user to verify functional rules before implementing tests
2. **Keep tests focused**: Each test should verify one specific behavior
3. **Use helper functions**: Create reusable helpers for common setup tasks
4. **Test error cases**: Don't just test happy paths, test validation and error conditions
5. **Clear naming**: Use descriptive test function names that explain what is being tested
6. **Minimal setup**: Only create the cards and state necessary for the specific test
7. **Document why**: Add comments explaining why specific setup is needed for non-obvious cases
