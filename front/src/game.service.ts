import type { Faction, ICardTemplate } from "./types/template";

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
	console.log(json);
	return json as ICardTemplate[];
};
