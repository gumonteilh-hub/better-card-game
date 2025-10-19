import { createFileRoute, Link } from "@tanstack/react-router";
import { useId } from "react";
import { DeckSummary } from "../components/DeckSummary";
import { type IUserInfo, useUserInfo } from "../utils/useUserInfo";

export const Route = createFileRoute("/")({
	component: RouteComponent,
});

function RouteComponent() {
	const { userInfos, saveUserInfo } = useUserInfo();

	const isDeckValid = (userInfos: IUserInfo | undefined): boolean => {
		return (
			userInfos !== undefined &&
			userInfos.name !== undefined &&
			userInfos.deck !== undefined &&
			userInfos.deck.cards.length === 30
		);
	};

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
							id={useId()}
							type="text"
							value={userInfos?.name ?? ""}
						></input>
					</label>
				</div>
				<nav className="navigation-menu">
					<ul>
						<Link
							className="link link-play"
							disabled={!isDeckValid(userInfos)}
							to={"/game"}
						>
							Jouer !
						</Link>
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
						faction={userInfos.deck.faction}
						cards={userInfos.deck.cards}
					></DeckSummary>
				</div>
			)}
		</div>
	);
}
