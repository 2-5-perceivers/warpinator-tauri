export type WarpEvent =
  | { RemoteAdded: string }
  | { RemoteUpdated: string }
  | { TransferAdded: [string, string] }
  | { TransferUpdated: [string, string] }
  | { TransferRemoved: [string, string] }
  | { MessageAdded: [string, string] }
  | { MessageRemoved: [string, string] };
