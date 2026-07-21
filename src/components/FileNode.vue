<script setup lang="ts">
import { ref } from "vue";
import { readDir } from "../lib/ipc";
import type { FileEntry } from "../types";

const props = defineProps<{
  entry: FileEntry;
  depth: number;
  sessionId: string | null;
}>();

const expanded = ref(false);
const children = ref<FileEntry[] | null>(null);
const loading = ref(false);

async function toggle() {
  if (!props.entry.isDir) return;
  expanded.value = !expanded.value;
  if (expanded.value && children.value === null) {
    loading.value = true;
    try {
      children.value = await readDir(props.entry.path, props.sessionId);
    } catch {
      children.value = [];
    } finally {
      loading.value = false;
    }
  }
}

function typeIcon(e: FileEntry): string {
  if (e.isDir) return expanded.value ? "i-lucide-folder-open" : "i-lucide-folder";
  const ext = e.name.split(".").pop()?.toLowerCase();
  switch (ext) {
    case "js":
    case "cjs":
    case "mjs":
      return "i-lucide-file-code-2";
    case "ts":
    case "tsx":
    case "jsx":
    case "vue":
    case "rs":
    case "go":
    case "py":
    case "java":
    case "c":
    case "cpp":
    case "h":
    case "sh":
      return "i-lucide-file-code-2";
    case "json":
    case "yaml":
    case "yml":
    case "toml":
    case "xml":
      return "i-lucide-file-json-2";
    case "css":
    case "scss":
    case "less":
      return "i-lucide-file-type";
    case "html":
    case "htm":
      return "i-lucide-file-code";
    case "md":
    case "mdx":
    case "txt":
    case "log":
      return "i-lucide-file-text";
    case "png":
    case "jpg":
    case "jpeg":
    case "gif":
    case "svg":
    case "webp":
    case "ico":
      return "i-lucide-file-image";
    case "zip":
    case "tar":
    case "gz":
    case "rar":
    case "7z":
      return "i-lucide-file-archive";
    case "lock":
      return "i-lucide-file-lock-2";
    default:
      return "i-lucide-file";
  }
}
</script>

<template>
  <div class="node">
    <div
      class="row"
      :style="{ paddingLeft: depth * 12 + 8 + 'px' }"
      @click="toggle"
    >
      <span class="chevron">
        <div
          v-if="entry.isDir"
          :class="expanded ? 'i-lucide-chevron-down' : 'i-lucide-chevron-right'"
        />
      </span>
      <div class="type-icon" :class="[typeIcon(entry), { dir: entry.isDir }]" />
      <span class="name">{{ entry.name }}</span>
    </div>
    <div v-if="expanded">
      <div v-if="loading" class="hint" :style="{ paddingLeft: (depth + 1) * 12 + 8 + 'px' }">
        loading…
      </div>
      <FileNode
        v-for="c in children ?? []"
        :key="c.path"
        :entry="c"
        :depth="depth + 1"
        :session-id="sessionId"
      />
    </div>
  </div>
</template>

<script lang="ts">
export default { name: "FileNode" };
</script>

<style scoped>
.row {
  display: flex;
  align-items: center;
  gap: 4px;
  height: 26px;
  padding-right: var(--space-2);
  font-size: var(--fs-sm);
  color: var(--text-secondary);
  cursor: pointer;
  border-radius: var(--radius-sm);
}
.row:hover {
  background: var(--bg-hover);
  color: var(--text-primary);
}
.chevron {
  width: 14px;
  display: grid;
  place-items: center;
  color: var(--text-muted);
  font-size: 12px;
  flex-shrink: 0;
}
.type-icon {
  font-size: 14px;
  color: var(--text-muted);
  flex-shrink: 0;
}
.type-icon.dir {
  color: var(--color-primary);
}
.name {
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  margin-left: 2px;
}
.hint {
  font-size: var(--fs-xs);
  color: var(--text-muted);
  height: 22px;
  display: flex;
  align-items: center;
}
</style>
