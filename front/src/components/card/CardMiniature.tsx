import { motion } from "framer-motion";
import { useMemo } from "react";
import placeholder from "../../assets/test.png";
import type { ICardInstance } from "../../types/game";
import { cardVariants } from "../../utils/cardVariants";
import { useGameContext } from "../../utils/useGameContext";
import { ActionWrapper } from "../wrapper/ActionWrapper";
import { Card } from "./Card";
import styles from "./CardMiniature.module.css";
import { TriggerEffect } from "./TriggerEffect";

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

	const triggerType = useMemo(() => {
		switch (animationState) {
			case "triggerOnDeath":
				return "onDeath";
			case "triggerOnPlay":
				return "onPlay";
			case "triggerOnAttack":
				return "onAttack";
			default:
				return null;
		}
	}, [animationState]);

	if (card.cardType.type !== "monster") {
		throw new Error("only monster can be in miniature");
	}

	return (
		<div className={styles.cardMiniatureContainer}>
			<ActionWrapper side={side} card={card} type={type}>
				<div className="untransformed">
					<motion.div
						className={`${styles.cardMiniature} ${type ? styles[type] : ""}`}
						variants={cardVariants}
						animate={animationState}
						style={{ willChange: "transform, opacity, filter" }}
						layout
						layoutId={`card-${card.id}`}
					>
						<TriggerEffect type={triggerType} />
						<div className={styles.cardBody}>
							<div className={styles.cardImage}>
								<img src={placeholder} alt="card" />
							</div>
						</div>
						<div className={styles.cardFooter}>
							<div className={styles.cardAttack}>
								<span>{card.cardType.attack}</span>
							</div>
							<div className={styles.cardHp}>
								<span>{card.cardType.hp}</span>
							</div>
						</div>
					</motion.div>
				</div>
			</ActionWrapper>
			<div className={`${styles.cardOverview} untransformed`}>
				<Card card={card} />
			</div>
		</div>
	);
};
