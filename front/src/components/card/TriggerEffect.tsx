import { AnimatePresence, motion, type Variants } from "framer-motion";
import onAlone from "../../assets/on_alone.svg";
import onAttack from "../../assets/on_attack.svg";
import onDeath from "../../assets/on_death.svg";
import onPlay from "../../assets/on_play.svg";
import onKill from "../../assets/on_kill.svg";
import onDamaged from "../../assets/on_damaged.svg";
import onDefend from "../../assets/on_defend.svg";
import onTurnEnd from "../../assets/on_turn_end.svg";
import onTurnStart from "../../assets/on_turn_start.svg";
import onSurrounded from "../../assets/on_surrounded.svg";
import type { AnimationState } from "../../engine/animationEngine";
import styles from "./TriggerEffect.module.css";

const overlayVariants: Variants = {
	hidden: {
		scale: 0,
		opacity: 0,
		rotate: -45,
	},
	visible: {
		scale: [0, 1.2, 1],
		opacity: [0, 1, 1],
		rotate: [-45, 10, 0],
		transition: {
			duration: 0.4,
			times: [0, 0.6, 1],
			ease: "easeOut",
		},
	},
	exit: {
		scale: 1.3,
		opacity: 0,
		transition: {
			duration: 0.2,
			ease: "easeIn",
		},
	},
};

export const TriggerEffect = ({ type }: { type?: AnimationState }) => {
	const getIcon = () => {
		switch (type) {
			case "triggerOnDeath":
				return (
					<img
						className={styles.triggerEffectImage}
						src={onDeath}
						alt="onDeath"
					/>
				);
			case "triggerOnPlay":
				return (
					<img
						className={styles.triggerEffectImage}
						src={onPlay}
						alt="onPlay"
					/>
				);
			case "triggerOnAttack":
				return (
					<img
						className={styles.triggerEffectImage}
						src={onAttack}
						alt="onAttack"
					/>
				);
			case "triggerOnAlone":
				return (
					<img
						className={styles.triggerEffectImage}
						src={onAlone}
						alt="onAlone"
					/>
				);
			case "triggerOnSurrounded":
				return (
					<img
						className={styles.triggerEffectImage}
						src={onSurrounded}
						alt="onSurrounded"
					/>
				);
			case "TriggerOnDamaged":
				return (
					<img
						className={styles.triggerEffectImage}
						src={onDamaged}
						alt="onDamaged"
					/>
				);
			case "triggerOnKill":
				return (
					<img
						className={styles.triggerEffectImage}
						src={onKill}
						alt="onKill"
					/>
				);
			case "triggerOnTurnEnd":
				return (
					<img
						className={styles.triggerEffectImage}
						src={onTurnEnd}
						alt="onTurnEnd"
					/>
				);
			case "TriggerOnTurnStart":
				return (
					<img
						className={styles.triggerEffectImage}
						src={onTurnStart}
						alt="onTurnStart"
					/>
				);
			case "triggerOnDefend":
				return (
					<img
						className={styles.triggerEffectImage}
						src={onDefend}
						alt="onDefend"
					/>
				);
			default:
				return null;
		}
	};

	const getColorClass = () => {
		switch (type) {
			case "triggerOnDeath":
				return styles.death;
			case "triggerOnPlay":
				return styles.play;
			case "triggerOnAttack":
				return styles.attack;
			default:
				return "";
		}
	};

	return (
		<AnimatePresence>
			{type && (
				<motion.div
					className={`${styles.overlay} ${getColorClass()}`}
					variants={overlayVariants}
					initial="hidden"
					animate="visible"
					exit="exit"
				>
					{getIcon()}
				</motion.div>
			)}
		</AnimatePresence>
	);
};
