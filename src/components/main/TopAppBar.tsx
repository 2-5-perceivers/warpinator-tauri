import React from "react";
import { WindowControls } from "@/components/window-controls/WindowControls.tsx";

export function TopBar({
  children,
  os,
}: {
  children?: React.ReactNode;
  os: string;
}) {
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
