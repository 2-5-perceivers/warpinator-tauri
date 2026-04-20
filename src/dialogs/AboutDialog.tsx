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

export function AboutContent() {
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
    <div className="h-full overflow-hidden">
      <div className="no-scrollbar h-full overflow-y-auto px-4 pb-4">
        <p className="mb-4 leading-normal text-muted-foreground text-sm">
          Warpinator is a free, open-source tool for sending and receiving files
          between computers on the same network.
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
    </div>
  );
}
