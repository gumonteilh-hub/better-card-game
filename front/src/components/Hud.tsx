import { useMemo } from "react";
import demon from "../assets/hero-demon.png";
import dragon from "../assets/hero-dragon.png";
import human from "../assets/hero-human.png";
import shield from "../assets/shield.svg";
import sword from "../assets/sword.svg";
import type { IHeroInfo } from "../types/game";
import { CardMiniature, type ICard } from "./Card";

export const CardBack = () => {
	return (
		<div className="card-back">
			<div className="spiral"></div>
		</div>
	);
};

const CardMiniatureBack = () => {
	return (
		<div className="card-miniature-back">
			<div className="spiral"></div>
		</div>
	);
};

export const Deck = () => {
	return (
		<div className="deck">
			<CardBack></CardBack>
			<CardBack></CardBack>
			<CardBack></CardBack>
		</div>
	);
};

interface IHeroPortraitProps {
	hero: IHeroInfo;
}
export const HeroPortrait = ({ hero }: IHeroPortraitProps) => {
	const imgSrc = useMemo(() => {
		switch (hero.faction) {
			case "HUMAN":
				return human;
			case "DRAGON":
				return dragon;
			case "DEMON":
				return demon;
			case "COMMON":
				throw new Error("Player can't be from common faction");
		}
	}, [hero.faction]);

	return (
		<div className="hero-portrait ">
			<div className="hero-image-slot untransformed">
				<img src={imgSrc} alt="the hero of the player" />
				<div className="player-hp">
					<HeartIcon className="heart-icon" />
					<span className="hp-value ">{hero.hp}</span>
				</div>
			</div>
			<div className="hero-name-slot untransformed">
				<p>{hero.name}</p>
			</div>
		</div>
	);
};

interface ITrapCardSlotEnemyProps {
	side: "enemy" | "player";
	card?: boolean | ICard;
}

export const TrapCardSlot = ({ side, card }: ITrapCardSlotEnemyProps) => {
	if (!card) {
		return <div className="trap-card-placeholder"></div>;
	}
	if (side === "player") {
		return (
			<div className="trap-card-placeholder">
				<CardMiniature card={card as ICard} />
			</div>
		);
	}
	return (
		<div className="trap-card-placeholder">
			<CardMiniatureBack />
		</div>
	);
};

interface IManaHudProps {
	current: number;
	max: number;
}

export const ManaHud = ({ current, max }: IManaHudProps) => {
	return (
		<div className="mana-crystal untransformed">
			{current} /{max}
		</div>
	);
};

interface IFieldSlotProps {
	type: "attack" | "defense" | "both";
}

export const FieldSlot = ({ type }: IFieldSlotProps) => {
	return (
		<div className={`field-slot ${type}`}>
			{(type === "defense" || type === "both") && (
				<img className="shield" src={shield} aria-hidden />
			)}
			{(type === "attack" || type === "both") && (
				<img className="sword" src={sword} aria-hidden />
			)}
		</div>
	);
};

const HeartIcon = (props: React.SVGProps<SVGSVGElement>) => {
	return (
		<svg
			xmlns="http://www.w.org/2000/svg"
			viewBox="0 0 24 24"
			fill="currentColor"
			{...props}
		>
			<title>heart</title>
			<path d="M12 21.35l-1.45-1.32C5.4 15.36 2 12.28 2 8.5 2 5.42 4.42 3 7.5 3c1.74 0 3.41.81 4.5 2.09C13.09 3.81 14.76 3 16.5 3 19.58 3 22 5.42 22 8.5c0 3.78-3.4 6.86-8.55 11.54L12 21.35z" />
		</svg>
	);
};

export default HeartIcon;
