<script setup lang="ts">
import { computed, nextTick, onMounted, onUnmounted, ref, watch } from "vue";
import { useSessionsStore } from "../stores/sessions";
import { useTabsStore } from "../stores/tabs";

const sessions = useSessionsStore();
const tabs = useTabsStore();

const open = ref(false);
const query = ref("");
const activeIndex = ref(0);
const inputEl = ref<HTMLInputElement | null>(null);

interface Item {
  id: string;
  label: string;
  hint: string;
  icon: string;
  run: () => void;
}

const items = computed<Item[]>(() => {
  const q = query.value.trim().toLowerCase();
  const list: Item[] = [];

  // Action: new local shell
  list.push({
    id: "action:new-local",
    label: "New Local Shell",
    hint: "action",
    icon: "▪",
    run: () => tabs.openLocal("bash"),
  });

  // Sessions
  for (const s of sessions.sessions) {
    list.push({
      id: `session:${s.id}`,
      label: s.name,
      hint: `${s.group} · ${s.ssh ? `${s.ssh.user}@${s.ssh.host}` : "local"}`,
      icon: "⇅",
      run: () => {
        sessions.setActive(s.id);
        tabs.openSsh(s.name, s.id);
      },
    });
  }

  // Open tabs
  for (const t of tabs.tabs) {
    list.push({
      id: `tab:${t.id}`,
      label: t.title,
      hint: "open tab",
      icon: "▸",
      run: () => tabs.setActive(t.id),
    });
  }

  if (!q) return list;
  return list.filter(
    (i) =>
      i.label.toLowerCase().includes(q) || i.hint.toLowerCase().includes(q),
  );
});

watch(query, () => (activeIndex.value = 0));
watch(items, () => {
  if (activeIndex.value >= items.value.length) activeIndex.value = 0;
});

function show() {
  open.value = true;
  query.value = "";
  activeIndex.value = 0;
  nextTick(() => inputEl.value?.focus());
}

function hide() {
  open.value = false;
}

function choose(i: number) {
  const item = items.value[i];
  if (item) {
    item.run();
    hide();
  }
}

function onKeydown(e: KeyboardEvent) {
  if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === "k") {
    e.preventDefault();
    open.value ? hide() : show();
    return;
  }
  if (!open.value) return;
  if (e.key === "Escape") {
    e.preventDefault();
    hide();
  } else if (e.key === "ArrowDown") {
    e.preventDefault();
    activeIndex.value = Math.min(activeIndex.value + 1, items.value.length - 1);
  } else if (e.key === "ArrowUp") {
    e.preventDefault();
    activeIndex.value = Math.max(activeIndex.value - 1, 0);
  } else if (e.key === "Enter") {
    e.preventDefault();
    choose(activeIndex.value);
  }
}

onMounted(() => window.addEventListener("keydown", onKeydown));
onUnmounted(() => window.removeEventListener("keydown", onKeydown));

defineExpose({ show });
</script>

<template>
  <div v-if="open" class="overlay" @click.self="hide">
    <div class="palette">
      <div class="search">
        <span class="search-icon">⌕</span>
        <input
          ref="inputEl"
          v-model="query"
          type="text"
          placeholder="Search sessions, commands…"
        />
        <span class="kbd">Esc</span>
      </div>
      <div class="results">
        <div v-if="items.length === 0" class="empty">No matches</div>
        <button
          v-for="(item, i) in items"
          :key="item.id"
          class="result"
          :class="{ active: i === activeIndex }"
          @click="choose(i)"
          @mousemove="activeIndex = i"
        >
          <span class="result-icon">{{ item.icon }}</span>
          <span class="result-label">{{ item.label }}</span>
          <span class="result-hint">{{ item.hint }}</span>
        </button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.overlay {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.45);
  display: flex;
  justify-content: center;
  align-items: flex-start;
  padding-top: 12vh;
  z-index: 200;
}
.palette {
  width: 560px;
  max-width: 90vw;
  background: var(--bg-surface);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-lg);
  box-shadow: var(--shadow-lg);
  overflow: hidden;
}
.search {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  padding: var(--space-3) var(--space-4);
  border-bottom: 1px solid var(--border-subtle);
}
.search-icon {
  color: var(--text-muted);
  font-size: 16px;
}
.search input {
  flex: 1;
  background: none;
  border: none;
  color: var(--text-primary);
  font-size: var(--fs-lg);
}
.search input::placeholder {
  color: var(--text-muted);
}
.kbd {
  font-size: var(--fs-xs);
  color: var(--text-muted);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-sm);
  padding: 1px 6px;
}
.results {
  max-height: 360px;
  overflow-y: auto;
  padding: var(--space-2);
}
.empty {
  padding: var(--space-4);
  text-align: center;
  color: var(--text-muted);
  font-size: var(--fs-sm);
}
.result {
  width: 100%;
  display: flex;
  align-items: center;
  gap: var(--space-3);
  padding: var(--space-2) var(--space-3);
  border-radius: var(--radius-md);
  text-align: left;
}
.result.active {
  background: var(--bg-active);
}
.result-icon {
  color: var(--color-primary);
  font-size: 12px;
  width: 16px;
}
.result-label {
  color: var(--text-primary);
  font-size: var(--fs-md);
}
.result-hint {
  margin-left: auto;
  color: var(--text-muted);
  font-size: var(--fs-xs);
}
</style>
