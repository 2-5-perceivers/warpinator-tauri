import {
  Dialog,
  DialogClose,
  DialogContent,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog.tsx";
import { Button } from "@/components/ui/button.tsx";
import {
  Field,
  FieldContent,
  FieldDescription,
  FieldGroup,
  FieldLabel,
  FieldSeparator,
  FieldSet,
} from "@/components/ui/field.tsx";
import { Input } from "@/components/ui/input.tsx";
import { Avatar } from "@/components/ui/avatar.tsx";
import { Folder02Icon } from "@hugeicons/core-free-icons";
import { HugeiconsIcon } from "@hugeicons/react";
import { Switch } from "@/components/ui/switch.tsx";
import { ToggleGroup, ToggleGroupItem } from "@/components/ui/toggle-group.tsx";

export function SettingsDialog({
  open,
  onOpenChange,
}: {
  open: boolean;
  onOpenChange: (open: boolean) => void;
}) {
  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="m-8 h-full max-h-[80vh] overflow-hidden">
        <DialogHeader>
          <DialogTitle>Settings</DialogTitle>
        </DialogHeader>
        <div className="-mx-4 no-scrollbar overflow-y-auto px-4 pb-4">
          <FieldGroup>
            <FieldSet>
              <FieldLabel>User</FieldLabel>
              <FieldDescription>
                Customize how you are seen on the network
              </FieldDescription>
              <FieldGroup>
                <Field>
                  <FieldLabel htmlFor="display_name">Display name</FieldLabel>
                  <Input
                    id="display_name"
                    type="text"
                    placeholder="Warpinator User"
                  />
                </Field>
                <Field orientation="horizontal">
                  <FieldLabel>Profile picture</FieldLabel>
                  <Avatar className="h-8 w-8 rounded-lg after:border-0 bg-sidebar-primary"></Avatar>
                </Field>
              </FieldGroup>
            </FieldSet>
            <FieldSeparator />
            <FieldSet>
              <FieldLabel>Transfers and messages</FieldLabel>
              <FieldDescription>Change how transfer behave</FieldDescription>
              <FieldGroup>
                <Field orientation="horizontal">
                  <FieldContent>
                    <FieldLabel>Default destination</FieldLabel>
                    <FieldDescription>~/Downloads</FieldDescription>
                  </FieldContent>
                  <Button size="icon" variant="outline">
                    <HugeiconsIcon icon={Folder02Icon} />
                  </Button>
                </Field>
                <Field orientation="horizontal">
                  <FieldLabel htmlFor="notifications_transfers">
                    Show notifications for transfers
                  </FieldLabel>
                  <Switch id="notifications_transfers" />
                </Field>
                <Field orientation="horizontal">
                  <FieldLabel htmlFor="notifications_messages">
                    Show notifications for messages
                  </FieldLabel>
                  <Switch id="notifications_messages" />
                </Field>
                <Field orientation="horizontal">
                  <FieldLabel htmlFor="overwrite">
                    Check and warn if files already exists
                  </FieldLabel>
                  <Switch id="overwrite" />
                </Field>
                <Field>
                  <FieldLabel htmlFor="autoaccept">
                    Auto accept transfers from
                  </FieldLabel>
                  <ToggleGroup
                    type="single"
                    variant="outline"
                    id="autoaccept"
                    defaultValue="none"
                  >
                    <ToggleGroupItem value="none" className="flex-1/3">
                      None
                    </ToggleGroupItem>
                    <ToggleGroupItem value="favorites" className="flex-1/3">
                      Favourites
                    </ToggleGroupItem>
                    <ToggleGroupItem value="all" className="flex-1/3">
                      All
                    </ToggleGroupItem>
                  </ToggleGroup>
                </Field>
              </FieldGroup>
            </FieldSet>
            <FieldSeparator />
            <FieldSet>
              <FieldLabel>Network</FieldLabel>
              <FieldDescription>Change network interactions</FieldDescription>
              <FieldGroup>
                <Field>
                  <FieldLabel htmlFor="group_code">Group code</FieldLabel>
                  <Input id="group_code" type="text" placeholder="Warpinator" />
                </Field>
                <Field>
                  <FieldLabel htmlFor="port_transfers">
                    Port for transfers
                  </FieldLabel>
                  <Input id="port_transfers" type="text" placeholder="42000" />
                </Field>
                <Field>
                  <FieldLabel htmlFor="port_registration">
                    Port for registration
                  </FieldLabel>
                  <Input
                    id="port_registration"
                    type="text"
                    placeholder="42001"
                  />
                </Field>
              </FieldGroup>
            </FieldSet>
            <FieldSeparator />
            <FieldSet>
              <FieldLabel>Aspect</FieldLabel>
              <FieldGroup>
                <Field>
                  <FieldLabel htmlFor="theme">Theme</FieldLabel>
                  <ToggleGroup
                    type="single"
                    variant="outline"
                    id="autoaccept"
                    defaultValue="system"
                  >
                    <ToggleGroupItem value="system" className="flex-1/3">
                      System
                    </ToggleGroupItem>
                    <ToggleGroupItem value="light" className="flex-1/3">
                      Light
                    </ToggleGroupItem>
                    <ToggleGroupItem value="dark" className="flex-1/3">
                      Dark
                    </ToggleGroupItem>
                  </ToggleGroup>
                </Field>
              </FieldGroup>
            </FieldSet>
          </FieldGroup>
        </div>
        <DialogFooter>
          <DialogClose asChild>
            <Button variant="outline">Save</Button>
          </DialogClose>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
