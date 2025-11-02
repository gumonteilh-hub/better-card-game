import type { JSX } from "react";
import { useGameContext } from "../../utils/useGameContext";
import styles from "./TargetWrapper.module.css";

interface ITargetWrapperProps {
	active: boolean;
	children: JSX.Element;
	id: number | string;
}

export const TargetWrapper = ({
	active,
	children,
	id,
}: ITargetWrapperProps) => {
	const { handleTargetSelect } = useGameContext();
	if (!active) return children;

	return (
		<button
			type="button"
			className={styles.selectTargetButton}
			onClick={() => handleTargetSelect(id)}
		>
			{children}
		</button>
	);
};
