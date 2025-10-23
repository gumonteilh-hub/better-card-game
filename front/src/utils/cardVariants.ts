import type { Variants } from "framer-motion";
import type { ActionType } from "../types/action";

export const getAnimationDuration = (actionType: ActionType): number => {
	switch (actionType) {
		case "Draw":
			return 700;
		case "Heal":
			return 500;
		case "Destroy":
			return 900;
		case "ReceiveDamage":
			return 500;
		case "Summon":
			return 700;
		case "Attack":
			return 800;
		case "Win":
		case "TriggerOnDeath":
		case "TriggerOnPlay":
		case "TriggerOnAttack":
		case "IncreaseMaxMana":
		case "RefreshMana":
		case "BurnCard":
			return 0;
	}
};

export const cardVariants: Variants = {
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

	// Apparition (summon, draw)
	summoned: {
		scale: [0, 1.1, 1],
		rotate: [0, 5, 0],
		opacity: [0, 1],
		y: [50, -10, 0],
		transition: {
			duration: 0.7,
			times: [0, 0.6, 1],
			ease: "easeOut",
		},
	},

	drawed: {
		scale: [0, 1.1, 1],
		rotate: [0, 5, 0],
		opacity: [0, 1],
		y: [50, -10, 0],
		transition: {
			duration: 0.7,
			times: [0, 0.6, 1],
			ease: "easeOut",
		},
	},

	attacking: {
		x: [0, 80, 0],
		y: [0, -30, 0],
		rotate: [0, -20, 0],
		scale: [1, 1.15, 1],
		transition: {
			duration: 0.8,
			times: [0, 0.5, 1],
			ease: [0.43, 0.13, 0.23, 0.96],
		},
	},

	attacked: {
		x: [0, -25, 8, -5, 0],
		y: [0, 5, -3, 0],
		rotate: [0, 8, -4, 2, 0],
		scale: [1, 0.92, 1.05, 0.98, 1],
		filter: [
			"brightness(1)",
			"brightness(1.6)",
			"brightness(0.9)",
			"brightness(1.1)",
			"brightness(1)",
		],
		transition: {
			duration: 0.8,
			times: [0, 0.2, 0.5, 0.75, 1],
			ease: "easeOut",
		},
	},

	damaged: {
		x: [0, -12, 12, -10, 10, -6, 6, 0],
		rotate: [0, -3, 3, -2, 2, 0],
		filter: [
			"brightness(1)",
			"brightness(1.5)",
			"brightness(0.8)",
			"brightness(1)",
		],
		transition: {
			duration: 0.5,
			ease: "easeOut",
		},
	},

	healed: {
		y: [0, -15, -10, 0],
		scale: [1, 1.08, 1.05, 1],
		filter: [
			"brightness(1) saturate(1)",
			"brightness(1.3) saturate(1.2)",
			"brightness(1.1) saturate(1.1)",
			"brightness(1) saturate(1)",
		],
		transition: {
			duration: 0.5,
			ease: "easeOut",
		},
	},

	dying: {
		scale: [1, 1.2, 0],
		rotate: [0, 10, 180],
		opacity: [1, 1, 0],
		y: [0, -20, 30],
		transition: {
			duration: 0.9,
			times: [0, 0.3, 1],
			ease: "easeIn",
		},
	},
};
