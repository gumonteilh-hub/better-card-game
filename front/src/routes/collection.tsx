import { createFileRoute, Link, useNavigate } from "@tanstack/react-router";
import { useEffect, useState } from "react";
import { CardTemplate } from "../components/Card";
import { DeckSummary } from "../components/DeckSummary";
import { getCollection } from "../game.service";
import type { Faction, ICardTemplate } from "../types/template";
import { useUserInfo } from "../utils/useUserInfo";

export const Route = createFileRoute("/collection")({
	component: RouteComponent,
});

function RouteComponent() {
	const { userInfos, saveUserInfo } = useUserInfo();
	const navigate = useNavigate({ from: "/collection" });

	const [temporaryDeck, setTemporaryDeck] = useState<ICardTemplate[]>(
		userInfos?.deck?.cards ?? [],
	);
	const [selectedFaction, setSelectedFaction] = useState<Faction>(
		userInfos?.deck?.faction ?? "HUMAN",
	);
	const [collection, setCollection] = useState<ICardTemplate[]>([]);

	useEffect(() => {
		setTemporaryDeck(userInfos?.deck?.cards ?? []);
		setSelectedFaction(userInfos?.deck?.faction ?? "HUMAN");
	}, [userInfos]);

	const addCardToDeck = (card: ICardTemplate) => {
		const count = temporaryDeck.filter((c) => c.id === card.id).length;
		if (count < 2) {
			setTemporaryDeck([...temporaryDeck, card]);
		}
	};

	const removeCardFromDeck = (cardId: string) => {
		const index = temporaryDeck.findIndex((c) => c.id === cardId);
		if (index !== -1) {
			const newDeck = [...temporaryDeck];
			newDeck.splice(index, 1);
			setTemporaryDeck(newDeck);
		}
	};

	useEffect(() => {
		getCollection(selectedFaction).then((cards) => {
			setCollection(
				cards.sort((a, b) => {
					if (a.cost === b.cost) {
						return a.name.localeCompare(b.name);
					}
					return a.cost - b.cost;
				}),
			);
		});
	}, [selectedFaction]);

	function handleSave(): void {
		saveUserInfo({
			...userInfos,
			deck: {
				faction: selectedFaction,
				cards: temporaryDeck,
			},
		});
		navigate({ to: "/" });
	}

	return (
		<div className="deck-builder-container">
			<div className="deck-builder-main">
				<div className="deck-builder-header">
					<h1>Deck Builder</h1>
					<div className="deck-builder-actions">
						<Link to={"/"} className="deck-builder-btn deck-builder-btn-back">
							Retour
						</Link>
						<button
							type="submit"
							className="deck-builder-btn deck-builder-btn-save"
							onClick={handleSave}
							disabled={temporaryDeck.length !== 30}
						>
							Sauvegarder
						</button>
						<select
							onChange={(e) =>
								setSelectedFaction(e.currentTarget.value as Faction)
							}
						>
							<option value={"HUMAN"}>Human</option>
							<option value={"DRAGON"}>Dragon</option>
							<option value={"DEMON"}>Demon</option>
						</select>
					</div>
				</div>

				<div className="collection-grid">
					{collection.map((c) => {
						return (
							<button
								key={c.id}
								type="button"
								className="button-unstyled"
								onClick={() => addCardToDeck(c)}
							>
								<CardTemplate card={c} />
							</button>
						);
					})}
				</div>
			</div>

			<div className="deck-sidebar">
				<DeckSummary
					cards={temporaryDeck}
					faction={selectedFaction}
					onCardClick={removeCardFromDeck}
				/>
			</div>
		</div>
	);
}
