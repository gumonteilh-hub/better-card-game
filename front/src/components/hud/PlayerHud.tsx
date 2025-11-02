import { ManaCrystal } from "./ManaCrystal";
import { AttackModeButton, MoveModeButton } from "./SwitchModeButtons";

interface IHudProps {
	currentMana: number;
	maxMana: number;
	side: "player" | "enemy";
}
export const PlayerHud = ({ currentMana, maxMana, side }: IHudProps) => {
	return (
		<div className="hud">
			<div className="hud-left-side">
				<ManaCrystal content={`${currentMana} /${maxMana}`} />
			</div>
			<div className="hud-right-side">
				<AttackModeButton side={side} />
				<MoveModeButton side={side} />
			</div>
		</div>
	);
};
