import type { IHeroInfo } from "../types/game";
import { Card, CardMiniature, type ICard } from "./Card";
import { Draggable, Droppable } from "./DragAndDrop";
import {
	CardBack,
	Deck,
	FieldSlot,
	HeroPortrait,
	ManaHud,
	TrapCardSlot,
} from "./Hud";

type ICommonProps = {
	currentMana: number;
	maxMana: number;
	hero: IHeroInfo;
	field: Record<number, ICard>;
	handleCardInteract?: (cardId: number) => void;
	playableCards?: number[];
};

type IPlayerBoardAllyProps = ICommonProps & {
	side: "player";
	hand: ICard[];
	secredCard?: ICard;
};

type IPlayerBoardEnemyProps = ICommonProps & {
	side: "enemy";
	hand: number;
	secredCard: boolean;
};

type IPlayerBoardProps = IPlayerBoardAllyProps | IPlayerBoardEnemyProps;

export const testCard: ICard = {
	id: 0,
	attack: 6,
	cost: 5,
	defense: 2,
	description: "Une simple carte de test pour configurer le css de la carte",
	effects: [],
	keywords: ["Charge", "Invisible"],
	name: "Tester",
	tribut: "Human",
};

const PlayerBoard = ({
	side,
	currentMana,
	maxMana,
	hero,
	field,
	hand,
	secredCard,
	handleCardInteract,
	playableCards,
}: IPlayerBoardProps) => {
	return (
		<div className={`board ${side}`}>
			<div className="left-panel">
				<div className="hud-slot">
					<ManaHud current={currentMana} max={maxMana} />
				</div>
				<div className="hero-slot">
					<HeroPortrait hero={hero} />
				</div>
			</div>
			<div className="middle-panel">
				<Field
					field={field}
					side={side}
					handleCardInteract={handleCardInteract}
				/>
				<div className="hand">
					{side === "player"
						? hand.map((c) => (
								<Draggable
									key={c.id}
									id={`card-${c.id}`}
									cardId={c.id}
									enabled={playableCards?.includes(c.id) ?? false}
								>
									<Card card={c} />
								</Draggable>
							))
						: [...Array(hand).keys()].map((index) => (
								<CardBack key={index}></CardBack>
							))}
				</div>
			</div>

			<div className="right-panel">
				<div className="trap-card-slot">
					<TrapCardSlot side={side} card={secredCard}></TrapCardSlot>
				</div>
				<div className="deck-slot">
					<Deck />
				</div>
			</div>
		</div>
	);
};
interface IFieldProps {
	field: Record<number, ICard>;
	side: "enemy" | "player";
	handleCardInteract?: (cardId: number) => void;
}
const Field = ({ field, side, handleCardInteract }: IFieldProps) => {
	return (
		<div className="field">
			<div className="column">
				<CardWrapper
					handleCardInteract={handleCardInteract}
					side={side}
					type="attack"
					card={field[0]}
					position={0}
				/>
				<CardWrapper
					handleCardInteract={handleCardInteract}
					side={side}
					type="defense"
					card={field[1]}
					position={1}
				/>
			</div>
			<div className="column">
				<CardWrapper
					handleCardInteract={handleCardInteract}
					side={side}
					type="both"
					card={field[2]}
					position={2}
				/>
			</div>
			<div className="column">
				<CardWrapper
					handleCardInteract={handleCardInteract}
					side={side}
					type="attack"
					card={field[3]}
					position={3}
				/>
				<CardWrapper
					handleCardInteract={handleCardInteract}
					side={side}
					type="defense"
					card={field[4]}
					position={4}
				/>
			</div>
			<div className="column">
				<CardWrapper
					handleCardInteract={handleCardInteract}
					side={side}
					type="both"
					card={field[5]}
					position={5}
				/>
			</div>
			<div className="column">
				<CardWrapper
					handleCardInteract={handleCardInteract}
					side={side}
					type="attack"
					card={field[6]}
					position={6}
				/>
				<CardWrapper
					handleCardInteract={handleCardInteract}
					side={side}
					type="defense"
					card={field[7]}
					position={7}
				/>
			</div>
		</div>
	);
};
interface ICardWrapperProps {
	card?: ICard;
	type: "attack" | "defense" | "both";
	side: "enemy" | "player";
	position: number;
	handleCardInteract?: (cardId: number) => void;
}
const CardWrapper = ({
	card,
	type,
	side,
	position,
	handleCardInteract,
}: ICardWrapperProps) => {
	if (card) {
		return (
			<CardMiniature
				handleCardInteract={handleCardInteract}
				type={type}
				card={card}
			/>
		);
	} else {
		if (side === "player") {
			return (
				<Droppable id={`field-${position}`} position={position}>
					<FieldSlot type={type} />
				</Droppable>
			);
		} else {
			return <FieldSlot type={type} />;
		}
	}
};

export default PlayerBoard;
