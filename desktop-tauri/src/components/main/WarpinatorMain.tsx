import { EmptyMain } from "@/components/main/EmptyMain.tsx";
import { TopBar } from "@/components/main/TopAppBar.tsx";
import { useRemoteContext } from "@/contexts/RemoteContext.tsx";
import { RemoteTransfers } from "@/components/main/RemoteTransfers.tsx";
import { SidebarProvider } from "@/components/ui/sidebar.tsx";

export function WarpinatorMain({ os }: { os: string }) {
  const { selectedRemoteUuid } = useRemoteContext();

  return (
    <SidebarProvider
      className="flex flex-col flex-1 overflow-hidden"
      defaultOpen={false}
    >
      <TopBar os={os} />
      {selectedRemoteUuid ? (
        <RemoteTransfers selectedRemoteUuid={selectedRemoteUuid} />
      ) : (
        <EmptyMain />
      )}
    </SidebarProvider>
  );
}
