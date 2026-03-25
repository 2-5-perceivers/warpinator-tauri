import { Avatar, AvatarFallback, AvatarImage } from "@/components/ui/avatar";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuGroup,
  DropdownMenuItem,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import {
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
  useSidebar,
} from "@/components/ui/sidebar";
import { HugeiconsIcon } from "@hugeicons/react";
import {
  ArrowReloadHorizontalIcon,
  InformationSquareIcon,
  Link01Icon,
  LogoutSquare01Icon,
  Settings01Icon,
  UserIcon,
} from "@hugeicons/core-free-icons";
import React from "react";
import { AboutDialog } from "@/components/sidebar/AboutDialog.tsx";

export function SidebarUser({
  user,
}: {
  user: {
    name: string;
    address: string;
    avatar: string | undefined | null;
  };
}) {
  const { isMobile } = useSidebar();
  const [isAboutDialogOpen, setAboutDialogOpen] = React.useState(false);

  return (
    <>
      <SidebarMenu>
        <SidebarMenuItem>
          <DropdownMenu>
            <DropdownMenuTrigger asChild>
              <SidebarMenuButton
                size="lg"
                className="data-[state=open]:bg-sidebar-accent data-[state=open]:text-sidebar-accent-foreground"
              >
                <Avatar className="h-8 w-8 rounded-lg after:border-0 bg-sidebar-primary">
                  {user.avatar ? (
                    <AvatarImage
                      src={user.avatar}
                      alt={user.name}
                      className="rounded-lg"
                    />
                  ) : (
                    <AvatarFallback className="rounded-lg bg-transparent text-sidebar-primary-foreground">
                      <HugeiconsIcon icon={UserIcon} />
                    </AvatarFallback>
                  )}
                </Avatar>
                <div className="grid flex-1 text-left text-sm leading-tight">
                  <span className="truncate font-medium">{user.name}</span>
                  <span className="truncate text-xs">{user.address}</span>
                </div>
              </SidebarMenuButton>
            </DropdownMenuTrigger>
            <DropdownMenuContent
              side={isMobile ? "bottom" : "right"}
              align="end"
              sideOffset={4}
            >
              <DropdownMenuGroup>
                <DropdownMenuItem>
                  <HugeiconsIcon icon={Link01Icon} />
                  Manual connection
                </DropdownMenuItem>
              </DropdownMenuGroup>
              <DropdownMenuSeparator />
              <DropdownMenuGroup>
                <DropdownMenuItem>
                  <HugeiconsIcon icon={ArrowReloadHorizontalIcon} />
                  Restart
                </DropdownMenuItem>
                <DropdownMenuItem>
                  <HugeiconsIcon icon={Settings01Icon} />
                  Settings
                </DropdownMenuItem>
                <DropdownMenuItem onSelect={() => setAboutDialogOpen(true)}>
                  <HugeiconsIcon icon={InformationSquareIcon} />
                  About
                </DropdownMenuItem>
              </DropdownMenuGroup>
              <DropdownMenuSeparator />
              <DropdownMenuItem>
                <HugeiconsIcon icon={LogoutSquare01Icon} />
                Quit
              </DropdownMenuItem>
            </DropdownMenuContent>
          </DropdownMenu>
        </SidebarMenuItem>
      </SidebarMenu>
      <AboutDialog open={isAboutDialogOpen} onOpenChange={setAboutDialogOpen} />
    </>
  );
}
