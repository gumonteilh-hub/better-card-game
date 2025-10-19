import type { IGameUpdate } from "./types/game";
import type { Faction, ICardTemplate, IDeck } from "./types/template";

export const getCollection = async (
	faction: Faction,
): Promise<ICardTemplate[]> => {
	const response = await fetch(`api/collection/${faction}`, {
		method: "GET",
		headers: {
			"Content-Type": "application/json",
		},
	});
	const json = await response.json();
	return json as ICardTemplate[];
};

export const startGame = async (deck: IDeck): Promise<IGameUpdate> => {
	const body = {
		faction: deck.faction,
		cards: deck.cards.map((c) => c.id),
	};
	const response = await fetch("api/game/start", {
		method: "POST",
		headers: {
			"Content-Type": "application/json",
		},
		body: JSON.stringify(body),
	});
	const json = await response.json();
	return json as IGameUpdate;
};

export const playCard = async (
	gameId: string,
	cardId: number,
	position: number,
): Promise<IGameUpdate> => {
	try {
		const response = await fetch(
			`api/game/${gameId}/play_card/${cardId}/${position}`,
			{
				method: "POST",
				headers: {
					"Content-Type": "application/json",
				},
			},
		);
		const json = await response.json();
		return json as IGameUpdate;
	} catch (error) {
		return Promise.reject(error);
	}
};

export const endTurn = async (gameId: string): Promise<IGameUpdate> => {
	try {
		const response = await fetch(`api/game/${gameId}/end_turn`, {
			method: "POST",
			headers: {
				"Content-Type": "application/json",
			},
		});
		const json = await response.json();
		return json as IGameUpdate;
	} catch (error) {
		return Promise.reject(error);
	}
};

export const attack = async (
	gameId: string,
	cardId: number,
	targetId: number,
): Promise<IGameUpdate> => {
	try {
		const response = await fetch(
			`api/game/${gameId}/attack/${cardId}/${targetId}`,
			{
				method: "POST",
				headers: {
					"Content-Type": "application/json",
				},
			},
		);
		const json = await response.json();
		return json as IGameUpdate;
	} catch (error) {
		return Promise.reject(error);
	}
};
