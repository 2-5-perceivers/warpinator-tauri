import { Avatar, AvatarFallback, AvatarImage } from "@/components/ui/avatar";
import { SidebarMenuButton, SidebarMenuItem } from "@/components/ui/sidebar";
import { HugeiconsIcon } from "@hugeicons/react";
import { ArrowRight01Icon, UserIcon } from "@hugeicons/core-free-icons";
import { RemoteState } from "@/types/remote.ts";

export function SidebarRemote({
  remote,
}: {
  remote: {
    name: string;
    address: string;
    avatar: string | undefined | null;
    state: RemoteState;
  };
}) {
  return (
    <SidebarMenuItem>
      <SidebarMenuButton
        size="lg"
        className="data-[state=open]:bg-sidebar-accent data-[state=open]:text-sidebar-accent-foreground group"
        data-remote-state={remote.state.type}
      >
        <Avatar className="h-8 w-8 rounded-lg after:border-0 bg-transparent group-data-[remote-state=awaiting_duplex]:animate-pulse group-data-[remote-state=connecting]:animate-pulse group-data-[remote-state=error]:outline outline-destructive">
          {remote.avatar ? (
            <AvatarImage
              src={remote.avatar}
              alt={remote.name}
              className="rounded-lg not-group-data-[remote-state=connected]:opacity-50"
            />
          ) : (
            <AvatarFallback className="rounded-lg bg-sidebar-primary text-sidebar-primary-foreground not-data-[remote-state=connected]:opacity-50">
              <HugeiconsIcon icon={UserIcon} />
            </AvatarFallback>
          )}
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
