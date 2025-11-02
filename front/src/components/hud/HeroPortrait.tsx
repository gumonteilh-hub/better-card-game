import { motion } from "framer-motion";
import { useMemo } from "react";
import demon from "../../assets/hero-demon.png";
import dragon from "../../assets/hero-dragon.png";
import human from "../../assets/hero-human.png";
import type { IHeroInfo } from "../../types/game";
import { heroVariants } from "../../utils/heroVariants";
import { useGameContext } from "../../utils/useGameContext";
import { TargetWrapper } from "../wrapper/TargetWrapper";
import { HeartIcon } from "./HeartIcon";
import styles from "./HeroPortrait.module.css";

interface IHeroPortraitProps {
	hero: IHeroInfo;
	side: "enemy" | "player";
}
export const HeroPortrait = ({ hero, side }: IHeroPortraitProps) => {
	const { canAttackPlayer, animationMap } = useGameContext();

	const animationState = useMemo(
		() => animationMap.get(hero.id) ?? "idle",
		[animationMap, hero.id],
	);

	const imgSrc = useMemo(() => {
		switch (hero.faction) {
			case "HUMAN":
				return human;
			case "DRAGON":
				return dragon;
			case "DEMON":
				return demon;
			case "COMMON":
				throw new Error("Player can't be from common faction");
		}
	}, [hero.faction]);

	return (
		<div className={styles.container}>
			<div className={`${styles.imageSlot} untransformed`}>
				<motion.div
					className={styles.animationWrapper}
					variants={heroVariants}
					animate={animationState}
					style={{ willChange: "transform, opacity, filter" }}
					layout
					layoutId={`hero-${hero.id}`}
				>
					<TargetWrapper
						active={side === "enemy" && canAttackPlayer}
						id={hero.id}
					>
						<img src={imgSrc} alt="the hero of the player" />
					</TargetWrapper>
				</motion.div>
				<div className={styles.hp}>
					<HeartIcon className={styles.icon} />
					<span className={styles.value}>{hero.hp}</span>
				</div>
			</div>
			<div className={`${styles.nameSlot} untransformed`}>
				<p>{hero.name}</p>
			</div>
		</div>
	);
};
