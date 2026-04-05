import { invoke } from "@tauri-apps/api/core";

export interface RegistrationInfo {
  ip: string;
  port: number;
}

export async function getRegistrationInfo() {
  return await invoke<RegistrationInfo>("get_registration_info");
}
