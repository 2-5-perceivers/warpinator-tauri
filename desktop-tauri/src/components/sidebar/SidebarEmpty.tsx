import { HugeiconsIcon } from "@hugeicons/react";
import { ComputerPhoneSyncIcon, Link01Icon } from "@hugeicons/core-free-icons";
import {
  Empty,
  EmptyContent,
  EmptyDescription,
  EmptyHeader,
  EmptyMedia,
  EmptyTitle,
} from "@/components/ui/empty.tsx";
import { Button } from "@/components/ui/button.tsx";

export function SidebarEmpty({
  setManualConnectionDialogOpen,
}: {
  setManualConnectionDialogOpen: (open: boolean) => void;
}) {
  return (
    <Empty>
      <EmptyHeader>
        <EmptyMedia variant="icon" className="size-12">
          <HugeiconsIcon icon={ComputerPhoneSyncIcon} className="size-6" />
        </EmptyMedia>
        <EmptyTitle>No Devices Found</EmptyTitle>
        <EmptyDescription className="max-w-xs text-pretty">
          Check the group code or try manually connecting.
        </EmptyDescription>
      </EmptyHeader>
      <EmptyContent>
        <Button
          variant="outline"
          onClick={() => setManualConnectionDialogOpen(true)}
        >
          <HugeiconsIcon icon={Link01Icon} />
          Manual connection
        </Button>
      </EmptyContent>
    </Empty>
  );
}
