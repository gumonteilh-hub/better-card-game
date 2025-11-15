import { useCallback, useState } from "react";
import type { IGameState } from "../../types/game";
import type { ICardWithTarget } from "../GameContextProvider";

export const useSelectTargetForEffect = (gameState: IGameState | undefined) => {
	const [playedCardWaitingForTargets, setPlayedCardWaitingForTargets] =
		useState<ICardWithTarget>();
	const [selectedTargetsForEffect, setSelectedTargetsForEffect] = useState<
		number[]
	>([]);

	const cancelPlayerCardWaitingForTargets = useCallback(() => {
		setPlayedCardWaitingForTargets(undefined);
		setSelectedTargetsForEffect([]);
	}, []);

	const selectTargetForEffect = useCallback(
		(cardId: number) => {
			if (!playedCardWaitingForTargets) return;
			const currentTargets = [...selectedTargetsForEffect];
			if (currentTargets.includes(cardId)) {
				setSelectedTargetsForEffect(currentTargets.filter((c) => c !== cardId));
				return;
			}

			if (currentTargets.length < playedCardWaitingForTargets.target.amount) {
				currentTargets.push(cardId);
			}

			setSelectedTargetsForEffect(currentTargets);
		},
		[playedCardWaitingForTargets, selectedTargetsForEffect],
	);

	const playCardWithPotentialTargets = useCallback(
		(cardId: number, position: number | undefined) => {
			const card = gameState?.player.hand.find((c) => c.id === cardId);
			if (card?.playTarget) {
				setPlayedCardWaitingForTargets({
					card,
					target: card.playTarget,
					position,
				});
			}
		},
		[gameState?.player.hand],
	);

	return {
		selectedTargetsForEffect,
		selectTargetForEffect,
		playedCardWaitingForTargets,
		playCardWithPotentialTargets,
		cancelPlayerCardWaitingForTargets,
	};
};
