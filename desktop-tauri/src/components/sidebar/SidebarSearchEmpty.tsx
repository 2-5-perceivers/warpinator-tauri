import { HugeiconsIcon } from "@hugeicons/react";
import { SearchAreaIcon } from "@hugeicons/core-free-icons";
import {
  Empty,
  EmptyDescription,
  EmptyHeader,
  EmptyMedia,
  EmptyTitle,
} from "@/components/ui/empty.tsx";

export function SidebarSearchEmpty() {
  return (
    <Empty>
      <EmptyHeader>
        <EmptyMedia variant="icon" className="size-12">
          <HugeiconsIcon icon={SearchAreaIcon} className="size-6" />
        </EmptyMedia>
        <EmptyTitle>No Devices Found</EmptyTitle>
        <EmptyDescription className="max-w-xs text-pretty">
          Check the name or clear the search.
        </EmptyDescription>
      </EmptyHeader>
    </Empty>
  );
}
