import { useState, useEffect } from "react";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/tauri";

/**
 * Go サイドカーが立ち上げた HTTP API のベース URL を取得するフック。
 *
 * 1. listen('backend-ready', ...) でポートを受け取る (主経路)
 * 2. もしイベントを取り損ねた場合は invoke('get_backend_port') で 300ms ごとにポーリング
 */
export function useBackend() {
  const [apiBase, setApiBase] = useState<string>("");
  const [isReady, setIsReady] = useState(false);

  useEffect(() => {
    let cancelled = false;

    function applyPort(port: number) {
      if (!cancelled) {
        setApiBase(`http://localhost:${port}`);
        setIsReady(true);
      }
    }

    const unlistenPromise = listen<number>("backend-ready", (event) => {
      applyPort(event.payload);
    });

    const poll = setInterval(async () => {
      try {
        const port = await invoke<number | null>("get_backend_port");
        if (port) {
          applyPort(port);
          clearInterval(poll);
        }
      } catch {
        // Tauri コマンドが使えない環境 (ブラウザでの開発時等) は無視
      }
    }, 300);

    return () => {
      cancelled = true;
      clearInterval(poll);
      unlistenPromise.then((fn) => fn());
    };
  }, []);

  return { apiBase, isReady };
}
