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
  <aside class="w-56 shrink-0 flex flex-col gap-6 py-6 px-4 bg-neutral-900 border-r border-neutral-800">
    <!-- Logo / title -->
    <div class="flex items-center gap-2 px-1">
      <svg xmlns="http://www.w3.org/2000/svg" class="w-6 h-6 text-indigo-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
          d="M15 5v2m0 4v2m0 4v2M5 5a2 2 0 00-2 2v3a2 2 0 110 4v3a2 2 0 002 2h14a2 2 0 002-2v-3a2 2 0 110-4V7a2 2 0 00-2-2H5z" />
      </svg>
      <span class="text-white font-bold tracking-wide">Game Library</span>
    </div>

    <!-- Search -->
    <div class="relative">
      <svg xmlns="http://www.w3.org/2000/svg" class="absolute left-2.5 top-2.5 w-4 h-4 text-neutral-500 pointer-events-none"
        fill="none" viewBox="0 0 24 24" stroke="currentColor">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
          d="M21 21l-4.35-4.35M11 19a8 8 0 100-16 8 8 0 000 16z" />
      </svg>
      <input
        type="text"
        placeholder="Search…"
        :value="search"
        @input="emit('update:search', ($event.target as HTMLInputElement).value)"
        class="w-full pl-8 pr-3 py-2 text-sm bg-neutral-800 text-white placeholder-neutral-500
               rounded-md border border-neutral-700 focus:outline-none focus:ring-2 focus:ring-indigo-500"
      />
    </div>

    <!-- Platform filter -->
    <div class="flex flex-col gap-1">
      <p class="text-xs font-semibold uppercase tracking-wider text-neutral-500 px-1 mb-1">Platform</p>

      <button
        v-for="opt in ([
          { value: 'all',    label: 'All Games',  count: totalGames },
          { value: 'steam',  label: 'Steam',      count: steamCount },
          { value: 'custom', label: 'Custom',     count: customCount },
        ] as const)"
        :key="opt.value"
        @click="emit('update:platformFilter', opt.value)"
        class="flex items-center justify-between w-full px-3 py-1.5 rounded-md text-sm transition-colors"
        :class="platformFilter === opt.value
          ? 'bg-indigo-600 text-white font-semibold'
          : 'text-neutral-400 hover:bg-neutral-800 hover:text-white'"
      >
        <span>{{ opt.label }}</span>
        <span class="text-xs opacity-70">{{ opt.count }}</span>
      </button>
    </div>

    <!-- Sort -->
    <div class="flex flex-col gap-2">
      <p class="text-xs font-semibold uppercase tracking-wider text-neutral-500 px-1">Sort by</p>
      <select
        :value="sortOption"
        @change="emit('update:sortOption', ($event.target as HTMLSelectElement).value as SortOption)"
        class="w-full px-3 py-2 text-sm bg-neutral-800 text-white rounded-md border border-neutral-700
               focus:outline-none focus:ring-2 focus:ring-indigo-500 cursor-pointer"
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
      class="flex items-center justify-center gap-2 w-full px-3 py-2 rounded-md
             bg-indigo-600 hover:bg-indigo-500 text-white text-sm font-semibold transition-colors"
    >
      <svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
      </svg>
      Add Game
    </button>
  </aside>
</template>
