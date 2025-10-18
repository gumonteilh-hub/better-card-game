import { useDraggable, useDroppable } from "@dnd-kit/core";
import type { JSX } from "react";

export function Droppable({
	children,
	id,
}: {
	children: JSX.Element;
	id: string;
}) {
	const { isOver, setNodeRef } = useDroppable({
		id,
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
	style: externalStyle,
}: {
	children: JSX.Element;
	id: string;
	style?: React.CSSProperties;
}) {
	const { attributes, listeners, setNodeRef, transform } = useDraggable({
		id,
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
			className="is-playable-highlight"
			{...listeners}
			{...attributes}
		>
			{children}
		</div>
	);
}
