import type { ActionType, IAction } from "../types/action";
import type { IGameState } from "../types/game";

export const getAnimationDuration = (actionType: ActionType): number => {
	switch (actionType) {
		case "Destroy":
			return 700;
		case "Heal":
		case "ReceiveDamage":
			return 400;
		case "Summon":
			return 500;
		case "Attack":
			return 500;
		case "Boost":
			return 300;
		case "Win":
			return 2000;
		case "TriggerOnDeath":
		case "TriggerOnPlay":
		case "TriggerOnAttack":
		case "TriggerOnSurrounded":
		case "TriggerOnAlone":
		case "TriggerOnKill":
		case "TriggerOnTurnStart":
		case "TriggerOnTurnEnd":
		case "TriggerOnDamaged":
		case "TriggerOnDefend":
		case "IncreaseAbsoluteMaxMana":
			return 600;
		case "IncreaseMaxMana":
		case "RefreshMana":
		case "BurnCard":
		case "StartTurn":
		case "EndTurn":
		case "UpdateGameView":
		case "EnemyDraw":
		case "Draw":
			return 0;
	}
};

export type AnimationState =
	| "summoned"
	| "attacking"
	| "enemyAttacking"
	| "enemyAttacked"
	| "attacked"
	| "healed"
	| "dying"
	| "damaged"
	| "boosted"
	| "winned"
	| "drawed"
	| "triggerOnDeath"
	| "triggerOnPlay"
	| "triggerOnAlone"
	| "triggerOnSurrounded"
	| "triggerOnKill"
	| "triggerOnTurnEnd"
	| "TriggerOnTurnStart"
	| "TriggerOnDamaged"
	| "triggerOnDefend"
	| "triggerOnAttack";

export const computeAnimationState = (
	actions: IAction[],
	intermediateState: IGameState,
) => {
	const animationMap = new Map<number, AnimationState>();

	for (const action of actions) {
		switch (action.type) {
			case "Draw": {
				animationMap.set(action.value.card.id, "drawed");
				break;
			}
			case "Heal": {
				animationMap.set(action.value.target, "healed");
				break;
			}
			case "Destroy": {
				animationMap.set(action.value.target, "dying");
				break;
			}
			case "ReceiveDamage": {
				animationMap.set(action.value.target, "damaged");
				break;
			}
			case "Summon": {
				animationMap.set(action.value.target.id, "summoned");
				break;
			}
			case "Attack": {
				if (
					Object.values(intermediateState.player.field).some(
						(c) => c.id === action.value.initiator,
					)
				) {
					animationMap.set(action.value.initiator, "attacking");
					animationMap.set(action.value.target, "enemyAttacked");
				} else {
					animationMap.set(action.value.initiator, "enemyAttacking");
					animationMap.set(action.value.target, "attacked");
				}
				break;
			}
			case "Boost": {
				animationMap.set(action.value.target, "boosted");
				break;
			}
			case "Win": {
				animationMap.set(action.value, "winned");
				break;
			}
			case "TriggerOnDeath": {
				animationMap.set(action.value, "triggerOnDeath");
				break;
			}
			case "TriggerOnPlay": {
				animationMap.set(action.value, "triggerOnPlay");
				break;
			}
			case "TriggerOnAttack": {
				animationMap.set(action.value, "triggerOnAttack");
				break;
			}
			case "TriggerOnAlone": {
				animationMap.set(action.value, "triggerOnAlone");
				break;
			}
			case "TriggerOnSurrounded": {
				animationMap.set(action.value, "triggerOnSurrounded");
				break;
			}
			case "TriggerOnKill": {
				animationMap.set(action.value, "triggerOnKill");
				break;
			}
			case "TriggerOnTurnEnd": {
				animationMap.set(action.value, "triggerOnTurnEnd");
				break;
			}
			case "TriggerOnTurnStart": {
				animationMap.set(action.value, "TriggerOnTurnStart");
				break;
			}
			case "TriggerOnDamaged": {
				animationMap.set(action.value, "TriggerOnDamaged");
				break;
			}
			case "TriggerOnDefend": {
				animationMap.set(action.value, "triggerOnDefend");
				break;
			}
			case "IncreaseAbsoluteMaxMana":
			case "BurnCard":
			case "RefreshMana":
			case "IncreaseMaxMana":
			case "StartTurn":
			case "UpdateGameView":
			case "EnemyDraw":
			case "EndTurn":
		}
	}

	return animationMap;
};
