import { type JSX, useMemo } from "react";
import type { ICardInstance } from "../../types/game";
import { attackReady, isValidTarget } from "../../utils/gameRules";
import { useGameContext } from "../../utils/useGameContext";
import styles from "./ActionWrapper.module.css";
import { TargetWrapper } from "./TargetWrapper";

interface IActionWrapperProps {
	card: ICardInstance;
	type?: "attack" | "defense" | "both";
	side: "enemy" | "player";
	children: JSX.Element;
}

export const ActionWrapper = ({
	children,
	type,
	side,
	card,
}: IActionWrapperProps) => {
	const {
		handleSelectCard,
		selectedCard,
		isAnimating,
		inputMode,
		gameState,
		playedCardWaitingForTargets,
		selectTargetForEffect,
		selectedTargetsForEffect,
	} = useGameContext();

	const canMove = useMemo(
		() =>
			gameState.player.moveCount > 0 &&
			(!selectedCard || selectedCard === card.id),
		[card.id, selectedCard, gameState.player.moveCount],
	);

	const canAttack = useMemo(
		() =>
			(!selectedCard || selectedCard === card.id) &&
			(type === "attack" || type === "both") &&
			attackReady(card),
		[card.id, selectedCard, type, card],
	);

	const canBeAttacked = useMemo(
		() => inputMode === "attack" && selectedCard,
		[inputMode, selectedCard],
	);

	const canBeTargetted = useMemo(
		() => isValidTarget(card, playedCardWaitingForTargets),
		[card, playedCardWaitingForTargets],
	);

	if (isAnimating) {
		return children;
	}

	if (playedCardWaitingForTargets) {
		if (canBeTargetted) {
			if (selectedTargetsForEffect.includes(card.id)) {
				return (
					<button
						className={styles.selectedTargetButton}
						type="button"
						onClick={() => selectTargetForEffect(card.id)}
					>
						{children}
					</button>
				);
			}
			return (
				<button
					className={styles.startAttackButton}
					type="button"
					onClick={() => selectTargetForEffect(card.id)}
				>
					{children}
				</button>
			);
		} else {
			return children;
		}
	}

	if (side === "player") {
		if (
			(inputMode === "move" && canMove) ||
			(inputMode === "attack" && canAttack)
		) {
			return (
				<button
					className={styles.startAttackButton}
					type="button"
					onClick={() => handleSelectCard(card.id)}
				>
					{children}
				</button>
			);
		}
	} else {
		return (
			<TargetWrapper active={!!canBeAttacked} id={card.id}>
				{children}
			</TargetWrapper>
		);
	}

	return children;
};
