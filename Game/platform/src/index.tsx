/* @refresh reload */
import { render } from "solid-js/web";

import Game from "./Game";
import './styles.css'

render(() => <Game />, document.getElementById("root") as HTMLElement);
