import { Terminal } from "xterm";

export class ReconnectOverlayAddon {
  private _overlay: HTMLElement | null = null;
  private _dotsInterval: number | null = null;

  activate(_terminal: Terminal) {
    // Nothing to do on activate; overlay is injected on demand
  }

  dispose() {
    this.hide();
  }

  show(attempt: number, maxAttempts: number, delayMs: number) {
    if (!this._overlay) {
      this._overlay = this._createOverlay();
    }

    const attemptEl = this._overlay.querySelector<HTMLElement>(".rco-attempt")!;
    const barFill = this._overlay.querySelector<HTMLElement>(".rco-bar-fill")!;
    const timerEl = this._overlay.querySelector<HTMLElement>(".rco-timer")!;

    attemptEl.textContent = `attempt ${attempt} of ${maxAttempts}`;

    // Animate progress bar across the delay
    barFill.style.transition = "none";
    barFill.style.width = "0%";
    // Force reflow so the transition reset takes effect before we start
    barFill.getBoundingClientRect();
    barFill.style.transition = `width ${delayMs}ms linear`;
    barFill.style.width = "100%";

    // Countdown timer
    let remaining = Math.ceil(delayMs / 1000);
    timerEl.textContent = `reconnecting in ${remaining}s`;
    if (this._dotsInterval) clearInterval(this._dotsInterval);
    this._dotsInterval = window.setInterval(() => {
      remaining = Math.max(0, remaining - 1);
      timerEl.textContent = `reconnecting in ${remaining}s`;
    }, 1000);

    this._overlay.classList.remove("rco-hidden");
  }

  hide() {
    if (this._dotsInterval) {
      clearInterval(this._dotsInterval);
      this._dotsInterval = null;
    }
    if (this._overlay) {
      this._overlay.classList.add("rco-hidden");
    }
  }

  private _createOverlay(): HTMLElement {
    // Inject styles once
    if (!document.getElementById("rco-styles")) {
      const style = document.createElement("style");
      style.id = "rco-styles";
      style.textContent = `
        .rco-backdrop {
          position: absolute;
          inset: 0;
          display: flex;
          align-items: center;
          justify-content: center;
          background: rgba(10, 11, 15, 0.85);
          backdrop-filter: blur(4px);
          -webkit-backdrop-filter: blur(4px);
          z-index: 9999;
          transition: opacity 0.2s ease;
        }
        .rco-backdrop.rco-hidden {
          opacity: 0;
          pointer-events: none;
        }
        .rco-box {
          display: flex;
          flex-direction: column;
          align-items: center;
          gap: 14px;
          padding: 28px 40px;
          background: #13151c;
          border: 1px solid #7AA2F7;
          border-radius: 6px;
          box-shadow: 0 0 0 1px rgba(122,162,247,0.08), 0 16px 48px rgba(0,0,0,0.8);
          min-width: 280px;
          font-family: "CaskaydiaMono Nerd Font", ui-monospace, monospace;
          color: #A9B1D6;
        }
        .rco-icon {
          width: 36px;
          height: 36px;
          border: 2px solid #3d4f7a;
          border-top-color: #7AA2F7;
          border-radius: 50%;
          animation: rco-spin 0.85s linear infinite;
        }
        @keyframes rco-spin {
          to { transform: rotate(360deg); }
        }
        .rco-title {
          font-size: 14px;
          font-weight: 700;
          color: #ffffff;
          letter-spacing: 0.12em;
          text-transform: uppercase;
        }
        .rco-timer {
          font-size: 12px;
          color: #C0CAF5;
          letter-spacing: 0.02em;
        }
        .rco-attempt {
          font-size: 11px;
          color: #7AA2F7;
          letter-spacing: 0.06em;
        }
        .rco-bar-track {
          width: 100%;
          height: 3px;
          background: #1e2535;
          border-radius: 2px;
          overflow: hidden;
        }
        .rco-bar-fill {
          height: 100%;
          width: 0%;
          background: linear-gradient(90deg, #3d59a1, #7AA2F7);
          border-radius: 2px;
        }
      `;
      document.head.appendChild(style);
    }

    const backdrop = document.createElement("div");
    backdrop.className = "rco-backdrop rco-hidden";
    backdrop.innerHTML = `
      <div class="rco-box">
        <div class="rco-icon"></div>
        <div class="rco-title">Connection lost</div>
        <div class="rco-timer">reconnecting…</div>
        <div class="rco-attempt"></div>
        <div class="rco-bar-track">
          <div class="rco-bar-fill"></div>
        </div>
      </div>
    `;

    // The terminal's containing element is the first parent with position != static
    // xterm renders into a div; we want to overlay that exact container.
    // We attach to document.body as a fallback, but ideally to the terminal wrapper.
    document.body.appendChild(backdrop);
    return backdrop;
  }

  /** Attach overlay to a specific container instead of body */
  attachTo(container: HTMLElement) {
    if (this._overlay && this._overlay.parentElement !== container) {
      container.style.position ||= "relative";
      container.appendChild(this._overlay);
    }
  }
}
