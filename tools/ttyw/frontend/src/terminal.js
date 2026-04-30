import { Terminal } from "xterm";
import { FitAddon } from "xterm-addon-fit";
import { ClipboardAddon } from "@xterm/addon-clipboard";
import { WebglAddon } from "@xterm/addon-webgl";
import { ImageAddon } from "@xterm/addon-image";
import { SearchAddon } from "@xterm/addon-search";
export class TerminalClient {
  constructor() {
    this.term = new Terminal();

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
      sixelPaletteLimit: 256, // initial sixel palette size
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
    this.ws = null;
  }

  mount(el) {
    this.term.open(el);
    this.fitAddon.fit();

    // resize handling
    window.addEventListener("resize", () => this.resize());
  }

  connect() {
    // const protocol = location.protocol === "https:" ? "wss" : "ws";
    // const url = `${protocol}://${location.host}/ws`;
    const url = "ws://127.0.0.1:7681/ws";

    this.ws = new WebSocket(url);
    this.ws.binaryType = "arraybuffer";
    this.ws.onopen = () => {
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
      this.ws.send(
        JSON.stringify({
          kind: "input",
          data,
        }),
      );
    });
  }

  handleMessage(raw) {
    let msg;
    try {
      msg = JSON.parse(raw);
    } catch {
      // fallback: treat as plain terminal output
      this.term.write(raw);
      return;
    }

    switch (msg.kind) {
      case "output":
        this.term.write(msg.data);
        break;

      case "set_option":
        this.term.setOption(msg.key, msg.value);
        break;

      case "config":
        for (const [k, v] of Object.entries(msg.options)) {
          this.term.setOption(k, v);
        }
        break;

      case "write":
        this.term.write(msg.data);
        break;
    }
  }

  // optional: manual API for frontend (rarely needed)
  setTheme(theme) {
    this.term.setOption("theme", theme);
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
