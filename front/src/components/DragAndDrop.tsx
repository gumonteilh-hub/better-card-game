import { useDraggable, useDroppable } from "@dnd-kit/core";
import type { JSX } from "react";

export type CardType = "monster" | "spell";

export function Droppable({
	children,
	id,
	position,
	accepts,
	customClassName = "",
}: {
	children: JSX.Element[] | JSX.Element;
	id: string;
	position: number;
	accepts: CardType[];
	customClassName?: string;
}) {
	const { isOver, setNodeRef, active } = useDroppable({
		id,
		data: {
			position: position,
			accepts: accepts,
		},
	});
	const state =
		isOver &&
			active?.data.current &&
			accepts.includes(active?.data.current.type)
			? "hover"
			: "";

	return (
		<div ref={setNodeRef} className={`dropzone ${state} ${customClassName}`}>
			{children}
		</div>
	);
}

export function Draggable({
	children,
	id,
	cardId,
	enabled,
	style: externalStyle,
	cardType,
}: {
	children: JSX.Element;
	id: string;
	cardId: number;
	enabled: boolean;
	style?: React.CSSProperties;
	cardType: CardType;
}) {
	const { attributes, listeners, setNodeRef, transform } = useDraggable({
		id,
		disabled: !enabled,
		data: {
			id: cardId,
			type: cardType,
		},
	});
	const style = transform
		? {
			transform: `translate3d(${transform.x}px, ${transform.y}px, 0)`,
			...externalStyle,
		}
		: externalStyle;

	return (
		<div
			ref={setNodeRef}
			style={style}
			className={enabled ? "is-playable-highlight" : ""}
			{...listeners}
			{...attributes}
		>
			{children}
		</div>
	);
}
