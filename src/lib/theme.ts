import { invoke } from "@tauri-apps/api/core";

export interface ThemeSettings {
  theme: "dark" | "light" | "system";
}

export const initialTheme = await invoke<ThemeSettings>("get_theme_settings");
