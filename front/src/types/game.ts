import type { IAction, PlayerId } from "./action";
import type { Faction, ICardTemplate } from "./template";

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
	secretCard?: ICard;
	field: Record<number, ICard>;
	maxMana: number;
	currentMana: number;
	hand: ICard[];
	hero: IHeroInfo;
	deckSize: number;
}

export interface IEnemyInfo {
	secretCard: boolean;
	field: Record<number, ICard>;
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

export interface ICard {
	id: number;
	asleep: boolean;
	attackCount: number;
	attack: number;
	hp: number;
	location: Location;
	template: ICardTemplate;
}

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
