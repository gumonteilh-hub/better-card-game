import { createFileRoute, Link, useNavigate } from "@tanstack/react-router";
import { useEffect, useRef, useState } from "react";
import { toast } from "sonner";
import { DeckSummary } from "../components/DeckSummary";
import { findCurrentGame } from "../service/game.service";
import { type IUserInfo, useUserInfo } from "../utils/useUserInfo";

export const Route = createFileRoute("/")({
	component: RouteComponent,
});

function RouteComponent() {
	const { userInfos, saveUserInfo } = useUserInfo();
	const [currentGame, setCurrentGame] = useState<string | undefined>();

	useEffect(() => {
		if (userInfos) {
			findCurrentGame(userInfos.userId).then((gameId) => {
				setCurrentGame(gameId);
			});
		}
	}, [userInfos]);

	if (!userInfos) {
		return <>Loading</>;
	}

	return (
		<div className="homepage">
			<div className="menu">
				<div>
					<label>
						Pseudo :
						<input
							onChange={(e) =>
								saveUserInfo({ ...userInfos, name: e.currentTarget.value })
							}
							type="text"
							value={userInfos?.name ?? ""}
						></input>
					</label>
				</div>
				<nav className="navigation-menu">
					<ul>
						{currentGame ? (
							<Link to={"/game/$gameId"} params={{ gameId: currentGame }}>
								Reprendre la partie
							</Link>
						) : (
							<Matchmaking />
						)}
					</ul>
					<ul>
						<Link className="link" to={"/collection"}>
							Ma collection
						</Link>
					</ul>
				</nav>
			</div>
			{userInfos?.deck && (
				<div className="deck-recap">
					<DeckSummary
						archetype={userInfos.deck.archetype}
						cards={userInfos.deck.cards}
					></DeckSummary>
				</div>
			)}
		</div>
	);
}

type MatchmakingMessage =
	| {
		type: "waiting";
	}
	| {
		type: "gameFound";
		gameId: string;
	};

const Matchmaking = () => {
	const { userInfos } = useUserInfo();
	const wsRef = useRef<WebSocket | null>(null);
	const dialogRef = useRef<HTMLDialogElement | null>(null);
	const [open, setOpen] = useState(false);
	const navigate = useNavigate();

	useEffect(() => {
		if (dialogRef.current?.onclose) {
			dialogRef.current.onclose = (_: Event) => {
				setOpen(false);
			};
		}
	});

	useEffect(() => {
		if (!userInfos?.deck || !userInfos.userId || !open) return;

		const ws = new WebSocket(
			`ws://${window.location.host}/matchmaking/${userInfos.userId}`,
		);
		wsRef.current = ws;

		ws.onopen = () => {
			if (!userInfos?.deck) return;
			const deck = {
				archetype: userInfos.deck.archetype,
				cards: userInfos.deck.cards.map((c) => c.id),
			};
			ws.send(
				JSON.stringify({
					type: "joinQueue",
					value: { deck },
				}),
			);
		};

		ws.onmessage = (e: MessageEvent) => {
			const message: MatchmakingMessage = JSON.parse(e.data);

			switch (message.type) {
				case "waiting": {
					break;
				}
				case "gameFound": {
					ws.close();
					navigate({
						to: "/game/$gameId",
						params: { gameId: message.gameId },
					});
					break;
				}
			}
		};

		ws.onerror = (error) => {
			toast.error(`WebSocket error: ${error}`);
		};

		ws.onclose = () => {
			toast.info("WebSocket disconnected from matchmaking");
		};

		return () => {
			if (ws.readyState === WebSocket.OPEN) {
				ws.close();
			}
			wsRef.current = null;
		};
	}, [userInfos?.deck, navigate, open, userInfos?.userId]);

	const handleCancel = () => {
		dialogRef.current?.close();
	};

	return (
		<>
			<button
				type="button"
				className="link link-play"
				disabled={!isDeckValid(userInfos)}
				onClick={() => {
					dialogRef.current?.showModal();
					setOpen(true);
				}}
			>
				Jouer !
			</button>

			<dialog ref={dialogRef} className="matchmaking-dialog">
				<div className="matchmaking-content">
					<h2 className="matchmaking-title">Matchmaking</h2>

					<div className="matchmaking-status">
						<div className="spinner" />
						<p>🔍 Recherche d'adversaire...</p>
					</div>

					<button
						type="button"
						className="cancel-button"
						onClick={handleCancel}
					>
						Annuler
					</button>
				</div>
			</dialog>
		</>
	);
};

const isDeckValid = (userInfos: IUserInfo | undefined): boolean => {
	return (
		userInfos !== undefined &&
		userInfos.name !== undefined &&
		userInfos.deck !== undefined &&
		userInfos.deck.cards.length === 30
	);
};
