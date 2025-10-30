import type { IGameState, IGameUpdate } from "./types/game";
import type { Faction, ICardTemplate, IDeck } from "./types/template";
import { apiFetch } from "./utils/api";

export const getCollection = async (
	faction: Faction,
): Promise<ICardTemplate[]> => {
	return apiFetch<ICardTemplate[]>(`/api/collection/${faction}`, {
		method: "GET",
	});
};

export const getGameInfo = (gameId: string) => {
	return apiFetch<IGameState>(`/api/game/${gameId}`, {
		method: "GET",
	});
};

export const startGame = async (deck: IDeck): Promise<string> => {
	const body = {
		faction: deck.faction,
		cards: deck.cards.map((c) => c.id),
	};
	return apiFetch<string>("/api/start", {
		method: "POST",
		body: JSON.stringify(body),
	});
};

export const playMonster = async (
	gameId: string,
	cardId: number,
	position: number,
): Promise<IGameUpdate> => {
	return apiFetch<IGameUpdate>(
		`/api/game/${gameId}/play_monster/${cardId}/${position}`,
		{
			method: "POST",
		},
	);
};

export const playSpell = async (
	gameId: string,
	cardId: number,
): Promise<IGameUpdate> => {
	return apiFetch<IGameUpdate>(
		`/api/game/${gameId}/play_spell/${cardId}`,
		{
			method: "POST",
		},
	);
};

export const endTurn = async (gameId: string): Promise<IGameUpdate> => {
	return apiFetch<IGameUpdate>(`/api/game/${gameId}/end_turn`, {
		method: "POST",
	});
};

export const attack = async (
	gameId: string,
	cardId: number,
	targetId: number | string,
): Promise<IGameUpdate> => {
	return apiFetch<IGameUpdate>(
		`/api/game/${gameId}/attack/${cardId}/${targetId}`,
		{
			method: "POST",
		},
	);
};

export const move = async (
	gameId: string,
	cardId: number,
	targetPosition: number,
): Promise<IGameUpdate> => {
	return apiFetch<IGameUpdate>(
		`/api/game/${gameId}/move/${cardId}/${targetPosition}`,
		{
			method: "POST",
		},
	);
};
