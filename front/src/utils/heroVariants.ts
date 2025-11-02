import type { Variants } from "framer-motion";
import type { ActionType } from "../types/action";

export const getAnimationDuration = (actionType: ActionType): number => {
	switch (actionType) {
		case "Destroy":
			return 700;
		case "Heal":
		case "ReceiveDamage":
			return 400;
		case "Summon":
			return 500;
		case "Attack":
			return 500;
		case "Boost":
			return 300;
		case "Win":
			return 1200;
		case "TriggerOnDeath":
		case "TriggerOnPlay":
		case "TriggerOnAttack":
		case "IncreaseMaxMana":
		case "RefreshMana":
		case "BurnCard":
		case "Draw":
			return 0;
	}
};

export const heroVariants: Variants = {
	idle: {
		x: 0,
		y: 0,
		rotate: 0,
		scale: 1,
		opacity: 1,
		filter: "brightness(1) saturate(1)",
		transition: {
			duration: 0.3,
			ease: "easeOut",
		},
	},

	damaged: {
		x: [0, -12, 12, -10, 10, -6, 6, 0],
		rotate: [0, -3, 3, -2, 2, 0],
		transition: {
			duration: 0.4,
			ease: "easeOut",
		},
	},

	healed: {
		scale: [1, 1.05, 1],
		transition: {
			duration: 0.4,
			ease: "easeOut",
		},
	},

	winned: {
		scale: [1, 2, 1.8, 2],
		rotate: [0, -8, 8, -5, 5, 0],
		x: ["0vw", "25vw", "25vw"],
		y: ["0vh", "-10vh", "-10vh"],
		filter: [
			"brightness(1) saturate(1) drop-shadow(0 0 0px gold)",
			"brightness(2) saturate(2) drop-shadow(0 0 80px rgba(255, 215, 0, 0.9)) drop-shadow(0 0 120px rgba(255, 215, 0, 0.6))",
			"brightness(1.8) saturate(1.8) drop-shadow(0 0 60px rgba(255, 215, 0, 0.8)) drop-shadow(0 0 100px rgba(255, 215, 0, 0.5))",
			"brightness(2) saturate(2) drop-shadow(0 0 80px rgba(255, 215, 0, 0.9)) drop-shadow(0 0 120px rgba(255, 215, 0, 0.6))",
		],
		zIndex: 9999,
		transition: {
			duration: 2,
			ease: "easeInOut",
			repeat: Infinity,
			repeatType: "reverse",
		},
	},
};
