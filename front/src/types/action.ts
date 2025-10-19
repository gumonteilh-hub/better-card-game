import type { ICard } from "../components/Card";

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
	| RefreshManaAction
	| IncreaseMaxManaAction;

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
	| "IncreaseMaxMana"
	| "RefreshMana";

type EntityId = number;
type PlayerId = string;

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
		card: ICard;
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
		target: EntityId;
		amount: number;
	};
};

type SummonAction = {
	type: "Summon";
	value: {
		source: Location;
		target: ICard;
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
