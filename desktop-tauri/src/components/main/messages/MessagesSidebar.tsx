import {
  Sidebar,
  SidebarContent,
  SidebarFooter,
  SidebarHeader,
} from "@/components/ui/sidebar.tsx";
import { MessagesEmpty } from "./MessagesEmpty";
import { useMessages } from "@/hooks/use-messages.ts";
import { MessageBubble } from "./MessageBubble";
import { Field, FieldLabel } from "@/components/ui/field";
import { Button } from "@/components/ui/button.tsx";
import { HugeiconsIcon } from "@hugeicons/react";
import { SentIcon } from "@hugeicons/core-free-icons";
import { Textarea } from "@/components/ui/textarea.tsx";
import { invoke } from "@tauri-apps/api/core";
import { useEffect, useMemo, useRef, useState } from "react";

export function MessagesSidebar({
  selectedRemoteUuid,
}: {
  selectedRemoteUuid: string;
}) {
  const messages = useMessages(selectedRemoteUuid);

  const listRef = useRef<HTMLDivElement | null>(null);
  const sortedMessages = useMemo(
    () =>
      [...messages].sort(
        (a, b) =>
          new Date(a.timestamp).getTime() - new Date(b.timestamp).getTime(),
      ),
    [messages],
  );
  const groupedMessages = useMemo(
    () =>
      sortedMessages.map((message, index) => {
        const prev = sortedMessages[index - 1];
        const next = sortedMessages[index + 1];

        return {
          message,
          groupedWithPrev: !!prev && prev.direction === message.direction,
          groupedWithNext: !!next && next.direction === message.direction,
        };
      }),
    [sortedMessages],
  );

  const [draft, setDraft] = useState("");
  const textareaRef = useRef<HTMLTextAreaElement | null>(null);

  const sendMessage = async (message: string) => {
    await invoke("send_message", {
      remoteUuid: selectedRemoteUuid,
      content: message,
    });
  };

  const handleSend = async () => {
    const content = draft.trim();
    if (!content) return;

    await sendMessage(content);
    setDraft("");

    if (textareaRef.current) {
      textareaRef.current.style.height = "auto";
    }
  };

  useEffect(() => {
    const el = listRef.current;
    if (!el) return;
    el.scrollTop = el.scrollHeight;
  }, [sortedMessages.length, selectedRemoteUuid]);

  return (
    <Sidebar side={"right"} className="mt-11 h-auto">
      <SidebarHeader className="h-11 flex-row items-center px-3.5 gap-3">
        <span className="text-l font-medium">Messages</span>
      </SidebarHeader>
      <SidebarContent className="px-1.5 flex-1 overflow-hidden">
        {sortedMessages.length === 0 ? (
          <MessagesEmpty />
        ) : (
          <div
            ref={listRef}
            className="flex flex-col h-full overflow-y-auto pr-1 py-2"
          >
            {groupedMessages.map(
              ({ message, groupedWithPrev, groupedWithNext }) => (
                <MessageBubble
                  key={message.uuid}
                  message={message}
                  groupedWithPrev={groupedWithPrev}
                  groupedWithNext={groupedWithNext}
                />
              ),
            )}
          </div>
        )}
      </SidebarContent>
      <SidebarFooter className="border-t border-border p-2.5">
        <Field orientation="horizontal" className="items-end">
          <FieldLabel htmlFor="input-button-group" className="sr-only">
            Search
          </FieldLabel>
          <Textarea
            id="input-button-group"
            placeholder="Message..."
            rows={1}
            className="min-h-auto max-h-24 h-auto resize-none"
            value={draft}
            ref={textareaRef}
            onChange={(e) => setDraft(e.currentTarget.value)}
            onKeyDown={(e) => {
              if (e.key === "Enter" && e.ctrlKey) {
                e.preventDefault();
                void handleSend();
              }
            }}
            onInput={(e) => {
              const el = e.currentTarget;
              el.style.height = "auto";
              el.style.height = `${el.scrollHeight}px`;
            }}
          />
          <Button size="icon-lg" variant="default" onClick={handleSend}>
            <HugeiconsIcon
              icon={SentIcon}
              className="translate-y-px -translate-x-px"
            />
          </Button>
        </Field>
      </SidebarFooter>
    </Sidebar>
  );
}
