import {
  Dialog,
  DialogClose,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import { Button } from "@/components/ui/button";
import { useEffect, useState } from "react";
import { getVersion, getTauriVersion } from "@tauri-apps/api/app";
import { platform } from "@tauri-apps/plugin-os";

export function AboutDialog({
  open,
  onOpenChange,
}: {
  open: boolean;
  onOpenChange: (open: boolean) => void;
}) {
  const [appInfo, setAppInfo] = useState({
    version: "",
    tauriVersion: "",
    platform: "",
  });

  useEffect(() => {
    const fetchAppInfo = async () => {
      const [version, tauriVersion, platformName] = await Promise.all([
        getVersion(),
        getTauriVersion(),
        platform(),
      ]);
      setAppInfo({ version, tauriVersion, platform: platformName });
    };

    fetchAppInfo();
  }, []);

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>About Warpinator</DialogTitle>
          <DialogDescription>
            A reimplementation of the Warpinator file transfer tool in Rust and
            Tauri.
          </DialogDescription>
        </DialogHeader>
        <div className="-mx-4 no-scrollbar max-h-[50vh] overflow-y-auto px-4">
          <p className="mb-4 leading-normal">
            Warpinator is a free, open-source tool for sending and receiving
            files between computers on the same network.
          </p>
          <p className="mb-4 leading-normal">
            This version is built with modern web technologies for a
            cross-platform experience.
          </p>
          <div className="text-sm text-muted-foreground">
            <p>Version: {appInfo.version}</p>
            <p>Tauri Version: {appInfo.tauriVersion}</p>
            <p>Platform: {appInfo.platform}</p>
          </div>
        </div>
        <DialogFooter>
          <DialogClose asChild>
            <Button variant="outline">Close</Button>
          </DialogClose>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
