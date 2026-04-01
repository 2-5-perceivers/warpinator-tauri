import "./App.css";
import { TooltipProvider } from "@/components/ui/tooltip";
import { SidebarProvider } from "@/components/ui/sidebar";
import { platform } from "@tauri-apps/plugin-os";
import { WarpinatorSidebar } from "@/components/sidebar/WarpinatorSidebar.tsx";
import { WarpinatorMain } from "@/components/main/WarpinatorMain.tsx";
import { RemoteProvider } from "@/contexts/RemoteContext.tsx";
import { ThemeProvider } from "@/contexts/ThemeProvider.tsx";
import { SettingsProvider } from "@/contexts/SettingsProvider.tsx";

function App() {
  const os = platform();
  return (
    <ThemeProvider>
      <SettingsProvider>
        <TooltipProvider>
          <SidebarProvider
            style={{ minHeight: 0 }}
            className="h-screen overflow-hidden bg-background"
          >
            <RemoteProvider>
              <WarpinatorSidebar os={os} />
              <WarpinatorMain os={os} />
            </RemoteProvider>
          </SidebarProvider>
        </TooltipProvider>
      </SettingsProvider>
    </ThemeProvider>
  );
}

export default App;
