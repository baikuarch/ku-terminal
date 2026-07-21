<script setup lang="ts">
import { onMounted, ref } from "vue";
import { useSessionsStore } from "../stores/sessions";
import { useTabsStore } from "../stores/tabs";
import ResourceMonitor from "./ResourceMonitor.vue";
import AddSessionDialog from "./AddSessionDialog.vue";
import type { SessionDef } from "../types";
import { dialogs } from "../lib/dialogs";
import { deleteSshPassword, saveSshPassword } from "../lib/ipc";

defineProps<{ collapsed?: boolean }>();
defineEmits<{ (e: "toggle"): void }>();

const sessions = useSessionsStore();
const tabs = useTabsStore();
const showAdd = ref(false);
const presetGroup = ref<string | undefined>(undefined);
const openGroupMenu = ref<string | null>(null);
const editingSession = ref<SessionDef | null>(null);

onMounted(() => {
  if (!sessions.loaded) sessions.load();
});

function onFilter(e: Event) {
  sessions.setFilter((e.target as HTMLInputElement).value);
}

function openSession(id: string, name: string) {
  sessions.setActive(id);
  tabs.openSsh(name, id);
}

async function onSave(def: SessionDef, password?: string) {
  if (editingSession.value) await sessions.updateSession(def);
  else await sessions.addSession(def);
  if (password && def.ssh?.credentialId) {
    try {
      await saveSshPassword(def.ssh.credentialId, password);
    } catch {
      await dialogs.alert({
        title: "Password not saved",
        message: "The session was saved, but its password could not be stored in the system credential store.",
      });
    }
  }
  closeDialog();
}

async function removeSession(id: string, name: string) {
  if (await dialogs.confirm({
    title: "Delete session",
    message: `Delete session "${name}"? This cannot be undone.`,
  confirmLabel: "Delete",
  tone: "danger",
  })) {
    const credentialId = sessions.byId(id)?.ssh?.credentialId ?? id;
    await sessions.removeSession(id);
    void deleteSshPassword(credentialId);
  }
}

function newSession(group?: string) {
  editingSession.value = null;
  presetGroup.value = group;
  showAdd.value = true;
  openGroupMenu.value = null;
}

function editSession(session: SessionDef) {
  editingSession.value = session;
  presetGroup.value = undefined;
  showAdd.value = true;
}

function closeDialog() {
  showAdd.value = false;
  editingSession.value = null;
  presetGroup.value = undefined;
}

async function newGroup() {
  const name = await dialogs.prompt({
    title: "New group",
    message: "Enter a name for the new session group.",
    placeholder: "Group name",
    confirmLabel: "Create",
  });
  if (name && !sessions.addGroup(name)) {
    await dialogs.alert({
      title: "Unable to create group",
      message: "A group with this name already exists, or the name is empty.",
    });
  }
}

function toggleGroupMenu(name: string) {
  openGroupMenu.value = openGroupMenu.value === name ? null : name;
}

async function renameGroup(name: string) {
  openGroupMenu.value = null;
  const next = await dialogs.prompt({
    title: "Rename group",
    message: `Choose a new name for "${name}".`,
    value: name,
    placeholder: "Group name",
    confirmLabel: "Rename",
  });
  if (next === null) return;
  const ok = await sessions.renameGroup(name, next);
  if (!ok) {
    await dialogs.alert({
      title: "Unable to rename group",
      message: "The name is empty, unchanged, or already in use.",
    });
  }
}

async function deleteGroup(name: string, count: number) {
  openGroupMenu.value = null;
  const msg =
    count > 0
      ? `Delete group "${name}" and its ${count} session(s)?`
      : `Delete group "${name}"?`;
  if (await dialogs.confirm({
    title: "Delete group",
    message: msg,
    confirmLabel: "Delete",
    tone: "danger",
  })) {
    await sessions.deleteGroup(name);
  }
}
</script>

