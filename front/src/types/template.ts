export type Faction = "HUMAN" | "DRAGON" | "DEMON" | "COMMON";

export type Keywords = "CHARGE" | "WINDFURRY" | "INVISBLE";

export interface IDeck {
	faction: Faction;
	cards: ICardTemplate[];
}

export interface ICardTemplate {
	id: string;
	name: string;
	description: string;
	cost: number;
	attack: number;
	defense: number;
	keywords: Keywords[];
	faction: Faction;
}
