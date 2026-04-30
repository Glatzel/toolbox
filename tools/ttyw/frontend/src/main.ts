import "xterm/css/xterm.css";
import { TerminalClient } from "./terminal.js";
const term_element = document.getElementById("term");
if (term_element) {
  new TerminalClient(term_element);
} else {
  console.error("Terminal element not found");
}
