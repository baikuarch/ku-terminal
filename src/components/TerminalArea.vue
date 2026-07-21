<script setup lang="ts">
import { onMounted } from "vue";
import TabBar from "./TabBar.vue";
import TerminalView from "./TerminalView.vue";
import { useTabsStore } from "../stores/tabs";

const tabs = useTabsStore();

onMounted(() => {
  if (tabs.tabs.length === 0) tabs.openLocal("bash");
});
</script>

<template>
  <section class="terminal-area">
    <TabBar />
    <div class="term-stack">
      <!-- keep every terminal alive; only toggle visibility so history/scroll persist -->
      <div
        v-for="t in tabs.tabs"
        :key="t.id"
        class="term-slot"
        v-show="tabs.activeTabId === t.id"
      >
        <TerminalView :tab="t" />
      </div>
      <div v-if="tabs.tabs.length === 0" class="empty">
        No sessions open. Press + to start a shell.
      </div>
    </div>
  </section>
</template>

<style scoped>
.terminal-area {
  display: flex;
  flex-direction: column;
  height: 100%;
  background: var(--bg-app);
  border-radius: var(--radius-lg);
  overflow: hidden;
  border: 1px solid var(--border-subtle);
}
.term-stack {
  position: relative;
  flex: 1;
  min-height: 0;
}
.term-slot {
  position: absolute;
  inset: 0;
}
.empty {
  display: grid;
  place-items: center;
  height: 100%;
  color: var(--text-muted);
  font-size: var(--fs-md);
}
</style>
