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
import React, { useEffect } from "react";
import { AboutDialog } from "@/dialogs/AboutDialog.tsx";
import { SettingsDialog } from "@/dialogs/SettingsDialog.tsx";
import { exit } from "@tauri-apps/plugin-process";
import { Configuration, getConfiguration } from "@/lib/configuration.ts";

export function SidebarUser({ avatar }: { avatar?: string }) {
  const { isMobile } = useSidebar();
  const [isAboutDialogOpen, setAboutDialogOpen] = React.useState(false);
  const [isSettingsDialogOpen, setSettingsDialogOpen] = React.useState(false);
  const [configuration, setConfiguration] = React.useState<Configuration>();

  const quitApp = () => {
    exit(0).catch(() => {
      console.log("Failed to quit the app");
    });
  };

  useEffect(() => {
    // Fetch the configuration when the component mounts
    getConfiguration().then(setConfiguration);
  }, []);

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
                  {avatar ? (
                    <AvatarImage
                      src={avatar}
                      alt={configuration?.display_name}
                      className="rounded-lg"
                    />
                  ) : (
                    <AvatarFallback className="rounded-lg bg-transparent text-sidebar-primary-foreground">
                      <HugeiconsIcon icon={UserIcon} />
                    </AvatarFallback>
                  )}
                </Avatar>
                <div className="grid flex-1 text-left text-sm leading-tight">
                  <span className="truncate font-medium">
                    {configuration?.display_name}
                  </span>
                  <span className="truncate text-xs">
                    {configuration?.username}@{configuration?.hostname}
                  </span>
                </div>
                <HugeiconsIcon icon={Settings01Icon} />
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
                <DropdownMenuItem onSelect={() => setSettingsDialogOpen(true)}>
                  <HugeiconsIcon icon={Settings01Icon} />
                  Settings
                </DropdownMenuItem>
                <DropdownMenuItem onSelect={() => setAboutDialogOpen(true)}>
                  <HugeiconsIcon icon={InformationSquareIcon} />
                  About
                </DropdownMenuItem>
              </DropdownMenuGroup>
              <DropdownMenuSeparator />
              <DropdownMenuItem onSelect={quitApp}>
                <HugeiconsIcon icon={LogoutSquare01Icon} />
                Quit
              </DropdownMenuItem>
            </DropdownMenuContent>
          </DropdownMenu>
        </SidebarMenuItem>
      </SidebarMenu>
      <SettingsDialog
        open={isSettingsDialogOpen}
        onOpenChange={setSettingsDialogOpen}
      />
      <AboutDialog open={isAboutDialogOpen} onOpenChange={setAboutDialogOpen} />
    </>
  );
}
