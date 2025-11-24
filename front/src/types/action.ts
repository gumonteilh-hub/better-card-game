import type { ICardInstance, IGameState, Location } from "./game";

export type IAction =
	| BurnAction
	| DrawAction
	| HealAction
	| DestroyAction
	| ReceiveDamageAction
	| SummonAction
	| AttackAction
	| WinAction
	| TriggerOnAttackAction
	| TriggerOnPlayAction
	| TriggerOnDeathAction
	| TriggerOnSurroundedAction
	| TriggerOnAloneAction
	| TriggerOnKillAction
	| TriggerOnTurnEndAction
	| TriggerOnTurnStartAction
	| TriggerOnDamagedAction
	| TriggerOnDefendAction
	| RefreshManaAction
	| BoostAction
	| UpdateGameViewAction
	| StartTurnAction
	| EndTurnAction
	| EnemyDrawAction
	| IncreaseMaxManaAction
	| IncreaseAbsoluteMaxManaAction
	| CardStolenAction;

export type ActionType =
	| "BurnCard"
	| "Draw"
	| "Heal"
	| "Destroy"
	| "ReceiveDamage"
	| "Summon"
	| "Attack"
	| "Win"
	| "TriggerOnDeath"
	| "TriggerOnPlay"
	| "TriggerOnAttack"
	| "TriggerOnSurrounded"
	| "TriggerOnAlone"
	| "TriggerOnKill"
	| "TriggerOnTurnStart"
	| "TriggerOnTurnEnd"
	| "TriggerOnDamaged"
	| "TriggerOnDefend"
	| "IncreaseMaxMana"
	| "Boost"
	| "StartTurn"
	| "EndTurn"
	| "EnemyDraw"
	| "UpdateGameView"
	| "RefreshMana"
	| "IncreaseAbsoluteMaxMana"
	| "CardStolen";

export type EntityId = number;
export type PlayerId = number;

type EnemyDrawAction = {
	type: "EnemyDraw";
};

type UpdateGameViewAction = {
	type: "UpdateGameView";
	value: {
		player: PlayerId;
		game: IGameState;
	};
};

type EndTurnAction = {
	type: "EndTurn";
	value: PlayerId;
};

type StartTurnAction = {
	type: "StartTurn";
	value: PlayerId;
};

type BoostAction = {
	type: "Boost";
	value: {
		target: number;
		attack: number;
		hp: number;
	};
};

type BurnAction = {
	type: "BurnCard";
	value: {
		player: PlayerId;
		card: EntityId;
	};
};

type DrawAction = {
	type: "Draw";
	value: {
		player: PlayerId;
		card: ICardInstance;
	};
};

type HealAction = {
	type: "Heal";
	value: {
		target: EntityId;
		amount: number;
	};
};

type DestroyAction = {
	type: "Destroy";
	value: {
		target: EntityId;
	};
};

type ReceiveDamageAction = {
	type: "ReceiveDamage";
	value: {
		target: EntityId | PlayerId;
		amount: number;
	};
};

type SummonAction = {
	type: "Summon";
	value: {
		source: Location;
		destination: number;
		target: ICardInstance;
		owner: PlayerId;
	};
};

type AttackAction = {
	type: "Attack";
	value: {
		initiator: EntityId;
		target: EntityId;
	};
};

type WinAction = {
	type: "Win";
	value: PlayerId;
};

type TriggerOnDeathAction = {
	type: "TriggerOnDeath";
	value: EntityId;
};

type TriggerOnPlayAction = {
	type: "TriggerOnPlay";
	value: EntityId;
};

type TriggerOnAttackAction = {
	type: "TriggerOnAttack";
	value: EntityId;
};

type TriggerOnAloneAction = {
	type: "TriggerOnAlone";
	value: EntityId;
};

type TriggerOnKillAction = {
	type: "TriggerOnKill";
	value: EntityId;
};
type TriggerOnDamagedAction = {
	type: "TriggerOnDamaged";
	value: EntityId;
};
type TriggerOnTurnStartAction = {
	type: "TriggerOnTurnStart";
	value: EntityId;
};
type TriggerOnTurnEndAction = {
	type: "TriggerOnTurnEnd";
	value: EntityId;
};
type TriggerOnDefendAction = {
	type: "TriggerOnDefend";
	value: EntityId;
};
type TriggerOnSurroundedAction = {
	type: "TriggerOnSurrounded";
	value: EntityId;
};
type IncreaseMaxManaAction = {
	type: "IncreaseMaxMana";
	value: {
		player: PlayerId;
		amount: number;
	};
};

type RefreshManaAction = {
	type: "RefreshMana";
	value: {
		player: PlayerId;
		amount: number;
	};
};

type IncreaseAbsoluteMaxManaAction = {
	type: "IncreaseAbsoluteMaxMana";
	value: {
		player: PlayerId;
		amount: number;
	};
};

type CardStolenAction = {
	type: "CardStolen";
	value: {
		thief: PlayerId;
		victim: PlayerId;
		card: ICardInstance;
		fromLocation: Location;
	};
};
