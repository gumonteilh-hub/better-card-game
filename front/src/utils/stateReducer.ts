import type { IAction } from "../types/action";
import type { IGameState } from "../types/game";
export const applyAction = (state: IGameState, action: IAction): IGameState => {
	switch (action.type) {
		case "Summon":
			return applySummon(state, action);
		case "Attack":
			return applyAttack(state, action);
		case "ReceiveDamage":
			return applyReceiveDamage(state, action);
		case "Destroy":
			return applyDestroy(state, action);
		case "Draw":
			return applyDraw(state, action);
		case "Heal":
			return applyHeal(state, action);
		case "IncreaseMaxMana":
			return applyIncreaseMaxMana(state, action);
		case "RefreshMana":
			return applyRefreshMana(state, action);
		case "Win":
			return applyWin(state, action);
		default:
			return state;
	}
};

const applySummon = (
	state: IGameState,
	action: Extract<IAction, { type: "Summon" }>,
): IGameState => {
	let newState = { ...state };

	if (action.value.owner === state.player.hero.id) {
		if (action.value.source.type === "Hand") {
			newState = {
				...newState,
				player: {
					...newState.player,
					hand: state.player.hand.filter(
						(c) => c.id !== action.value.target.id,
					),
					currentMana:
						newState.player.currentMana - action.value.target.template.cost,
				},
			};
		}
		newState = {
			...newState,
			player: {
				...newState.player,
				field: {
					...newState.player.field,
					[action.value.destination]: action.value.target,
				},
			},
		};
	} else {
		if (action.value.source.type === "Hand") {
			newState = {
				...newState,
				enemy: {
					...newState.enemy,
					hand: newState.enemy.hand - 1,
					currentMana:
						newState.enemy.currentMana - action.value.target.template.cost,
				},
			};
		}
		newState = {
			...newState,
			enemy: {
				...newState.enemy,
				field: {
					...newState.enemy.field,
					[action.value.destination]: action.value.target,
				},
			},
		};
	}
	console.log(newState);
	return newState;
};

const applyAttack = (
	state: IGameState,
	action: Extract<IAction, { type: "Attack" }>,
): IGameState => {
	return {
		...state,
		player: {
			...state.player,
			field: Object.fromEntries(
				Object.entries(state.player.field).map(([key, value]) => {
					if (value.id === action.value.initiator) {
						return [key, { ...value, attackCount: value.attackCount + 1 }];
					} else {
						return [key, value];
					}
				}),
			),
		},
	};
};

const applyReceiveDamage = (
	state: IGameState,
	action: Extract<IAction, { type: "ReceiveDamage" }>,
): IGameState => {
	if (action.value.target === state.player.hero.id) {
		return {
			...state,
			player: {
				...state.player,
				hero: {
					...state.player.hero,
					hp: Math.max(state.player.hero.hp - action.value.amount, 0),
				},
			},
		};
	}
	if (action.value.target === state.enemy.hero.id) {
		return {
			...state,
			enemy: {
				...state.enemy,
				hero: {
					...state.enemy.hero,
					hp: Math.max(state.enemy.hero.hp - action.value.amount, 0),
				},
			},
		};
	}

	return {
		...state,
		player: {
			...state.player,
			field: Object.fromEntries(
				Object.entries(state.player.field).map(([key, c]) => {
					if (c.id === action.value.target) {
						if (c.defense <= action.value.amount) {
							return [key, { ...c, defense: 0 }];
						}
						return [key, { ...c, defense: c.defense - action.value.amount }];
					}
					return [key, c];
				}),
			),
		},
		enemy: {
			...state.enemy,
			field: Object.fromEntries(
				Object.entries(state.enemy.field).map(([key, c]) => {
					if (c.id === action.value.target) {
						if (c.defense <= action.value.amount) {
							return [key, { ...c, defense: 0 }];
						}
						return [key, { ...c, defense: c.defense - action.value.amount }];
					}
					return [key, c];
				}),
			),
		},
	};
};

const applyDestroy = (
	state: IGameState,
	action: Extract<IAction, { type: "Destroy" }>,
): IGameState => {
	return {
		...state,
		player: {
			...state.player,
			field: Object.fromEntries(
				Object.entries(state.player.field).filter(
					([_, c]) => c.id !== action.value.target,
				),
			),
		},
		enemy: {
			...state.enemy,
			field: Object.fromEntries(
				Object.entries(state.enemy.field).filter(
					([_, c]) => c.id !== action.value.target,
				),
			),
		},
	};
};

const applyDraw = (
	state: IGameState,
	action: Extract<IAction, { type: "Draw" }>,
): IGameState => {
	let newState = { ...state };
	if (action.value.player === state.player.hero.id) {
		newState = {
			...newState,
			player: {
				...newState.player,
				hand: [...newState.player.hand, action.value.card],
			},
		};
	} else {
		newState = {
			...newState,
			enemy: {
				...newState.enemy,
				hand: newState.enemy.hand + 1,
			},
		};
	}
	return newState;
};

const applyHeal = (
	state: IGameState,
	action: Extract<IAction, { type: "Heal" }>,
): IGameState => {
	return {
		...state,
		player: {
			...state.player,
			field: Object.fromEntries(
				Object.entries(state.player.field).map(([key, c]) => {
					if (c.id === action.value.target) {
						const healedDefense = c.defense + action.value.amount;
						if (c.template.defense > healedDefense) {
							return [key, { ...c, defense: c.template.defense }];
						}
						return [key, { ...c, defense: healedDefense }];
					}
					return [key, c];
				}),
			),
		},
		enemy: {
			...state.enemy,
			field: Object.fromEntries(
				Object.entries(state.enemy.field).map(([key, c]) => {
					if (c.id === action.value.target) {
						const healedDefense = c.defense + action.value.amount;
						if (c.template.defense > healedDefense) {
							return [key, { ...c, defense: c.template.defense }];
						}
						return [key, { ...c, defense: healedDefense }];
					}
					return [key, c];
				}),
			),
		},
	};
};

function applyIncreaseMaxMana(
	state: IGameState,
	action: Extract<IAction, { type: "IncreaseMaxMana" }>,
): IGameState {
	if (action.value.player === state.player.hero.id) {
		return {
			...state,
			player: {
				...state.player,
				maxMana: state.player.maxMana + action.value.amount,
			},
		};
	} else {
		return {
			...state,
			enemy: {
				...state.enemy,
				maxMana: state.enemy.maxMana + action.value.amount,
			},
		};
	}
}

function applyRefreshMana(
	state: IGameState,
	action: Extract<IAction, { type: "RefreshMana" }>,
): IGameState {
	if (action.value.player === state.player.hero.id) {
		return {
			...state,
			player: {
				...state.player,
				currentMana: state.player.currentMana + action.value.amount,
			},
		};
	} else {
		return {
			...state,
			enemy: {
				...state.enemy,
				currentMana: state.enemy.currentMana + action.value.amount,
			},
		};
	}
}

function applyWin(
	state: IGameState,
	action: Extract<IAction, { type: "Win" }>,
): IGameState {
	return { ...state, winnerId: action.value };
}
