import {
  Empty,
  EmptyDescription,
  EmptyHeader,
  EmptyMedia,
  EmptyTitle,
} from "@/components/ui/empty.tsx";
import { CursorRectangleSelection02Icon } from "@hugeicons/core-free-icons";
import { HugeiconsIcon } from "@hugeicons/react";

export function EmptyMain() {
  return (
    <Empty>
      <EmptyHeader>
        <EmptyMedia variant="icon">
          <HugeiconsIcon icon={CursorRectangleSelection02Icon} />
        </EmptyMedia>
        <EmptyTitle>No Device Selected</EmptyTitle>
        <EmptyDescription className="max-w-xs text-pretty">
          Select a device from the sidebar to view details and manage transfers.
        </EmptyDescription>
      </EmptyHeader>
    </Empty>
  );
}
