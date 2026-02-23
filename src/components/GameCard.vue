<script setup lang="ts">
import type { Game } from "../types/game";

defineProps<{
  game: Game;
  focused: boolean;
}>();

const emit = defineEmits<{
  launch: [];
  focus: [];
}>();

function onKeyDown(e: KeyboardEvent) {
  if (e.key === "Enter" || e.key === " ") {
    e.preventDefault();
    emit("launch");
  }
}
</script>

<template>
  <div
    tabindex="0"
    role="button"
    :aria-label="`Launch ${game.title}`"
    class="group relative flex flex-col rounded-md overflow-hidden cursor-pointer select-none
           bg-zinc-900 border border-zinc-800 transition-all duration-150
           hover:border-zinc-600
           focus:outline-none"
    :class="focused ? 'ring-2 ring-zinc-500 border-zinc-600' : ''"
    @click="emit('launch')"
    @keydown="onKeyDown"
    @focus="emit('focus')"
  >
    <!-- Cover art -->
    <div class="aspect-[2/3] w-full bg-zinc-800 overflow-hidden">
      <img
        v-if="game.coverImage"
        :src="game.coverImage"
        :alt="game.title"
        class="w-full h-full object-cover"
        loading="lazy"
      />
      <div
        v-else
        class="w-full h-full flex items-center justify-center text-zinc-700"
      >
        <svg xmlns="http://www.w3.org/2000/svg" class="w-10 h-10" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1"
            d="M15 5v2m0 4v2m0 4v2M5 5a2 2 0 00-2 2v3a2 2 0 110 4v3a2 2 0 002 2h14a2 2 0 002-2v-3a2 2 0 110-4V7a2 2 0 00-2-2H5z" />
        </svg>
      </div>
    </div>

    <!-- Bottom bar -->
    <div class="px-2.5 py-2 flex flex-col gap-1 border-t border-zinc-800">
      <p class="text-white text-xs font-medium leading-tight line-clamp-2">{{ game.title }}</p>
      <span
        class="self-start text-[10px] font-medium px-1.5 py-0.5 rounded-sm text-zinc-400 bg-zinc-800"
      >
        {{ game.platform === "steam" ? "Steam" : game.platform === "epic" ? "Epic" : "Custom" }}
      </span>
    </div>

    <!-- Hover overlay with play icon -->
    <div
      class="absolute inset-0 flex items-center justify-center
             bg-black/0 group-hover:bg-black/50 transition-colors duration-150"
      :class="focused ? 'bg-black/50' : ''"
    >
      <svg
        xmlns="http://www.w3.org/2000/svg"
        class="w-10 h-10 text-white drop-shadow opacity-0 group-hover:opacity-100 transition-opacity duration-150"
        :class="focused ? 'opacity-100' : ''"
        viewBox="0 0 24 24"
        fill="currentColor"
      >
        <path d="M8 5v14l11-7z" />
      </svg>
    </div>
  </div>
</template>
