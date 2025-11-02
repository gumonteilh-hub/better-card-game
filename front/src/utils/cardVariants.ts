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
			return 2000;
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
		opacity: [0, 1],
		y: [50, -10, 0],
		transition: {
			duration: 0.5,
			times: [0, 0.6, 1],
			ease: "easeOut",
		},
	},

	enemyAttacking: {
		y: [0, 70, 0],
		scale: [1, 1.15, 1],
		transition: {
			duration: 0.5,
			times: [0, 0.5, 1],
			ease: [0.43, 0.13, 0.23, 0.96],
		},
	},

	attacking: {
		y: [0, -70, 0],
		scale: [1, 1.15, 1],
		transition: {
			duration: 0.5,
			times: [0, 0.5, 1],
			ease: [0.43, 0.13, 0.23, 0.96],
		},
	},

	enemyAttacked: {
		y: [0, -35, 0],
		scale: [1, 0.9, 0.85, 0.9, 1],
		transition: {
			duration: 0.5,
			times: [0, 0.25, 0.5, 0.75, 1],
			ease: "easeOut",
		},
	},

	attacked: {
		y: [0, 35, 0],
		scale: [1, 0.9, 0.85, 0.9, 1],
		transition: {
			duration: 0.5,
			times: [0, 0.25, 0.5, 0.75, 1],
			ease: "easeOut",
		},
	},

	damaged: {
		x: [0, -12, 12, -10, 10, -6, 6, 0],
		rotate: [0, -3, 3, -2, 2, 0],
		backgroundColor: ["#f4ac45", "#ff0000", "#f4ac45"],
		transition: {
			duration: 0.4,
			ease: "easeOut",
		},
	},

	healed: {
		scale: [1, 1.05, 1],
		backgroundColor: ["#f4ac45", "#008000", "#f4ac45"],
		transition: {
			duration: 0.4,
			ease: "easeOut",
		},
	},

	dying: {
		scale: [1, 1.2, 0],
		rotate: [0, 10, 180],
		opacity: [1, 1, 0],
		y: [0, -20, 30],
		filter: ["grayscale(0%)", "grayscale(50%)", "grayscale(100%)"],
		transition: {
			duration: 0.7,
			times: [0, 0.3, 1],
			ease: "easeIn",
		},
	},

	boosted: {
		y: [0, -12, -5, 0],
		scale: [1, 1.12, 1.08, 1.03],
		rotate: [0, -3, 2, 0],
		filter: [
			"brightness(1) saturate(1)",
			"brightness(1.4) saturate(1.3)",
			"brightness(1.2) saturate(1.15)",
			"brightness(1) saturate(1)",
		],
		transition: {
			duration: 0.3,
			times: [0, 0.3, 0.6, 1],
			ease: "easeOut",
		},
	},
};
