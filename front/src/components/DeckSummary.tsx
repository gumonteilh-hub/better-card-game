import { useMemo } from "react";
import type { Archetype, ICardTemplate, TemplateId } from "../types/template";
import styles from "./DeckSummary.module.css";
import { ManaCrystal } from "./hud/ManaCrystal";

interface DeckSummaryProps {
	cards: ICardTemplate[];
	archetype: Archetype;
	onCardClick?: (cardId: TemplateId) => void;
	showTitle?: boolean;
}
export const DeckSummary = ({
	archetype,
	cards,
	onCardClick,
	showTitle = true,
}: DeckSummaryProps) => {
	const deckSummary = useMemo(() => {
		const summary = new Map<
			TemplateId,
			{ card: ICardTemplate; count: number }
		>();
		cards.forEach((card) => {
			const existing = summary.get(card.id);
			if (existing) {
				existing.count++;
			} else {
				summary.set(card.id, { card, count: 1 });
			}
		});
		return Array.from(summary.values()).sort((a, b) => {
			if (a.card.cost === b.card.cost) {
				return a.card.name.localeCompare(b.card.name);
			}
			return a.card.cost - b.card.cost;
		});
	}, [cards]);

	return (
		<div className={styles.deckSummary}>
			{showTitle && (
				<div className={styles.header}>
					<h2>Mon Deck</h2>
					<h3>{archetype.value}</h3>
					<div
						className={`${styles.count} ${cards.length === 30 ? styles.complete : styles.incomplete}`}
					>
						{cards.length} / 30
					</div>
				</div>
			)}
			<div className={styles.list}>
				{deckSummary.map(({ card, count }) => (
					<button
						type="button"
						key={card.id}
						className={`${styles.item} ${onCardClick ? styles.clickable : ""}`}
						onClick={() => onCardClick?.(card.id)}
					>
						<div className={styles.itemInfo}>
							<div className={styles.itemCost}>
								<ManaCrystal content={card.cost.toString()} />
							</div>
							<div className={styles.itemName}>{card.name}</div>
						</div>
						<div className={styles.itemCount}>x{count}</div>
					</button>
				))}
			</div>
		</div>
	);
};
