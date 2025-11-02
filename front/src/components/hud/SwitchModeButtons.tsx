import attack from "../../assets/attack_button.png";
import move from "../../assets/move_button.png";
import { useGameContext } from "../../utils/useGameContext";
import styles from "./SwitchModeButtons.module.css";

interface IActionButtonProps {
	side: "player" | "enemy";
}
export const AttackModeButton = ({ side }: IActionButtonProps) => {
	const { inputMode, handleSetInputMode } = useGameContext();
	if (side === "player") {
		const status = inputMode === "attack" ? styles.active : "";
		return (
			<button
				type="button"
				onClick={() => handleSetInputMode("attack")}
				className={`${styles.button} ${status}`}
			>
				<img className={styles.image} src={attack} alt="attack" />
			</button>
		);
	}
	return (
		<div className={styles.container}>
			<img className={styles.image} src={attack} alt="attack" />
		</div>
	);
};

export const MoveModeButton = ({ side }: IActionButtonProps) => {
	const { inputMode, handleSetInputMode, gameState } = useGameContext();
	if (side === "player") {
		const status = inputMode === "move" ? styles.active : "";
		return (
			<button
				type="button"
				onClick={() => handleSetInputMode("move")}
				className={`${styles.button} ${status}`}
			>
				<span className={styles.moveCounter}>
					{gameState.player.moveCount} /{gameState.player.maxMove}
				</span>
				<img className={styles.image} src={move} alt="move" />
			</button>
		);
	}
	return (
		<div className={styles.container}>
			<img className={styles.image} src={move} alt="move" />
		</div>
	);
};
