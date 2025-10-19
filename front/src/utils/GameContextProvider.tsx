import {
	createContext,
	type JSX,
	useCallback,
	useContext,
	useMemo,
	useState,
} from "react";
import { attack } from "../game.service";
import type { IGameState, IGameUpdate } from "../types/game";
import { useGameEngine } from "./useGameEngine";

interface IGameContext {
	gameState: IGameState;
	selectedAttackingCard?: number;
	isAnimating: boolean;
	updateGameState: (newState: IGameUpdate) => void;
	handleTargetSelect: (cardId: number) => void;
	playableCards: number[];
	handleAttackStart: (cardId: number) => void;
	handleUnselectAttackingCard: () => void;
}

const GameContext = createContext<IGameContext | null>(null);

export const GameContextProvider = ({
	children,
}: {
	children: JSX.Element;
}) => {
	const { isAnimating, gameState, updateGameState } = useGameEngine();
	const [selectedAttackingCard, setSelectedAttackingCard] = useState<number>();

	const playableCards = useMemo(() => {
		if (!gameState || isAnimating) return [];

		return gameState.player.hand
			.filter((card) => card.template.cost <= gameState.player.currentMana)
			.map((c) => c.id);
	}, [gameState, isAnimating]);

	const handleAttackStart = useCallback(
		(cardId: number) => {
			if (!gameState || isAnimating) return;
			setSelectedAttackingCard(cardId);
		},
		[gameState, isAnimating],
	);

	const handleUnselectAttackingCard = useCallback(() => {
		setSelectedAttackingCard(undefined);
	}, []);

	const handleTargetSelect = useCallback(
		(cardId: number) => {
			if (!gameState || isAnimating || !selectedAttackingCard) return;

			attack(gameState.gameId, selectedAttackingCard, cardId).then((res) => {
				updateGameState(res);
			});
			setSelectedAttackingCard(undefined);
		},
		[gameState, isAnimating, selectedAttackingCard, updateGameState],
	);

	if (!gameState) {
		return <>Loading</>;
	}

	return (
		<GameContext
			value={{
				gameState,
				isAnimating,
				selectedAttackingCard,
				updateGameState,
				handleTargetSelect,
				playableCards,
				handleAttackStart,
				handleUnselectAttackingCard,
			}}
		>
			{children}
		</GameContext>
	);
};

export const useGameContext = () => {
	const gameContext = useContext(GameContext);
	if (!gameContext) {
		throw new Error("gameContext should not be null");
	}

	return gameContext;
};
