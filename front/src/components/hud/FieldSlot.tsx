import shield from "../../assets/shield.svg";
import sword from "../../assets/sword.svg";
import { useGameContext } from "../../utils/useGameContext";
import { MoveWrapper } from "../wrapper/MoveWrapper";
import styles from "./FieldSlot.module.css";

interface IFieldSlotProps {
	type: "attack" | "defense" | "both";
	side: "enemy" | "player";
	position: number;
}

export const FieldSlot = ({ type, position, side }: IFieldSlotProps) => {
	const { moveTargets } = useGameContext();

	return (
		<MoveWrapper
			active={moveTargets.includes(position) && side === "player"}
			id={position}
		>
			<div className={styles.container}>
				{(type === "defense" || type === "both") && (
					<img className={styles.shield} src={shield} aria-hidden />
				)}
				{(type === "attack" || type === "both") && (
					<img className={styles.sword} src={sword} aria-hidden />
				)}
			</div>
		</MoveWrapper>
	);
};
