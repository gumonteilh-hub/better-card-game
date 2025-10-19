# Plan d'Implémentation : Animations Framer Motion avec Variants

**Date:** 2025-01-19
**Architecture:** Game Engine → État d'animation → Cartes gèrent leurs animations
**Approche:** Variants déclaratifs (Best Practice 2025)

---

## 📋 Table des Matières

1. [Vue d'ensemble](#vue-densemble)
2. [Étape 1 : Installation et Setup](#étape-1--installation-et-setup)
3. [Étape 2 : Architecture du système d'animation](#étape-2--architecture-du-système-danimation)
4. [Étape 3 : Définition des Variants](#étape-3--définition-des-variants)
5. [Étape 4 : Intégration dans le Game Engine](#étape-4--intégration-dans-le-game-engine)
6. [Étape 5 : Mise à jour des composants Card](#étape-5--mise-à-jour-des-composants-card)
7. [Étape 6 : Gestion des animations complexes](#étape-6--gestion-des-animations-complexes)
8. [Étape 7 : Optimisations de performance](#étape-7--optimisations-de-performance)
9. [Bonnes pratiques 2025](#bonnes-pratiques-2025)
10. [Points d'attention](#points-dattention)

---

## Vue d'ensemble

### Principe de fonctionnement

```
Action Queue → Game Engine → État Animation → Card Variants → Animation CSS/JS
     ↓              ↓               ↓                ↓                ↓
  [Attack]    Détecte action   {cardId: state}   "attacking"    Transform/Rotate
```

### Avantages de cette approche

✅ **Déclaratif** - Les cartes définissent leurs propres animations
✅ **Réutilisable** - Les variants peuvent être partagés
✅ **Performant** - Framer Motion optimise automatiquement
✅ **Maintenable** - Séparation claire des responsabilités
✅ **Testable** - État d'animation séparé de la logique

---

## Étape 1 : Installation et Setup

### 1.1 Installation

```bash
npm install framer-motion
```

### 1.2 Vérification de version

Framer Motion v11+ (2025) apporte des améliorations de performance pour les animations de layout et de grandes quantités d'éléments.

```bash
npm list framer-motion
```

### 1.3 Configuration TypeScript (si nécessaire)

Ajouter dans `tsconfig.json` si tu rencontres des problèmes de types :

```json
{
  "compilerOptions": {
    "skipLibCheck": true
  }
}
```

---

## Étape 2 : Architecture du système d'animation

### 2.1 Créer le type d'état d'animation

**Fichier:** `front/src/types/animation.ts`

```typescript
import type { IAction } from './action';

/**
 * États d'animation possibles pour une entité (carte, héros, etc.)
 */
export type AnimationState =
  | 'idle'           // Repos, pas d'animation
  | 'entering'       // Apparition (summon, draw)
  | 'attacking'      // Animation d'attaque
  | 'takingDamage'   // Réception de dégâts
  | 'healing'        // Soin
  | 'dying'          // Destruction
  | 'buffed'         // Buff temporaire
  | 'exhausted';     // Épuisé (après attaque)

/**
 * Map des états d'animation par ID d'entité
 */
export type AnimationStateMap = {
  [entityId: string]: {
    state: AnimationState;
    timestamp: number; // Pour éviter les doublons
  };
};

/**
 * Configuration d'animation pour chaque type d'action
 */
export interface AnimationConfig {
  duration: number;           // Durée en ms
  state: AnimationState;      // État à appliquer
  affectsTarget?: boolean;    // Si l'animation concerne la cible
  affectsInitiator?: boolean; // Si l'animation concerne l'initiateur
}
```

### 2.2 Créer le mapper Action → Animation State

**Fichier:** `front/src/utils/animationMapper.ts`

```typescript
import type { IAction } from '../types/action';
import type { AnimationConfig, AnimationState } from '../types/animation';

/**
 * Map chaque type d'action vers sa configuration d'animation
 */
export const ACTION_ANIMATION_CONFIG: Record<IAction['type'], AnimationConfig> = {
  'Attack': {
    duration: 800,
    state: 'attacking',
    affectsInitiator: true,
  },
  'ReceiveDamage': {
    duration: 500,
    state: 'takingDamage',
    affectsTarget: true,
  },
  'Summon': {
    duration: 700,
    state: 'entering',
    affectsTarget: true,
  },
  'Draw': {
    duration: 600,
    state: 'entering',
    affectsTarget: true,
  },
  'Heal': {
    duration: 500,
    state: 'healing',
    affectsTarget: true,
  },
  'Destroy': {
    duration: 900,
    state: 'dying',
    affectsTarget: true,
  },
  'BurnCard': {
    duration: 700,
    state: 'dying',
    affectsTarget: true,
  },
  'Win': {
    duration: 1500,
    state: 'idle',
  },
  'TriggerOnDeath': {
    duration: 800,
    state: 'idle',
  },
  'TriggerOnPlay': {
    duration: 800,
    state: 'idle',
  },
  'TriggerOnAttack': {
    duration: 800,
    state: 'idle',
  },
  'IncreaseMaxMana': {
    duration: 400,
    state: 'idle',
  },
  'RefreshMana': {
    duration: 400,
    state: 'idle',
  },
};

/**
 * Extrait les IDs d'entités affectées par une action
 */
export const getAffectedEntities = (action: IAction): string[] => {
  const config = ACTION_ANIMATION_CONFIG[action.type];
  const entities: string[] = [];

  switch (action.type) {
    case 'Attack':
      if (config.affectsInitiator) {
        entities.push(String(action.value.initiator));
      }
      if (config.affectsTarget) {
        entities.push(String(action.value.target));
      }
      break;

    case 'ReceiveDamage':
    case 'Heal':
    case 'Destroy':
      entities.push(String(action.value.target));
      break;

    case 'Summon':
      entities.push(String(action.value.target.id));
      break;

    case 'Draw':
      entities.push(String(action.value.card.id));
      break;

    // Les autres actions n'affectent pas d'entités visuelles directement
    default:
      break;
  }

  return entities;
};
```

---

## Étape 3 : Définition des Variants

### 3.1 Créer le fichier de variants réutilisables

**Fichier:** `front/src/animations/cardVariants.ts`

```typescript
import type { Variants } from 'framer-motion';

/**
 * Variants pour les animations de cartes
 *
 * Best Practice 2025:
 * - Utiliser des keyframes [] pour les animations complexes
 * - Spécifier les times pour contrôler le timing
 * - Utiliser ease pour des courbes d'animation naturelles
 */
export const cardVariants: Variants = {
  // État au repos
  idle: {
    x: 0,
    y: 0,
    rotate: 0,
    scale: 1,
    opacity: 1,
    filter: 'brightness(1) saturate(1)',
    transition: {
      duration: 0.3,
      ease: 'easeOut',
    },
  },

  // Apparition (summon, draw)
  entering: {
    scale: [0, 1.1, 1],
    rotate: [0, 5, 0],
    opacity: [0, 1],
    y: [50, -10, 0],
    transition: {
      duration: 0.7,
      times: [0, 0.6, 1],
      ease: 'easeOut',
    },
  },

  // Animation d'attaque
  attacking: {
    x: [0, 80, 0],
    y: [0, -30, 0],
    rotate: [0, -20, 0],
    scale: [1, 1.15, 1],
    transition: {
      duration: 0.8,
      times: [0, 0.5, 1],
      ease: [0.43, 0.13, 0.23, 0.96], // Custom easing
    },
  },

  // Réception de dégâts
  takingDamage: {
    x: [0, -12, 12, -10, 10, -6, 6, 0],
    rotate: [0, -3, 3, -2, 2, 0],
    filter: [
      'brightness(1)',
      'brightness(1.5)',
      'brightness(0.8)',
      'brightness(1)',
    ],
    transition: {
      duration: 0.5,
      ease: 'easeOut',
    },
  },

  // Soin
  healing: {
    y: [0, -15, -10, 0],
    scale: [1, 1.08, 1.05, 1],
    filter: [
      'brightness(1) saturate(1)',
      'brightness(1.3) saturate(1.2)',
      'brightness(1.1) saturate(1.1)',
      'brightness(1) saturate(1)',
    ],
    transition: {
      duration: 0.5,
      ease: 'easeOut',
    },
  },

  // Destruction
  dying: {
    scale: [1, 1.2, 0],
    rotate: [0, 10, 180],
    opacity: [1, 1, 0],
    y: [0, -20, 30],
    transition: {
      duration: 0.9,
      times: [0, 0.3, 1],
      ease: 'easeIn',
    },
  },

  // Buff temporaire
  buffed: {
    filter: [
      'brightness(1)',
      'brightness(1.4)',
      'brightness(1.2)',
      'brightness(1)',
    ],
    scale: [1, 1.05, 1],
    transition: {
      duration: 0.4,
      ease: 'easeOut',
    },
  },

  // Épuisé (après attaque)
  exhausted: {
    opacity: [1, 0.6, 0.7],
    filter: 'grayscale(0.3)',
    transition: {
      duration: 0.3,
    },
  },
};

/**
 * Variants pour les overlays de damage/heal
 */
export const damageOverlayVariants: Variants = {
  idle: {
    opacity: 0,
    backgroundColor: 'rgba(255, 0, 0, 0)',
  },
  takingDamage: {
    opacity: [0, 0.6, 0],
    backgroundColor: [
      'rgba(255, 0, 0, 0)',
      'rgba(255, 0, 0, 0.6)',
      'rgba(255, 0, 0, 0)',
    ],
    transition: {
      duration: 0.3,
    },
  },
};

export const healOverlayVariants: Variants = {
  idle: {
    opacity: 0,
  },
  healing: {
    opacity: [0, 0.5, 0],
    backgroundColor: [
      'rgba(0, 255, 100, 0)',
      'rgba(0, 255, 100, 0.4)',
      'rgba(0, 255, 100, 0)',
    ],
    transition: {
      duration: 0.4,
    },
  },
};
```

### 3.2 Variants pour les nombres de HP/Attack qui changent

**Fichier:** `front/src/animations/numberVariants.ts`

```typescript
import type { Variants } from 'framer-motion';

/**
 * Variants pour animer les changements de stats
 */
export const statChangeVariants: Variants = {
  idle: {
    scale: 1,
    color: '#ffffff',
  },
  damage: {
    scale: [1, 1.3, 1],
    color: ['#ffffff', '#ff0000', '#ffffff'],
    transition: {
      duration: 0.4,
    },
  },
  heal: {
    scale: [1, 1.3, 1],
    color: ['#ffffff', '#00ff66', '#ffffff'],
    transition: {
      duration: 0.4,
    },
  },
  buff: {
    scale: [1, 1.2, 1],
    color: ['#ffffff', '#ffaa00', '#ffffff'],
    transition: {
      duration: 0.3,
    },
  },
};
```

---

## Étape 4 : Intégration dans le Game Engine

### 4.1 Ajouter le state d'animation dans game.tsx

**Fichier:** `front/src/routes/game.tsx`

```typescript
import { useState, useEffect, useRef } from 'react';
import type { AnimationStateMap } from '../types/animation';
import { ACTION_ANIMATION_CONFIG, getAffectedEntities } from '../utils/animationMapper';

function RouteComponent() {
  const { userInfos } = useUserInfo();
  const [gameState, setGameState] = useState<IGameState>();
  const [actionQueue, setActionQueue] = useState<IAction[]>([]);
  const [currentAction, setCurrentAction] = useState<IAction | null>(null);
  const [isAnimating, setIsAnimating] = useState(false);

  // 🆕 État des animations
  const [animationStates, setAnimationStates] = useState<AnimationStateMap>({});

  const finalState = useRef<IGameState>(null);

  useEffect(() => {
    if (gameState && actionQueue.length > 0 && !isAnimating) {
      setIsAnimating(true);

      const action = actionQueue[0];
      const config = ACTION_ANIMATION_CONFIG[action.type];

      // 🆕 Définir l'action courante
      setCurrentAction(action);

      // 🆕 Mettre à jour les états d'animation des entités affectées
      const affectedEntities = getAffectedEntities(action);
      const timestamp = Date.now();

      setAnimationStates(prev => {
        const newStates = { ...prev };
        for (const entityId of affectedEntities) {
          newStates[entityId] = {
            state: config.state,
            timestamp,
          };
        }
        return newStates;
      });

      // Appliquer l'action au gameState
      // (si l'action doit être appliquée AVANT l'animation)
      const shouldApplyBefore = !['Destroy', 'BurnCard'].includes(action.type);

      if (shouldApplyBefore) {
        const newState = applyAction(gameState, action);
        setGameState(newState);
      }

      // Attendre la fin de l'animation
      setTimeout(() => {
        // Appliquer après si nécessaire
        if (!shouldApplyBefore) {
          const newState = applyAction(gameState, action);
          setGameState(newState);
        }

        // Nettoyer les états d'animation
        setAnimationStates(prev => {
          const newStates = { ...prev };
          for (const entityId of affectedEntities) {
            // Ne nettoie que si c'est toujours la même animation
            if (newStates[entityId]?.timestamp === timestamp) {
              delete newStates[entityId];
            }
          }
          return newStates;
        });

        // Passer à l'action suivante
        setActionQueue(prev => prev.slice(1));
        setCurrentAction(null);
        setIsAnimating(false);
      }, config.duration);
    }
  }, [actionQueue, gameState, isAnimating]);

  // ... reste du code

  return (
    <div className="main">
      <DndContext autoScroll={false}>
        <PlayerBoard
          side="enemy"
          field={gameState.enemy.field}
          animationStates={animationStates} // 🆕 Passer l'état
          {...otherProps}
        />

        <div className="middle-part">
          <button type="button">end turn</button>
        </div>

        <PlayerBoard
          side="player"
          field={gameState.player.field}
          animationStates={animationStates} // 🆕 Passer l'état
          {...otherProps}
        />
      </DndContext>
    </div>
  );
}
```

---

## Étape 5 : Mise à jour des composants Card

### 5.1 Modifier le composant Card

**Fichier:** `front/src/components/Card.tsx`

```typescript
import { motion } from 'framer-motion';
import type { AnimationState, AnimationStateMap } from '../types/animation';
import { cardVariants, damageOverlayVariants, healOverlayVariants } from '../animations/cardVariants';
import { statChangeVariants } from '../animations/numberVariants';

interface CardProps {
  id: string;
  hp: number;
  attack: number;
  cost: number;
  name: string;
  image?: string;
  // ... autres props

  // 🆕 Props d'animation
  animationStates?: AnimationStateMap;
}

export const Card = ({
  id,
  hp,
  attack,
  cost,
  name,
  image,
  animationStates,
  ...otherProps
}: CardProps) => {

  // 🆕 Récupérer l'état d'animation pour cette carte
  const animationState: AnimationState =
    animationStates?.[id]?.state || 'idle';

  // 🆕 Déterminer si les stats ont changé (pour animer les nombres)
  const prevHp = useRef(hp);
  const prevAttack = useRef(attack);
  const [hpAnimState, setHpAnimState] = useState<'idle' | 'damage' | 'heal'>('idle');
  const [attackAnimState, setAttackAnimState] = useState<'idle' | 'buff'>('idle');

  useEffect(() => {
    if (hp < prevHp.current) {
      setHpAnimState('damage');
      setTimeout(() => setHpAnimState('idle'), 400);
    } else if (hp > prevHp.current) {
      setHpAnimState('heal');
      setTimeout(() => setHpAnimState('idle'), 400);
    }
    prevHp.current = hp;
  }, [hp]);

  useEffect(() => {
    if (attack !== prevAttack.current) {
      setAttackAnimState('buff');
      setTimeout(() => setAttackAnimState('idle'), 300);
    }
    prevAttack.current = attack;
  }, [attack]);

  return (
    <motion.div
      className="card"
      variants={cardVariants}
      initial="idle"
      animate={animationState}
      // 🆕 Performance optimization (Best Practice 2025)
      style={{
        willChange: 'transform, opacity, filter',
      }}
      // 🆕 Layout animation pour les réarrangements
      layout
      layoutId={`card-${id}`}
    >
      {/* Overlay pour les effets de dégâts/soins */}
      <motion.div
        className="card-overlay"
        variants={damageOverlayVariants}
        animate={animationState === 'takingDamage' ? 'takingDamage' : 'idle'}
        style={{
          position: 'absolute',
          inset: 0,
          borderRadius: 'inherit',
          pointerEvents: 'none',
        }}
      />

      <motion.div
        className="card-overlay"
        variants={healOverlayVariants}
        animate={animationState === 'healing' ? 'healing' : 'idle'}
        style={{
          position: 'absolute',
          inset: 0,
          borderRadius: 'inherit',
          pointerEvents: 'none',
        }}
      />

      {/* Image de la carte */}
      <div className="card-image">
        {image && <img src={image} alt={name} />}
      </div>

      {/* Stats avec animations */}
      <div className="card-stats">
        <motion.div
          className="card-hp"
          variants={statChangeVariants}
          animate={hpAnimState}
        >
          {hp}
        </motion.div>

        <motion.div
          className="card-attack"
          variants={statChangeVariants}
          animate={attackAnimState}
        >
          {attack}
        </motion.div>

        <div className="card-cost">{cost}</div>
      </div>

      <div className="card-name">{name}</div>
    </motion.div>
  );
};
```

### 5.2 Modifier PlayerBoard pour passer les props

**Fichier:** `front/src/components/PlayerBoard.tsx`

```typescript
import { AnimatePresence } from 'framer-motion';
import type { AnimationStateMap } from '../types/animation';

interface PlayerBoardProps {
  side: 'player' | 'enemy';
  field: ICard[];
  hand: ICard[];
  // ... autres props

  // 🆕
  animationStates?: AnimationStateMap;
}

export default function PlayerBoard({
  side,
  field,
  hand,
  animationStates,
  ...otherProps
}: PlayerBoardProps) {

  return (
    <div className={`player-board ${side}`}>
      {/* Field avec AnimatePresence pour les entrées/sorties */}
      <div className="field">
        <AnimatePresence mode="popLayout">
          {field.map(card => (
            <Card
              key={card.id}
              {...card}
              animationStates={animationStates} // 🆕
            />
          ))}
        </AnimatePresence>
      </div>

      {/* Hand */}
      <div className="hand">
        <AnimatePresence mode="popLayout">
          {hand.map(card => (
            <Card
              key={card.id}
              {...card}
              animationStates={animationStates} // 🆕
            />
          ))}
        </AnimatePresence>
      </div>
    </div>
  );
}
```

---

## Étape 6 : Gestion des animations complexes

### 6.1 Animation d'attaque avec projectile (optionnel)

Pour des animations encore plus avancées, tu peux créer un composant Projectile :

**Fichier:** `front/src/components/Projectile.tsx`

```typescript
import { motion } from 'framer-motion';
import { useEffect, useState } from 'react';

interface ProjectileProps {
  from: { x: number; y: number };
  to: { x: number; y: number };
  onComplete: () => void;
}

export const Projectile = ({ from, to, onComplete }: ProjectileProps) => {
  return (
    <motion.div
      className="projectile"
      initial={{
        x: from.x,
        y: from.y,
        opacity: 1,
        scale: 0.5
      }}
      animate={{
        x: to.x,
        y: to.y,
        opacity: [1, 1, 0],
        scale: [0.5, 1, 0.8]
      }}
      transition={{
        duration: 0.4,
        ease: 'easeOut',
      }}
      onAnimationComplete={onComplete}
      style={{
        position: 'fixed',
        width: '20px',
        height: '20px',
        borderRadius: '50%',
        backgroundColor: '#ff4444',
        boxShadow: '0 0 10px rgba(255, 68, 68, 0.8)',
        pointerEvents: 'none',
        zIndex: 1000,
      }}
    />
  );
};
```

### 6.2 Hook personnalisé pour gérer les projectiles

**Fichier:** `front/src/hooks/useProjectileAnimation.ts`

```typescript
import { useState, useCallback } from 'react';

interface Projectile {
  id: string;
  from: { x: number; y: number };
  to: { x: number; y: number };
}

export const useProjectileAnimation = () => {
  const [projectiles, setProjectiles] = useState<Projectile[]>([]);

  const fireProjectile = useCallback((from: DOMRect, to: DOMRect) => {
    const id = `projectile-${Date.now()}`;

    setProjectiles(prev => [...prev, {
      id,
      from: { x: from.left + from.width / 2, y: from.top + from.height / 2 },
      to: { x: to.left + to.width / 2, y: to.top + to.height / 2 },
    }]);
  }, []);

  const removeProjectile = useCallback((id: string) => {
    setProjectiles(prev => prev.filter(p => p.id !== id));
  }, []);

  return { projectiles, fireProjectile, removeProjectile };
};
```

---

## Étape 7 : Optimisations de performance

### 7.1 Best Practices 2025

#### 1. **willChange** pour les propriétés animées

```typescript
// Dans Card.tsx
<motion.div
  style={{
    // Optimisation : prévenir le navigateur des transformations
    willChange: 'transform, opacity, filter',
  }}
>
```

#### 2. **layout** pour les animations de position automatiques

```typescript
// Anime automatiquement les changements de position dans le DOM
<motion.div layout layoutId={`card-${id}`}>
```

#### 3. **AnimatePresence mode="popLayout"**

```typescript
// Évite les sauts lors des entrées/sorties multiples
<AnimatePresence mode="popLayout">
  {items.map(item => <Item key={item.id} />)}
</AnimatePresence>
```

#### 4. **Utiliser motion values pour éviter les re-renders**

```typescript
import { useMotionValue, useTransform } from 'framer-motion';

const Card = ({ hp }) => {
  const motionHp = useMotionValue(hp);
  const displayHp = useTransform(motionHp, latest => Math.round(latest));

  // Mise à jour sans re-render
  useEffect(() => {
    motionHp.set(hp);
  }, [hp]);
};
```

### 7.2 CSS optimizations

**Fichier:** `front/src/css/Card.css`

```css
.card {
  /* GPU acceleration */
  transform: translateZ(0);
  backface-visibility: hidden;

  /* Prevent layout shift */
  contain: layout style paint;
}

.card-overlay {
  /* Compositing layer */
  will-change: opacity, background-color;
}
```

### 7.3 Reducir les animations pour les préférences utilisateur

```typescript
import { useReducedMotion } from 'framer-motion';

const Card = ({ ...props }) => {
  const shouldReduceMotion = useReducedMotion();

  const variants = shouldReduceMotion
    ? reducedMotionVariants
    : cardVariants;

  return <motion.div variants={variants} />;
};
```

---

## Bonnes pratiques 2025

### ✅ DO

1. **Utiliser variants** pour les animations déclaratives et réutilisables
2. **AnimatePresence** pour toutes les listes qui changent
3. **layout** pour les animations de position automatiques
4. **willChange** pour optimiser les performances
5. **Définir les variants inline ou dans un fichier séparé** selon la taille
6. **useReducedMotion** pour respecter les préférences utilisateur
7. **motion values** pour éviter les re-renders inutiles

### ❌ DON'T

1. ❌ Ne pas animer `width`/`height` directement → utiliser `scale`
2. ❌ Ne pas utiliser `useAnimate` pour des animations simples → variants suffisent
3. ❌ Ne pas oublier `key` dans les listes animées
4. ❌ Ne pas animer trop d'éléments simultanément (>50-100)
5. ❌ Ne pas utiliser `initial={false}` sans raison (empêche les animations d'entrée)

---

## Points d'attention

### 🔍 Testing

1. **Tester sur différents navigateurs** (Chrome, Firefox, Safari)
2. **Tester les performances** avec DevTools (120 FPS idéal)
3. **Tester avec animations désactivées** (préférences système)
4. **Tester avec beaucoup de cartes** (10+ cartes simultanées)

### 🐛 Debugging

```typescript
// Ajouter des logs pour débugger
<motion.div
  animate={animationState}
  onAnimationStart={() => console.log('Animation started', animationState)}
  onAnimationComplete={() => console.log('Animation completed', animationState)}
/>
```

### 📊 Monitoring des performances

```typescript
// Hook pour mesurer les performances
const useAnimationPerformance = () => {
  useEffect(() => {
    const observer = new PerformanceObserver((list) => {
      for (const entry of list.getEntries()) {
        if (entry.duration > 16.67) { // > 60fps
          console.warn('Slow animation detected:', entry);
        }
      }
    });

    observer.observe({ entryTypes: ['measure'] });
    return () => observer.disconnect();
  }, []);
};
```

---

## Migration Step-by-Step

### Phase 1 : Setup (1h)
- [ ] Installer framer-motion
- [ ] Créer les types d'animation
- [ ] Créer le mapper action → animation

### Phase 2 : Variants (2h)
- [ ] Définir les variants de base (idle, entering, dying)
- [ ] Définir les variants d'action (attacking, takingDamage, healing)
- [ ] Créer les variants pour les overlays

### Phase 3 : Game Engine (2h)
- [ ] Ajouter l'état `animationStates` dans game.tsx
- [ ] Modifier le useEffect pour mettre à jour les états d'animation
- [ ] Gérer le cleanup des états après animation

### Phase 4 : Components (3h)
- [ ] Modifier Card.tsx pour utiliser motion
- [ ] Ajouter les overlays d'animation
- [ ] Animer les changements de stats
- [ ] Modifier PlayerBoard pour AnimatePresence

### Phase 5 : Optimisations (2h)
- [ ] Ajouter willChange
- [ ] Optimiser les CSS
- [ ] Tester les performances
- [ ] Ajouter useReducedMotion

### Phase 6 : Polish (2h)
- [ ] Ajuster les timings
- [ ] Ajuster les easing curves
- [ ] Ajouter les particules/effets optionnels
- [ ] Tests finaux

**Temps total estimé : 12h**

---

## Ressources

- [Framer Motion Docs](https://www.framer.com/motion/)
- [Motion Variants Guide](https://motion.dev/docs/react-animation)
- [Performance Best Practices](https://blog.maximeheckel.com/posts/advanced-animation-patterns-with-framer-motion/)

---

**Bon courage pour l'implémentation ! 🚀**
