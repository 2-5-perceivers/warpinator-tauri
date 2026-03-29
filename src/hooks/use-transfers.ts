import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";
import { WarpEvent } from "@/types/warp-event";
import { Transfer } from "@/types/transfer.ts";

export function useTransfers(remoteUuid: string | null) {
  const [transfers, setTransfers] = useState<Transfer[]>([]);

  useEffect(() => {
    if (!remoteUuid) {
      return;
    }

    const fetchTransfers = async () => {
      const transfers = await invoke<Transfer[]>("get_transfers", {
        remoteUuid,
      });
      if (!transfers) return;
      setTransfers(
        transfers.map((transfer) => {
          return { ...transfer, timestamp: new Date(transfer.timestamp) };
        }),
      );
    };

    fetchTransfers();

    const unlisten = listen<WarpEvent>("warp-event", async (event) => {
      const ev = event.payload;
      if ("TransferAdded" in ev || "TransferUpdated" in ev) {
        await fetchTransfers();
      }
    });

    return () => {
      unlisten.then((f) => f());
    };
  }, [remoteUuid]);

  return transfers;
}
