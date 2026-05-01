import { TerminalClient } from "./terminal.js";
import "./main.css";
const term_element = document.getElementById("term");
if (term_element) {
  new TerminalClient(term_element);
} else {
  console.error("Terminal element not found");
}
