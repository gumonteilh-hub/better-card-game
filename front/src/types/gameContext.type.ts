import { createContext } from "react";
import type { AnimationState } from "../engine/animationEngine";
import type { IInputMode } from "../engine/GameContextProvider";
import type { IGameState } from "./game";

export interface IGameContext {
	gameState: IGameState;
	selectedCard?: number;
	isAnimating: boolean;
	endTurn: () => void;
	playSpell: (cardId: number) => void;
	playMonster: (cardId: number, position: number) => void;
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
