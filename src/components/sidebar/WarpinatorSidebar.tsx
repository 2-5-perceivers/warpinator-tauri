import {
  Sidebar,
  SidebarContent,
  SidebarFooter,
  SidebarHeader,
  SidebarMenu,
} from "@/components/ui/sidebar.tsx";
import { MacOSControls } from "@/components/window-controls/MacOSControls.tsx";
import { SidebarSearch } from "@/components/sidebar/SidebarSearch.tsx";
import { SidebarUser } from "@/components/sidebar/SidebarUser.tsx";
import { SidebarEmpty } from "@/components/sidebar/SidebarEmpty.tsx";
import { useRemotes } from "@/hooks/use-remotes.ts";
import { SidebarRemote } from "@/components/sidebar/SidebarRemote.tsx";
import React, { useState } from "react";
import { SidebarSearchEmpty } from "@/components/sidebar/SidebarSearchEmpty.tsx";
import { useRemoteContext } from "@/contexts/RemoteContext.tsx";
import { ManualConnectionDialog } from "@/dialogs/ManualConnectionDialog.tsx";
import { useSettings } from "@/contexts/SettingsProvider.tsx";

export function WarpinatorSidebar({ os }: { os: string }) {
  const remotes = useRemotes();
  const [search, setSearch] = React.useState("");
  const { selectedRemoteUuid, setSelectedRemote } = useRemoteContext();
  const { settings } = useSettings();

  const [isManualConnectionDialogOpen, setManualConnectionDialogOpen] =
    useState(false);

  const filteredRemotes = React.useMemo(() => {
    if (!search) {
      return remotes;
    }

    return remotes.filter((remote) => {
      const remoteName = remote.display_name.toLowerCase();
      const remoteAddress = (
        remote.username +
        "@" +
        remote.hostname
      ).toLowerCase();
      const query = search.toLowerCase();

      return remoteName.includes(query) || remoteAddress.includes(query);
    });
  }, [remotes, search]);

  const sortedRemotes = React.useMemo(() => {
    const favorites = new Set(settings?.favorites ?? []);

    return [...filteredRemotes].sort((a, b) => {
      const aFavorite = favorites.has(a.uuid);
      const bFavorite = favorites.has(b.uuid);

      if (aFavorite === bFavorite) return 0;
      return aFavorite ? -1 : 1;
    });
  }, [filteredRemotes, settings?.favorites]);

  return (
    <>
      <Sidebar collapsible="icon">
        <SidebarHeader
          data-tauri-drag-region
          className="h-11 flex-row items-center px-3.5 gap-3 bg-secondary/20 border-b border-sidebar-border/50"
        >
          {os === "macos" && <MacOSControls />}
          <span
            className="text-l font-medium text-secondary-foreground"
            data-tauri-drag-region
          >
            Warpinator
          </span>
        </SidebarHeader>

        <SidebarContent className="px-1.5 py-2 h-full flex flex-col">
          <SidebarSearch
            value={search}
            onChange={(e) => setSearch(e.target.value)}
          />
          {remotes.length == 0 ? (
            <SidebarEmpty
              setManualConnectionDialogOpen={setManualConnectionDialogOpen}
            />
          ) : sortedRemotes.length == 0 ? (
            <SidebarSearchEmpty />
          ) : (
            <SidebarMenu>
              {sortedRemotes.map((remote) => (
                <SidebarRemote
                  key={remote.uuid}
                  remote={{
                    name: remote.display_name,
                    address: remote.username + "@" + remote.hostname,
                    picture: remote.picture,
                    state: remote.state,
                  }}
                  isFavorite={
                    settings?.favorites.includes(remote.uuid) ?? false
                  }
                  onClick={() => setSelectedRemote(remote.uuid)}
                  isSelected={selectedRemoteUuid === remote.uuid}
                />
              ))}
            </SidebarMenu>
          )}
        </SidebarContent>

        <SidebarFooter className="border-t border-border p-2.5">
          <SidebarUser
            setManualConnectionDialogOpen={setManualConnectionDialogOpen}
          />
        </SidebarFooter>
      </Sidebar>
      <ManualConnectionDialog
        open={isManualConnectionDialogOpen}
        onOpenChange={setManualConnectionDialogOpen}
      />
    </>
  );
}
