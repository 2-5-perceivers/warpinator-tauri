import { getCurrentWindow } from "@tauri-apps/api/window";

export function MacOSControls({
  minimize = true,
  maximize = true,
}: {
  minimize?: boolean;
  maximize?: boolean;
}) {
  const win = getCurrentWindow();
  return (
    <div className="flex items-center gap-1.5 pointer-events-auto">
      <button
        onClick={() => win.close()}
        className="w-3 h-3 rounded-full bg-[#ff5f57]"
      />
      {minimize && (
        <button
          onClick={() => win.minimize()}
          className="w-3 h-3 rounded-full bg-[#febc2e]"
        />
      )}
      {maximize && (
        <button
          onClick={() => win.toggleMaximize()}
          className="w-3 h-3 rounded-full bg-[#28c840]"
        />
      )}
    </div>
  );
}
