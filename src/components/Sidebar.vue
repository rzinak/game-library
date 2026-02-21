<script setup lang="ts">
import type { PlatformFilter, SortOption } from "../types/game";

defineProps<{
  search: string;
  platformFilter: PlatformFilter;
  sortOption: SortOption;
  totalGames: number;
  steamCount: number;
  customCount: number;
}>();

const emit = defineEmits<{
  "update:search": [value: string];
  "update:platformFilter": [value: PlatformFilter];
  "update:sortOption": [value: SortOption];
  addGame: [];
}>();
</script>

<template>
  <aside class="w-52 shrink-0 flex flex-col gap-5 py-5 px-3 bg-zinc-950 border-r border-zinc-800">
    <!-- Logo -->
    <div class="flex items-center gap-2 px-2 py-1">
      <svg xmlns="http://www.w3.org/2000/svg" class="w-5 h-5 text-zinc-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5"
          d="M15 5v2m0 4v2m0 4v2M5 5a2 2 0 00-2 2v3a2 2 0 110 4v3a2 2 0 002 2h14a2 2 0 002-2v-3a2 2 0 110-4V7a2 2 0 00-2-2H5z" />
      </svg>
      <span class="text-sm font-semibold text-white tracking-tight">Game Library</span>
    </div>

    <!-- Search -->
    <div class="relative">
      <svg xmlns="http://www.w3.org/2000/svg" class="absolute left-2.5 top-2.5 w-3.5 h-3.5 text-zinc-500 pointer-events-none"
        fill="none" viewBox="0 0 24 24" stroke="currentColor">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
          d="M21 21l-4.35-4.35M11 19a8 8 0 100-16 8 8 0 000 16z" />
      </svg>
      <input
        type="text"
        placeholder="Search…"
        :value="search"
        @input="emit('update:search', ($event.target as HTMLInputElement).value)"
        class="w-full pl-8 pr-3 py-1.5 text-sm bg-zinc-900 text-white placeholder-zinc-500
               rounded-md border border-zinc-800 focus:outline-none focus:border-zinc-600 focus:ring-1 focus:ring-zinc-600 transition-colors"
      />
    </div>

    <!-- Platform filter -->
    <div class="flex flex-col gap-0.5">
      <p class="text-xs text-zinc-500 px-2 mb-1 font-medium">Platform</p>

      <button
        v-for="opt in ([
          { value: 'all',    label: 'All',    count: totalGames },
          { value: 'steam',  label: 'Steam',  count: steamCount },
          { value: 'custom', label: 'Custom', count: customCount },
        ] as const)"
        :key="opt.value"
        @click="emit('update:platformFilter', opt.value)"
        class="flex items-center justify-between w-full px-2 py-1.5 rounded-md text-sm transition-colors"
        :class="platformFilter === opt.value
          ? 'bg-zinc-800 text-white font-medium'
          : 'text-zinc-400 hover:bg-zinc-900 hover:text-zinc-200'"
      >
        <span>{{ opt.label }}</span>
        <span class="text-xs text-zinc-600">{{ opt.count }}</span>
      </button>
    </div>

    <!-- Sort -->
    <div class="flex flex-col gap-1.5">
      <p class="text-xs text-zinc-500 px-2 font-medium">Sort by</p>
      <select
        :value="sortOption"
        @change="emit('update:sortOption', ($event.target as HTMLSelectElement).value as SortOption)"
        class="w-full px-2 py-1.5 text-sm bg-zinc-900 text-white rounded-md border border-zinc-800
               focus:outline-none focus:border-zinc-600 cursor-pointer transition-colors"
      >
        <option value="alpha">A – Z</option>
        <option value="recentlyAdded">Recently Added</option>
      </select>
    </div>

    <!-- Spacer -->
    <div class="flex-1" />

    <!-- Add custom game -->
    <button
      @click="emit('addGame')"
      class="flex items-center justify-center gap-1.5 w-full px-3 py-2 rounded-md
             border border-zinc-700 text-zinc-300 text-sm font-medium
             hover:bg-zinc-800 hover:text-white transition-colors"
    >
      <svg xmlns="http://www.w3.org/2000/svg" class="w-3.5 h-3.5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
      </svg>
      Add Game
    </button>
  </aside>
</template>
