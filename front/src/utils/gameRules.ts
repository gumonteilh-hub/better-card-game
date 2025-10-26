import type { ICard } from "../types/game";

export const attackReady = (card: ICard) => {
	let maxAttackPerTurn = 1;
	if (card.template.keywords?.includes("CHARGE")) {
		maxAttackPerTurn = 2;
	}

	return !card.asleep && card.attackCount < maxAttackPerTurn;
};
