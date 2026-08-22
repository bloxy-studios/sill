// Canvas grid renderer: draws sill-core snapshots imperatively.
// Cells, not DOM — React never re-renders per frame
// (docs/design/performance.md).

import { FLAG, type Color, type Snapshot } from "./types";

export interface Theme {
  background: string;
  foreground: string;
  cursor: string;
  palette: string[]; // 16 ANSI colors
}

export const DEFAULT_THEME: Theme = {
  background: "#101216",
  foreground: "#d8dee9",
  cursor: "#d8dee9",
  palette: [
    "#1c1f26",
    "#e06c75",
    "#98c379",
    "#e5c07b",
    "#61afef",
    "#c678dd",
    "#56b6c2",
    "#c8ccd4",
    "#5c6370",
    "#e06c75",
    "#98c379",
    "#e5c07b",
    "#61afef",
    "#c678dd",
    "#56b6c2",
    "#ffffff",
  ],
};

/** xterm 256-color palette entry → css color. */
function indexed(theme: Theme, i: number): string {
  if (i < 16) return theme.palette[i];
  if (i < 232) {
    const v = i - 16;
    const steps = [0, 95, 135, 175, 215, 255];
    const r = steps[Math.floor(v / 36) % 6];
    const g = steps[Math.floor(v / 6) % 6];
    const b = steps[v % 6];
    return `rgb(${r},${g},${b})`;
  }
  const gray = 8 + (i - 232) * 10;
  return `rgb(${gray},${gray},${gray})`;
}

function css(theme: Theme, c: Color, isFg: boolean): string {
  switch (c.t) {
    case "Default":
      return theme.foreground;
    case "DefaultBg":
      return isFg ? theme.foreground : theme.background;
    case "Indexed":
      return indexed(theme, c.v);
    case "Rgb":
      return `rgb(${c.v[0]},${c.v[1]},${c.v[2]})`;
  }
}

export interface CellMetrics {
  width: number;
  height: number;
  baseline: number;
}

const FONT_STACK =
  'ui-monospace, "SF Mono", Menlo, Consolas, "DejaVu Sans Mono", monospace';

export class GridRenderer {
  private ctx: CanvasRenderingContext2D;
  private dpr = 1;
  readonly fontSize = 13;
  metrics: CellMetrics = { width: 8, height: 18, baseline: 13 };

  constructor(private canvas: HTMLCanvasElement) {
    const ctx = canvas.getContext("2d");
    if (!ctx) throw new Error("2d canvas context unavailable");
    this.ctx = ctx;
    this.measure();
  }

  private font(bold: boolean, italic: boolean): string {
    return `${italic ? "italic " : ""}${bold ? "600 " : ""}${
      this.fontSize * this.dpr
    }px ${FONT_STACK}`;
  }

  private measure() {
    this.dpr = Math.max(1, window.devicePixelRatio || 1);
    this.ctx.font = this.font(false, false);
    const m = this.ctx.measureText("M");
    const width = m.width / this.dpr;
    const height = Math.ceil(this.fontSize * 1.4);
    this.metrics = {
      width,
      height,
      baseline: Math.round(this.fontSize * 1.05),
    };
  }

  /** Size the canvas backing store for a cols×rows grid; returns CSS size. */
  fit(cols: number, rows: number): { cssWidth: number; cssHeight: number } {
    this.measure();
    const cssWidth = Math.ceil(cols * this.metrics.width);
    const cssHeight = rows * this.metrics.height;
    this.canvas.width = Math.ceil(cssWidth * this.dpr);
    this.canvas.height = Math.ceil(cssHeight * this.dpr);
    this.canvas.style.width = `${cssWidth}px`;
    this.canvas.style.height = `${cssHeight}px`;
    return { cssWidth, cssHeight };
  }

  /** How many cells fit into a CSS-pixel box. */
  gridFor(cssWidth: number, cssHeight: number): { cols: number; rows: number } {
    this.measure();
    return {
      cols: Math.max(2, Math.floor(cssWidth / this.metrics.width)),
      rows: Math.max(1, Math.floor(cssHeight / this.metrics.height)),
    };
  }

  draw(snap: Snapshot, theme: Theme = DEFAULT_THEME) {
    const { ctx, dpr } = this;
    const cw = this.metrics.width * dpr;
    const ch = this.metrics.height * dpr;

    ctx.fillStyle = theme.background;
    ctx.fillRect(0, 0, this.canvas.width, this.canvas.height);
    ctx.textBaseline = "alphabetic";

    snap.rows.forEach((row, r) => {
      let col = 0;
      const y = r * ch;
      for (const run of row.runs) {
        const chars = [...run.text];
        const wide = (run.flags & FLAG.WIDE) !== 0;
        const cellSpan = wide ? 2 : 1;
        const runCols = chars.length * cellSpan;

        const inverse = (run.flags & FLAG.INVERSE) !== 0;
        let fg = css(theme, run.fg, true);
        let bg = css(theme, run.bg, false);
        if (inverse) [fg, bg] = [bg, fg];

        if (bg !== theme.background) {
          ctx.fillStyle = bg;
          ctx.fillRect(col * cw, y, runCols * cw, ch);
        }

        if ((run.flags & FLAG.HIDDEN) === 0 && run.text.trim().length > 0) {
          ctx.font = this.font(
            (run.flags & FLAG.BOLD) !== 0,
            (run.flags & FLAG.ITALIC) !== 0,
          );
          ctx.fillStyle = fg;
          ctx.globalAlpha = (run.flags & FLAG.DIM) !== 0 ? 0.6 : 1;
          if (wide) {
            chars.forEach((chr, i) => {
              ctx.fillText(
                chr,
                (col + i * 2) * cw,
                y + this.metrics.baseline * dpr,
              );
            });
          } else {
            ctx.fillText(run.text, col * cw, y + this.metrics.baseline * dpr);
          }
          ctx.globalAlpha = 1;
        }

        if ((run.flags & (FLAG.UNDERLINE | FLAG.STRIKEOUT)) !== 0) {
          ctx.strokeStyle = fg;
          ctx.lineWidth = dpr;
          if ((run.flags & FLAG.UNDERLINE) !== 0) {
            const uy = y + ch - 2 * dpr;
            ctx.beginPath();
            ctx.moveTo(col * cw, uy);
            ctx.lineTo((col + runCols) * cw, uy);
            ctx.stroke();
          }
          if ((run.flags & FLAG.STRIKEOUT) !== 0) {
            const sy = y + ch / 2;
            ctx.beginPath();
            ctx.moveTo(col * cw, sy);
            ctx.lineTo((col + runCols) * cw, sy);
            ctx.stroke();
          }
        }

        col += runCols;
      }
    });

    if (snap.cursor.visible && snap.cursor.row >= 0) {
      ctx.fillStyle = theme.cursor;
      ctx.globalAlpha = 0.85;
      ctx.fillRect(snap.cursor.col * cw, snap.cursor.row * ch, cw, ch);
      ctx.globalAlpha = 1;
    }
  }
}
