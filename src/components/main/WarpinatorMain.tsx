import { EmptyMain } from "@/components/main/EmptyMain.tsx";
import { TopBar } from "@/components/main/TopAppBar.tsx";
import { useRemoteContext } from "@/contexts/RemoteContext.tsx";
import { RemoteTransfers } from "@/components/main/RemoteTransfers.tsx";

export function WarpinatorMain({ os }: { os: string }) {
  const { selectedRemoteUuid } = useRemoteContext();

  return (
    <div className="flex flex-col flex-1 overflow-hidden">
      <TopBar os={os} />
      {selectedRemoteUuid ? (
        <RemoteTransfers selectedRemoteUuid={selectedRemoteUuid} />
      ) : (
        <EmptyMain />
      )}
    </div>
  );
}
