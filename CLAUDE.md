# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a strategic card game with positioning mechanics as a central feature. The game includes three factions (Human, Dragon, Demon), each with unique gameplay mechanics. The project is a full-stack application with a Rust backend and React frontend.

## Architecture

### Monorepo Structure

- **`back/`**: Rust backend (Axum web server)
- **`front/`**: React frontend (Vite + TypeScript + TanStack Router)

### Backend Architecture (Rust)

The backend is organized into several key modules:

- **`game/`**: Core game logic
  - `mod.rs`: Main Game struct with public API methods (`play_monster`, `play_spell`, `attack`, `move_card`, `next_turn`)
  - `types.rs`: Core types (`Location`, `Event`, `InstanceId`, `PlayerId`)
  - `card.rs`: Card instances and keywords
  - `effects.rs`: Effect system for card abilities
  - `logic.rs`: Effect execution logic
  - `action.rs`: Actions sent to frontend for UI updates
  - `view.rs`: Public game state serialization (hides private info like opponent's hand)
  - `events.rs`: Event management system
  - `player.rs`: Player state management

- **`collection/`**: Card definitions and deck management
  - `types.rs`: Card templates, effects, and targets
  - `common.rs`, `demon.rs`, `dragon.rs`, `human.rs`: Faction-specific card collections
  - `mod.rs`: Collection access and IA deck generation

- **`ia/`**: AI opponent logic
  - `summon.rs`: AI summoning strategy
  - `attack.rs`: AI attack strategy

- **`main.rs`**: Axum HTTP server with REST API endpoints
  - State management using `HashMap<Uuid, Game>` with Tokio Mutex
  - API routes: `/collection/{faction}`, `/start`, `/game/{game_id}/*`

- **`lib.rs`**: Public library API exposing game functions
  - Wraps game logic and returns `GameViewResponse` (actions + public game state)

### Backend Key Concepts

**Effect Queue System**: The game uses an effect queue (`VecDeque<Effect>`) that processes game actions. When a player action occurs, effects are queued and then resolved via `compute_commands()`, which executes all effects and returns `Action`s for the frontend to animate.

**Game State**:
- Entities (cards) are stored in a `HashMap<InstanceId, CardInstance>`
- Players are tracked in `HashMap<PlayerId, PlayerInstance>`
- Board positions: 8 slots (0-7) with specific attack/defense properties
  - Attack positions: [0, 2, 3, 5, 6]
  - Defense positions: [1, 2, 4, 5, 7]
  - Hybrid positions (2, 5): both attack and defense

**Location System**: Cards move through locations: `Deck → Hand → Field(position) → Graveyard`

### Frontend Architecture (React + TypeScript)

- **`routes/`**: TanStack Router pages
  - `index.tsx`: Home/menu page
  - `collection.tsx`: Deck builder
  - `game/`: Game view components

- **`components/`**: React components for UI elements

- **`game.service.ts`**: API client for backend communication

- **`types/`**: TypeScript type definitions matching backend serialization

- **`utils/`**: Utility functions including API fetch wrapper

- **Vite proxy**: Proxies `/api` requests to `http://localhost:9999` (backend)

## Development Commands

### Backend (Rust)

```bash
# Run the backend server (from project root or back/)
cd back && cargo run

# Build the backend
cd back && cargo build

# Run tests
cd back && cargo test

# The server runs on http://localhost:9999
```

### Frontend (React)

```bash
# Run the development server (from project root or front/)
cd front && bun run dev

# Build for production
cd front && bun run build

# Lint and format code
cd front && bun run lint

# Preview production build
cd front && bun run preview

# The dev server runs on http://localhost:5173 (default Vite port)
# API requests to /api are proxied to the backend
```

### Full Stack Development

To develop both frontend and backend:
1. Start backend: `cd back && cargo run`
2. Start frontend in another terminal: `cd front && bun run dev`
3. Open browser to frontend URL (typically http://localhost:5173)

## Game Rules Reference

See `GAME_RULES.md` for complete game mechanics. Key concepts:

- **Board**: 8 positions with attack/defense properties
- **Turn phases**: Draw phase → Play phase → End phase
- **Card keywords**: Téméraire (rush), Entouré (surrounded), Solitaire (alone), Agile, Démoniaque, etc.
- **Factions**: Human (swarm + Entouré), Dragon (powerful + Solitaire), Demon (fear mechanic)

## Code Patterns

### Adding New Card Effects

1. Define template effect in `back/src/collection/types.rs` (`TemplateEffect` enum)
2. Add conversion logic in `convert_to_effect()` function
3. Define runtime effect in `back/src/game/effects.rs` (`Effect` enum)
4. Implement execution in `back/src/game/logic.rs` (`execute_effect()`)
5. Add corresponding action in `back/src/game/action.rs` if UI needs to animate it
6. Add the corresponding action in front in `front/src/types/action.ts`
7. Mirror runtime effect in front in `front/src/utils/stateReducer.ts`
8. If there is a needed animation, map the action to an animation in `front/src/utils/useGameEngine.ts`
9. Add the css animation from framer-motion in `front/src/utils/cardVariants.ts`

### Adding New Cards

Add cards to the appropriate faction file in `back/src/collection/`:
- `common.rs`: Neutral cards
- `human.rs`: Human faction
- `dragon.rs`: Dragon faction
- `demon.rs`: Demon faction

### API Error Handling

Backend uses `Result<T, Error>` with custom error types in `back/src/error.rs`. The main.rs converts these to HTTP responses. Frontend `apiFetch()` utility handles errors and throws for non-200 responses.

## Testing Notes

### Running Tests

- Backend: Use `cargo test` to run Rust unit tests from the `back/` directory

### Functional Testing Methodology

For detailed methodology on creating functional tests for game mechanics, see `.claude/testing-methodology.md`

**Quick summary**: Follow a 4-step process:
1. Create a test file for the mechanic
2. Document functional rules (get user validation before proceeding)
3. Create isolated tests for each rule
4. Structure each test: Initialize → Modify state → Test as user → Assert results

## Important Implementation Details

- **Turn flow**: Player actions queue effects → `compute_commands()` executes all effects → AI plays (if AI turn) → frontend receives actions to animate
- **Card instances** have unique IDs; multiple copies of same card template get different instance IDs
- **Asleep mechanic**: Monsters can't attack on their first turn unless they have Téméraire (rush) keyword
- **Movement system**: Players get 3 movement points per turn; monsters can move to linked adjacent positions
