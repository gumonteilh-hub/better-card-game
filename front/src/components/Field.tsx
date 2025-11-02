import type { ICardInstance } from "../types/game";
import { CardMiniature } from "./card/CardMiniature";
import { Droppable } from "./DragAndDrop";
import { FieldSlot } from "./hud/FieldSlot";

interface IFieldProps {
	field: Record<number, ICardInstance>;
	side: "enemy" | "player";
}
export const Field = ({ field, side }: IFieldProps) => {
	return (
		<Droppable
			accepts={["spell"]}
			id={`field-${side}`}
			customClassName="field"
			position={0}
		>
			<div className="column">
				<CardWrapper side={side} type="attack" card={field[0]} position={0} />
				<CardWrapper side={side} type="defense" card={field[1]} position={1} />
			</div>
			<div className="column">
				<CardWrapper side={side} type="both" card={field[2]} position={2} />
			</div>
			<div className="column">
				<CardWrapper side={side} type="attack" card={field[3]} position={3} />
				<CardWrapper side={side} type="defense" card={field[4]} position={4} />
			</div>
			<div className="column">
				<CardWrapper side={side} type="both" card={field[5]} position={5} />
			</div>
			<div className="column">
				<CardWrapper side={side} type="attack" card={field[6]} position={6} />
				<CardWrapper side={side} type="defense" card={field[7]} position={7} />
			</div>
		</Droppable>
	);
};
interface ICardWrapperProps {
	card?: ICardInstance;
	type: "attack" | "defense" | "both";
	side: "enemy" | "player";
	position: number;
}
const CardWrapper = ({ card, type, side, position }: ICardWrapperProps) => {
	if (card) {
		return <CardMiniature side={side} type={type} card={card} />;
	} else {
		if (side === "player") {
			return (
				<Droppable
					accepts={["monster", "spell"]}
					id={`field-${position}`}
					position={position}
					customClassName="field-slot-dropzone"
				>
					<FieldSlot side={side} position={position} type={type} />
				</Droppable>
			);
		} else {
			return <FieldSlot side={side} position={position} type={type} />;
		}
	}
};
