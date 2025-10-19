import type { ICard } from "../types/game";

export const canAttack = (card: ICard) => {
	let maxAttackPerTurn = 1;
	if (card.template.keywords?.includes("CHARGE")) {
		maxAttackPerTurn = 2;
	}

	return !card.asleep && card.attackCount < maxAttackPerTurn;
};
