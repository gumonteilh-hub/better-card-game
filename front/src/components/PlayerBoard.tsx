import { Card, CardMiniature, type ICard } from "./Card";
import { CardBack, Deck, HeroPortrait, ManaHud, TrapCardSlot } from "./Hud";
import { Draggable } from "./DragAndDrop";

interface IPlayerBoardProps {
	side: 'player' | 'enemy'
}

export const testCard: ICard = {
	attack: 6,
	cost: 5,
	defense: 2,
	description: 'Une simple carte de test pour configurer le css de la carte',
	effects: [],
	keywords: ['Charge', 'Invisible'],
	name: 'Tester',
	tribut: 'Human'
}

const PlayerBoard = ({ side }: IPlayerBoardProps) => {

	return <div className={`board ${side} `}>
		<div className='left-panel'>
			<div className='hud-slot'>
				<ManaHud current={2} max={9} />
			</div>
			<div className='hero-slot'>
				<HeroPortrait />
			</div>
		</div>
		<div className='middle-panel'>
			<div className='field'>
				<div className='column'>
					<CardMiniature card={testCard} />
					<CardMiniature card={testCard} />
				</div>
				<div className='column'>
					<CardMiniature card={testCard} />
				</div>
				<div className='column'>
					<CardMiniature card={testCard} />
					<CardMiniature card={testCard} />
				</div>
				<div className='column'>
					<CardMiniature card={testCard} />
				</div>
				<div className='column'>
					<CardMiniature card={testCard} />
					<CardMiniature card={testCard} />
				</div>
			</div>
			<div className='hand'>
				{side == 'player' ? <Draggable id='cardTest'>
					<Card card={testCard} />
				</Draggable> : <CardBack></CardBack>}

			</div>
		</div>

		<div className='right-panel'>
			<div className='trap-card-slot'>
				<TrapCardSlot></TrapCardSlot>
			</div>
			<div className="deck-slot">
				<Deck />
			</div>
		</div>
	</div>
}

export default PlayerBoard;
