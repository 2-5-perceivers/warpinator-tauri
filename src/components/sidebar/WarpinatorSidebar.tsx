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
import React from "react";
import { SidebarSearchEmpty } from "@/components/sidebar/SidebarSearchEmpty.tsx";

export function WarpinatorSidebar({ os }: { os: string }) {
  const remotes = useRemotes();
  const [search, setSearch] = React.useState("");

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

  return (
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
          <SidebarEmpty />
        ) : filteredRemotes.length == 0 ? (
          <SidebarSearchEmpty />
        ) : (
          <SidebarMenu>
            {filteredRemotes.map((remote) => (
              <SidebarRemote
                key={remote.uuid}
                remote={{
                  name: remote.display_name,
                  address: remote.username + "@" + remote.hostname,
                  avatar: remote.picture_data,
                  state: remote.state,
                }}
              />
            ))}
          </SidebarMenu>
        )}
      </SidebarContent>

      <SidebarFooter className="border-t border-border p-2.5">
        <SidebarUser
          user={{ name: "User", address: "192.168.0.1", avatar: undefined }}
        />
      </SidebarFooter>
    </Sidebar>
  );
}
