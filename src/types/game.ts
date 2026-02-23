import { convertFileSrc } from "@tauri-apps/api/core";

export type Platform = "steam" | "epic" | "custom";

export interface SteamGame {
  app_id: number;
  name: string;
  install_dir: string;
}

export interface CustomGame {
  id: string;
  title: string;
  executable: string;
  cover_image: string | null;
  tags: string[];
  notes: string | null;
}

export interface EpicGame {
  app_name: string;
  display_name: string;
  install_location: string;
  catalog_namespace: string;
  catalog_item_id: string;
  cover_image: string | null;
}

/** Unified view model used throughout the UI */
export interface Game {
  /** Stable key: `steam-<appid>`, `epic-<app_name>`, or `custom-<uuid>` */
  key: string;
  title: string;
  platform: Platform;
  coverImage: string | null;
  /** steam app id — present for Steam games */
  appId?: number;
  /** executable path — present for custom games */
  executable?: string;
  /** pre-computed Epic launcher URI — present for Epic games */
  epicLaunchUri?: string;
  tags: string[];
}

export type SortOption = "alpha" | "recentlyAdded";
export type PlatformFilter = "all" | "steam" | "epic" | "custom";

export function fromSteamGame(g: SteamGame): Game {
  return {
    key: `steam-${g.app_id}`,
    title: g.name,
    platform: "steam",
    coverImage: `https://cdn.cloudflare.steamstatic.com/steam/apps/${g.app_id}/library_600x900.jpg`,
    appId: g.app_id,
    tags: [],
  };
}

export function fromCustomGame(g: CustomGame): Game {
  return {
    key: `custom-${g.id}`,
    title: g.title,
    platform: "custom",
    coverImage: resolveCustomCover(g.cover_image),
    executable: g.executable,
    tags: g.tags,
  };
}

function resolveCustomCover(path: string | null): string | null {
  if (!path) return null;
  return convertFileSrc(path);
}

export function fromEpicGame(g: EpicGame): Game {
  return {
    key: `epic-${g.app_name}`,
    title: g.display_name,
    platform: "epic",
    coverImage: g.cover_image ? convertFileSrc(g.cover_image) : null,
    epicLaunchUri: `com.epicgames.launcher://apps/${g.catalog_namespace}%3A${g.catalog_item_id}%3A${g.app_name}?action=launch&silent=true`,
    tags: [],
  };
}
