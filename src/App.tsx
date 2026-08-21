// Sill Phase 2: one live terminal session.
// React renders chrome (title bar, exit overlay); the grid itself is drawn
// imperatively on canvas from Rust snapshots — no per-frame React work.

import { useEffect, useRef, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import { GridRenderer } from "./lib/renderer";
import { encodeKey, encodePaste } from "./lib/keys";
import type {
  EventPayload,
  SessionId,
  Snapshot,
  SnapshotPayload,
} from "./lib/types";
import "./App.css";

function App() {
  const containerRef = useRef<HTMLDivElement>(null);
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const rendererRef = useRef<GridRenderer | null>(null);
  const sessionRef = useRef<SessionId | null>(null);
  const snapshotRef = useRef<Snapshot | null>(null);

  const [title, setTitle] = useState("Sill");
  const [exitCode, setExitCode] = useState<number | null | undefined>(
    undefined,
  );

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

      const id = await invoke<SessionId>("create_session", { cols, rows });
      if (disposed) return;
      sessionRef.current = id;

      unlisteners.push(
        await listen<SnapshotPayload>("sill://snapshot", ({ payload }) => {
          if (payload.id !== sessionRef.current) return;
          snapshotRef.current = payload.snapshot;
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
        snapshotRef.current = snap;
        renderer.draw(snap);
      }

      // Resize: container box → grid → PTY.
      const ro = new ResizeObserver(() => {
        const r = container.getBoundingClientRect();
        const g = renderer.gridFor(r.width, r.height);
        renderer.fit(g.cols, g.rows);
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

  useEffect(() => {
    const onKeyDown = (e: KeyboardEvent) => {
      const sid = sessionRef.current;
      if (sid === null) return;
      const data = encodeKey(e);
      if (data !== null) {
        e.preventDefault();
        void invoke("session_input", { id: sid, data });
      }
    };

    const onPaste = (e: ClipboardEvent) => {
      const sid = sessionRef.current;
      const text = e.clipboardData?.getData("text");
      if (sid === null || !text) return;
      e.preventDefault();
      const bracketed = snapshotRef.current?.bracketed_paste ?? false;
      void invoke("session_input", {
        id: sid,
        data: encodePaste(text, bracketed),
      });
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
  }, []);

  return (
    <div className="app">
      <header className="titlebar" data-tauri-drag-region>
        <span className="title">{title}</span>
      </header>
      <div className="terminal" ref={containerRef}>
        <canvas ref={canvasRef} />
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
