//@ts-ignore
import { For, createEffect, createSignal, onMount } from "solid-js";
import { invoke } from "@tauri-apps/api/tauri";
import style from './style.module.css'

class Player {
	x: number;
	y: number;
	color: string;
	local: HTMLDivElement | void;
	constructor(x: number, y: number, color: string) {
		this.x = x;
		this.y = y;
		this.color = color;
		this.local = undefined;
	}
	moveUp() { this.y++; }
	moveDown() { this.y--; }
	moveRight() { this.x++; }
	moveLeft() { this.x--; }
	render(map: HTMLDivElement) {
		if (this.local) this.local.remove()
		const newLocal = document.createElement("DIV")
		newLocal.className = style.player
		newLocal.style.setProperty("--color", this.color)
		const rowMap = map.querySelector(`[aria-rowindex="${this.x}"]`)
		const local = rowMap?.querySelector(`[aria-colindex="${this.y}"]`)
		local?.appendChild(newLocal)
		this.local = newLocal as HTMLDivElement
	}
}

class Goal extends Player { }

interface RustReponse {
	players: Array<Player>
}

function Game() {

	const [players, setPlayers] = createSignal<Array<Player>>([])
	const [dimensions, setDimensions] = createSignal([0, 0])

	let map: any;

	async function greet() {
		// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
		const result: RustReponse = await invoke("greet", { teste: "Hello Server !" })

		console.log(result)

		setPlayers(result.players)
	}

	onMount(() => {
		(async () => {
			const dimensions: Array<number> = await invoke("init")
			setDimensions(dimensions)
		})();
	})

	// setTimeout(() => {
	// 	const player = new Player(0, 1, "red")
	// 	const player2 = new Player(0, 1, "blue")
	// 	player.render(map)
	// 	player2.render(map)

	// }, 1000);

	return (
		<div ref={map} class={style.root}>
			<button onclick={greet}>GET PLAYERS</button>
			<For each={Array.from(Array(dimensions()[0]))}>
				{(_, index) => (
					<div aria-rowindex={index()} class={style.line}>
						<For each={Array.from(Array(dimensions()[1]))}>
							{(_, index) => (
								<div aria-colindex={index()} class={style.column}></div>
							)}
						</For>
					</div>
				)}
			</For>
		</div>
	);
}

export default Game;
