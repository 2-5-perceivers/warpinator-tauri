import "@/App.css";
import { ThemeProvider } from "@/contexts/ThemeProvider.tsx";
import { platform } from "@tauri-apps/plugin-os";
import { MacOSControls } from "@/components/window-controls/MacOSControls.tsx";
import { WindowControls } from "@/components/window-controls/WindowControls.tsx";
import { AboutContent } from "@/dialogs/AboutDialog.tsx";

export function AboutWindow() {
  const os = platform();

  return (
    <div
      id="app-window"
      className="floating h-screen overflow-hidden bg-background"
    >
      <ThemeProvider>
        <div className="flex h-full flex-col">
          <div className="relative h-11 bg-sidebar mb-4" data-tauri-drag-region>
            <span className="pointer-events-none absolute left-1/2 top-1/2 -translate-x-1/2 -translate-y-1/2 text-sm font-medium">
              About
            </span>
            <div className="pointer-events-none flex h-full items-center justify-between ps-4 ">
              {os === "macos" ? (
                <MacOSControls minimize={false} maximize={false} />
              ) : (
                <div />
              )}
              {os !== "macos" && (
                <WindowControls os={os} minimize={false} maximize={false} />
              )}
            </div>
          </div>
          <AboutContent />
        </div>
      </ThemeProvider>
    </div>
  );
}
