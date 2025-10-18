import { DndContext } from "@dnd-kit/core";
import { createFileRoute } from "@tanstack/react-router";
import { useEffect } from "react";
import PlayerBoard from "../components/PlayerBoard";

export const Route = createFileRoute("/game")({
	component: RouteComponent,
});

function RouteComponent() {
	useEffect(() => {
		fetch("api/start", {
			method: "POST",
			headers: {
				"Content-Type": "application/json",
			},
			body: JSON.stringify({
				faction: "HUMAN",
				cards: [],
			}),
		})
			.then((res) => {
				res.json().then((data) => {
					console.log(data);
				});
			})
			.catch((err) => {
				console.log(err);
			});
	}, []);

	return (
		<div className="main">
			<DndContext autoScroll={false}>
				<PlayerBoard side="enemy"></PlayerBoard>
				<div className="middle-part">
					<button type="button">end turn</button>
				</div>
				<PlayerBoard side="player"></PlayerBoard>
			</DndContext>
		</div>
	);
}
