//@ts-ignore
import { For, createSignal } from "solid-js";
import { invoke } from "@tauri-apps/api/tauri";

interface Player {
	nome: string;
}

interface RustReponse {
	players: Array<Player>
}

function App() {

	const [players, setPlayers] = createSignal<Array<Player>>([])

	async function greet() {
		// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
		const result: RustReponse = await invoke("greet", { teste: "Hello Server !" })

		console.log(result)

		setPlayers(result.players)
	}

	return (
		<div class="container">
			<h1>Welcome to Tauri!</h1>
			<button onClick={greet}>CALL RUST</button>
			<div>
				<For each={players()}>
					{(player) => <div>{player.nome}</div>}
				</For>
			</div>
		</div>
	);
}

export default App;
