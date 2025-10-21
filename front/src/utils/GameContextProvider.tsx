import { type JSX, useCallback, useMemo, useState } from "react";
import { attack } from "../game.service";
import { GameContext } from "./useGameContext";
import { useGameEngine } from "./useGameEngine";

const defensePositions = [1, 2, 4, 5, 7];

export const GameContextProvider = ({
	children,
}: {
	children: JSX.Element;
}) => {
	const { isAnimating, gameState, updateGameState } = useGameEngine();
	const [selectedAttackingCard, setSelectedAttackingCard] = useState<number>();

	const canAttackPlayer = useMemo(() => {
		if (!gameState || !selectedAttackingCard || isAnimating) return false;

		for (const pos of defensePositions) {
			if (gameState.enemy.field[pos] !== undefined) {
				return false;
			}
		}

		return true;
	}, [gameState, selectedAttackingCard, isAnimating]);

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
		(cardId: number | string) => {
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
				canAttackPlayer,
			}}
		>
			{children}
		</GameContext>
	);
};
