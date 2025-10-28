import { motion } from "framer-motion";
import { type JSX, useMemo } from "react";
import test from "../assets/test.png";
import type { ICardInstance } from "../types/game";
import type { ICardTemplate } from "../types/template";
import { cardVariants } from "../utils/cardVariants";
import { attackReady } from "../utils/gameRules";
import { useGameContext } from "../utils/useGameContext";
import { TargetWrapper } from "./Hud";

interface ICardProps {
	card: ICardInstance;
}

export const Card = ({ card }: ICardProps) => {
	return (
		<div className="card">
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
						{card.cardType.type === "monster" &&
							card.cardType.keywords?.map((k) => <strong key={k}>{k} </strong>)}
						<br />
						{card.description}
					</p>
				</div>
			</div>
			<div className="card-footer">
				{card.cardType.type === "monster" && (
					<div className="card-attack">
						<span>{card.cardType.attack}</span>
					</div>
				)}
				<div className="tribut">{card.faction}</div>
				{card.cardType.type === "monster" && (
					<div className="card-hp">
						<span>{card.cardType.hp}</span>
					</div>
				)}
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
						{card.cardType.type === "monster" &&
							card.cardType.keywords?.map((k) => <strong key={k}>{k} </strong>)}
						<br />
						{card.description}
					</p>
				</div>
			</div>
			<div className="card-footer">
				{card.cardType.type === "monster" && (
					<div className="card-attack">
						<span>{card.cardType.attack}</span>
					</div>
				)}
				<div className="tribut">{card.faction}</div>
				{card.cardType.type === "monster" && (
					<div className="card-hp">
						<span>{card.cardType.hp}</span>
					</div>
				)}
			</div>
		</div>
	);
};

interface ICardMiniatureProps {
	card: ICardInstance;
	type?: "attack" | "defense" | "both";
	side: "enemy" | "player";
}

export const CardMiniature = ({ card, type, side }: ICardMiniatureProps) => {
	const { animationMap } = useGameContext();
	const animationState = useMemo(
		() => animationMap.get(card.id) ?? "idle",
		[animationMap, card.id],
	);

	if (card.cardType.type !== "monster") {
		throw new Error("only monster can be in miniature");
	}

	return (
		<div className="card-miniature-container">
			<ActionWrapper side={side} card={card} type={type}>
				<div className="untransformed">
					<motion.div
						className={`card card-miniature  ${type}`}
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
								<span>{card.cardType.attack}</span>
							</div>
							<div className="card-hp">
								<span>{card.cardType.hp}</span>
							</div>
						</div>
					</motion.div>
				</div>
			</ActionWrapper>
			<div className={`card-overview untransformed`}>
				<Card card={card} />
			</div>
		</div>
	);
};

interface IActionWrapperProps {
	card: ICardInstance;
	type?: "attack" | "defense" | "both";
	side: "enemy" | "player";
	children: JSX.Element;
}

const ActionWrapper = ({ children, type, side, card }: IActionWrapperProps) => {
	const { handleSelectCard, selectedCard, isAnimating, inputMode, gameState } =
		useGameContext();

	const canMove = useMemo(
		() =>
			gameState.player.moveCount > 0 &&
			(!selectedCard || selectedCard === card.id),
		[card.id, selectedCard, gameState.player.moveCount],
	);

	const canAttack = useMemo(
		() =>
			(!selectedCard || selectedCard === card.id) &&
			(type === "attack" || type === "both") &&
			attackReady(card),
		[card.id, selectedCard, type, card],
	);

	const canBeAttacked = useMemo(
		() => inputMode === "attack" && selectedCard,
		[inputMode, selectedCard],
	);

	if (isAnimating) {
		return children;
	}

	if (side === "player") {
		if (
			(inputMode === "move" && canMove) ||
			(inputMode === "attack" && canAttack)
		) {
			return (
				<button
					className="start-attack-button"
					type="button"
					onClick={() => handleSelectCard(card.id)}
				>
					{children}
				</button>
			);
		}
	} else {
		return (
			<TargetWrapper active={!!canBeAttacked} id={card.id}>
				{children}
			</TargetWrapper>
		);
	}

	return children;
};
