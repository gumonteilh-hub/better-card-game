import { useMemo } from "react";
import type { Faction, ICardTemplate } from "../types/template";

interface DeckSummaryProps {
	cards: ICardTemplate[];
	faction: Faction;
	onCardClick?: (cardId: string) => void;
	showTitle?: boolean;
}
export const DeckSummary = ({
	faction,
	cards,
	onCardClick,
	showTitle = true,
}: DeckSummaryProps) => {
	const deckSummary = useMemo(() => {
		const summary = new Map<string, { card: ICardTemplate; count: number }>();
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
		<div className="deck-summary">
			{showTitle && (
				<div className="deck-summary-header">
					<h2>Mon Deck</h2>
					<h3>{faction}</h3>
					<div
						className={`deck-summary-count ${cards.length === 30 ? "complete" : "incomplete"}`}
					>
						{cards.length} / 30
					</div>
				</div>
			)}
			<div className="deck-summary-list">
				{deckSummary.map(({ card, count }) => (
					<button
						type="button"
						key={card.id}
						className={`deck-summary-item ${onCardClick ? "clickable" : ""}`}
						onClick={() => onCardClick?.(card.id)}
					>
						<div className="deck-summary-item-info">
							<div className="deck-summary-item-cost">{card.cost}</div>
							<div className="deck-summary-item-name">{card.name}</div>
						</div>
						<div className="deck-summary-item-count">x{count}</div>
					</button>
				))}
			</div>
		</div>
	);
};
