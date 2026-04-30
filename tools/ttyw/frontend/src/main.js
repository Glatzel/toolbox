import "xterm/css/xterm.css";
import { TerminalClient } from "./terminal.js";
const term = new TerminalClient();
term.mount(document.getElementById("term"));
term.connect();
