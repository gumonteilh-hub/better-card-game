import type { ICardInstance } from "../types/game";

export const attackReady = (card: ICardInstance) => {
	if (card.cardType.type !== "monster") return false;
	let maxAttackPerTurn = 1;
	if (card.cardType.keywords?.includes("WINDFURRY")) {
		maxAttackPerTurn = 2;
	}

	return !card.cardType.asleep && card.cardType.attackCount < maxAttackPerTurn;
};
