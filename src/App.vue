<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import Sidebar from "./components/Sidebar.vue";
import GameGrid from "./components/GameGrid.vue";
import AddGameModal from "./components/AddGameModal.vue";
import LaunchConfirmDialog from "./components/LaunchConfirmDialog.vue";
import {
  fromSteamGame,
  fromCustomGame,
  type Game,
  type CustomGame,
  type SteamGame,
  type PlatformFilter,
  type SortOption,
} from "./types/game";

// ── State ──────────────────────────────────────────────────────────────────

const allGames = ref<Game[]>([]);
const loading = ref(true);
const loadError = ref("");
const showAddModal = ref(false);
const pendingLaunch = ref<Game | null>(null);
const focusedIndex = ref(0);
const notification = ref<{ message: string; type: "error" | "info" } | null>(null);
let notificationTimer = 0;

function showNotification(message: string, type: "error" | "info" = "error") {
  clearTimeout(notificationTimer);
  notification.value = { message, type };
  notificationTimer = window.setTimeout(() => { notification.value = null; }, 5000);
}

const search = ref("");
const platformFilter = ref<PlatformFilter>("all");
const sortOption = ref<SortOption>("alpha");

// ── Data loading ───────────────────────────────────────────────────────────

async function loadGames() {
  loading.value = true;
  loadError.value = "";
  try {
    const [steamGames, customGames] = await Promise.all([
      invoke<SteamGame[]>("get_steam_games").catch(() => [] as SteamGame[]),
      invoke<CustomGame[]>("get_custom_games"),
    ]);

    allGames.value = [
      ...steamGames.map(fromSteamGame),
      ...customGames.map(fromCustomGame),
    ];
  } catch (e) {
    loadError.value = String(e);
  } finally {
    loading.value = false;
  }
}

// ── Filtered & sorted view ─────────────────────────────────────────────────

const steamCount = computed(() => allGames.value.filter((g) => g.platform === "steam").length);
const customCount = computed(() => allGames.value.filter((g) => g.platform === "custom").length);

const filteredGames = computed<Game[]>(() => {
  let result = allGames.value;

  if (platformFilter.value !== "all") {
    result = result.filter((g) => g.platform === platformFilter.value);
  }

  if (search.value.trim()) {
    const q = search.value.toLowerCase();
    result = result.filter(
      (g) =>
        g.title.toLowerCase().includes(q) ||
        g.tags.some((t) => t.toLowerCase().includes(q))
    );
  }

  if (sortOption.value === "alpha") {
    result = [...result].sort((a, b) => a.title.localeCompare(b.title));
  }

  return result;
});

// ── Launch ─────────────────────────────────────────────────────────────────

function requestLaunch(game: Game) {
  pendingLaunch.value = game;
}

async function confirmLaunch() {
  const game = pendingLaunch.value;
  if (!game) return;
  pendingLaunch.value = null;
  try {
    await invoke("launch_game", {
      key: game.key,
      appId: game.appId ?? null,
      executable: game.executable ?? null,
    });
  } catch (e) {
    showNotification(String(e));
  }
}

function cancelLaunch() {
  pendingLaunch.value = null;
}

// ── Add game ───────────────────────────────────────────────────────────────

function onGameAdded(custom: CustomGame) {
  allGames.value.push(fromCustomGame(custom));
  showAddModal.value = false;
}

// ── Keyboard navigation ────────────────────────────────────────────────────

function onKeyDown(e: KeyboardEvent) {
  if (pendingLaunch.value) return;
  if (showAddModal.value) return;

  const count = filteredGames.value.length;
  if (count === 0) return;

  const cols = Math.floor(
    (document.getElementById("game-grid-area")?.clientWidth ?? 800) / 154
  );

  switch (e.key) {
    case "ArrowRight":
      e.preventDefault();
      focusedIndex.value = Math.min(focusedIndex.value + 1, count - 1);
      break;
    case "ArrowLeft":
      e.preventDefault();
      focusedIndex.value = Math.max(focusedIndex.value - 1, 0);
      break;
    case "ArrowDown":
      e.preventDefault();
      focusedIndex.value = Math.min(focusedIndex.value + cols, count - 1);
      break;
    case "ArrowUp":
      e.preventDefault();
      focusedIndex.value = Math.max(focusedIndex.value - cols, 0);
      break;
    case "Enter":
      e.preventDefault();
      if (filteredGames.value[focusedIndex.value]) {
        requestLaunch(filteredGames.value[focusedIndex.value]);
      }
      break;
  }
}

// ── Gamepad navigation ─────────────────────────────────────────────────────

type GamepadButtonName = "right" | "left" | "down" | "up" | "a" | "b";

interface ButtonState {
  pressed: boolean;
  lastAt: number;
}

const gamepadState = new Map<`${number}-${GamepadButtonName}`, ButtonState>();

const INITIAL_REPEAT_DELAY = 400;
const HELD_REPEAT_INTERVAL = 150;

function gamepadButtonId(padIndex: number, btn: GamepadButtonName): `${number}-${GamepadButtonName}` {
  return `${padIndex}-${btn}`;
}

const BUTTON_MAP: Record<number, GamepadButtonName> = {
  0: "a",
  1: "b",
  12: "up",
  13: "down",
  14: "left",
  15: "right",
};

let rafId = 0;

