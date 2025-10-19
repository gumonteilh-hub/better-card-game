import type { IAction } from "../types/action";
import type { IGameState } from "../types/game";
/*
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
				hand: state.hand.filter((c) => c.id != action.value.target.id),
				mana: newState.mana - action.value.target.template.cost,
				field: [...newState.field, action.value.target],
			};
		}
		newState = { ...newState, field: [...newState.field, action.value.target] };
	} else {
		if (action.value.source.type === "Hand") {
			newState = {
				...newState,
				ennemy_hand_size: newState.ennemy_hand_size - 1,
				ennemy_mana: newState.ennemy_mana - action.value.target.template.cost,
			};
		}
		newState = {
			...newState,
			ennemy_field: [...newState.ennemy_field, action.value.target],
		};
	}
	return newState;
};

const applyAttack = (
	state: IGameState,
	_action: Extract<IAction, { type: "Attack" }>,
): IGameState => {
	return state;
};

const applyReceiveDamage = (
	state: IGameState,
	action: Extract<IAction, { type: "ReceiveDamage" }>,
): IGameState => {
	return {
		...state,
		field: state.field.map((c) => {
			if (c.id === action.value.target) {
				if (c.hp <= action.value.amount) {
					return { ...c, hp: 0 };
				}
				return { ...c, hp: c.hp - action.value.amount };
			}
			return c;
		}),
		ennemy_field: state.ennemy_field.map((c) => {
			if (c.id === action.value.target) {
				if (c.hp <= action.value.amount) {
					return { ...c, hp: 0 };
				}
				return { ...c, hp: c.hp - action.value.amount };
			}
			return c;
		}),
	};
};

const applyDestroy = (
	state: IGameState,
	action: Extract<IAction, { type: "Destroy" }>,
): IGameState => {
	return {
		...state,
		ennemy_field: state.ennemy_field.filter((c) => c.id != action.value.target),
		field: state.field.filter((c) => c.id != action.value.target),
	};
};

const applyDraw = (
	state: IGameState,
	action: Extract<IAction, { type: "Draw" }>,
): IGameState => {
	let newState = { ...state };
	if (action.value.player === state.player_info.id) {
		newState = { ...newState, hand: [...newState.hand, action.value.card] };
	} else {
		newState = { ...newState, ennemy_hand_size: newState.ennemy_hand_size + 1 };
	}
	return newState;
};

const applyHeal = (
	state: IGameState,
	action: Extract<IAction, { type: "Heal" }>,
): IGameState => {
	return {
		...state,
		field: state.field.map((c) => {
			if (c.id === action.value.target) {
				const healedHp = c.hp + action.value.amount;
				if (c.template.base_hp > healedHp) {
					return { ...c, hp: c.template.base_hp };
				}
				return { ...c, hp: healedHp };
			}
			return c;
		}),
		ennemy_field: state.ennemy_field.map((c) => {
			if (c.id === action.value.target) {
				const healedHp = c.hp + action.value.amount;
				if (c.template.base_hp > healedHp) {
					return { ...c, hp: c.template.base_hp };
				}
				return { ...c, hp: healedHp };
			}
			return c;
		}),
	};
};

function applyIncreaseMaxMana(
	state: IGameState,
	action: Extract<IAction, { type: "IncreaseMaxMana" }>,
): IGameState {
	if (action.value.player === state.player_info.id) {
		return { ...state, base_mana: state.base_mana + action.value.amount };
	} else {
		return {
			...state,
			ennemy_base_mana: state.ennemy_base_mana + action.value.amount,
		};
	}
}

function applyRefreshMana(
	state: IGameState,
	action: Extract<IAction, { type: "RefreshMana" }>,
): IGameState {
	if (action.value.player === state.player_info.id) {
		return { ...state, mana: state.mana + action.value.amount };
	} else {
		return { ...state, ennemy_mana: state.ennemy_mana + action.value.amount };
	}
}

function applyWin(
	state: IGameState,
	action: Extract<IAction, { type: "Win" }>,
): IGameState {
	return { ...state, winner_id: action.value };
}
*/
