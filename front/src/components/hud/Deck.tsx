import { CardBack } from "../card/CardBack";
import styles from "./Deck.module.css";

export const Deck = () => {
	return (
		<div className={styles.deck}>
			<CardBack></CardBack>
			<CardBack></CardBack>
			<CardBack></CardBack>
		</div>
	);
};
