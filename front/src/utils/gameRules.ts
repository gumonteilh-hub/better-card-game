import type { ICardWithTarget } from "../engine/GameContextProvider";
import type { ICardInstance } from "../types/game";

export const attackReady = (card: ICardInstance) => {
	if (card.cardType.type !== "monster") return false;
	let maxAttackPerTurn = 1;
	if (card.cardType.keywords?.includes("WINDFURRY")) {
		maxAttackPerTurn = 2;
	}

	return !card.cardType.asleep && card.cardType.attackCount < maxAttackPerTurn;
};

export const isValidTarget = (
	card: ICardInstance,
	cardWithTarget: ICardWithTarget | undefined,
) => {
	console.log({ card });
	console.log({ cardWithTarget });
	if (!cardWithTarget) return false;

	const matcher = cardWithTarget.target.matcher;
	switch (matcher.type) {
		case "race": {
			return card.race === matcher.value;
		}
		case "class": {
			return card.class === matcher.value;
		}
		case "owner": {
			return card.owner === matcher.value;
		}
	}
};
