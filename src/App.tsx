import "./App.css";
import { TooltipProvider } from "@/components/ui/tooltip";
import { SidebarProvider } from "@/components/ui/sidebar";
import { platform } from "@tauri-apps/plugin-os";
import { WarpinatorSidebar } from "@/components/sidebar/WarpinatorSidebar.tsx";
import { WarpinatorMain } from "@/components/main/WarpinatorMain.tsx";

function App() {
  const os = platform();
  return (
    <TooltipProvider>
      <SidebarProvider
        style={{ minHeight: 0 }}
        className="h-screen overflow-hidden bg-background"
      >
        <WarpinatorSidebar os={os} />
        <WarpinatorMain os={os} />
      </SidebarProvider>
    </TooltipProvider>
  );
}

export default App;
