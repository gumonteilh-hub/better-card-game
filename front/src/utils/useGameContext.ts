import { createContext, useContext } from "react";
import type { IGameState, IGameUpdate } from "../types/game";

interface IGameContext {
	gameState: IGameState;
	selectedAttackingCard?: number;
	isAnimating: boolean;
	updateGameState: (newState: IGameUpdate) => void;
	handleTargetSelect: (cardId: number | string) => void;
	playableCards: number[];
	handleAttackStart: (cardId: number) => void;
	handleUnselectAttackingCard: () => void;
	canAttackPlayer: boolean;
}

export const GameContext = createContext<IGameContext | null>(null);

export const useGameContext = () => {
	const gameContext = useContext(GameContext);
	if (!gameContext) {
		throw new Error("gameContext should not be null");
	}

	return gameContext;
};
