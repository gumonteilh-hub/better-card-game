import type { IAction, PlayerId } from "./action";
import type { Faction, Keywords, TemplateId } from "./template";

export interface IGameUpdate {
	actions: IAction[];
	gameView: IGameState;
}

export interface IGameState {
	gameId: string;
	playerId: string;
	turn: number;
	winnerId?: PlayerId;
	enemy: IEnemyInfo;
	player: IPlayerInfo;
}

export interface IPlayerInfo {
	secretCard?: ICardInstance;
	field: Record<number, ICardInstance>;
	maxMana: number;
	currentMana: number;
	moveCount: number;
	maxMove: number;
	hand: ICardInstance[];
	hero: IHeroInfo;
	deckSize: number;
}

export interface IEnemyInfo {
	secretCard: boolean;
	field: Record<number, ICardInstance>;
	maxMana: number;
	currentMana: number;
	hand: number;
	hero: IHeroInfo;
	deckSize: number;
}

export interface IHeroInfo {
	id: number;
	name: string;
	hp: number;
	faction: Faction;
}

type InstanceId = number;

export type ICardInstance = {
	id: InstanceId;
	template_id: TemplateId;
	name: string;
	description: string;
	faction: Faction;
	cost: number;
	owner: PlayerId;
	location: Location;
	cardType: ICardTypeInstance;
};

export type ICardTypeInstance = IMonsterCardInstance | ISpellCardInstance;

// biome-ignore lint/suspicious/noExplicitAny: <no need to have typed effect in front>
export type Effect = any;

export type IMonsterCardInstance = {
	type: "monster";
	attack: number;
	hp: number;
	max_hp: number;
	asleep: boolean;
	attackCount: number;
	keywords: Keywords[];
	onPlay: Effect[];
	onAttack: Effect[];
	onDeath: Effect[];
};

export type ISpellCardInstance = {
	type: "spell";
	effect: Effect[];
};

export type Location =
	| {
			type: "Hand";
	  }
	| {
			type: "Field";
			value: number;
	  }
	| {
			type: "Deck";
	  }
	| {
			type: "Graveyard";
	  };