<template>
  <aside class="sidebar" :class="{ collapsed }">
    <!-- Collapsed: thin icon rail -->
    <template v-if="collapsed">
      <div class="rail">
        <button class="rail-btn" title="Expand sidebar" @click="$emit('toggle')">
          <div class="i-lucide-panel-left-open text-16px" />
        </button>
        <div class="rail-divider" />
        <button
          v-for="s in sessions.sessions"
          :key="s.id"
          class="rail-item"
          :class="{ active: sessions.activeSessionId === s.id }"
          :title="`${s.name} · ${s.group}`"
          @click="openSession(s.id, s.name)"
        >
          <span class="status-dot" :class="s.status" />
        </button>
        <button class="rail-btn add" title="New session" @click="newSession()">
          <div class="i-lucide-plus text-16px" />
        </button>
      </div>
    </template>

    <!-- Expanded: full sidebar -->
    <template v-else>
      <div class="sidebar-body">
        <div class="section-head">
          <span class="section-title">Sessions</span>
          <div class="head-actions">
            <button class="add-btn" title="New session" @click="newSession()">
              <div class="i-lucide-plus text-15px" />
            </button>
            <button class="add-btn" title="New group" @click="newGroup()">
              <div class="i-lucide-folder-plus text-15px" />
            </button>
            <button
              class="add-btn"
              title="Collapse sidebar"
              @click="$emit('toggle')"
            >
              <div class="i-lucide-panel-left-close text-15px" />
            </button>
          </div>
        </div>
        <div class="filter">
          <span class="filter-icon">⌕</span>
          <input
            type="text"
            placeholder="Filter sessions…"
            :value="sessions.filter"
            @input="onFilter"
          />
        </div>

        <div class="groups">
          <div v-for="g in sessions.groups" :key="g.name" class="group">
            <div class="group-header">
              <button class="group-toggle" @click="sessions.toggleGroup(g.name)">
                <div
                  class="chevron"
                  :class="g.collapsed ? 'i-lucide-chevron-right' : 'i-lucide-chevron-down'"
                />
                <span class="group-label">{{ g.name.toUpperCase() }}</span>
                <span class="group-count">{{ g.items.length }}</span>
              </button>
              <div class="group-menu-wrap">
                <button
                  class="group-more"
                  title="Group options"
                  @click="toggleGroupMenu(g.name)"
                >
                  <div class="i-lucide-more-horizontal text-13px" />
                </button>
                <div
                  v-if="openGroupMenu === g.name"
                  class="menu-overlay"
                  @click="openGroupMenu = null"
                />
                <div v-if="openGroupMenu === g.name" class="group-menu">
                  <button class="menu-item" @click="newSession(g.name)">
                    <div class="i-lucide-plus text-13px" />
                    <span>Add Session</span>
                  </button>
                  <button class="menu-item" @click="renameGroup(g.name)">
                    <div class="i-lucide-pencil text-13px" />
                    <span>Rename Group</span>
                  </button>
                  <button
                    class="menu-item danger"
                    @click="deleteGroup(g.name, g.items.length)"
                  >
                    <div class="i-lucide-trash-2 text-13px" />
                    <span>Delete Group</span>
                  </button>
                </div>
              </div>
            </div>
            <template v-if="!g.collapsed">
              <div v-for="s in g.items" :key="s.id" class="session-row">
                <button
                  class="session-item"
                  :class="{ active: sessions.activeSessionId === s.id }"
                  @click="openSession(s.id, s.name)"
                >
                  <span class="status-dot" :class="s.status" />
                  <span class="session-name">{{ s.name }}</span>
                </button>
                <div class="session-actions">
                  <button
                    class="session-action"
                    title="Edit session"
                    @click="editSession(s)"
                  >
                    <div class="i-lucide-pencil text-13px" />
                  </button>
                  <button
                    class="session-action delete"
                    title="Delete session"
                    @click="removeSession(s.id, s.name)"
                  >
                    <div class="i-lucide-trash-2 text-13px" />
                  </button>
                </div>
              </div>
              <div v-if="g.items.length === 0" class="group-empty">
                No sessions
              </div>
            </template>
          </div>
        </div>
      </div>

      <ResourceMonitor />
    </template>

    <AddSessionDialog
      v-if="showAdd"
      :group="presetGroup"
      :session="editingSession ?? undefined"
      @close="closeDialog"
      @save="onSave"
    />
  </aside>
