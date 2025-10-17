import { DndContext } from '@dnd-kit/core'
import './css/App.css'
import './css/Hud.css'
import './css/Card.css'
import './css/Board.css'
import PlayerBoard from './components/PlayerBoard'

function App() {

	function handleClick(): void {
		fetch("api/start", {
			method: "POST",
			headers: {
				"Content-Type": "application/json",
			},
			body: JSON.stringify({
				faction: "HUMAN",
				cards: [],
			}),
		}).then(res => {
			res.json().then(data => {
				console.log(data);
			})
		}).catch(err => {
			console.log(err)
		})
	}

	return (
		<div className='main'>
			<DndContext autoScroll={false}>
				<PlayerBoard side='enemy'></PlayerBoard>
				<div className='middle-part'>
					<button onClick={handleClick}>end turn</button>
				</div>
				<PlayerBoard side='player'></PlayerBoard>
			</DndContext >
		</div>
	)
}


export default App
