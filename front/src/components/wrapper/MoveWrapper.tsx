import type { JSX } from "react";
import { useGameContext } from "../../utils/useGameContext";
import styles from "./MoveWrapper.module.css";

interface IMoveWrapperProps {
	active: boolean;
	children: JSX.Element;
	id: number;
}

export const MoveWrapper = ({ active, children, id }: IMoveWrapperProps) => {
	const { handleMoveSelect } = useGameContext();
	if (!active) return children;

	return (
		<button
			type="button"
			className={styles.selectMoveButton}
			onClick={() => handleMoveSelect(id)}
		>
			{children}
		</button>
	);
};
