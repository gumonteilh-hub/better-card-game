export type Archetype =
	| {
		type: "race";
		value: Race;
	}
	| {
		type: "class";
		value: Class;
	};

export type Race = "HUMAN" | "DRAGON" | "DEMON" | "COMMON";
export type Class = "WARRIOR" | "MAGE" | "ROGUE" | "COMMON";

export type Keywords = "CHARGE" | "WINDFURRY" | "INVISBLE";

export type TemplateId = number;

// biome-ignore lint/suspicious/noExplicitAny: <no need to know the effect type in front>
type TemplateEffect = any;

export interface IDeck {
	archetype: Archetype;
	cards: ICardTemplate[];
}

export interface ICardTemplate {
	id: TemplateId;
	name: string;
	description: string;
	cost: number;
	race: Race;
	class: Class;
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
