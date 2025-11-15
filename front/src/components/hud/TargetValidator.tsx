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

	console.log({ playedCardWaitingForTargets });

	const isButtonDisabled =
		playedCardWaitingForTargets?.card.playTarget?.strict &&
		playedCardWaitingForTargets.card.playTarget.amount !==
			selectedTargetsForEffect.length;

	const remainingTargets =
		(playedCardWaitingForTargets?.card.playTarget?.amount ?? 0) -
		selectedTargetsForEffect.length;

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
				<div style={{ position: "relative", flex: 1 }}>
					<button
						type="button"
						disabled={isButtonDisabled}
						onClick={handleConfirm}
						className={styles.confirmButton}
					>
						Confirmer
					</button>
					{isButtonDisabled && remainingTargets > 0 && (
						<span className={styles.warningMessage}>
							Sélectionnez {remainingTargets} cible
							{remainingTargets > 1 ? "s" : ""} supplémentaire
							{remainingTargets > 1 ? "s" : ""}
						</span>
					)}
				</div>
			</div>
		</div>
	);
};
