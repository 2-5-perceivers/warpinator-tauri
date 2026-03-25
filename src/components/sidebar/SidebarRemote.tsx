import { Avatar, AvatarFallback, AvatarImage } from "@/components/ui/avatar";
import { SidebarMenuButton, SidebarMenuItem } from "@/components/ui/sidebar";
import { HugeiconsIcon } from "@hugeicons/react";
import { ArrowRight01Icon, UserIcon } from "@hugeicons/core-free-icons";

export function SidebarRemote({
  remote,
}: {
  remote: {
    name: string;
    address: string;
    avatar: string | undefined | null;
  };
}) {
  return (
    <SidebarMenuItem>
      <SidebarMenuButton
        size="lg"
        className="data-[state=open]:bg-sidebar-accent data-[state=open]:text-sidebar-accent-foreground"
      >
        <Avatar className="h-8 w-8 rounded-lg after:border-0 bg-sidebar-primary/30">
          {remote.avatar ? (
            <AvatarImage
              src={remote.avatar}
              alt={remote.name}
              className="rounded-lg"
            />
          ) : (
            <AvatarFallback className="rounded-lg bg-transparent text-sidebar-primary-foreground/60">
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
