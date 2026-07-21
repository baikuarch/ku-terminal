<script setup lang="ts">
import { onMounted, ref, watch } from "vue";
import { readDir, homeDir } from "../lib/ipc";
import type { FileEntry } from "../types";
import FileNode from "./FileNode.vue";
import { useTabsStore } from "../stores/tabs";

const tabs = useTabsStore();
const rootPath = ref("");
const entries = ref<FileEntry[]>([]);
const error = ref("");
const sessionId = ref<string | null>(null);

async function load(path: string) {
  try {
    error.value = "";
    rootPath.value = path;
    entries.value = await readDir(path, sessionId.value);
  } catch (e) {
    error.value = String(e);
    entries.value = [];
  }
}

async function loadForActive() {
  const active = tabs.activeTab;
  const sid = active?.kind === "ssh" && active.sessionId ? active.sessionId : null;
  sessionId.value = sid;
  try {
    const home = await homeDir(sid);
    await load(home);
  } catch (e) {
    error.value = String(e);
  }
}

onMounted(loadForActive);

// Re-root the tree when switching between local and SSH sessions.
watch(
  () => {
    const a = tabs.activeTab;
    return a ? `${a.id}:${a.sessionId ?? ""}` : "";
  },
  loadForActive,
);
</script>

<template>
  <aside class="explorer">
    <div class="header">
      <span class="title">FILE EXPLORER</span>
      <div class="header-actions">
        <button class="icon-btn" title="New folder">
          <div class="i-lucide-folder-plus text-13px" />
        </button>
        <button class="icon-btn" title="New file">
          <div class="i-lucide-file-plus text-13px" />
        </button>
      </div>
    </div>

    <div class="path" :title="rootPath">
      <div class="i-lucide-folder-open path-icon text-14px" />
      <span class="path-text">{{ rootPath || "…" }}</span>
    </div>

    <div class="tree">
      <div v-if="error" class="error">{{ error }}</div>
      <FileNode
        v-for="e in entries"
        :key="e.path"
        :entry="e"
        :depth="0"
        :session-id="sessionId"
      />
    </div>
  </aside>
</template>

<style scoped>
.explorer {
  display: flex;
  flex-direction: column;
  height: 100%;
  background: var(--bg-app);
  border-left: 1px solid var(--border-subtle);
  padding-bottom: var(--space-3);
}
.header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: var(--space-3);
  padding-bottom: var(--space-2);
}
.title {
  font-size: var(--fs-xs);
  color: var(--text-muted);
  letter-spacing: 0.6px;
}
.header-actions {
  display: flex;
  gap: var(--space-1);
}
.icon-btn {
  width: 24px;
  height: 24px;
  border-radius: var(--radius-sm);
  color: var(--text-secondary);
  display: grid;
  place-items: center;
  font-size: 12px;
}
.icon-btn:hover {
  background: var(--bg-hover);
  color: var(--text-primary);
}
.path {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  margin: 0 var(--space-3) var(--space-2);
  padding: 6px var(--space-2);
  background: var(--bg-input);
  border: 1px solid var(--border-subtle);
  border-radius: var(--radius-md);
  font-size: var(--fs-xs);
  color: var(--text-secondary);
}
.path-icon {
  color: var(--color-primary);
  flex-shrink: 0;
}
.path-text {
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  direction: rtl;
  text-align: left;
}
.tree {
  flex: 1;
  min-height: 0;
  overflow-y: auto;
  padding: 0 var(--space-2);
}
.error {
  color: var(--color-tertiary);
  font-size: var(--fs-xs);
  padding: var(--space-2);
}
</style>