function pollGamepads() {
  const pads = navigator.getGamepads();
  const now = performance.now();
  const count = filteredGames.value.length;

  for (const pad of pads) {
    if (!pad) continue;

    const cols = Math.floor(
      (document.getElementById("game-grid-area")?.clientWidth ?? 800) / 154
    );

    for (const [btnIndex, name] of Object.entries(BUTTON_MAP) as [string, GamepadButtonName][]) {
      const button = pad.buttons[Number(btnIndex)];
      if (!button) continue;

      const id = gamepadButtonId(pad.index, name);
      const state = gamepadState.get(id) ?? { pressed: false, lastAt: 0 };

      const stickX = pad.axes[0] ?? 0;
      const stickY = pad.axes[1] ?? 0;

      const isPressed =
        button.pressed ||
        (name === "right" && stickX > 0.5) ||
        (name === "left" && stickX < -0.5) ||
        (name === "down" && stickY > 0.5) ||
        (name === "up" && stickY < -0.5);

      const shouldFire =
        isPressed &&
        (!state.pressed
          ? true
          : now - state.lastAt > (state.lastAt === 0 ? INITIAL_REPEAT_DELAY : HELD_REPEAT_INTERVAL));

      if (shouldFire && count > 0) {
        if (showAddModal.value) {
          gamepadState.set(id, { pressed: true, lastAt: now });
          continue;
        }

        if (pendingLaunch.value) {
          if (name === "a") confirmLaunch();
          else if (name === "b") cancelLaunch();
          gamepadState.set(id, { pressed: true, lastAt: now });
          continue;
        }

        switch (name) {
          case "right":
            focusedIndex.value = Math.min(focusedIndex.value + 1, count - 1);
            break;
          case "left":
            focusedIndex.value = Math.max(focusedIndex.value - 1, 0);
            break;
          case "down":
            focusedIndex.value = Math.min(focusedIndex.value + cols, count - 1);
            break;
          case "up":
            focusedIndex.value = Math.max(focusedIndex.value - cols, 0);
            break;
          case "a":
            if (filteredGames.value[focusedIndex.value]) {
              requestLaunch(filteredGames.value[focusedIndex.value]);
            }
            break;
          case "b":
            showAddModal.value = false;
            break;
        }
        gamepadState.set(id, { pressed: true, lastAt: now });
      } else if (!isPressed && state.pressed) {
        gamepadState.set(id, { pressed: false, lastAt: 0 });
      }
    }
  }

  rafId = requestAnimationFrame(pollGamepads);
}

// ── Lifecycle ──────────────────────────────────────────────────────────────

onMounted(() => {
  loadGames();
  window.addEventListener("keydown", onKeyDown);
  rafId = requestAnimationFrame(pollGamepads);
});

onUnmounted(() => {
  window.removeEventListener("keydown", onKeyDown);
  cancelAnimationFrame(rafId);
});
</script>

<template>
  <div class="flex h-screen bg-zinc-950 text-white overflow-hidden">
    <Sidebar
      :search="search"
      :platform-filter="platformFilter"
      :sort-option="sortOption"
      :total-games="allGames.length"
      :steam-count="steamCount"
      :custom-count="customCount"
      @update:search="search = $event; focusedIndex = 0"
      @update:platform-filter="platformFilter = $event; focusedIndex = 0"
      @update:sort-option="sortOption = $event"
      @add-game="showAddModal = true"
    />

    <main id="game-grid-area" class="flex-1 overflow-y-auto px-6 py-6 relative">
      <!-- Notification banner -->
      <Transition name="slide-down">
        <div
          v-if="notification"
          class="flex items-start gap-3 mb-4 px-4 py-3 rounded-md text-sm border"
          :class="notification.type === 'error'
            ? 'bg-red-950/50 border-red-900 text-red-300'
            : 'bg-zinc-900 border-zinc-700 text-zinc-300'"
        >
          <span class="flex-1">{{ notification.message }}</span>
          <button @click="notification = null" class="text-zinc-500 hover:text-white transition-colors">
            <svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
            </svg>
          </button>
        </div>
      </Transition>

      <!-- Loading -->
      <div v-if="loading" class="flex items-center justify-center h-full">
        <svg class="animate-spin w-6 h-6 text-zinc-500" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
          <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4" />
          <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8v8H4z" />
        </svg>
      </div>

      <!-- Error -->
      <div v-else-if="loadError" class="flex flex-col items-center justify-center h-full gap-3 text-zinc-400">
        <p class="text-sm font-medium text-white">Failed to load library</p>
        <p class="text-sm text-zinc-500">{{ loadError }}</p>
        <button
          @click="loadGames"
          class="mt-2 px-4 py-2 text-sm rounded-md border border-zinc-700 hover:bg-zinc-800 transition-colors"
        >
          Retry
        </button>
      </div>

      <!-- Library header + grid -->
      <template v-else>
        <div class="flex items-center justify-between mb-6">
          <h1 class="text-sm font-medium text-zinc-400">
            {{ platformFilter === "all" ? "All Games" : platformFilter === "steam" ? "Steam" : "Custom" }}
          </h1>
          <span class="text-xs text-zinc-600">{{ filteredGames.length }} game{{ filteredGames.length !== 1 ? "s" : "" }}</span>
        </div>

        <GameGrid
          :games="filteredGames"
          :focused-index="focusedIndex"
          @launch="requestLaunch"
          @update:focused-index="focusedIndex = $event"
        />
      </template>
    </main>

    <AddGameModal
      v-if="showAddModal"
      @close="showAddModal = false"
      @added="onGameAdded"
    />

    <LaunchConfirmDialog
      v-if="pendingLaunch"
      :game="pendingLaunch"
      @confirm="confirmLaunch"
      @cancel="cancelLaunch"
    />
  </div>
</template>

<style>
.slide-down-enter-active,
.slide-down-leave-active {
  transition: opacity 0.15s ease, transform 0.15s ease;
}
.slide-down-enter-from,
.slide-down-leave-to {
  opacity: 0;
  transform: translateY(-6px);
}
</style>
