import { useCallback, useEffect, useState } from "react";
import { getGameInfo } from "../game.service";
import type { ActionType, IAction } from "../types/action";
import type { IGameState, IGameUpdate } from "../types/game";
import { getAnimationDuration } from "./cardVariants";
import { applyAction } from "./stateReducer";

const animationBefore: ActionType[] = ["Destroy", "Win"];

export const useGameEngine = (gameId: string) => {
	const [gameState, setGameState] = useState<IGameState>();
	const [finalGameState, setFinalGameState] = useState<IGameState>();
	const [actionQueue, setActionQueue] = useState<IAction[]>([]);
	const [isAnimating, setIsAnimating] = useState(false);
	const [animationMap, setAnimationMap] = useState<Map<number, AnimationState>>(
		new Map(),
	);

	useEffect(() => {
		getGameInfo(gameId).then((res) => {
			setGameState(res);
			setFinalGameState(res);
		});
	}, [gameId]);

	const updateGameState = useCallback((newState: IGameUpdate) => {
		setActionQueue(newState.actions);
		setFinalGameState(newState.gameView);
	}, []);

	useEffect(() => {
		if (gameState && !isAnimating) {
			if (actionQueue.length > 0) {
				setIsAnimating(true);
				const processedActions: number[] = [0];
				const currentType = actionQueue[0].type;
				const group: IAction[] = [actionQueue[0]];

				let i = 1;
				while (i < actionQueue.length && actionQueue[i].type === currentType) {
					processedActions.push(i);
					group.push(actionQueue[i]);
					i++;
				}

				let intermediateState = { ...gameState };
				for (const action of group) {
					intermediateState = applyAction(intermediateState, action);
				}

				const isAnimationBefore = animationBefore.includes(currentType);

				if (!isAnimationBefore) {
					setGameState(intermediateState);
				}
				setActionQueue(
					actionQueue.filter((_, i) => !processedActions.includes(i)),
				);
				setAnimationMap(computeAnimationState(group));

				const animationDuration = getAnimationDuration(currentType);

				setTimeout(() => {
					if (isAnimationBefore) {
						setGameState(intermediateState);
					}
					setAnimationMap(new Map());
					setIsAnimating(false);
				}, animationDuration);
			} else {
				setGameState(finalGameState);
			}
		}
	}, [actionQueue, gameState, isAnimating, finalGameState]);

	return { isAnimating, gameState, updateGameState, animationMap };
};

export type AnimationState =
	| "summoned"
	| "attacking"
	| "attacked"
	| "healed"
	| "dying"
	| "damaged"
	| "boosted"
	| "drawed";

const computeAnimationState = (actions: IAction[]) => {
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
				animationMap.set(action.value.initiator, "attacking");
				animationMap.set(action.value.target, "attacked");
				break;
			}
			case "Boost": {
				animationMap.set(action.value.target, "boosted");
			}
		}
	}

	return animationMap;
};
