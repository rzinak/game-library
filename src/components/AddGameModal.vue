<script setup lang="ts">
import { reactive, ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import FileExplorer from "./FileExplorer.vue";
import type { CustomGame } from "../types/game";

const emit = defineEmits<{
  close: [];
  added: [game: CustomGame];
}>();

const form = reactive({
  title: "",
  executable: "",
  coverImage: "",
  tags: "",
  notes: "",
  submitting: false,
  error: "",
});

// Which file explorer is open: "executable" | "cover" | null
const explorerMode = ref<"executable" | "cover" | null>(null);

async function submit() {
  if (!form.title.trim() || !form.executable.trim()) {
    form.error = "Title and executable are required.";
    return;
  }

  form.submitting = true;
  form.error = "";
  try {
    const game = await invoke<CustomGame>("add_game", {
      title: form.title.trim(),
      executable: form.executable.trim(),
      coverImage: form.coverImage.trim() || null,
      tags: form.tags.split(",").map((t) => t.trim()).filter(Boolean),
      notes: form.notes.trim() || null,
    });
    emit("added", game);
  } catch (e) {
    form.error = String(e);
  } finally {
    form.submitting = false;
  }
}

function onExplorerSelect(path: string) {
  if (explorerMode.value === "executable") {
    form.executable = path;
  } else if (explorerMode.value === "cover") {
    form.coverImage = path;
  }
  explorerMode.value = null;
}

function onBackdropClick(e: MouseEvent) {
  if (e.target === e.currentTarget) emit("close");
}

function onKeyDown(e: KeyboardEvent) {
  if (e.key === "Escape" && !explorerMode.value) emit("close");
}
</script>

<template>
  <div
    class="fixed inset-0 z-50 flex items-center justify-center bg-black/70 backdrop-blur-sm"
    @click="onBackdropClick"
    @keydown="onKeyDown"
  >
    <div class="w-full max-w-md bg-neutral-900 rounded-xl shadow-2xl border border-neutral-700 p-6">

      <div class="flex items-center justify-between mb-5">
        <h2 class="text-white text-lg font-bold">Add Custom Game</h2>
        <button
          @click="emit('close')"
          class="text-neutral-500 hover:text-white transition-colors"
          aria-label="Close"
        >
          <svg xmlns="http://www.w3.org/2000/svg" class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
          </svg>
        </button>
      </div>

      <form @submit.prevent="submit" class="flex flex-col gap-4">

        <!-- Title -->
        <div>
          <label class="block text-sm text-neutral-400 mb-1">
            Title <span class="text-red-400">*</span>
          </label>
          <input
            v-model="form.title"
            type="text"
            placeholder="e.g. Hollow Knight"
            class="w-full px-3 py-2 bg-neutral-800 text-white text-sm rounded-md border border-neutral-700
                   focus:outline-none focus:ring-2 focus:ring-indigo-500 placeholder-neutral-600"
          />
        </div>

        <!-- Executable -->
        <div>
          <label class="block text-sm text-neutral-400 mb-1">
            Executable <span class="text-red-400">*</span>
          </label>
          <div class="flex gap-2">
            <input
              v-model="form.executable"
              type="text"
              placeholder="Path to executable or .app bundle"
              class="flex-1 min-w-0 px-3 py-2 bg-neutral-800 text-white text-sm rounded-md border border-neutral-700
                     focus:outline-none focus:ring-2 focus:ring-indigo-500 placeholder-neutral-600"
            />
            <button
              type="button"
              @click="explorerMode = 'executable'"
              class="shrink-0 flex items-center gap-1.5 px-3 py-2 text-sm rounded-md
                     bg-neutral-700 hover:bg-neutral-600 text-neutral-200 transition-colors"
              title="Browse for executable"
            >
              <svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                  d="M3 7a2 2 0 012-2h4l2 2h8a2 2 0 012 2v9a2 2 0 01-2 2H5a2 2 0 01-2-2V7z" />
              </svg>
              Browse
            </button>
          </div>
        </div>

        <!-- Cover image -->
        <div>
          <label class="block text-sm text-neutral-400 mb-1">Cover image</label>
          <div class="flex gap-2">
            <input
              v-model="form.coverImage"
              type="text"
              placeholder="Path to image (optional)"
              class="flex-1 min-w-0 px-3 py-2 bg-neutral-800 text-white text-sm rounded-md border border-neutral-700
                     focus:outline-none focus:ring-2 focus:ring-indigo-500 placeholder-neutral-600"
            />
            <button
              type="button"
              @click="explorerMode = 'cover'"
              class="shrink-0 flex items-center gap-1.5 px-3 py-2 text-sm rounded-md
                     bg-neutral-700 hover:bg-neutral-600 text-neutral-200 transition-colors"
              title="Browse for cover image"
            >
              <svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                  d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14
                     m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z" />
              </svg>
              Browse
            </button>
          </div>
        </div>

        <!-- Tags -->
        <div>
          <label class="block text-sm text-neutral-400 mb-1">Tags</label>
          <input
            v-model="form.tags"
            type="text"
            placeholder="rpg, indie, metroidvania (comma-separated)"
            class="w-full px-3 py-2 bg-neutral-800 text-white text-sm rounded-md border border-neutral-700
                   focus:outline-none focus:ring-2 focus:ring-indigo-500 placeholder-neutral-600"
          />
        </div>

        <!-- Notes -->
        <div>
          <label class="block text-sm text-neutral-400 mb-1">Notes</label>
          <textarea
            v-model="form.notes"
            rows="2"
            placeholder="Optional notes…"
            class="w-full px-3 py-2 bg-neutral-800 text-white text-sm rounded-md border border-neutral-700
                   focus:outline-none focus:ring-2 focus:ring-indigo-500 placeholder-neutral-600 resize-none"
          />
        </div>

        <p v-if="form.error" class="text-red-400 text-sm">{{ form.error }}</p>

        <div class="flex justify-end gap-3 pt-1">
          <button
            type="button"
            @click="emit('close')"
            class="px-4 py-2 text-sm text-neutral-400 hover:text-white transition-colors rounded-md
                   border border-neutral-700 hover:border-neutral-500"
          >
            Cancel
          </button>
          <button
            type="submit"
            :disabled="form.submitting"
            class="px-4 py-2 text-sm font-semibold bg-indigo-600 hover:bg-indigo-500 text-white
                   rounded-md transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
          >
            {{ form.submitting ? "Adding…" : "Add Game" }}
          </button>
        </div>
      </form>
    </div>
  </div>

  <!-- File explorer overlays (z-index 60, above this modal at 50) -->
  <FileExplorer
    v-if="explorerMode === 'executable'"
    title="Select Executable"
    @select="onExplorerSelect"
    @cancel="explorerMode = null"
  />
  <FileExplorer
    v-if="explorerMode === 'cover'"
    title="Select Cover Image"
    @select="onExplorerSelect"
    @cancel="explorerMode = null"
  />
</template>
