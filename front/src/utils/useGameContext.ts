import { createContext, useContext } from "react";
import type { IGameState, IGameUpdate } from "../types/game";
import type { IInputMode } from "./GameContextProvider";
import type { AnimationState } from "./useGameEngine";

interface IGameContext {
	gameState: IGameState;
	selectedCard?: number;
	isAnimating: boolean;
	updateGameState: (newState: IGameUpdate) => void;
	handleTargetSelect: (cardId: number | string) => void;
	handleMoveSelect: (pos: number) => void;
	playableCards: number[];
	handleSelectCard: (cardId: number) => void;
	canAttackPlayer: boolean;
	animationMap: Map<number, AnimationState>;
	inputMode: IInputMode;
	handleSetInputMode: (inputMode: IInputMode) => void;
	moveTargets: number[];
}

export const GameContext = createContext<IGameContext | null>(null);

export const useGameContext = () => {
	const gameContext = useContext(GameContext);
	if (!gameContext) {
		throw new Error("gameContext should not be null");
	}

	return gameContext;
};
