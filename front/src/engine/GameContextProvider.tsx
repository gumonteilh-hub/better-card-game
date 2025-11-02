import { type JSX, useCallback, useMemo, useState } from "react";
import { useGameEngine } from "../engine/gameEngine";
import { attack, move } from "../service/game.service";
import { GameContext } from "../types/gameContext.type";

export const attackPositions = [0, 2, 3, 5, 6];
export const defensePositions = [1, 2, 4, 5, 7];

export type IInputMode = "attack" | "move";

const linkedPositions = [
	[1, 2],
	[0, 2],
	[0, 1, 3, 4],
	[2, 4, 5],
	[2, 3, 5],
	[3, 4, 6, 7],
	[5, 7],
	[5, 6],
];

export const GameContextProvider = ({
	gameId,
	children,
}: {
	gameId: string;
	children: JSX.Element;
}) => {
	const { isAnimating, gameState, updateGameState, animationMap } =
		useGameEngine(gameId);
	const [selectedCard, setSelectedCard] = useState<number>();
	const [inputMode, setInputMode] = useState<IInputMode>("attack");

	const canAttackPlayer = useMemo(() => {
		if (!gameState || !selectedCard || isAnimating || inputMode !== "attack")
			return false;

		for (const pos of defensePositions) {
			if (gameState.enemy.field[pos] !== undefined) {
				return false;
			}
		}

		return true;
	}, [gameState, selectedCard, isAnimating, inputMode]);

	const moveTargets = useMemo(() => {
		if (!gameState || isAnimating || inputMode !== "move" || !selectedCard)
			return [];

		const card = Object.entries(gameState?.player.field).find(
			([_, value]) => value.id === selectedCard,
		);
		if (!card) return [];

		const [startingPosition, _] = card;

		return linkedPositions[parseInt(startingPosition, 10)];
	}, [gameState, inputMode, isAnimating, selectedCard]);

	const playableCards = useMemo(() => {
		if (!gameState || isAnimating) return [];

		return gameState.player.hand
			.filter((card) => card.cost <= gameState.player.currentMana)
			.map((c) => c.id);
	}, [gameState, isAnimating]);

	const handleSelectCard = useCallback(
		(cardId: number) => {
			if (!gameState || isAnimating) return;
			if (cardId === selectedCard) {
				setSelectedCard(undefined);
			} else {
				setSelectedCard(cardId);
			}
		},
		[gameState, isAnimating, selectedCard],
	);

	const handleMoveSelect = useCallback(
		(pos: number) => {
			if (!gameState || isAnimating || !selectedCard) return;

			setSelectedCard(undefined);
			move(gameState.gameId, selectedCard, pos).then((res) => {
				updateGameState(res);
			});
		},
		[gameState, isAnimating, selectedCard, updateGameState],
	);

	const handleTargetSelect = useCallback(
		(cardId: number | string) => {
			if (!gameState || isAnimating || !selectedCard) return;

			setSelectedCard(undefined);
			attack(gameState.gameId, selectedCard, cardId).then((res) => {
				updateGameState(res);
			});
		},
		[gameState, isAnimating, selectedCard, updateGameState],
	);

	const handleSetInputMode = useCallback((inputMode: IInputMode) => {
		setSelectedCard(undefined);
		setInputMode(inputMode);
	}, []);

	if (!gameState) {
		return <>Loading</>;
	}

	return (
		<GameContext
			value={{
				gameState,
				isAnimating,
				selectedCard,
				updateGameState,
				handleTargetSelect,
				handleMoveSelect,
				playableCards,
				handleSelectCard,
				canAttackPlayer,
				animationMap,
				inputMode,
				handleSetInputMode,
				moveTargets,
			}}
		>
			{children}
		</GameContext>
	);
};
