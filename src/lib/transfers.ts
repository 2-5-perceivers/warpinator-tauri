import { Transfer } from "@/types/transfer.ts";
import { invoke } from "@tauri-apps/api/core";

// Accept a transfer in Waiting for permission state
export async function acceptTransfer(transfer: Transfer) {
  await invoke("accept_transfer", {
    remoteUuid: transfer.remote_uuid,
    transferUuid: transfer.uuid,
  });
}

// Cancel or decline a transfer in Waiting for permission state
export async function cancelTransfer(transfer: Transfer) {
  await invoke("cancel_transfer", {
    remoteUuid: transfer.remote_uuid,
    transferUuid: transfer.uuid,
  });
}

// Stop a transfer in pregress
export async function stopTransfer(transfer: Transfer) {
  await invoke("stop_transfer", {
    remoteUuid: transfer.remote_uuid,
    transferUuid: transfer.uuid,
  });
}

// Remove a transfer in a final state
export async function removeTransfer(transfer: Transfer) {
  await invoke("remove_transfer", {
    remoteUuid: transfer.remote_uuid,
    transferUuid: transfer.uuid,
  });
}

// Remove a transfer in a final state
export async function openTransfer(transfer: Transfer) {
  await invoke("open_transfer", {
    remoteUuid: transfer.remote_uuid,
    transferUuid: transfer.uuid,
  });
}
