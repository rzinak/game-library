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
    class="group relative flex flex-col rounded-lg overflow-hidden cursor-pointer select-none
           bg-neutral-800 transition-transform duration-150
           hover:scale-105 hover:shadow-xl hover:shadow-black/60
           focus:outline-none"
    :class="focused ? 'ring-4 ring-indigo-400 scale-105 shadow-xl shadow-black/60' : ''"
    @click="emit('launch')"
    @keydown="onKeyDown"
    @focus="emit('focus')"
  >
    <!-- Cover art -->
    <div class="aspect-[2/3] w-full bg-neutral-700 overflow-hidden">
      <img
        v-if="game.coverImage"
        :src="game.coverImage"
        :alt="game.title"
        class="w-full h-full object-cover"
        loading="lazy"
      />
      <div
        v-else
        class="w-full h-full flex items-center justify-center text-neutral-500"
      >
        <svg xmlns="http://www.w3.org/2000/svg" class="w-12 h-12" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5"
            d="M15 5v2m0 4v2m0 4v2M5 5a2 2 0 00-2 2v3a2 2 0 110 4v3a2 2 0 002 2h14a2 2 0 002-2v-3a2 2 0 110-4V7a2 2 0 00-2-2H5z" />
        </svg>
      </div>
    </div>

    <!-- Bottom bar: title + badge -->
    <div class="px-2 py-2 flex flex-col gap-1">
      <p class="text-white text-sm font-medium leading-tight line-clamp-2">{{ game.title }}</p>
      <span
        class="self-start text-xs font-semibold px-1.5 py-0.5 rounded"
        :class="game.platform === 'steam'
          ? 'bg-sky-600 text-white'
          : 'bg-violet-600 text-white'"
      >
        {{ game.platform === "steam" ? "Steam" : "Custom" }}
      </span>
    </div>

    <!-- Hover / focus overlay with play icon -->
    <div
      class="absolute inset-0 flex items-center justify-center
             bg-black/0 group-hover:bg-black/40 transition-colors duration-150"
      :class="focused ? 'bg-black/40' : ''"
    >
      <svg
        xmlns="http://www.w3.org/2000/svg"
        class="w-12 h-12 text-white drop-shadow-lg opacity-0 group-hover:opacity-100 transition-opacity duration-150"
        :class="focused ? 'opacity-100' : ''"
        viewBox="0 0 24 24"
        fill="currentColor"
      >
        <path d="M8 5v14l11-7z" />
      </svg>
    </div>
  </div>
</template>
