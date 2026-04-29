import { Terminal } from "xterm";
import { FitAddon } from "xterm-addon-fit";
import "xterm/css/xterm.css";

// ===== TERMINAL =====
const term = new Terminal({
  cursorBlink: true,
  fontFamily: "CaskaydiaMono Nerd Font",
});

const fit = new FitAddon();
term.loadAddon(fit);

const container = document.getElementById("term");
term.open(container);
fit.fit();

// ===== CONNECTION =====
const protocol = location.protocol === "https:" ? "wss" : "ws";
const ws = new WebSocket(`${protocol}://${location.host}/ws`);
ws.binaryType = "arraybuffer";

// ===== DATA FLOW =====

// backend → terminal
ws.onmessage = (e) => {
  if (typeof e.data === "string") {
    term.write(e.data);
  } else {
    term.write(new Uint8Array(e.data));
  }
};

// terminal → backend
term.onData((data) => {
  ws.send(data);
});

// ===== RESIZE =====
function sendResize() {
  if (ws.readyState !== WebSocket.OPEN) return;

  ws.send(
    JSON.stringify({
      type: "resize",
      cols: term.cols,
      rows: term.rows,
    }),
  );
}

window.addEventListener("resize", () => {
  fit.fit();
  sendResize();
});

// initial resize (wait for WS open)
ws.onopen = () => {
  sendResize();
};
