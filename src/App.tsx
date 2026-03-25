import "./App.css";
import { TooltipProvider } from "@/components/ui/tooltip";
import { SidebarProvider } from "@/components/ui/sidebar";
import { platform } from "@tauri-apps/plugin-os";
import { HugeiconsIcon } from "@hugeicons/react";
import { CursorRectangleSelection02Icon } from "@hugeicons/core-free-icons";
import { WindowControls } from "@/components/window-controls/WindowControls.tsx";
import { WarpinatorSidebar } from "@/components/sidebar/WarpinatorSidebar.tsx";
import React from "react";

function TopBar({ children, os }: { children?: React.ReactNode; os: string }) {
  return (
    <div className="h-11 shrink-0 flex items-center relative border-b border-sidebar-border/50 bg-sidebar">
      <div data-tauri-drag-region className="absolute inset-0" />
      <div className="relative flex items-center justify-between w-full px-4 pe-0 pointer-events-none">
        <div className="pointer-events-auto">{children}</div>
        <div className="pointer-events-auto">
          <WindowControls os={os} />
        </div>
      </div>
    </div>
  );
}

function EmptyMain() {
  return (
    <div className="flex-1 flex flex-col items-center justify-center gap-3 text-center bg-background">
      <HugeiconsIcon
        icon={CursorRectangleSelection02Icon}
        className="size-12"
      />
      <div>
        <p className="text-sm font-medium text-foreground">
          No device selected
        </p>
        <p className="text-xs text-foreground/50 mt-1 leading-relaxed">
          Select a device on the left
          <br />
          to see its transfers
        </p>
      </div>
    </div>
  );
}

function App() {
  const os = platform();
  return (
    <TooltipProvider>
      <SidebarProvider
        style={{ minHeight: 0 }}
        className="h-screen overflow-hidden bg-background"
      >
        <WarpinatorSidebar os={os} />
        <div className="flex flex-col flex-1 overflow-hidden">
          <TopBar os={os} />
          <EmptyMain />
        </div>
      </SidebarProvider>
    </TooltipProvider>
  );
}

export default App;
