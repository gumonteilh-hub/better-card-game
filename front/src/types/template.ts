export type Faction = "HUMAN" | "DRAGON" | "DEMON" | "COMMON";

export type Keywords = "CHARGE" | "WINDFURRY" | "INVISBLE";

export type TemplateId = number;

// biome-ignore lint/suspicious/noExplicitAny: <no need to know the effect type in front>
type TemplateEffect = any;

export interface IDeck {
	faction: Faction;
	cards: ICardTemplate[];
}

export interface ICardTemplate {
	id: TemplateId;
	name: string;
	description: string;
	cost: number;
	faction: Faction;
	cardType: ICardTypeTemplate;
}

type ICardTypeTemplate = ISplellCardTemplate | IMonsterCardTemplate;

type IMonsterCardTemplate = {
	type: "monster";
	attack: number;
	hp: number;
	keywords: Keywords[];
	onPlay: TemplateEffect[];
	onAttack: TemplateEffect[];
	onDeath: TemplateEffect[];
};

type ISplellCardTemplate = {
	type: "spell";
	effect: TemplateEffect[];
};
