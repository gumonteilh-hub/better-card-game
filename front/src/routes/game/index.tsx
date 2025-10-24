import { createFileRoute, useNavigate } from "@tanstack/react-router";
import { useEffect } from "react";
import { startGame } from "../../game.service";
import { useUserInfo } from "../../utils/useUserInfo";

export const Route = createFileRoute("/game/")({
	component: RouteComponent,
});

function RouteComponent() {
	const { userInfos } = useUserInfo();
	const navigate = useNavigate({ from: "/game" });

	useEffect(() => {
		if (userInfos?.deck?.cards.length === 30) {
			startGame(userInfos.deck).then((id) => {
				navigate({ to: `/game/${id}` });
			});
		}
	}, [navigate, userInfos?.deck]);

	return <>Loading</>;
}
