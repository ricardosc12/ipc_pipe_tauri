//@ts-ignore
import { For, createEffect, createSignal, onMount } from "solid-js";
import { invoke } from "@tauri-apps/api/tauri";
import style from './style.module.css'

const playerColor = (id: number) => {
	let colors: any = {
		0: "red",
		1: "blue",
		2: "green",
		4: "yellow"
	}
	return colors[id] ? colors[id] : "red"
}

function checkEqualPlayer(newPlayer: Array<Player>, oldPlayer: Array<Player>) {
	return JSON.stringify(newPlayer) == JSON.stringify(oldPlayer)
}

class Player {
	x: number;
	y: number;
	id: number;
	name: string;
	constructor(x: number, y: number, id: number) {
		this.x = x;
		this.y = y;
		this.id = id;
		this.name = `Player ${id}`;
	}
	moveUp() { this.y++; }
	moveDown() { this.y--; }
	moveRight() { this.x++; }
	moveLeft() { this.x--; }
	render(map: HTMLDivElement) {
		if (!map) return
		const rowMap = map.querySelector(`[aria-rowindex="${this.x}"]`)
		const local = rowMap?.querySelector(`[aria-colindex="${this.y}"]`)
		document.getElementById(String(this.id))?.remove()
		const newLocal = document.createElement("DIV")
		newLocal.className = style.player
		newLocal.id = String(this.id);
		newLocal.role = "player";
		newLocal.style.setProperty("--color", playerColor(this.id))
		local?.appendChild(newLocal)
	}
}

// class Goal extends Player { }

interface RustReponse {
	players: Array<Player>
}

function Game() {

	const [players, setPlayers] = createSignal<Array<Player>>([], { equals: checkEqualPlayer })
	const [dimensions, setDimensions] = createSignal([0, 0])
	const [goal, setGoal] = createSignal<Array<number>>()

	let map: any;

	onMount(() => {
		(async () => {
			const map: any = await invoke("init")
			setDimensions(map.dimensions)
			setGoal(map.goal)
		})();

		setInterval(() => {
			(async () => {
				const result: RustReponse = await invoke("greet")

				const resultPlayers: Array<Player> = result.players.map(item => {
					return new Player(item.x, item.y, item.id)
				})

				setPlayers(resultPlayers)
			})();
		})
	})

	createEffect(() => {
		let _players = players()
		let setPlayers = new Set()
		_players.forEach(player => {
			setPlayers.add(Number(player.id))
			player.render(map)
		})
		let renderedPlayers = (map as HTMLElement).querySelectorAll("[role='player']")
		renderedPlayers.forEach(player => {
			if (!setPlayers.has(Number(player.id))) player.remove()
		})
	})

	return (
		<div ref={map} class={style.root}>
			<For each={Array.from(Array(dimensions()[0]))}>
				{(_, indexX) => (
					<div aria-rowindex={indexX()} class={style.line}>
						<For each={Array.from(Array(dimensions()[1]))}>
							{(_, indexY) => (
								<div aria-colindex={indexY()} class={style.column}>
									{(goal()?.[0]==indexX() && goal()?.[1] == indexY())?<div class={style.goal}></div>:''}
								</div>
							)}
						</For>
					</div>
				)}
			</For>
		</div>
	);
}

export default Game;
