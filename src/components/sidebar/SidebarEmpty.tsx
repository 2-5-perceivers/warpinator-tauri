import { HugeiconsIcon } from "@hugeicons/react";
import { ComputerPhoneSyncIcon } from "@hugeicons/core-free-icons";

export function SidebarEmpty() {
  return (
    <div className="flex-1 flex flex-col items-center justify-center gap-3 text-center">
      <HugeiconsIcon
        icon={ComputerPhoneSyncIcon}
        className="size-12 text-sidebar-primary/60"
      />
      <div>
        <p className="text-foreground/50">No device found</p>
      </div>
    </div>
  );
}
