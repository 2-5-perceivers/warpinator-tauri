export type RemoteState =
  | { type: "connected" }
  | { type: "disconnected" }
  | { type: "connecting" }
  | { type: "awaiting_duplex" }
  | {
      type: "error";
      content:
        | "ssl_error"
        | "group_code_mismatch"
        | "no_certificate"
        | "duplex_error";
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
  picture: string | null;
  state: RemoteState;
  service_static: boolean;
  service_available: boolean;
}
