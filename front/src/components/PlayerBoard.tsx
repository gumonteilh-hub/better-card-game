import type { ICardInstance, IHeroInfo } from "../types/game";
import { useGameContext } from "../utils/useGameContext";
import { Card } from "./card/Card";
import { CardBack } from "./card/CardBack";
import { Draggable } from "./DragAndDrop";
import { Field } from "./Field";
import { Deck } from "./hud/Deck";
import { HeroPortrait } from "./hud/HeroPortrait";
import { PlayerHud } from "./hud/PlayerHud";
import { TrapCardSlot } from "./hud/TrapCardSlot";

type ICommonProps = {
	currentMana: number;
	maxMana: number;
	hero: IHeroInfo;
	field: Record<number, ICardInstance>;
};
type IPlayerBoardAllyProps = ICommonProps & {
	side: "player";
	hand: ICardInstance[];
	secredCard?: ICardInstance;
};
type IPlayerBoardEnemyProps = ICommonProps & {
	side: "enemy";
	hand: number;
	secredCard: boolean;
};
type IPlayerBoardProps = IPlayerBoardAllyProps | IPlayerBoardEnemyProps;

const PlayerBoard = ({
	side,
	currentMana,
	maxMana,
	hero,
	field,
	hand,
	secredCard,
}: IPlayerBoardProps) => {
	return (
		<div className={`board ${side}`}>
			<div className="left-panel">
				<div className="hud-slot">
					<PlayerHud side={side} currentMana={currentMana} maxMana={maxMana} />
				</div>
				<div className="hero-slot">
					<HeroPortrait hero={hero} side={side} />
				</div>
			</div>
			<div className="middle-panel">
				<Field field={field} side={side} />
				{side === "player" ? (
					<Hand side={side} hand={hand}></Hand>
				) : (
					<Hand side={side} hand={hand}></Hand>
				)}
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

type IHandProps =
	| {
			side: "player";
			hand: ICardInstance[];
	  }
	| { side: "enemy"; hand: number };

const Hand = ({ side, hand }: IHandProps) => {
	const { playableCards } = useGameContext();
	return (
		<div className="hand">
			{side === "player"
				? hand.map((c, index) => (
						<Draggable
							cardType={c.cardType.type}
							key={c.id}
							id={`card-${c.id}`}
							cardId={c.id}
							enabled={playableCards?.includes(c.id) ?? false}
							style={{ zIndex: index }}
						>
							<Card card={c} />
						</Draggable>
					))
				: hand > 0 &&
					[...Array(hand).keys()].map((index) => (
						<CardBack key={index}></CardBack>
					))}
		</div>
	);
};

export default PlayerBoard;
