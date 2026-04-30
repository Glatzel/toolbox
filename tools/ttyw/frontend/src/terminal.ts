import { Terminal } from "@xterm/xterm";
import { FitAddon } from "@xterm/addon-fit";
import { ClipboardAddon } from "@xterm/addon-clipboard";
import { WebglAddon } from "@xterm/addon-webgl";
import { ImageAddon } from "@xterm/addon-image";
import { SearchAddon } from "@xterm/addon-search";
import { ReconnectOverlayAddon } from "./addon/overlay";

export class TerminalClient {
  term: Terminal;
  fitAddon: FitAddon;
  ws!: WebSocket;
  reconnectAttempts: number = 0;
  maxReconnectAttempts = 10;
  reconnectTimer: number | null = null;

  private _reconnectOverlay: ReconnectOverlayAddon;

  constructor(el: HTMLElement) {
    this.term = new Terminal({
      cursorBlink: true,
      cursorStyle: "block",
      cursorInactiveStyle: "none",
      fontSize: 14,
      fontFamily: "CaskaydiaMono Nerd Font,ui-monospace",
      fontWeight: "normal",
      ignoreBracketedPasteMode: true,
      theme: {
        background: "#1D2026",
        selectionBackground: "#33467C",
        black: "#15161E",
        red: "#F7768E",
        green: "#9ECE6A",
        yellow: "#E0AF68",
        blue: "#7AA2F7",
        magenta: "#BB9AF7",
        cyan: "#7DCFFF",
        white: "#A9B1D6",
        brightBlack: "#5F6A99",
        brightRed: "#F7768E",
        brightGreen: "#9ECE6A",
        brightYellow: "#E0AF68",
        brightBlue: "#7AA2F7",
        brightMagenta: "#BB9AF7",
        brightCyan: "#7DCFFF",
        brightWhite: "#C0CAF5",
      },
    });

    this.fitAddon = new FitAddon();
    this.term.loadAddon(this.fitAddon);

    this.term.loadAddon(new WebglAddon());
    this.term.loadAddon(new ClipboardAddon());
    this.term.loadAddon(new SearchAddon());
    const customSettings = {
      enableSizeReports: true, // whether to enable CSI t reports (see below)
      pixelLimit: 16777216, // max. pixel size of a single image
      sixelSupport: true, // enable sixel support
      sixelScrolling: true, // whether to scroll on image output
      sixelPaletteLimit: 1024, // initial sixel palette size
      sixelSizeLimit: 25000000, // size limit of a single sixel sequence
      storageLimit: 128, // FIFO storage limit in MB
      showPlaceholder: true, // whether to show a placeholder for evicted images
      iipSupport: true, // enable iTerm IIP support
      iipSizeLimit: 20000000, // size limit of a single IIP sequence
      kittySupport: true, // enable Kitty graphics support
      kittySizeLimit: 20000000, // size limit of a single Kitty sequence
    };
    const imageAddon = new ImageAddon(customSettings);
    this.term.loadAddon(imageAddon);

    this._reconnectOverlay = new ReconnectOverlayAddon();
    this.term.loadAddon(this._reconnectOverlay);

    this.term.open(el);
    this.fitAddon.fit();
    window.addEventListener("resize", () => this.resize());
    this.connect();
  }
  scheduleReconnect() {
    if (this.reconnectTimer) return; // prevent duplicates

    if (this.reconnectAttempts >= this.maxReconnectAttempts) {
      console.error("Max reconnect attempts reached");
      return;
    }

    const delay = Math.min(1000 * 2 ** this.reconnectAttempts, 10000);
    console.log(`Reconnecting in ${delay} ms`);

    this._reconnectOverlay.show(
      this.reconnectAttempts + 1,
      this.maxReconnectAttempts,
      delay,
    );

    this.reconnectTimer = window.setTimeout(() => {
      this.reconnectTimer = null;
      this.reconnectAttempts++;
      this.connect();
    }, delay);
  }
  connect() {
    const protocol = location.protocol === "https:" ? "wss" : "ws";
    const url = `${protocol}://${location.host}/ws`;
    console.log(url);

    this.ws = new WebSocket(url);
    this.ws.binaryType = "arraybuffer";

    this.ws.onopen = () => {
      console.log("WS connected");
      this._reconnectOverlay.hide();
      this.reconnectAttempts = 0;
      this.resize();
    };

    // backend → terminal
    this.ws.onmessage = (event) => {
      if (typeof event.data === "string") {
        this.handleMessage(event.data);
      } else {
        this.term.write(new Uint8Array(event.data));
      }
    };

    // terminal → backend
    this.term.onData((data) => {
      if (this.ws && this.ws.readyState === WebSocket.OPEN) {
        this.ws.send(data);
      } else {
        console.warn("WS not open, dropping message");
      }
    });

    this.ws.onclose = () => {
      console.warn("WS closed");
      this.scheduleReconnect();
    };

    this.ws.onerror = (err) => {
      console.error("WS error", err);
      this.ws.close(); // ensures onclose fires
    };
  }

  handleMessage(raw: string) {
    let msg;
    try {
      msg = JSON.parse(raw);
      switch (msg.kind) {
        case "config":
          this.term.options = { ...this.term.options, ...msg.config };
          break;

        default:
          console.warn(`Unknown message kind: ${msg.kind}`);
          break;
      }
    } catch {
      // fallback: treat as plain terminal output
      this.term.write(raw);
    }
  }
  resize() {
    if (!this.ws || this.ws.readyState !== WebSocket.OPEN) {
      console.log("WebSocket not open, cannot resize");
      return;
    }
    this.fitAddon.fit();
    console.log(`Resizing: cols=${this.term.cols}, rows=${this.term.rows}`);
    this.ws.send(
      JSON.stringify({
        kind: "resize",
        cols: this.term.cols,
        rows: this.term.rows,
      }),
    );
  }
}
