<script setup lang="ts">
import { getCurrentWindow } from "@tauri-apps/api/window";
import BrandMark from "./BrandMark.vue";

defineEmits<{ (e: "open-palette"): void }>();

const appWindow = getCurrentWindow();

function minimizeWindow() {
  void appWindow.minimize();
}

function toggleMaximize() {
  void appWindow.toggleMaximize();
}

function closeWindow() {
  void appWindow.close();
}
</script>

<template>
  <header
    class="top-bar h-12 grid grid-cols-[240px_1fr_300px] items-center bg-app border-b border-border-subtle"
    data-tauri-drag-region
  >
    <div class="flex items-center px-4" data-tauri-drag-region>
      <BrandMark />
    </div>

    <div
      class="justify-self-center w-full max-w-420px flex items-center gap-2 px-3 h-8 bg-input border border-border-subtle rounded-md cursor-pointer hover:border-border-default"
      @click="$emit('open-palette')"
    >
      <div class="i-lucide-search text-txt-muted text-14px" />
      <span class="flex-1 text-txt-muted text-13px">Search resources, commands…</span>
      <span
        class="text-11px text-txt-muted border border-border-default rounded-sm px-1.5 py-px"
        >Ctrl K</span
      >
    </div>

    <div class="justify-self-end h-full flex items-center">
      <div class="flex gap-2 px-3" data-tauri-drag-region>
      <button class="icon-btn" title="Notifications">
        <div class="i-lucide-bell text-15px" />
      </button>
      <button class="icon-btn" title="Layout">
        <div class="i-lucide-layout-panel-left text-15px" />
      </button>
      <button class="icon-btn text-primary! hover:text-primary-hover!" title="Account">
        <div class="i-lucide-circle-user-round text-16px" />
      </button>
      </div>
      <div class="window-controls h-full flex border-l border-border-subtle">
        <button
          class="window-control"
          aria-label="Minimize window"
          title="Minimize"
          @mousedown.stop
          @click.stop="minimizeWindow"
        >
          <span class="i-lucide-minus text-16px" />
        </button>
        <button
          class="window-control"
          aria-label="Maximize window"
          title="Maximize"
          @mousedown.stop
          @click.stop="toggleMaximize"
        >
          <span class="i-lucide-square text-14px" />
        </button>
        <button
          class="window-control window-control--close"
          aria-label="Close window"
          title="Close"
          @mousedown.stop
          @click.stop="closeWindow"
        >
          <span class="i-lucide-x text-17px" />
        </button>
      </div>
    </div>
  </header>
</template>

<style scoped>
.top-bar {
  -webkit-app-region: drag;
}

.window-controls,
.window-control,
.top-bar :deep(button),
.top-bar [role="button"] {
  -webkit-app-region: no-drag;
}

.window-control {
  display: grid;
  width: 44px;
  height: 100%;
  place-items: center;
  color: var(--text-secondary);
  transition: color 0.12s ease, background-color 0.12s ease;
}

.window-control:hover {
  color: var(--text-primary);
  background: var(--bg-hover);
}

.window-control--close:hover {
  color: #fff;
  background: var(--color-tertiary);
}
</style>
