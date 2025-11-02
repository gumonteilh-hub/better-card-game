import { useContext } from "react";
import { GameContext } from "../types/gameContext.type";

export const useGameContext = () => {
	const gameContext = useContext(GameContext);
	if (!gameContext) {
		throw new Error("gameContext should not be null");
	}

	return gameContext;
};
