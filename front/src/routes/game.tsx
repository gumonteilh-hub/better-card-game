import { DndContext } from "@dnd-kit/core";
import { createFileRoute } from "@tanstack/react-router";
import { useCallback, useMemo, useState } from "react";
import PlayerBoard from "../components/PlayerBoard";
import { attack, endTurn, playCard } from "../game.service";
import { useGameEngine } from "../utils/useGameEngine";

export const Route = createFileRoute("/game")({
	component: RouteComponent,
});

function RouteComponent() {
	const { isAnimating, gameState, updateGameState } = useGameEngine();
	const [selectedAttackingCard, setSelectedAttackingCard] = useState<number>();

	const playableCards = useMemo(() => {
		if (!gameState || isAnimating) return [];

		return gameState.player.hand
			.filter((card) => card.template.cost <= gameState.player.currentMana)
			.map((c) => c.id);
	}, [gameState, isAnimating]);

	// biome-ignore lint/suspicious/noExplicitAny: <type inherited from dnd-kit>
	const handleDragEnd = (event: any) => {
		if (isAnimating || !gameState) return;
		const { active, over } = event;
		if (over && active) {
			playCard(
				gameState.gameId,
				active.data.current.id,
				over.data.current.position,
			).then((res) => {
				updateGameState(res);
			});
		}
	};

	function handleEndTurn(): void {
		if (isAnimating || !gameState) return;

		setSelectedAttackingCard(undefined);
		endTurn(gameState?.gameId).then((res) => {
			updateGameState(res);
		});
	}

	const handleAttackStart = useCallback(
		(cardId: number) => {
			if (!gameState || isAnimating) return;
			setSelectedAttackingCard(cardId);
		},
		[gameState, isAnimating],
	);

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
		<div className="main">
			<DndContext autoScroll={false} onDragEnd={handleDragEnd}>
				<PlayerBoard
					handleCardInteract={
						selectedAttackingCard ? handleTargetSelect : undefined
					}
					side="enemy"
					secredCard={gameState.enemy.secretCard}
					field={gameState.enemy.field}
					maxMana={gameState.enemy.maxMana}
					currentMana={gameState.enemy.currentMana}
					hand={gameState.enemy.hand}
					hero={gameState.enemy.hero}
				></PlayerBoard>
				<div className="middle-part">
					<button onClick={handleEndTurn} type="button">
						end turn
					</button>
				</div>
				<PlayerBoard
					handleCardInteract={
						!selectedAttackingCard ? handleAttackStart : undefined
					}
					playableCards={playableCards}
					side="player"
					secredCard={gameState.player.secretCard}
					field={gameState.player.field}
					maxMana={gameState.player.maxMana}
					currentMana={gameState.player.currentMana}
					hand={gameState.player.hand}
					hero={gameState.player.hero}
				></PlayerBoard>
			</DndContext>
		</div>
	);
}
