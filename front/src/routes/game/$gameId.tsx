import { DndContext } from "@dnd-kit/core";
import { createFileRoute } from "@tanstack/react-router";
import PlayerBoard from "../../components/PlayerBoard";
import { endTurn, playCard } from "../../game.service";
import { useGameContext } from "../../utils/useGameContext";
import { GameContextProvider } from "../../utils/GameContextProvider";

export const Route = createFileRoute("/game/$gameId")({
	component: RouteComponent,
});

function RouteComponent() {
	const { gameId } = Route.useParams();
	return (
		<GameContextProvider gameId={gameId}>
			<Game></Game>
		</GameContextProvider>
	);
}

const Game = () => {
	const { gameState, isAnimating, updateGameState } = useGameContext();

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

		endTurn(gameState?.gameId).then((res) => {
			updateGameState(res);
		});
	}

	return (
		<div className="main">
			<DndContext autoScroll={false} onDragEnd={handleDragEnd}>
				<PlayerBoard
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
};
