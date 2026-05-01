import { TerminalClient } from "./terminal.js";
import "./main.css";
await document.fonts.load('14px "CaskaydiaMono Nerd Font"');
const term_element = document.getElementById("term");
if (term_element) {
  new TerminalClient(term_element);
} else {
  console.error("Terminal element not found");
}
