import { useCallback, useEffect, useState } from "react";
import { startGame } from "../game.service";
import type { IAction } from "../types/action";
import type { IGameState, IGameUpdate } from "../types/game";
import { useUserInfo } from "./useUserInfo";

export const useGameEngine = () => {
	const { userInfos } = useUserInfo();
	const [gameState, setGameState] = useState<IGameState>();
	const [actionQueue, setActionQueue] = useState<IAction[]>([]);
	const [isAnimating, setIsAnimating] = useState(false);

	const updateGameState = useCallback((newState: IGameUpdate) => {
		setActionQueue(newState.actions);
		setGameState(newState.gameView);
	}, []);

	useEffect(() => {
		if (gameState && actionQueue.length > 0 && !isAnimating) {
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

			const intermediateState = { ...gameState };
			/*
			for (const action of group) {
				intermediateState = applyAction(intermediateState, action);
			}
			*/

			setGameState(intermediateState);
			setActionQueue(
				actionQueue.filter((_, i) => !processedActions.includes(i)),
			);

			setIsAnimating(false);
			/*const animationDuration = getAnimationDuration(currentType);

			setTimeout(() => {
			}, animationDuration);
			*/
		}
	}, [actionQueue, gameState, isAnimating]);

	useEffect(() => {
		if (userInfos?.deck) {
			startGame(userInfos.deck).then((res) => {
				updateGameState(res);
			});
		}
	}, [userInfos, updateGameState]);

	return { isAnimating, gameState, updateGameState };
};
/*
const getAnimationDuration = (actionType: ActionType): number => {
	switch (actionType) {
		case "BurnCard":
			return 700;
		case "Draw":
			return 600;
		case "Heal":
			return 500;
		/*		case ActionType.DESTROY: return 900;
		case ActionType.RECEIVE_DAMAGE: return 400;
		case ActionType.SUMMON: return 700;
		case ActionType.ATTACK: return 500;
		case ActionType.WIN: return 1500;
		case ActionType.TRIGGER_ON_DEATH:
		case ActionType.TRIGGER_ON_PLAY:
		case ActionType.TRIGGER_ON_ATTACK:
			return 800;
		default:
			return 500;
	}
};

const animationBefore: ActionType[] = ["Destroy", "Win"];
*/
