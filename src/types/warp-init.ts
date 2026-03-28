export interface WarpinatorInit {
  theme: "dark" | "light" | "system";
  display_name: string;
  hostname: string;
  username: string;
  ip: string;
}

export const warpinator_init: WarpinatorInit = (window as any)
  .__WARPINATOR_INIT__;

console.log("warp init", warpinator_init);
