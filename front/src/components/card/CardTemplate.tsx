import placeholder from "../../assets/test.png";
import type { ICardTemplate } from "../../types/template";
import { ManaCrystal } from "../hud/ManaCrystal";
import styles from "./Card.module.css";

interface ICardTemplateProps {
	card: ICardTemplate;
}

export const CardTemplate = ({ card }: ICardTemplateProps) => {
	return (
		<div className={styles.card}>
			<div className={styles.cardHeader}>
				<div className={styles.cardCost}>
					<ManaCrystal content={card.cost.toString()} />
				</div>
				<div className={styles.cardName}>{card.name}</div>
			</div>
			<div className={styles.cardBody}>
				<div className={styles.cardImage}>
					<img src={placeholder} alt="card" />
				</div>
				<div className={styles.cardDescription}>
					<p>
						{card.cardType.type === "monster" &&
							card.cardType.keywords?.map((k) => <strong key={k}>{k} </strong>)}
						<br />
						{card.description}
					</p>
				</div>
			</div>
			<div className={styles.cardFooter}>
				{card.cardType.type === "monster" && (
					<div className={styles.cardAttack}>
						<span>{card.cardType.attack}</span>
					</div>
				)}
				<div className="tribut">
					{card.race !== "COMMON" && card.race} <br />
					{card.class !== "COMMON" && card.class}
				</div>
				{card.cardType.type === "monster" && (
					<div className={styles.cardHp}>
						<span>{card.cardType.hp}</span>
					</div>
				)}
			</div>
		</div>
	);
};
