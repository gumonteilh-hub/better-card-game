import { useGameContext } from "../../utils/useGameContext";
import styles from "./TargetValidator.module.css";

export const TargetValidator = () => {
	const {
		playedCardWaitingForTargets,
		selectedTargetsForEffect,
		cancelPlayerCardWaitingForTargets,
		playSpell,
		playMonster,
	} = useGameContext();

	function handleConfirm(): void {
		if (playedCardWaitingForTargets) {
			if (playedCardWaitingForTargets.card.cardType.type === "monster") {
				if (!playedCardWaitingForTargets.position) {
					throw new Error(
						"Illegal state : playedCardWaitingForTargets with type monster must have a position",
					);
				}
				playMonster(
					playedCardWaitingForTargets.card.id,
					playedCardWaitingForTargets.position,
					selectedTargetsForEffect,
				);
			}
			if (playedCardWaitingForTargets.card.cardType.type === "spell") {
				playSpell(
					playedCardWaitingForTargets.card.id,
					selectedTargetsForEffect,
				);
			}
			cancelPlayerCardWaitingForTargets();
		}
	}

	return (
		<div className={styles.container}>
			<div className={styles.quantityContainer}>
				<span className={styles.counter}>
					{selectedTargetsForEffect.length} /{" "}
					{playedCardWaitingForTargets?.target.amount}
				</span>
			</div>
			<div className={styles.validateContainer}>
				<button
					type="button"
					onClick={cancelPlayerCardWaitingForTargets}
					className={styles.cancelButton}
				>
					✕
				</button>
				<button
					type="button"
					onClick={handleConfirm}
					className={styles.confirmButton}
				>
					Confirmer
				</button>
			</div>
		</div>
	);
};
