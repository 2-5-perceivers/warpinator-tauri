import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";
import { WarpEvent } from "@/types/warp-event";
import { Message } from "@/types/message.ts";

export function useMessages(remoteUuid: string | null) {
  const [messages, setMessages] = useState<Message[]>([]);

  useEffect(() => {
    if (!remoteUuid) {
      return;
    }

    const fetchMessages = async () => {
      const messages = await invoke<Message[]>("get_messages", {
        remoteUuid,
      });
      if (!messages) return;
      setMessages(
        messages.map((message) => {
          return { ...message, timestamp: new Date(message.timestamp) };
        }),
      );
    };

    fetchMessages();

    const unlisten = listen<WarpEvent>("warp-event", async (event) => {
      const ev = event.payload;
      if ("MessageAdded" in ev || "MessageRemoved" in ev) {
        await fetchMessages();
      }
    });

    return () => {
      unlisten.then((f) => f());
    };
  }, [remoteUuid]);

  return messages;
}
