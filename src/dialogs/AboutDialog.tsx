import {
  Dialog,
  DialogClose,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog.tsx";
import { Button } from "@/components/ui/button.tsx";
import { useEffect, useState } from "react";
import { getVersion, getTauriVersion } from "@tauri-apps/api/app";
import { platform } from "@tauri-apps/plugin-os";
import {
  Item,
  ItemActions,
  ItemContent,
  ItemDescription,
  ItemMedia,
  ItemTitle,
} from "@/components/ui/item.tsx";
import { HugeiconsIcon } from "@hugeicons/react";
import {
  AlertSquareIcon,
  BinaryCodeIcon,
  CodeSimpleIcon,
  LinkSquare01Icon,
} from "@hugeicons/core-free-icons";

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
      <DialogContent className="m-8 h-fit max-h-[80vh] overflow-hidden flex flex-col">
        <DialogHeader>
          <DialogTitle>About Warpinator</DialogTitle>
          <DialogDescription>
            A reimplementation of the Warpinator file transfer tool in Rust and
            Tauri.
          </DialogDescription>
        </DialogHeader>
        <div className="-mx-4 no-scrollbar overflow-y-auto px-4 pb-4">
          <p className="mb-4 leading-normal">
            Warpinator is a free, open-source tool for sending and receiving
            files between computers on the same network.
          </p>
          <div className="flex flex-col h-min gap-2">
            <Item variant="outline">
              <ItemMedia variant="icon">
                <HugeiconsIcon icon={BinaryCodeIcon} />
              </ItemMedia>
              <ItemContent>
                <ItemTitle>Version</ItemTitle>
                <ItemDescription>
                  {appInfo.version} | Tauri {appInfo.tauriVersion} |{" "}
                  {appInfo.platform}
                </ItemDescription>
              </ItemContent>
            </Item>
            <Item variant="outline" asChild>
              <a
                href="https://github.com/2-5-perceivers/warpinator-tauri"
                target="_blank"
              >
                <ItemMedia variant="icon">
                  <HugeiconsIcon icon={CodeSimpleIcon} />
                </ItemMedia>
                <ItemContent>
                  <ItemTitle>See source code</ItemTitle>
                  <ItemDescription>
                    2-5-perceivers/warpinator-tauri
                  </ItemDescription>
                </ItemContent>
                <ItemActions>
                  <HugeiconsIcon icon={LinkSquare01Icon} className="size-4" />
                </ItemActions>
              </a>
            </Item>
            <Item variant="outline" asChild>
              <a
                href="https://github.com/2-5-perceivers/warpinator-tauri/issues"
                target="_blank"
              >
                <ItemMedia variant="icon">
                  <HugeiconsIcon icon={AlertSquareIcon} />
                </ItemMedia>
                <ItemContent>
                  <ItemTitle>Issues</ItemTitle>
                  <ItemDescription>
                    Have an issue? Report it here!
                  </ItemDescription>
                </ItemContent>
                <ItemActions>
                  <HugeiconsIcon icon={LinkSquare01Icon} className="size-4" />
                </ItemActions>
              </a>
            </Item>
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
