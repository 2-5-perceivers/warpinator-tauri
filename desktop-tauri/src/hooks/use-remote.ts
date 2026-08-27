import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";
import { WarpEvent } from "@/types/warp-event";
import { Remote } from "@/types/remote";

export function useRemote(remoteUuid: string | null) {
  const [remote, setRemote] = useState<Remote | null>(null);

  useEffect(() => {
    if (!remoteUuid) {
      setRemote(null);
      return;
    }

    const fetchRemote = async () => {
      const remote = await invoke<Remote | null>("get_remote", {
        remoteUuid,
      });
      if (!remote) return;
      setRemote(remote);
    };

    fetchRemote();

    const unlisten = listen<WarpEvent>("warp-event", async (event) => {
      const ev = event.payload;
      if ("RemoteUpdated" in ev && ev.RemoteUpdated == remoteUuid) {
        await fetchRemote();
      }
    });

    return () => {
      unlisten.then((f) => f());
    };
  }, [remoteUuid]);

  return remote;
}
