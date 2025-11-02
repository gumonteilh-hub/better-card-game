import { AnimatePresence, motion, type Variants } from "framer-motion";
import onAttack from "../../assets/on_attack.svg";
import onDeath from "../../assets/on_death.svg";
import onPlay from "../../assets/on_play.svg";
import styles from "./TriggerEffect.module.css";

interface TriggerEffectProps {
	type: "onDeath" | "onPlay" | "onAttack" | null;
}

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

export const TriggerEffect = ({ type }: TriggerEffectProps) => {
	const getIcon = () => {
		switch (type) {
			case "onDeath":
				return (
					<img
						className={styles.triggerEffectImage}
						src={onDeath}
						alt="onDeath"
					/>
				);
			case "onPlay":
				return (
					<img
						className={styles.triggerEffectImage}
						src={onPlay}
						alt="onPlay"
					/>
				);
			case "onAttack":
				return (
					<img
						className={styles.triggerEffectImage}
						src={onAttack}
						alt="onAttack"
					/>
				);
			default:
				return null;
		}
	};

	const getColorClass = () => {
		switch (type) {
			case "onDeath":
				return styles.death;
			case "onPlay":
				return styles.play;
			case "onAttack":
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
