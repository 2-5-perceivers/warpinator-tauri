import { WindowControls } from "@/components/window-controls/WindowControls.tsx";
import { useRemoteContext } from "@/contexts/RemoteContext.tsx";
import { useRemote } from "@/hooks/use-remote.ts";
import {
  Avatar,
  AvatarFallback,
  AvatarImage,
} from "@/components/ui/avatar.tsx";
import { HugeiconsIcon } from "@hugeicons/react";
import {
  HeartAddIcon,
  Message01Icon,
  UserIcon,
} from "@hugeicons/core-free-icons";
import { Badge } from "@/components/ui/badge.tsx";
import { Button } from "@/components/ui/button.tsx";
import { Tooltip, TooltipContent, TooltipTrigger } from "../ui/tooltip";

export function TopBar({ os }: { os: string }) {
  const { selectedRemoteUuid } = useRemoteContext();

  let children;
  const remote = useRemote(selectedRemoteUuid);
  if (remote) {
    const getBadgeVariant = () => {
      switch (remote.state.type) {
        case "connected":
          return "default";
        case "connecting":
        case "awaiting_duplex":
          return "secondary";
        case "disconnected":
          return "outline";
        case "error":
          return "destructive";
      }
    };

    const getBadgeText = () => {
      switch (remote.state.type) {
        case "connected":
          return "connected";
        case "connecting":
          return "connecting";
        case "awaiting_duplex":
          return "awaiting duplex";
        case "disconnected":
          return "disconnected";
        case "error":
          switch (remote.state.content) {
            case "ssl_error":
              return "SSL error";
            case "no_certificate":
              return "No certificate";
            case "duplex_error":
              return "Duplex error";
            default:
              return "Unknown error";
            // Ignore the group code error, as it should not show remotes in the first place with that error
          }
      }
    };

    children = (
      <div className="flex px-3 gap-2 items-center">
        <Avatar
          className="size-7 shrink-0 rounded-lg after:border-0 bg-transparent"
          key={remote.uuid}
        >
          {remote.picture_data ? (
            <AvatarImage
              src={remote.picture_data}
              alt={remote.display_name}
              className="rounded-lg"
            />
          ) : (
            <AvatarFallback className="rounded-lg bg-sidebar-primary text-sidebar-primary-foreground">
              <HugeiconsIcon icon={UserIcon} className="size-4" />
            </AvatarFallback>
          )}
        </Avatar>
        <h1 className="truncate min-w-0">{remote.display_name}</h1>
        <Badge variant={getBadgeVariant()} className="shrink-0">
          {getBadgeText()}
        </Badge>

        <Tooltip>
          <TooltipTrigger asChild>
            <Button
              className="pointer-events-auto ml-auto"
              size="icon-sm"
              variant="outline"
            >
              <HugeiconsIcon icon={HeartAddIcon} />
            </Button>
          </TooltipTrigger>
          <TooltipContent>Toggle favourites</TooltipContent>
        </Tooltip>
        <Tooltip>
          <TooltipTrigger asChild>
            <Button
              className="pointer-events-auto"
              size="icon-sm"
              variant="outline"
            >
              <HugeiconsIcon icon={Message01Icon} />
            </Button>
          </TooltipTrigger>
          <TooltipContent>Send message</TooltipContent>
        </Tooltip>
        <Button className="pointer-events-auto" size="sm">
          Send
        </Button>
      </div>
    );
  }
  return (
    <div className="h-11 shrink-0 flex items-center relative border-b border-sidebar-border/50 bg-sidebar">
      <div data-tauri-drag-region className="absolute inset-0" />
      <div className="relative flex items-center w-full pointer-events-none">
        <div className="pointer-events-none flex-1">{children}</div>
        <div className="pointer-events-auto">
          <WindowControls os={os} />
        </div>
      </div>
    </div>
  );
}
