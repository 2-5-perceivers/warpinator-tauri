import { EmptyMain } from "@/components/main/EmptyMain.tsx";
import { TopBar } from "@/components/main/TopAppBar.tsx";

export function WarpinatorMain({ os }: { os: string }) {
  return (
    <div className="flex flex-col flex-1 overflow-hidden">
      <TopBar os={os} />
      <EmptyMain />
    </div>
  );
}
