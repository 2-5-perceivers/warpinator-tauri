import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";

export type RemoteState =
  | { type: "Connected" }
  | { type: "Disconnected" }
  | { type: "Connecting" }
  | { type: "AwaitingDuplex" }
  | {
      type: "Error";
      content:
        | "SslError"
        | "GroupCodeMismatch"
        | "NoCertificate"
        | "DuplexError";
    };

export interface Remote {
  uuid: string;
  ip: string;
  port: number;
  auth_port: number;
  service_name: string;
  display_name: string;
  username: string;
  hostname: string;
  picture: number[] | null;
  picture_data: string | null;
  state: RemoteState;
  service_static: boolean;
  service_available: boolean;
}

export type WarpEvent =
  | { RemoteAdded: string }
  | { RemoteUpdated: string }
  | { TransferAdded: [string, string] }
  | { TransferUpdated: [string, string] }
  | { TransferRemoved: [string, string] };

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
        list.map((remote) => {
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
