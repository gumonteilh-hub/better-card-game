import mana from "../../assets/mana.svg";
import styles from "./ManaCrystal.module.css";

interface IManaCrystalProps {
	content: string;
}

export const ManaCrystal = ({ content }: IManaCrystalProps) => {
	return (
		<div className={`${styles.container} untransformed`}>
			<img className={styles.image} src={mana} alt="mana" />
			<span className={styles.content}>{content}</span>
		</div>
	);
};
