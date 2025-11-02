import { useNavigate } from "@tanstack/react-router";
import { useCallback, useEffect, useState } from "react";
import { getGameInfo } from "../service/game.service";
import type { ActionType, IAction } from "../types/action";
import type { IGameState, IGameUpdate } from "../types/game";
import {
	type AnimationState,
	computeAnimationState,
	getAnimationDuration,
} from "./animationEngine";
import { applyAction } from "./gameStateReducer";

const animationBefore: ActionType[] = ["Destroy", "Win"];

export const useGameEngine = (gameId: string) => {
	const [gameState, setGameState] = useState<IGameState>();
	const [finalGameState, setFinalGameState] = useState<IGameState>();
	const [actionQueue, setActionQueue] = useState<IAction[]>([]);
	const [isAnimating, setIsAnimating] = useState(false);
	const [animationMap, setAnimationMap] = useState<Map<number, AnimationState>>(
		new Map(),
	);
	const navigate = useNavigate({ from: "game/$gameId/" });

	useEffect(() => {
		if (!(gameState?.winnerId === undefined || gameState?.winnerId === null)) {
			navigate({ to: "/" });
		}
	}, [gameState?.winnerId, navigate]);

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
				setAnimationMap(computeAnimationState(group, intermediateState));

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
