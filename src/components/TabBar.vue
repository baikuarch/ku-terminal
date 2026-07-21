<script setup lang="ts">
import { useTabsStore } from "../stores/tabs";

const tabs = useTabsStore();

function icon(kind: "local" | "ssh") {
  return kind === "ssh" ? "⇅" : "▪";
}
</script>

<template>
  <div class="tabbar">
    <button
      v-for="t in tabs.tabs"
      :key="t.id"
      class="tab"
      :class="{ active: tabs.activeTabId === t.id }"
      @click="tabs.setActive(t.id)"
    >
      <span class="tab-icon">{{ icon(t.kind) }}</span>
      <span class="tab-title">{{ t.title }}</span>
      <span class="tab-close" @click.stop="tabs.close(t.id)">×</span>
    </button>
    <button class="tab-add" title="New local shell" @click="tabs.openLocal()">
      +
    </button>
  </div>
</template>

<style scoped>
.tabbar {
  display: flex;
  align-items: stretch;
  gap: 2px;
  height: 38px;
  padding: 0 var(--space-2);
  background: var(--bg-surface);
  border-bottom: 1px solid var(--border-subtle);
}
.tab {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  padding: 0 var(--space-3);
  color: var(--text-secondary);
  font-size: var(--fs-sm);
  border-top: 2px solid transparent;
  max-width: 180px;
}
.tab:hover {
  color: var(--text-primary);
  background: var(--bg-hover);
}
.tab.active {
  color: var(--text-primary);
  background: var(--bg-app);
  border-top-color: var(--color-primary);
}
.tab-icon {
  color: var(--color-primary);
  font-size: 11px;
}
.tab-title {
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}
.tab-close {
  color: var(--text-muted);
  border-radius: var(--radius-sm);
  width: 16px;
  height: 16px;
  display: grid;
  place-items: center;
  font-size: 14px;
}
.tab-close:hover {
  background: var(--bg-active);
  color: var(--color-tertiary);
}
.tab-add {
  width: 32px;
  color: var(--text-secondary);
  font-size: 16px;
}
.tab-add:hover {
  color: var(--color-primary);
  background: var(--bg-hover);
}
</style>
