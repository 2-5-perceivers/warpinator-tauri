import { HugeiconsIcon } from "@hugeicons/react";
import {
  Cancel01Icon,
  Remove01Icon,
  SquareIcon,
} from "@hugeicons/core-free-icons";
import { getCurrentWindow } from "@tauri-apps/api/window";

export function WindowControls({ os }: { os: string }) {
  if (os === "macos") return null;

  const win = getCurrentWindow();
  return (
    <div className="flex items-center gap-0.5 px-1">
      <button
        onClick={() => win.minimize()}
        className="h-9 w-9 rounded-lg flex items-center justify-center text-foreground/50 hover:text-foreground/85 hover:bg-foreground/7 transition-colors"
      >
        <HugeiconsIcon icon={Remove01Icon} className="size-4" />
      </button>
      <button
        onClick={() => win.toggleMaximize()}
        className="h-9 w-9 rounded-lg flex items-center justify-center text-foreground/50 hover:text-foreground/85 hover:bg-foreground/7 transition-colors"
      >
        <HugeiconsIcon icon={SquareIcon} className="size-3.5" />
      </button>
      <button
        onClick={() => win.close()}
        className="h-9 w-9 rounded-lg flex items-center justify-center text-foreground/50 hover:text-red-400 hover:bg-red-500/25 transition-colors"
      >
        <HugeiconsIcon icon={Cancel01Icon} className="size-5" />
      </button>
    </div>
  );
}
