export type MessageDirection = "sent" | "received";

export interface Message {
  uuid: string;
  remote_uuid: string;
  direction: MessageDirection;
  timestamp: Date;
  content: string;
}
