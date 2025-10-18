import test from "../assets/test.png";
import type { ICardTemplate } from "../types/template";

export interface ICard {
	cost: number;
	attack: number;
	defense: number;
	keywords: string[];
	effects: string[];
	tribut: string;
	name: string;
	description: string;
}

interface ICardProps {
	card: ICard;
}

export const Card = ({ card }: ICardProps) => {
	return (
		<div className="card">
			<div className="card-header">
				<div className="card-cost">{card.cost}</div>
				<div className="card-name">{card.name}</div>
			</div>
			<div className="card-body">
				<div className="card-image">
					<img src={test} alt="card" />
				</div>
				<div className="card-description">
					<p>
						{card.keywords.map((k) => (
							<strong key={k}>{k} </strong>
						))}
						<br />
						{card.description}
					</p>
				</div>
			</div>
			<div className="card-footer">
				<div className="card-attack">
					<span>{card.attack}</span>
				</div>
				<div className="tribut">{card.tribut}</div>
				<div className="card-defense">
					<span>{card.defense}</span>
				</div>
			</div>
		</div>
	);
};

interface ICardTemplateProps {
	card: ICardTemplate;
}

export const CardTemplate = ({ card }: ICardTemplateProps) => {
	return (
		<div className="card card-template">
			<div className="card-header">
				<div className="card-cost">{card.cost}</div>
				<div className="card-name">{card.name}</div>
			</div>
			<div className="card-body">
				<div className="card-image">
					<img src={test} alt="card" />
				</div>
				<div className="card-description">
					<p>
						{card.keywords.map((k) => (
							<strong key={k}>{k} </strong>
						))}
						<br />
						{card.description}
					</p>
				</div>
			</div>
			<div className="card-footer">
				<div className="card-attack">
					<span>{card.attack}</span>
				</div>
				<div className="tribut">{card.faction}</div>
				<div className="card-defense">
					<span>{card.defense}</span>
				</div>
			</div>
		</div>
	);
};

interface ICardMiniatureProps {
	card: ICard;
	type: "attack" | "defense" | "both";
}

export const CardMiniature = ({ card, type }: ICardMiniatureProps) => {
	return (
		<div className={`card card-miniature untransformed ${type}`}>
			<div className="card-body">
				<div className="card-image">
					<img src={test} alt="card" />
				</div>
			</div>
			<div className="card-footer">
				<div className="card-attack">
					<span>{card.attack}</span>
				</div>
				<div className="card-defense">
					<span>{card.defense}</span>
				</div>
			</div>
		</div>
	);
};
