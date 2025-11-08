import { createFileRoute, Link, useNavigate } from "@tanstack/react-router";
import { useEffect, useState } from "react";
import { CardTemplate } from "../components/card/CardTemplate";
import { DeckSummary } from "../components/DeckSummary";
import { getCollection } from "../service/game.service";
import type { Archetype, ICardTemplate, TemplateId } from "../types/template";
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
	const [selectedArchetype, setSelectedArchetype] = useState<Archetype>(
		userInfos?.deck?.archetype ?? { type: "race", value: "HUMAN" },
	);
	const [collection, setCollection] = useState<ICardTemplate[]>([]);

	useEffect(() => {
		setTemporaryDeck(userInfos?.deck?.cards ?? []);
		setSelectedArchetype(
			userInfos?.deck?.archetype ?? { type: "race", value: "HUMAN" },
		);
	}, [userInfos]);

	const addCardToDeck = (card: ICardTemplate) => {
		const count = temporaryDeck.filter((c) => c.id === card.id).length;
		if (count < 2) {
			setTemporaryDeck([...temporaryDeck, card]);
		}
	};

	const handleSelectArchetype = (value: string) => {
		switch (value) {
			case "HUMAN":
				setSelectedArchetype({ type: "race", value: "HUMAN" });
				break;
			case "DRAGON":
				setSelectedArchetype({ type: "race", value: "DRAGON" });
				break;
			case "DEMON":
				setSelectedArchetype({ type: "race", value: "DEMON" });
				break;
			case "WARRIOR":
				setSelectedArchetype({ type: "class", value: "WARRIOR" });
				break;
			case "MAGE":
				setSelectedArchetype({ type: "class", value: "MAGE" });
				break;
			case "ROGUE":
				setSelectedArchetype({ type: "class", value: "ROGUE" });
				break;
			default:
				throw new Error("Unknown archetype");
		}

		setTemporaryDeck([]);
	};

	const removeCardFromDeck = (cardId: TemplateId) => {
		const index = temporaryDeck.findIndex((c) => c.id === cardId);
		if (index !== -1) {
			const newDeck = [...temporaryDeck];
			newDeck.splice(index, 1);
			setTemporaryDeck(newDeck);
		}
	};

	useEffect(() => {
		getCollection(selectedArchetype).then((cards) => {
			setCollection(
				cards.sort((a, b) => {
					if (a.cost === b.cost) {
						return a.name.localeCompare(b.name);
					}
					return a.cost - b.cost;
				}),
			);
		});
	}, [selectedArchetype]);

	function handleSave(): void {
		saveUserInfo({
			...userInfos,
			deck: {
				archetype: selectedArchetype,
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
							onChange={(e) => handleSelectArchetype(e.currentTarget.value)}
						>
							<optgroup label="Race">
								<option value={"HUMAN"}>Human</option>
								<option value={"DRAGON"}>Dragon</option>
								<option value={"DEMON"}>Demon</option>
							</optgroup>
							<optgroup label="Class">
								<option value={"WARRIOR"}>Warrior</option>
								<option value={"MAGE"}>Mage</option>
								<option value={"ROGUE"}>Rogue</option>
							</optgroup>
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
					archetype={selectedArchetype}
					onCardClick={removeCardFromDeck}
				/>
			</div>
		</div>
	);
}
