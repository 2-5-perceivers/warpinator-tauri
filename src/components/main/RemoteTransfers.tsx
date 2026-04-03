import { useTransfers } from "@/hooks/use-transfers.ts";
import { TransferCard } from "@/components/main/TransferCard.tsx";
import { MessagesSidebar } from "@/components/main/messages/MessagesSidebar.tsx";

export function RemoteTransfers({
  selectedRemoteUuid,
}: {
  selectedRemoteUuid: string;
}) {
  const transfers = useTransfers(selectedRemoteUuid);
  if (!transfers) return;

  return (
    <div className="flex flex-row">
      <div className="flex flex-col flex-1 gap-2 p-2 overflow-y-auto">
        {transfers.map((transfer) => (
          <TransferCard key={transfer.uuid} transfer={transfer} />
        ))}
      </div>
      <MessagesSidebar selectedRemoteUuid={selectedRemoteUuid} />
    </div>
  );
}
