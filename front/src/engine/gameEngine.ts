import { useNavigate } from "@tanstack/react-router";
import { useCallback, useEffect, useRef, useState } from "react";
import { toast } from "sonner";
import type { ActionType, IAction } from "../types/action";
import type { IGameState, ServerMessage } from "../types/game";
import {
	type AnimationState,
	computeAnimationState,
	getAnimationDuration,
} from "./animationEngine";
import { applyAction } from "./gameStateReducer";

const animationBefore: ActionType[] = ["Destroy", "Win"];

type PlayerActionCommand =
	| {
			type: "playMonster";
			value: {
				cardId: number;
				position: number;
			};
	  }
	| {
			type: "playSpell";
			value: {
				cardId: number;
			};
	  }
	| {
			type: "endTurn";
	  }
	| {
			type: "attack";
			value: {
				initiator: number;
				target: number | string;
			};
	  }
	| {
			type: "move";
			value: {
				cardId: number;
				position: number;
			};
	  };

export const useGameEngine = (userId: string) => {
	const [gameState, setGameState] = useState<IGameState>();
	const [actionQueue, setActionQueue] = useState<IAction[]>([]);
	const [isAnimating, setIsAnimating] = useState(false);
	const [animationMap, setAnimationMap] = useState<Map<number, AnimationState>>(
		new Map(),
	);
	const navigate = useNavigate({ from: "game/$userId/" });
	const wsRef = useRef<WebSocket | null>(null);

	useEffect(() => {
		console.log(userId);
		const ws = new WebSocket(`ws://${window.location.host}/game/${userId}`);
		wsRef.current = ws;

		ws.onmessage = (e: MessageEvent) => {
			const action: ServerMessage = JSON.parse(e.data);

			switch (action.type) {
				case "action": {
					console.log(action.value);
					setActionQueue((prev) => [...prev, action.value]);
					break;
				}
				case "error": {
					toast.error(action.value);
					break;
				}
				case "message": {
					toast.message(action.value);
				}
			}
		};

		return () => {
			ws.close();
			wsRef.current = null;
		};
	}, [userId]);

	const attack = useCallback((initiator: number, target: number | string) => {
		if (wsRef.current) {
			wsRef.current.send(
				JSON.stringify({
					type: "attack",
					value: { initiator, target },
				} satisfies PlayerActionCommand),
			);
		}
	}, []);

	const playMonster = useCallback((cardId: number, position: number) => {
		if (wsRef.current) {
			wsRef.current.send(
				JSON.stringify({
					type: "playMonster",
					value: { cardId, position },
				} satisfies PlayerActionCommand),
			);
		}
	}, []);

	const move = useCallback((cardId: number, position: number) => {
		if (wsRef.current) {
			wsRef.current.send(
				JSON.stringify({
					type: "move",
					value: { cardId, position },
				} satisfies PlayerActionCommand),
			);
		}
	}, []);

	const playSpell = useCallback((cardId: number) => {
		if (wsRef.current) {
			wsRef.current.send(
				JSON.stringify({
					type: "playSpell",
					value: { cardId },
				} satisfies PlayerActionCommand),
			);
		}
	}, []);

	const endTurn = useCallback(() => {
		if (wsRef.current) {
			wsRef.current.send(
				JSON.stringify({
					type: "endTurn",
				} satisfies PlayerActionCommand),
			);
		}
	}, []);

	useEffect(() => {
		if (!(gameState?.winnerId === undefined || gameState?.winnerId === null)) {
			navigate({ to: "/" });
		}
	}, [gameState?.winnerId, navigate]);

	useEffect(() => {
		if (
			!gameState &&
			actionQueue.length === 1 &&
			actionQueue[0].type === "UpdateGameView"
		) {
			setGameState(actionQueue[0].value.game);
			setActionQueue([]);
		}
		if (gameState && !isAnimating) {
			if (actionQueue.length > 0) {
				setIsAnimating(true);
				const processedActions: number[] = [0];
				const currentType = actionQueue[0].type;
				const group: IAction[] = [actionQueue[0]];

				let i = 1;
				while (i < actionQueue.length && actionQueue[i].type === currentType) {
					processedActions.push(i);
					group.push(actionQueue[i]);
					i++;
				}

				let intermediateState = { ...gameState };
				for (const action of group) {
					intermediateState = applyAction(intermediateState, action);
				}
				console.log({ intermediateState });
				console.log({ currentType });

				const isAnimationBefore = animationBefore.includes(currentType);

				if (!isAnimationBefore) {
					setGameState(intermediateState);
				}
				setActionQueue((prev) =>
					prev.filter((_, i) => !processedActions.includes(i)),
				);
				setAnimationMap(computeAnimationState(group, intermediateState));

				const animationDuration = getAnimationDuration(currentType);

				setTimeout(() => {
					if (isAnimationBefore) {
						setGameState(intermediateState);
					}
					setAnimationMap(new Map());
					setIsAnimating(false);
				}, animationDuration);
			}
		}
	}, [actionQueue, gameState, isAnimating]);

	return {
		isAnimating,
		gameState,
		endTurn,
		attack,
		playMonster,
		playSpell,
		move,
		animationMap,
	};
};
