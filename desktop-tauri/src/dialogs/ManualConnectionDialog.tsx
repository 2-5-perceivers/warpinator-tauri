import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog.tsx";
import {
  getRegistrationInfo,
  RegistrationInfo,
} from "@/lib/registration-info.ts";
import { useEffect, useState } from "react";
import QRCode from "react-qr-code";
import { Button } from "@/components/ui/button.tsx";
import { HugeiconsIcon } from "@hugeicons/react";
import { Copy01Icon } from "@hugeicons/core-free-icons";
import { toast } from "sonner";
import { writeText } from "@tauri-apps/plugin-clipboard-manager";
import {
  InputGroup,
  InputGroupAddon,
  InputGroupInput,
  InputGroupText,
} from "@/components/ui/input-group.tsx";
import { invoke } from "@tauri-apps/api/core";

type DialogMode = "qr" | "input";

export type ConnectRemoteError =
  | "certificate_error"
  | "tls_error"
  | "ping_error"
  | "duplex_error"
  | "remote_worker_not_found";

export type ManualConnectionError =
  | "invalid_url"
  | "failed_to_register"
  | "unavailable"
  | "remote_internal"
  | "remote_unimplemented"
  | "already_connecting"
  | "already_connected"
  | { failed_to_connect: ConnectRemoteError };

function formatConnectRemoteError(error: ConnectRemoteError): string {
  switch (error) {
    case "certificate_error":
      return "Failed to receive certificate";
    case "tls_error":
      return "Failed to establish secure channel";
    case "ping_error":
      return "Remote did not respond to ping";
    case "duplex_error":
      return "Failed to establish duplex connection";
    case "remote_worker_not_found":
      return "Remote service not found";
  }
}

export function formatManualConnectionError(
  error: ManualConnectionError,
): string {
  switch (error) {
    case "invalid_url":
      return "Invalid URL";
    case "failed_to_register":
      return "Failed to register with remote";
    case "unavailable":
      return "Remote is unavailable";
    case "remote_internal":
      return "Remote encountered an internal error";
    case "remote_unimplemented":
      return "Remote does not support manual connections";
    case "already_connecting":
      return "Connection already in progress";
    case "already_connected":
      return "Already connected to this remote";
    default:
      if (
        typeof error === "object" &&
        error !== null &&
        "failed_to_connect" in error
      ) {
        return formatConnectRemoteError(error.failed_to_connect);
      }
      return "Unknown error";
  }
}

export function ManualConnectionDialog({
  open,
  onOpenChange,
}: {
  open: boolean;
  onOpenChange: (open: boolean) => void;
}) {
  const [mode, setMode] = useState<DialogMode>("qr");

  useEffect(() => {
    if (!open) {
      setMode("qr");
    }
  }, [open]);

  switch (mode) {
    case "qr":
      return (
        <QRDialog open={open} onOpenChange={onOpenChange} setMode={setMode} />
      );
    case "input":
      return <InputDialog open={open} onOpenChange={onOpenChange} />;
  }
}

function QRDialog({
  open,
  onOpenChange,
  setMode,
}: {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  setMode: (value: DialogMode) => void;
}) {
  const [registrationInfo, setRegistrationInfo] = useState<RegistrationInfo>();

  useEffect(() => {
    const fetchRegistrationInfo = async () => {
      const registrationInfo = await getRegistrationInfo();
      setRegistrationInfo(registrationInfo);
    };

    fetchRegistrationInfo();
  }, []);

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Manual Connection</DialogTitle>
          <DialogDescription>
            Scan the following QR code and open the link on other devices to
            initiate a connection, or share the provided IP address.
          </DialogDescription>
        </DialogHeader>

        {registrationInfo && (
          <div className="flex flex-col items-center">
            <QRCode
              value={`warpinator://${registrationInfo.ip}:${registrationInfo.port}`}
              bgColor="transparent"
              fgColor="currentColor"
              className="text-popover-foreground m-2"
            />

            <Button
              variant="outline"
              className="mt-4"
              onClick={() =>
                toast.promise(
                  () =>
                    writeText(
                      `warpinator://${registrationInfo.ip}:${registrationInfo.port}`,
                    ),
                  {
                    success: "IP address copied to clipboard",
                    error: "Failed to copy IP address",
                  },
                )
              }
            >
              {registrationInfo.ip}:{registrationInfo.port}{" "}
              <HugeiconsIcon icon={Copy01Icon} />
            </Button>
          </div>
        )}
        <DialogFooter>
          <Button variant="outline" onClick={() => setMode("input")}>
            Add connection
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}

function InputDialog({
  open,
  onOpenChange,
}: {
  open: boolean;
  onOpenChange: (open: boolean) => void;
}) {
  const [connectionUrl, setConnectionUrl] = useState<string>("");
  const [validUrl, setValidUrl] = useState<boolean>(true);
  const isValidIpWithPort = (input: string): boolean => {
    const pattern =
      /^(?:(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)|\[[a-fA-F0-9:]+]):([0-9]{1,5})$/;

    const match = input.match(pattern);
    if (!match) return false;

    const port = parseInt(match[1], 10);
    return port >= 0 && port <= 65535;
  };

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Manual Connection</DialogTitle>
          <DialogDescription>
            Initiate a connection to a specific address
          </DialogDescription>
        </DialogHeader>
        <InputGroup>
          <InputGroupAddon>
            <InputGroupText>warpinator://</InputGroupText>
          </InputGroupAddon>
          <InputGroupInput
            aria-invalid={!validUrl}
            placeholder="192.168.0.0:42001"
            className="pl-0.5!"
            value={connectionUrl}
            onChange={(e) => {
              let targetValue;

              if (e.target.value.startsWith("warpinator://")) {
                targetValue = e.target.value.slice("warpinator://".length);
              } else {
                targetValue = e.target.value;
              }
              setConnectionUrl(targetValue);
              setValidUrl(isValidIpWithPort(targetValue));
            }}
          />
        </InputGroup>

        <DialogFooter>
          <Button
            variant="outline"
            onClick={() => {
              if (validUrl && connectionUrl.length > 0) {
                toast.promise(
                  invoke("manual_connect_remote", { remoteUrl: connectionUrl }),
                  {
                    success: "Connection successful",
                    loading: "Connecting to " + connectionUrl,
                    error: formatManualConnectionError,
                  },
                );
                onOpenChange(false);
              }
            }}
            disabled={!(validUrl && connectionUrl.length > 0)}
          >
            Connect
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
