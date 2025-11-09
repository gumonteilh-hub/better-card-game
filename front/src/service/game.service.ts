import type { Archetype, ICardTemplate, IDeck } from "../types/template";
import { apiFetch } from "./api";

export const getCollection = async (
	archetype: Archetype,
): Promise<ICardTemplate[]> => {
	return apiFetch<ICardTemplate[]>(`/api/collection`, {
		method: "POST",
		body: JSON.stringify(archetype),
	});
};

interface StartGameInfo {
	gameId: string;
	userId: string;
}

export const startGame = async (
	userId: string,
	deck: IDeck,
): Promise<StartGameInfo> => {
	const body = {
		archetype: deck.archetype,
		cards: deck.cards.map((c) => c.id),
	};
	return apiFetch<StartGameInfo>(`/api/ia/${userId}`, {
		method: "POST",
		body: JSON.stringify(body),
	});
};

export const findCurrentGame = async (userId: string): Promise<string> => {
	return apiFetch<string>(`/api/user/${userId}`, {
		method: "GET",
	});
};
