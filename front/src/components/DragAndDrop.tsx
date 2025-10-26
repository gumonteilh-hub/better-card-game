import { useDraggable, useDroppable } from "@dnd-kit/core";
import type { JSX } from "react";

export function Droppable({
	children,
	id,
	position,
}: {
	children: JSX.Element;
	id: string;
	position: number;
}) {
	const { isOver, setNodeRef } = useDroppable({
		id,
		data: {
			position: position,
		},
	});
	const style = {
		backgroundColor: isOver ? "blue" : undefined,
	};

	return (
		<div ref={setNodeRef} style={style}>
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
}: {
	children: JSX.Element;
	id: string;
	cardId: number;
	enabled: boolean;
	style?: React.CSSProperties;
}) {
	const { attributes, listeners, setNodeRef, transform } = useDraggable({
		id,
		disabled: !enabled,
		data: {
			id: cardId,
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
