# Game Library — Project Plan

## Overview
A Tauri desktop application that acts as a personal game library manager, allowing the user to organize, browse, and launch games from a single place.

## Core Features

### Steam Integration
- Detect the local Steam installation and read the user's installed games
- Display Steam games with their metadata (title, cover art, playtime if available)
- Launch Steam games directly from the app (via Steam URI `steam://run/<appid>`)

### Non-Steam Games
- Manually link executables for games not on Steam
- Store custom metadata: title, cover image, genre, notes
- Launch non-Steam games by executing the linked binary

### Library Layout
- Grid/card-based layout similar to a game library (Steam / GOG Galaxy style)
- Game cards show cover art, title, and platform badge (Steam / Custom)
- Search and filter by title, platform, or tags
- Sorting options: alphabetical, recently added, playtime

## Tech Stack
- **Frontend:** Tauri + Vue 3 (TypeScript)
- **Backend (Rust):** Tauri commands for file system access, Steam VDF parsing, and process spawning
- **Styling:** Tailwind CSS for the library UI

## Architecture Notes
- Use Tauri `invoke` commands to communicate between frontend and Rust backend
- Steam library paths are read from `libraryfolders.vdf` in the Steam directory
- Non-Steam game entries are persisted in a local JSON/SQLite store managed by the Rust backend
- Use Java records for DTOs (when applicable to any JVM tooling in the project)
- Keep methods that handle data inside the relevant class (high cohesion)

## Project Structure (actual)
```
game-library/
├── src/                  # Vue 3 frontend
│   ├── components/
│   │   ├── GameCard.vue
│   │   ├── GameGrid.vue
│   │   ├── Sidebar.vue
│   │   ├── AddGameModal.vue
│   │   ├── LaunchConfirmDialog.vue
│   │   ├── FileExplorer.vue
│   │   └── VirtualKeyboard.vue
│   ├── composables/
│   │   └── useGamepad.ts
│   ├── types/
│   │   └── game.ts
│   ├── App.vue
│   └── main.ts
├── src-tauri/            # Rust backend
│   ├── src/
│   │   ├── main.rs
│   │   ├── lib.rs        # Tauri commands and app state
│   │   ├── steam.rs      # Steam VDF parsing & game discovery
│   │   ├── library.rs    # Custom game management (JSON persistence)
│   │   ├── launcher.rs   # Game process spawning and URI dispatch
│   │   └── fs_explorer.rs # File system utilities
│   └── tauri.conf.json
└── CLAUDE.md
```

## User Stories
1. As a user, I can open the app and see all my installed Steam games in a visual grid.
2. As a user, I can click a game card to launch the game.
3. As a user, I can add a non-Steam game by selecting its executable and setting a title and cover image.
4. As a user, I can search and filter my library by name or platform.
5. As a user, I can remove or edit a non-Steam game entry.
6. As a user, I can navigate and interact with the library using a gaming controller (gamepad) or mouse/keyboard.

## Input Support
- **Mouse & Keyboard:** standard navigation, click to select/launch, keyboard shortcuts for search and actions
- **Gamepad (Controller):** D-pad / left stick to move focus between game cards, A/Cross to launch, B/Circle to go back, trigger or bumper shortcuts for filtering
- Controller input handled via the frontend (e.g., `gamepad` browser API available in Tauri's WebView)
- Visual focus indicator must be clearly visible for controller navigation (big cursor / highlight on focused card)

## Active Technologies
- Rust (backend), TypeScript 5 + Vue 3 (frontend) + `serde_json` (already in Cargo.toml), `thiserror` (already used), (001-epic-games-integration)
- None — Epic manifests are read-only; no new persistence layer needed (001-epic-games-integration)

## Recent Changes
- 001-epic-games-integration: Added Rust (backend), TypeScript 5 + Vue 3 (frontend) + `serde_json` (already in Cargo.toml), `thiserror` (already used),
