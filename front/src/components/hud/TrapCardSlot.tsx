import type { ICardInstance } from "../../types/game";
import { CardMiniature } from "../card/CardMiniature";
import { CardMiniatureBack } from "../card/CardMiniatureBack";
import styles from "./TrapCardSlot.module.css";

interface ITrapCardSlotEnemyProps {
	side: "enemy" | "player";
	card?: boolean | ICardInstance;
}

export const TrapCardSlot = ({ side, card }: ITrapCardSlotEnemyProps) => {
	if (!card) {
		return <div className={styles.trapCardPlaceholder}></div>;
	}
	if (side === "player") {
		return (
			<div className={styles.trapCardPlaceholder}>
				<CardMiniature side={side} card={card as ICardInstance} />
			</div>
		);
	}
	return (
		<div className={styles.trapCardPlaceholder}>
			<CardMiniatureBack />
		</div>
	);
};
