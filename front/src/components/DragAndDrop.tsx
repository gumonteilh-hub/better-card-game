import { useDraggable, useDroppable } from "@dnd-kit/core";
import type { JSX } from "react";
import styles from "./DragAndDrop.module.css";

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
			? styles.hover
			: "";

	return (
		<div
			ref={setNodeRef}
			className={`${styles.dropzone} ${state} ${customClassName}`}
		>
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
			className={enabled ? styles.playable : ""}
			{...listeners}
			{...attributes}
		>
			{children}
		</div>
	);
}
