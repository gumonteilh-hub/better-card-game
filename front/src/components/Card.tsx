import { motion } from "framer-motion";
import { type JSX, useMemo } from "react";
import test from "../assets/test.png";
import type { ICard } from "../types/game";
import type { ICardTemplate } from "../types/template";
import { cardVariants } from "../utils/cardVariants";
import { canAttack } from "../utils/gameRules";
import { useGameContext } from "../utils/useGameContext";
import { TargetWrapper } from "./Hud";

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
	const { animationMap } = useGameContext();
	const animationState = useMemo(
		() => animationMap.get(card.id) ?? "idle",
		[animationMap, card.id],
	);
	return (
		<div className="card-miniature-container">
			<ActionWrapper side={side} card={card} type={type}>
				<motion.div
					className={`card card-miniature untransformed ${type}`}
					variants={cardVariants}
					animate={animationState}
					style={{ willChange: "transform, opacity, filter" }}
					layout
					layoutId={`card-${card.id}`}
				>
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
				</motion.div>
			</ActionWrapper>
			<div className={`card-overview untransformed`}>
				<Card card={card} />
			</div>
		</div>
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
		handleAttackStart,
		selectedAttackingCard,
		handleUnselectAttackingCard,
		isAnimating,
	} = useGameContext();

	if (isAnimating) {
		return children;
	}

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
		if (
			!selectedAttackingCard &&
			(type === "attack" || type === "both") &&
			canAttack(card)
		) {
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
		if (side === "enemy") {
			return (
				<TargetWrapper
					active={selectedAttackingCard !== undefined}
					id={card.id}
				>
					{children}
				</TargetWrapper>
			);
		}
	}

	return children;
};
