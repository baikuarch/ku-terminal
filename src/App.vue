<script setup lang="ts">
import { computed, ref } from "vue";
import TopBar from "./components/TopBar.vue";
import SessionSidebar from "./components/SessionSidebar.vue";
import TerminalArea from "./components/TerminalArea.vue";
import FileExplorer from "./components/FileExplorer.vue";
import CommandPalette from "./components/CommandPalette.vue";
import GlobalDialog from "./components/GlobalDialog.vue";
import { useSessionsStore } from "./stores/sessions";

const palette = ref<InstanceType<typeof CommandPalette> | null>(null);
const sessions = useSessionsStore();
const sidebarCollapsed = computed(() => sessions.sidebarCollapsed);

function toggleSidebar() {
  sessions.setSidebarCollapsed(!sidebarCollapsed.value);
}
</script>

<template>
  <div class="app">
    <TopBar @open-palette="palette?.show()" />
    <div class="body" :class="{ 'sidebar-collapsed': sidebarCollapsed }">
      <SessionSidebar
        class="col-sidebar"
        :collapsed="sidebarCollapsed"
        @toggle="toggleSidebar"
      />
      <main class="col-main">
        <TerminalArea />
      </main>
      <FileExplorer class="col-explorer" />
    </div>
    <CommandPalette ref="palette" />
    <GlobalDialog />
  </div>
</template>

<style scoped>
.app {
  display: flex;
  flex-direction: column;
  height: 100%;
  background: var(--bg-root);
}
.body {
  flex: 1;
  min-height: 0;
  display: grid;
  grid-template-columns: 240px 1fr 260px;
  grid-template-rows: minmax(0, 1fr);
  overflow: hidden;
  transition: grid-template-columns 0.18s ease;
}
.body.sidebar-collapsed {
  grid-template-columns: 48px 1fr 260px;
}
.body > * {
  min-height: 0;
  overflow: hidden;
}
.col-main {
  min-width: 0;
  padding: var(--space-3);
}
</style>
