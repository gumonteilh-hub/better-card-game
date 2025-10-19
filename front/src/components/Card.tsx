import type { JSX } from "react";
import test from "../assets/test.png";
import type { ICard } from "../types/game";
import type { ICardTemplate } from "../types/template";
import { useGameContext } from "../utils/GameContextProvider";
import { canAttack } from "../utils/gameRules";

interface ICardProps {
	card: ICard;
}

export const Card = ({ card }: ICardProps) => {
	return (
		<div className="card">
			<div className="card-header">
				<div className="card-cost">{card.template.cost}</div>
				<div className="card-name">{card.template.name}</div>
			</div>
			<div className="card-body">
				<div className="card-image">
					<img src={test} alt="card" />
				</div>
				<div className="card-description">
					<p>
						{card.template.keywords?.map((k) => (
							<strong key={k}>{k} </strong>
						))}
						<br />
						{card.template.description}
					</p>
				</div>
			</div>
			<div className="card-footer">
				<div className="card-attack">
					<span>{card.attack}</span>
				</div>
				<div className="tribut">{card.template.faction}</div>
				<div className="card-defense">
					<span>{card.defense}</span>
				</div>
			</div>
		</div>
	);
};

interface ICardTemplateProps {
	card: ICardTemplate;
}

export const CardTemplate = ({ card }: ICardTemplateProps) => {
	return (
		<div className="card card-template">
			<div className="card-header">
				<div className="card-cost">{card.cost}</div>
				<div className="card-name">{card.name}</div>
			</div>
			<div className="card-body">
				<div className="card-image">
					<img src={test} alt="card" />
				</div>
				<div className="card-description">
					<p>
						{card.keywords?.map((k) => (
							<strong key={k}>{k} </strong>
						))}
						<br />
						{card.description}
					</p>
				</div>
			</div>
			<div className="card-footer">
				<div className="card-attack">
					<span>{card.attack}</span>
				</div>
				<div className="tribut">{card.faction}</div>
				<div className="card-defense">
					<span>{card.defense}</span>
				</div>
			</div>
		</div>
	);
};

interface ICardMiniatureProps {
	card: ICard;
	type?: "attack" | "defense" | "both";
	side: "enemy" | "player";
}

export const CardMiniature = ({ card, type, side }: ICardMiniatureProps) => {
	return (
		<ActionWrapper side={side} card={card} type={type}>
			<div className={`card card-miniature untransformed ${type}`}>
				<div className="card-body">
					<div className="card-image">
						<img src={test} alt="card" />
					</div>
				</div>
				<div className="card-footer">
					<div className="card-attack">
						<span>{card.attack}</span>
					</div>
					<div className="card-defense">
						<span>{card.defense}</span>
					</div>
				</div>
			</div>
		</ActionWrapper>
	);
};

interface IActionWrapperProps {
	card: ICard;
	type?: "attack" | "defense" | "both";
	side: "enemy" | "player";
	children: JSX.Element;
}

const ActionWrapper = ({ children, type, side, card }: IActionWrapperProps) => {
	const {
		handleTargetSelect,
		handleAttackStart,
		selectedAttackingCard,
		handleUnselectAttackingCard,
	} = useGameContext();

	if (type) {
		if (side === "player")
			if (selectedAttackingCard && selectedAttackingCard === card.id) {
				return (
					<button
						className="start-attack-button"
						type="button"
						onClick={() => handleUnselectAttackingCard()}
					>
						{children}
					</button>
				);
			}
		if (!selectedAttackingCard && canAttack(card)) {
			return (
				<button
					className="start-attack-button"
					type="button"
					onClick={() => handleAttackStart(card.id)}
				>
					{children}
				</button>
			);
		}
		if (side === "enemy" && selectedAttackingCard) {
			return (
				<button
					className="start-attack-button"
					type="button"
					onClick={() => handleTargetSelect(card.id)}
				>
					{children}
				</button>
			);
		}
	}

	return children;
};
