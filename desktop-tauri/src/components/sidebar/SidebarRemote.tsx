import { Avatar, AvatarFallback, AvatarImage } from "@/components/ui/avatar";
import { SidebarMenuButton, SidebarMenuItem } from "@/components/ui/sidebar";
import { HugeiconsIcon } from "@hugeicons/react";
import { ArrowRight01Icon, UserIcon } from "@hugeicons/core-free-icons";
import { RemoteState } from "@/types/remote.ts";

export function SidebarRemote({
  remote,
  onClick,
  isSelected,
  isFavorite,
}: {
  remote: {
    name: string;
    address: string;
    picture: string | null;
    state: RemoteState;
  };
  onClick: () => void;
  isSelected: boolean;
  isFavorite: boolean;
}) {
  return (
    <SidebarMenuItem>
      <SidebarMenuButton
        size="lg"
        className="data-selected:bg-sidebar-accent data-selected:text-sidebar-accent-foreground group"
        data-remote-state={remote.state.type}
        data-selected={isSelected}
        onClick={onClick}
      >
        {isFavorite && (
          <div className="w-1 h-4 rounded-full bg-sidebar-primary">
            <span className="sr-only">Favourite remote</span>
          </div>
        )}
        <Avatar className="h-8 w-8 rounded-lg after:border-0 bg-transparent group-data-[remote-state=awaiting_duplex]:animate-pulse group-data-[remote-state=connecting]:animate-pulse group-data-[remote-state=error]:outline outline-destructive">
          <AvatarImage
            src={remote.picture ?? undefined}
            alt={remote.name}
            className="rounded-lg not-group-data-[remote-state=connected]:opacity-50"
          />
          <AvatarFallback className="rounded-lg bg-sidebar-primary text-sidebar-primary-foreground not-group-data-[remote-state=connected]:opacity-50">
            <HugeiconsIcon icon={UserIcon} />
          </AvatarFallback>
        </Avatar>
        <div className="grid flex-1 text-left text-sm leading-tight">
          <span className="truncate font-medium">{remote.name}</span>
          <span className="truncate text-xs">{remote.address}</span>
        </div>
        <HugeiconsIcon icon={ArrowRight01Icon} />
      </SidebarMenuButton>
    </SidebarMenuItem>
  );
}
