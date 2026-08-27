import { useEffect, useState } from "react";
import { getCurrentWindow } from "@tauri-apps/api/window";

export function useIsMaximized() {
  const [isMaximized, setIsMaximized] = useState<boolean>(false);

  useEffect(() => {
    function handleResize() {
      getCurrentWindow()
        .isMaximized()
        .then((newState) => setIsMaximized(newState));
    }
    handleResize();

    const unlisten = getCurrentWindow().listen("tauri://resize", async () => {
      handleResize();
    });
    return () => {
      unlisten.then((u) => u());
    };
  }, []);

  return isMaximized;
}