</template>

<style scoped>
.sidebar {
  display: flex;
  flex-direction: column;
  height: 100%;
  background: var(--bg-app);
  border-right: 1px solid var(--border-subtle);
  box-shadow: 8px 0 24px rgba(0, 0, 0, 0.09);
}
.sidebar-body {
  flex: 1;
  overflow-y: auto;
  padding: 14px 10px 16px;
  scrollbar-gutter: stable;
}
.section-head {
  display: flex;
  align-items: center;
  justify-content: space-between;
  min-height: 28px;
  margin: 0 2px 12px;
}
.section-title {
  font-size: var(--fs-xs);
  font-weight: 700;
  letter-spacing: 0.9px;
  color: var(--text-secondary);
}
.head-actions {
  display: flex;
  gap: 3px;
}
.add-btn {
  width: 26px;
  height: 26px;
  border-radius: var(--radius-sm);
  color: var(--text-secondary);
  display: grid;
  place-items: center;
  transition: background 0.16s ease, color 0.16s ease, transform 0.16s ease;
}
.add-btn:hover {
  background: var(--bg-hover);
  color: var(--text-primary);
}
.head-actions .add-btn:first-child:hover {
  color: var(--color-primary);
  background: rgba(97, 175, 239, 0.13);
}
.add-btn:active {
  transform: scale(0.92);
}
/* ── Collapsed icon rail ── */
.rail {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 6px;
  padding: 12px 0 10px;
  height: 100%;
  background: linear-gradient(90deg, rgba(33, 37, 43, 0.4), transparent);
}
.rail-btn {
  width: 32px;
  height: 32px;
  border-radius: 5px;
  color: var(--text-secondary);
  display: grid;
  place-items: center;
  transition: background 0.16s ease, color 0.16s ease, transform 0.16s ease;
}
.rail-btn:hover {
  background: rgba(97, 175, 239, 0.11);
  color: var(--color-primary);
}
.rail-btn:active { transform: scale(0.92); }
.rail-btn.add {
  margin-top: auto;
  color: var(--color-primary);
  border: 1px solid rgba(97, 175, 239, 0.25);
}
.rail-divider {
  width: 20px;
  height: 1px;
  background: var(--border-subtle);
  margin: 4px 0 6px;
}
.rail-item {
  position: relative;
  width: 32px;
  height: 32px;
  border-radius: 5px;
  display: grid;
  place-items: center;
  transition: background 0.16s ease;
}
.rail-item:hover {
  background: var(--bg-hover);
}
.rail-item.active {
  background: rgba(97, 175, 239, 0.14);
}
.rail-item.active::before {
  position: absolute;
  top: 8px;
  left: -8px;
  width: 3px;
  height: 16px;
  border-radius: 0 2px 2px 0;
  background: var(--color-primary);
  content: "";
}
.filter {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  height: 32px;
  padding: 0 9px;
  background: var(--bg-input);
  border: 1px solid var(--border-subtle);
  border-radius: 5px;
  margin-bottom: 17px;
  box-shadow: inset 0 1px 0 rgba(255, 255, 255, 0.015);
  transition: border-color 0.16s ease, background 0.16s ease, box-shadow 0.16s ease;
}
.filter:focus-within {
  border-color: var(--color-primary);
  background: #252a31;
  box-shadow: 0 0 0 2px rgba(97, 175, 239, 0.12);
}
.filter-icon {
  color: var(--text-muted);
}
.filter input {
  flex: 1;
  background: none;
  border: none;
  color: var(--text-primary);
  font-size: var(--fs-sm);
}
.filter input::placeholder {
  color: var(--text-muted);
}
.group {
  margin-bottom: 15px;
}
.group-header {
  display: flex;
  align-items: center;
  min-height: 23px;
  margin: 0 2px 5px;
}
.group-toggle {
  flex: 1;
  display: flex;
  align-items: center;
  gap: 4px;
  color: var(--text-muted);
  text-align: left;
  min-width: 0;
  transition: color 0.16s ease;
}
.group-toggle:hover {
  color: var(--text-secondary);
}
.chevron {
  font-size: 12px;
  flex-shrink: 0;
}
.group-label {
  font-size: var(--fs-xs);
  letter-spacing: 0.75px;
  font-weight: 650;
}
.group-count {
  font-size: var(--fs-xs);
  color: var(--text-muted);
  background: rgba(157, 165, 180, 0.08);
  border-radius: 3px;
  padding: 1px 5px;
  min-width: 18px;
  text-align: center;
}
.group-menu-wrap {
  position: relative;
}
.group-more {
  width: 20px;
  height: 20px;
  border-radius: var(--radius-sm);
  color: var(--text-muted);
  display: grid;
  place-items: center;
  opacity: 0;
  transition: opacity 0.12s;
}
.group-header:hover .group-more {
  opacity: 1;
}
.group-more:hover {
  background: var(--bg-hover);
  color: var(--text-primary);
}
.menu-overlay {
  position: fixed;
  inset: 0;
  z-index: 40;
}
.group-menu {
  position: absolute;
  top: 24px;
  right: 0;
  z-index: 50;
  min-width: 160px;
  background: var(--bg-surface);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-md);
  box-shadow: var(--shadow-md);
  padding: var(--space-1);
}
.menu-item {
  width: 100%;
  display: flex;
  align-items: center;
  gap: var(--space-2);
  padding: 6px var(--space-2);
  border-radius: var(--radius-sm);
  color: var(--text-secondary);
  font-size: var(--fs-sm);
  text-align: left;
}
.menu-item:hover {
  background: var(--bg-hover);
  color: var(--text-primary);
}
.menu-item.danger {
  color: var(--color-tertiary);
}
.menu-item.danger:hover {
  background: rgba(224, 108, 117, 0.12);
  color: var(--color-tertiary);
}
.group-empty {
  font-size: var(--fs-xs);
  color: var(--text-muted);
  padding: var(--space-1) var(--space-2) var(--space-1) 20px;
  font-style: italic;
}
.session-row {
  position: relative;
  display: flex;
  align-items: center;
  gap: 2px;
  width: 100%;
}
.session-item {
  position: relative;
  flex: 1;
  min-width: 0;
  display: flex;
  align-items: center;
  gap: 9px;
  min-height: 32px;
  padding: 6px 9px;
  border-radius: 5px;
  color: var(--text-secondary);
  font-size: var(--fs-md);
  text-align: left;
  transition: background 0.16s ease, color 0.16s ease, transform 0.16s ease;
}
.session-item:hover {
  background: rgba(157, 165, 180, 0.09);
  color: var(--text-primary);
}
.session-item.active {
  background: linear-gradient(90deg, rgba(97, 175, 239, 0.2), rgba(97, 175, 239, 0.075));
  color: var(--text-primary);
  font-weight: 500;
}
.session-item.active::before {
  position: absolute;
  top: 7px;
  left: 0;
  width: 3px;
  height: calc(100% - 14px);
  border-radius: 0 2px 2px 0;
  background: var(--color-primary);
  box-shadow: 0 0 8px rgba(97, 175, 239, 0.55);
  content: "";
}
.session-item:active {
  transform: scale(0.985);
}
.session-name {
  flex: 1;
  min-width: 0;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}
.session-actions {
  display: flex;
  flex: 0 0 46px;
  align-items: center;
  gap: 2px;
}
.session-action {
  width: 22px;
  height: 28px;
  border-radius: 4px;
  color: var(--text-muted);
  display: grid;
  place-items: center;
  opacity: 0;
  pointer-events: none;
  transition: opacity 0.16s ease, color 0.16s ease, background 0.16s ease;
}
.session-row:hover .session-action,
.session-row:focus-within .session-action {
  opacity: 1;
  pointer-events: auto;
}
.session-action:hover,
.session-action:focus-visible {
  color: var(--text-primary);
  background: var(--bg-hover);
}
.session-action.delete:hover,
.session-action.delete:focus-visible {
  color: var(--color-tertiary);
  background: rgba(224, 108, 117, 0.12);
}
</style>
