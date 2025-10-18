import dragon from '../assets/hero-dragon.png'
import sword from '../assets/sword.svg'
import shield from '../assets/shield.svg'

export const CardBack = () => {
	return <div className="card-back">
		<div className="spiral"></div>
	</div>
}

const CardMiniatureBack = () => {
	return <div className='card-miniature-back'>
		<div className="spiral"></div>
	</div>
}


export const Deck = () => {
	return <div className="deck">
		<CardBack></CardBack>
		<CardBack></CardBack>
		<CardBack></CardBack>
	</div>
}

export const HeroPortrait = () => {
	return <div className='hero-portrait '>
		<div className='hero-image-slot'>
			<img className='untransformed' src={dragon} />
		</div>
		<div className='hero-name-slot untransformed'>
			<p>Utilisateur</p>
		</div>

	</div>
}


export const TrapCardSlot = () => {

	return <div className="trap-card-placeholder">
		<CardMiniatureBack />
	</div>
}

interface IManaHudProps {
	current: number,
	max: number,
}

export const ManaHud = ({ current, max }: IManaHudProps) => {
	return <div className='mana-crystal untransformed'>{current} /{max}</div>
}

interface IFieldSlotProps {
	type: 'attack' | 'defense' | 'both'
}

export const FieldSlot = ({ type }: IFieldSlotProps) => {
	return <div className={`field-slot ${type}`}>
		{(type == 'defense' || type == 'both') && <img className='shield' src={shield} />}
		{(type == 'attack' || type == 'both') && <img className='sword' src={sword} />}
	</div>
}
