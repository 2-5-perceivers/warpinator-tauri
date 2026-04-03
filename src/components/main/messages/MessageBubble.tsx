import { Message } from "@/types/message";
import {
  ContextMenu,
  ContextMenuContent,
  ContextMenuItem,
  ContextMenuTrigger,
} from "@/components/ui/context-menu.tsx";
import { Copy01Icon, Delete02Icon } from "@hugeicons/core-free-icons";
import { HugeiconsIcon } from "@hugeicons/react";
import { toast } from "sonner";
import { writeText } from "@tauri-apps/plugin-clipboard-manager";
import { invoke } from "@tauri-apps/api/core";
import * as linkify from "linkifyjs";

function formatTimestamp(date: Date): string {
  if (Number.isNaN(date.getTime())) return "";
  return date.toLocaleTimeString([], { hour: "2-digit", minute: "2-digit" });
}

export function MessageBubble({
  message,
  groupedWithPrev = false,
  groupedWithNext = false,
}: {
  message: Message;
  groupedWithPrev?: boolean;
  groupedWithNext?: boolean;
}) {
  const isSent = message.direction === "sent";
  const timeLabel = formatTimestamp(message.timestamp);
  const tokenizedContent = (() => {
    const text = message.content ?? "";
    const links = linkify.find(text);
    if (links.length === 0) return text;

    const parts: React.ReactNode[] = [];
    let cursor = 0;

    links.forEach((link, index) => {
      if (link.start > cursor) {
        parts.push(text.slice(cursor, link.start));
      }

      parts.push(
        <a
          key={`link-${message.uuid}-${index}-${link.start}`}
          href={link.href}
          target="_blank"
          rel="noopener noreferrer"
          className="underline underline-offset-2 break-all"
        >
          {link.value}
        </a>,
      );

      cursor = link.end;
    });

    if (cursor < text.length) {
      parts.push(text.slice(cursor));
    }

    return parts;
  })();

  const deleteMessage = async () =>
    await invoke("remove_message", {
      remoteUuid: message.remote_uuid,
      messageUuid: message.uuid,
    });

  const roundingClass = isSent
    ? `${groupedWithPrev ? "rounded-tr-md" : "rounded-tr-2xl"} ${groupedWithNext ? "rounded-br-md" : "rounded-br-none"}`
    : `${groupedWithPrev ? "rounded-tl-md" : "rounded-tl-2xl"} ${groupedWithNext ? "rounded-bl-md" : "rounded-bl-none"}`;

  return (
    <ContextMenu>
      <ContextMenuTrigger
        className={`flex ${isSent ? "justify-end" : "justify-start"} ${groupedWithPrev ? "mt-0.5" : "mt-2"}`}
      >
        <div
          className={`max-w-[75%] rounded-2xl px-3 py-2 shadow-sm whitespace-pre-wrap wrap-break-word ${roundingClass} ${
            isSent
              ? "bg-primary text-primary-foreground"
              : "bg-muted text-foreground"
          }`}
        >
          <p className="text-sm leading-relaxed whitespace-pre-wrap wrap-break-word">
            <span>{tokenizedContent}</span>
            <span className="float-right pl-2 inline-block leading-5 text-xs text-muted-foreground/80 whitespace-nowrap translate-y-1.5">
              {timeLabel}
            </span>
          </p>
        </div>
      </ContextMenuTrigger>
      <ContextMenuContent>
        <ContextMenuItem
          onSelect={() =>
            toast.promise(() => writeText(message.content), {
              success: "Message copied to clipboard",
              error: "Failed to copy message",
            })
          }
        >
          <HugeiconsIcon icon={Copy01Icon} />
          Copy
        </ContextMenuItem>
        <ContextMenuItem variant="destructive" onSelect={deleteMessage}>
          <HugeiconsIcon icon={Delete02Icon} />
          Delete
        </ContextMenuItem>
      </ContextMenuContent>
    </ContextMenu>
  );
}
