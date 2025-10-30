import {
	type ClientRect,
	type Collision,
	DndContext,
	type DroppableContainer,
	pointerWithin,
	rectIntersection,
} from "@dnd-kit/core";
import type { Active, RectMap } from "@dnd-kit/core/dist/store";
import type { Coordinates } from "@dnd-kit/core/dist/types";
import { createFileRoute } from "@tanstack/react-router";
import PlayerBoard from "../../components/PlayerBoard";
import { endTurn, playMonster, playSpell } from "../../game.service";
import { GameContextProvider } from "../../utils/GameContextProvider";
import { useGameContext } from "../../utils/useGameContext";

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
		if (
			over &&
			active &&
			over.data.current.accepts.includes(active.data.current.type)
		) {
			if (active.data.current.type === "monster") {
				playMonster(
					gameState.gameId,
					active.data.current.id,
					over.data.current.position,
				).then((res) => {
					updateGameState(res);
				});
			}
			if (active.data.current.type === "spell") {
				playSpell(gameState.gameId, active.data.current.id).then((res) => {
					updateGameState(res);
				});
			}
		}
	};

	function handleEndTurn(): void {
		if (isAnimating || !gameState) return;

		endTurn(gameState?.gameId).then((res) => {
			updateGameState(res);
		});
	}

	function collisionHandler(args: {
		active: Active;
		collisionRect: ClientRect;
		droppableRects: RectMap;
		droppableContainers: DroppableContainer[];
		pointerCoordinates: Coordinates | null;
	}): Collision[] {
		const isSpell = args.active.data.current?.type === "spell";

		if (isSpell) {
			const intersectingDroppables = pointerWithin(args);
			const gameBoard = intersectingDroppables.find(
				(droppable) => droppable.id === "field-player",
			);

			return gameBoard ? [gameBoard] : [];
		}

		return rectIntersection(args);
	}

	return (
		<div className="main">
			<DndContext
				autoScroll={false}
				onDragEnd={handleDragEnd}
				collisionDetection={collisionHandler}
			>
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
