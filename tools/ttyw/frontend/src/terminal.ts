import { IDisposable, Terminal } from "@xterm/xterm";
import { FitAddon } from "@xterm/addon-fit";
import { ReconnectOverlayAddon } from "./addon/overlay";

export class TerminalClient {
  term: Terminal;
  fitAddon: FitAddon;
  ws!: WebSocket;
  reconnectAttempts: number = 0;
  maxReconnectAttempts = 10;
  reconnectTimer: number | null = null;

  private _reconnectOverlay: ReconnectOverlayAddon;
  private _dataListenerDispose: IDisposable | null = null;
  private _wsGeneration = 0;
  private _resizeHandler = () => this.resize();

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
    this._reconnectOverlay = new ReconnectOverlayAddon();
    this.term.loadAddon(this._reconnectOverlay);
    this.term.open(el);
    this.fitAddon.fit();
    window.addEventListener("resize", this._resizeHandler);
    this._loadHeavyAddons();
    this.connect();
  }
  private async _loadHeavyAddons() {
    const [
      { WebglAddon },
      { ImageAddon },
      { SearchAddon },
      { ClipboardAddon },
    ] = await Promise.all([
      import("@xterm/addon-webgl"),
      import("@xterm/addon-image"),
      import("@xterm/addon-search"),
      import("@xterm/addon-clipboard"),
    ]);

    const webgl = new WebglAddon();
    // WebGL can throw if context creation fails — fall back to canvas
    webgl.onContextLoss(() => webgl.dispose());
    this.term.loadAddon(webgl);

    this.term.loadAddon(new ClipboardAddon());
    this.term.loadAddon(new SearchAddon());
    this.term.loadAddon(
      new ImageAddon({
        enableSizeReports: true,
        pixelLimit: 16777216,
        sixelSupport: true,
        sixelScrolling: true,
        sixelPaletteLimit: 1024,
        sixelSizeLimit: 25000000,
        storageLimit: 128,
        showPlaceholder: true,
        iipSupport: true,
        iipSizeLimit: 20000000,
      }),
    );
  }
  connect() {
    const generation = ++this._wsGeneration;
    const protocol = location.protocol === "https:" ? "wss" : "ws";
    const url = `${protocol}://${location.host}/ws`;
    console.log(url);

    this.ws = new WebSocket(url);
    this.ws.binaryType = "arraybuffer";

    this.ws.onopen = () => {
      if (generation !== this._wsGeneration) return;
      console.log("WS connected");
      this._reconnectOverlay.hide();
      this.reconnectAttempts = 0;

      this._dataListenerDispose?.dispose();
      this._dataListenerDispose = this.term.onData((data) => {
        if (this.ws.readyState === WebSocket.OPEN) {
          this.ws.send(data);
        } else {
          console.warn("WS not open, dropping message");
        }
      });

      this.resize();
    };

    this.ws.onmessage = (event) => {
      if (generation !== this._wsGeneration) return;
      if (typeof event.data === "string") {
        this.handleMessage(event.data);
      } else {
        this.term.write(new Uint8Array(event.data));
      }
    };

    this.ws.onclose = () => {
      if (generation !== this._wsGeneration) return;
      console.warn("WS closed");
      this.scheduleReconnect();
    };

    this.ws.onerror = (err) => {
      if (generation !== this._wsGeneration) return;
      console.error("WS error", err);
      this.ws.close();
    };
  }

  scheduleReconnect() {
    if (this.reconnectTimer) return;

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

  destroy() {
    this._wsGeneration++;
    if (this.reconnectTimer) {
      window.clearTimeout(this.reconnectTimer);
      this.reconnectTimer = null;
    }
    this._dataListenerDispose?.dispose();
    this._dataListenerDispose = null;
    window.removeEventListener("resize", this._resizeHandler);
    this.ws?.close();
    this.term.dispose();
  }
}
