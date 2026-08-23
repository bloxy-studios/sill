// Sill Phase 3 (first slice): one live terminal session with selection,
// copy, and scrollback search.
// React renders chrome (title bar, search bar, exit overlay); the grid is
// drawn imperatively on canvas from Rust snapshots — no per-frame React
// work. Selection and search STATE live in Rust: this file only reports
// pointer positions and keystrokes (issue #4 layer mandate).

import { useCallback, useEffect, useRef, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import { GridRenderer } from "./lib/renderer";
import { encodeKey } from "./lib/keys";
import type {
  CellSide,
  EventPayload,
  SelectionMode,
  SessionId,
  Snapshot,
  SnapshotPayload,
} from "./lib/types";
import "./App.css";

const IS_MAC = navigator.userAgent.includes("Mac");
/** Lines per tick while dragging a selection past the viewport edge. */
const DRAG_SCROLL_LINES = 2;
const DRAG_SCROLL_INTERVAL_MS = 60;

interface Cell {
  row: number;
  col: number;
  side: CellSide;
}

interface DragState {
  active: boolean;
  lastCell: Cell | null;
  edge: -1 | 0 | 1;
  timer: number | null;
  raf: number;
  pending: Cell | null;
}

function App() {
  const containerRef = useRef<HTMLDivElement>(null);
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const rendererRef = useRef<GridRenderer | null>(null);
  const sessionRef = useRef<SessionId | null>(null);
  const gridRef = useRef({ cols: 80, rows: 24 });
  const dragRef = useRef<DragState>({
    active: false,
    lastCell: null,
    edge: 0,
    timer: null,
    raf: 0,
    pending: null,
  });

  const [title, setTitle] = useState("Sill");
  const [exitCode, setExitCode] = useState<number | null | undefined>(
    undefined,
  );

  // Search UI state. The match state itself lives in Rust; this is only
  // the input field and its not-found indicator (v1 has no counter).
  const [searchOpen, setSearchOpen] = useState(false);
  const [searchMiss, setSearchMiss] = useState(false);
  const [query, setQuery] = useState("");
  const searchInputRef = useRef<HTMLInputElement>(null);
  const queryRef = useRef("");
  queryRef.current = query;

  // Search commands are serialized client-side: two racing
  // session_search_start calls could apply out of order in Rust, leaving
  // the focus on a stale query. A promise chain keeps them deterministic.
  const searchChainRef = useRef<Promise<unknown>>(Promise.resolve());
  const searchSeqRef = useRef(0);

  const runSearch = useCallback((q: string) => {
    const sid = sessionRef.current;
    if (sid === null) return;
    const seq = ++searchSeqRef.current;
    searchChainRef.current = searchChainRef.current
      .then(async () => {
        if (q.length === 0) {
          await invoke("session_search_close", { id: sid });
          if (seq === searchSeqRef.current) setSearchMiss(false);
          return;
        }
        const found = await invoke<boolean>("session_search_start", {
          id: sid,
          query: q,
        });
        if (seq === searchSeqRef.current) setSearchMiss(!found);
      })
      // A failed invoke must not wedge the chain for later searches.
      .catch(() => {});
  }, []);

  const stepSearch = useCallback((forward: boolean) => {
    const sid = sessionRef.current;
    if (sid === null) return;
    const seq = ++searchSeqRef.current;
    searchChainRef.current = searchChainRef.current
      .then(async () => {
        const found = await invoke<boolean>("session_search_next", {
          id: sid,
          forward,
        });
        if (seq === searchSeqRef.current) setSearchMiss(!found);
      })
      .catch(() => {});
  }, []);

  const openSearch = useCallback(() => {
    setSearchOpen(true);
    requestAnimationFrame(() => {
      const input = searchInputRef.current;
      input?.focus();
      input?.select();
    });
    // Reopening with a remembered query resumes that search.
    if (queryRef.current.length > 0) runSearch(queryRef.current);
  }, [runSearch]);

  const closeSearch = useCallback(() => {
    setSearchOpen(false);
    setSearchMiss(false);
    const sid = sessionRef.current;
    if (sid !== null) void invoke("session_search_close", { id: sid });
  }, []);

  useEffect(() => {
    let disposed = false;
    const unlisteners: UnlistenFn[] = [];

    async function boot() {
      const canvas = canvasRef.current;
      const container = containerRef.current;
      if (!canvas || !container) return;

      const renderer = new GridRenderer(canvas);
      rendererRef.current = renderer;

      const rect = container.getBoundingClientRect();
      const { cols, rows } = renderer.gridFor(rect.width, rect.height);
      renderer.fit(cols, rows);
      gridRef.current = { cols, rows };

      const id = await invoke<SessionId>("create_session", { cols, rows });
      if (disposed) return;
      sessionRef.current = id;

      unlisteners.push(
        await listen<SnapshotPayload>("sill://snapshot", ({ payload }) => {
          if (payload.id !== sessionRef.current) return;
          rendererRef.current?.draw(payload.snapshot);
        }),
      );

      unlisteners.push(
        await listen<EventPayload>("sill://session-event", ({ payload }) => {
          const ev = payload.event;
          if (!("id" in ev) || ev.id !== sessionRef.current) return;
          if (ev.kind === "title_changed") {
            setTitle(ev.title.length > 0 ? ev.title : "Sill");
          } else if (ev.kind === "exited") {
            setExitCode(ev.exit_code ?? null);
          }
        }),
      );

      // First paint (shell prompt may have raced ahead of the listener).
      const snap = await invoke<Snapshot>("session_snapshot", { id });
      if (!disposed) {
        renderer.draw(snap);
      }

      // Resize: container box → grid → PTY.
      const ro = new ResizeObserver(() => {
        const r = container.getBoundingClientRect();
        const g = renderer.gridFor(r.width, r.height);
        renderer.fit(g.cols, g.rows);
        gridRef.current = g;
        const sid = sessionRef.current;
        if (sid !== null) {
          void invoke("session_resize", {
            id: sid,
            cols: g.cols,
            rows: g.rows,
          });
        }
      });
      ro.observe(container);
      unlisteners.push(() => ro.disconnect());
    }

    void boot();

    return () => {
      disposed = true;
      for (const un of unlisteners) un();
      const sid = sessionRef.current;
      if (sid !== null) void invoke("session_close", { id: sid });
      sessionRef.current = null;
    };
  }, []);

  // Mouse selection: pointer positions go to Rust, which owns the
  // selection (drag = simple, double-click = semantic, triple-click =
  // lines, Alt = block). Updates are coalesced to one per frame.
  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;

    const cellFromPointer = (e: MouseEvent): Cell | null => {
      const renderer = rendererRef.current;
      if (!renderer) return null;
      const rect = canvas.getBoundingClientRect();
      const g = gridRef.current;
      const x = Math.min(Math.max(e.clientX - rect.left, 0), rect.width - 1);
      const y = Math.min(Math.max(e.clientY - rect.top, 0), rect.height - 1);
      const col = Math.max(
        0,
        Math.min(Math.floor(x / renderer.metrics.width), g.cols - 1),
      );
      const row = Math.max(
        0,
        Math.min(Math.floor(y / renderer.metrics.height), g.rows - 1),
      );
      const side: CellSide =
        x - col * renderer.metrics.width < renderer.metrics.width / 2
          ? "left"
          : "right";
      return { row, col, side };
    };

    const flushUpdate = () => {
      const d = dragRef.current;
      d.raf = 0;
      const sid = sessionRef.current;
      const cell = d.pending;
      d.pending = null;
      if (sid === null || cell === null || !d.active) return;
      d.lastCell = cell;
      void invoke("session_selection_update", {
        id: sid,
        row: cell.row,
        col: cell.col,
        side: cell.side,
      });
    };

    const queueUpdate = (cell: Cell) => {
      const d = dragRef.current;
      d.pending = cell;
      if (d.raf === 0) d.raf = requestAnimationFrame(flushUpdate);
    };

    // Dragging past the top/bottom edge auto-scrolls (into history at the
    // top) while pinning the selection to the edge row.
    const setEdgeScroll = (edge: -1 | 0 | 1) => {
      const d = dragRef.current;
      if (d.edge === edge) return;
      d.edge = edge;
      if (d.timer !== null) {
        clearInterval(d.timer);
        d.timer = null;
      }
      if (edge === 0) return;
      d.timer = window.setInterval(() => {
        const sid = sessionRef.current;
        if (sid === null || !d.active) return;
        void invoke("session_scroll", {
          id: sid,
          delta: edge === -1 ? DRAG_SCROLL_LINES : -DRAG_SCROLL_LINES,
        });
        const g = gridRef.current;
        queueUpdate({
          row: edge === -1 ? 0 : g.rows - 1,
          col: d.lastCell?.col ?? 0,
          side: d.lastCell?.side ?? "left",
        });
      }, DRAG_SCROLL_INTERVAL_MS);
    };

    const onMouseDown = (e: MouseEvent) => {
      if (e.button !== 0) return;
      const sid = sessionRef.current;
      const cell = cellFromPointer(e);
      if (sid === null || cell === null) return;
      e.preventDefault();
      // preventDefault keeps focus where it was — pull it out of the
      // search field explicitly so typing goes back to the terminal.
      const active = document.activeElement;
      if (active instanceof HTMLElement && active.closest(".searchbar")) {
        active.blur();
      }
      const mode: SelectionMode = e.altKey
        ? "block"
        : e.detail >= 3
          ? "lines"
          : e.detail === 2
            ? "semantic"
            : "simple";
      const d = dragRef.current;
      d.active = true;
      d.lastCell = cell;
      void invoke("session_selection_start", {
        id: sid,
        row: cell.row,
        col: cell.col,
        side: cell.side,
        mode,
      });
    };

    const onMouseMove = (e: MouseEvent) => {
      const d = dragRef.current;
      if (!d.active) return;
      const rect = canvas.getBoundingClientRect();
      setEdgeScroll(
        e.clientY < rect.top ? -1 : e.clientY > rect.bottom ? 1 : 0,
      );
      const cell = cellFromPointer(e);
      if (cell) queueUpdate(cell);
    };

    const onMouseUp = () => {
      const d = dragRef.current;
      if (!d.active) return;
      d.active = false;
      setEdgeScroll(0);
    };

    canvas.addEventListener("mousedown", onMouseDown);
    window.addEventListener("mousemove", onMouseMove);
    window.addEventListener("mouseup", onMouseUp);
    const drag = dragRef.current;
    return () => {
      canvas.removeEventListener("mousedown", onMouseDown);
      window.removeEventListener("mousemove", onMouseMove);
      window.removeEventListener("mouseup", onMouseUp);
      if (drag.timer !== null) clearInterval(drag.timer);
      if (drag.raf !== 0) cancelAnimationFrame(drag.raf);
    };
  }, []);

  useEffect(() => {
    const onKeyDown = (e: KeyboardEvent) => {
      const sid = sessionRef.current;
      if (sid === null) return;
      // Keys typed into the search field belong to the search field.
      if (e.target instanceof HTMLElement && e.target.closest(".searchbar")) {
        return;
      }

      const key = e.key.toLowerCase();

      // Copy: Cmd+C on macOS, Ctrl+Shift+C everywhere. Rust makes it a
      // strict no-op without a selection; plain Ctrl+C stays SIGINT.
      const macCopy =
        IS_MAC &&
        e.metaKey &&
        !e.ctrlKey &&
        !e.altKey &&
        !e.shiftKey &&
        key === "c";
      const universalCopy =
        e.ctrlKey && e.shiftKey && !e.metaKey && !e.altKey && key === "c";
      if (macCopy || universalCopy) {
        e.preventDefault();
        void invoke("session_copy_selection", { id: sid });
        return;
      }

      // Search: Cmd+F (macOS) / Ctrl+F (elsewhere) — issue #4 decision.
      const searchChord = IS_MAC
        ? e.metaKey && !e.ctrlKey && !e.altKey && !e.shiftKey && key === "f"
        : e.ctrlKey && !e.metaKey && !e.altKey && !e.shiftKey && key === "f";
      if (searchChord) {
        e.preventDefault();
        openSearch();
        return;
      }

      const data = encodeKey(e);
      if (data !== null) {
        e.preventDefault();
        void invoke("session_input", { id: sid, data });
      }
    };

    const onPaste = (e: ClipboardEvent) => {
      // Pasting into the search field is the browser's business.
      if (e.target instanceof HTMLElement && e.target.closest(".searchbar")) {
        return;
      }
      const sid = sessionRef.current;
      const text = e.clipboardData?.getData("text");
      if (sid === null || !text) return;
      e.preventDefault();
      // Bracketed-paste wrapping happens in Rust against live engine mode —
      // no cached protocol state in the frontend.
      void invoke("session_paste", { id: sid, text });
    };

    const onWheel = (e: WheelEvent) => {
      const sid = sessionRef.current;
      if (sid === null) return;
      e.preventDefault();
      // Wheel up (negative deltaY) → into history (positive delta).
      const lines = Math.max(1, Math.round(Math.abs(e.deltaY) / 40)) * 3;
      void invoke("session_scroll", {
        id: sid,
        delta: e.deltaY < 0 ? lines : -lines,
      });
    };

    window.addEventListener("keydown", onKeyDown);
    window.addEventListener("paste", onPaste);
    window.addEventListener("wheel", onWheel, { passive: false });
    return () => {
      window.removeEventListener("keydown", onKeyDown);
      window.removeEventListener("paste", onPaste);
      window.removeEventListener("wheel", onWheel);
    };
  }, [openSearch]);

  return (
    <div className="app">
      <header className="titlebar" data-tauri-drag-region>
        <span className="title">{title}</span>
      </header>
      <div className="terminal" ref={containerRef}>
        <canvas ref={canvasRef} />
        {searchOpen && (
          <div className="searchbar">
            <input
              ref={searchInputRef}
              className={`search-input${searchMiss ? " miss" : ""}`}
              value={query}
              placeholder="Find in scrollback"
              spellCheck={false}
              autoComplete="off"
              onChange={(e) => {
                setQuery(e.target.value);
                runSearch(e.target.value);
              }}
              onKeyDown={(e) => {
                if (e.key === "Escape") {
                  e.preventDefault();
                  closeSearch();
                } else if (e.key === "Enter") {
                  e.preventDefault();
                  // Enter walks older (into history); Shift+Enter newer.
                  stepSearch(e.shiftKey);
                }
              }}
            />
            {searchMiss && <span className="search-miss">No matches</span>}
          </div>
        )}
        {exitCode !== undefined && (
          <div className="exit-overlay">
            <p>
              session exited
              {exitCode !== null ? ` (code ${exitCode})` : ""}
            </p>
          </div>
        )}
      </div>
    </div>
  );
}

export default App;
