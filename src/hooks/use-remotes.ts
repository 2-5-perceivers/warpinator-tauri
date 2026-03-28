import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";
import { WarpEvent } from "@/types/warp-event";
import { Remote } from "@/types/remote";

const toDataUrl = (bytes: number[]): string => {
  const binary = new Uint8Array(bytes).reduce(
    (acc, byte) => acc + String.fromCharCode(byte),
    "",
  );
  return `data:image/png;base64,${btoa(binary)}`;
};

export function useRemotes() {
  const [remotes, setRemotes] = useState<Remote[]>([]);

  useEffect(() => {
    const fetchRemotes = async () => {
      const list = await invoke<Remote[]>("get_remotes");
      setRemotes(
        list
          .filter((remote) => {
            return !(
              remote.state.type == "error" &&
              remote.state.content == "group_code_mismatch"
            );
          })
          .map((remote) => {
            if (remote.picture) {
              return {
                ...remote,
                picture_data: toDataUrl(remote.picture),
                picture: null,
              };
            } else {
              return { ...remote, picture_data: null };
            }
          }),
      );
    };

    fetchRemotes();

    const unlisten = listen<WarpEvent>("warp-event", async (event) => {
      const ev = event.payload;
      if ("RemoteAdded" in ev || "RemoteUpdated" in ev) {
        await fetchRemotes();
      }
    });

    return () => {
      unlisten.then((f) => f());
    };
  }, []);

  return remotes;
}
