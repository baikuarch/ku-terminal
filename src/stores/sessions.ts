import { defineStore, acceptHMRUpdate } from "pinia";
import type { AppConfig, SessionDef } from "../types";
import { loadConfig, saveConfig } from "../lib/ipc";

const FIXED_GROUPS = ["Production", "Staging", "Development"];
const CONFIG_VERSION = 3;
const LEGACY_STORAGE_KEYS = {
  customGroups: "ku:custom-groups",
  collapsedGroups: "ku:collapsed-groups",
  hiddenGroups: "ku:hidden-groups",
  sidebarCollapsed: "ku:sidebar-collapsed",
} as const;

function readLegacyList(key: string): string[] {
  try {
    const value = localStorage.getItem(key);
    if (!value) return [];
    const parsed: unknown = JSON.parse(value);
    return Array.isArray(parsed)
      ? parsed.filter((item): item is string => typeof item === "string")
      : [];
  } catch {
    return [];
  }
}

function uniqueNames(names: string[]): string[] {
  return [...new Set(names.map((name) => name.trim()).filter(Boolean))];
}

export const useSessionsStore = defineStore("sessions", {
  state: () => ({
    sessions: [] as SessionDef[],
    filter: "",
    activeSessionId: null as string | null,
    loaded: false,
    configVersion: CONFIG_VERSION,
    customGroups: [] as string[],
    collapsedGroups: [] as string[],
    hiddenGroups: [] as string[],
    sidebarCollapsed: false,
  }),
  getters: {
    allGroupNames(state): string[] {
      const set = new Set<string>(
        FIXED_GROUPS.filter((group) => !state.hiddenGroups.includes(group)),
      );
      for (const session of state.sessions) set.add(session.group);
      for (const group of state.customGroups) set.add(group);
      return [...set];
    },
    groups(state): { name: string; items: SessionDef[]; collapsed: boolean }[] {
      const filter = state.filter.trim().toLowerCase();
      const byGroup = new Map<string, SessionDef[]>();
      const fixedVisible = FIXED_GROUPS.filter(
        (group) => !state.hiddenGroups.includes(group),
      );
      const knownGroups = new Set<string>([
        ...fixedVisible,
        ...state.customGroups,
      ]);
      for (const group of knownGroups) byGroup.set(group, []);
      for (const session of state.sessions) {
        if (filter && !session.name.toLowerCase().includes(filter)) continue;
        if (!byGroup.has(session.group)) byGroup.set(session.group, []);
        byGroup.get(session.group)!.push(session);
      }
      const customGroups = [...byGroup.keys()]
        .filter((group) => !FIXED_GROUPS.includes(group))
        .sort();
      const order = [...fixedVisible, ...customGroups];
      return order
        .filter((group) => !filter || byGroup.get(group)!.length > 0)
        .map((group) => ({
          name: group,
          items: byGroup.get(group)!,
          collapsed: state.collapsedGroups.includes(group),
        }));
    },
    byId(state) {
      return (id: string | null) =>
        state.sessions.find((session) => session.id === id) ?? null;
    },
  },
  actions: {
    async load() {
      if (this.loaded) return;

      try {
        const config = await loadConfig();
        this.applyConfig(config);
        if (config.version < CONFIG_VERSION) {
          this.migrateLegacyStorage();
          await this.persist();
          this.clearLegacyStorage();
        }
        if (!this.activeSessionId && this.sessions.length) {
          this.activeSessionId = this.sessions[0].id;
        }
      } catch (error) {
        console.error("failed to load configuration", error);
      } finally {
        this.loaded = true;
      }
    },
    applyConfig(config: AppConfig) {
      this.configVersion = config.version;
      this.sessions = config.sessions;
      this.customGroups = uniqueNames(config.customGroups);
      this.collapsedGroups = uniqueNames(config.collapsedGroups);
      this.hiddenGroups = uniqueNames(config.hiddenGroups);
      this.sidebarCollapsed = config.sidebarCollapsed;
    },
    migrateLegacyStorage() {
      this.customGroups = uniqueNames([
        ...this.customGroups,
        ...readLegacyList(LEGACY_STORAGE_KEYS.customGroups),
      ]);
      this.collapsedGroups = uniqueNames([
        ...this.collapsedGroups,
        ...readLegacyList(LEGACY_STORAGE_KEYS.collapsedGroups),
      ]);
      this.hiddenGroups = uniqueNames([
        ...this.hiddenGroups,
        ...readLegacyList(LEGACY_STORAGE_KEYS.hiddenGroups),
      ]);
      this.sidebarCollapsed =
        this.sidebarCollapsed ||
        localStorage.getItem(LEGACY_STORAGE_KEYS.sidebarCollapsed) === "1";
    },
    clearLegacyStorage() {
      for (const key of Object.values(LEGACY_STORAGE_KEYS)) {
        localStorage.removeItem(key);
      }
    },
    toConfig(): AppConfig {
      return {
        version: CONFIG_VERSION,
        sessions: this.sessions,
        customGroups: this.customGroups,
        collapsedGroups: this.collapsedGroups,
        hiddenGroups: this.hiddenGroups,
        sidebarCollapsed: this.sidebarCollapsed,
      };
    },
    async persist() {
      try {
        await saveConfig(this.toConfig());
        this.configVersion = CONFIG_VERSION;
      } catch (error) {
        console.error("failed to save configuration", error);
      }
    },
    setFilter(value: string) {
      this.filter = value;
    },
    setActive(id: string) {
      this.activeSessionId = id;
    },
    setSidebarCollapsed(value: boolean) {
      this.sidebarCollapsed = value;
      void this.persist();
    },
    toggleGroup(name: string) {
      const index = this.collapsedGroups.indexOf(name);
      if (index >= 0) this.collapsedGroups.splice(index, 1);
      else this.collapsedGroups.push(name);
      void this.persist();
    },
    addGroup(name: string): boolean {
      const trimmed = name.trim();
      if (!trimmed || this.allGroupNames.includes(trimmed)) return false;
      this.customGroups.push(trimmed);
      void this.persist();
      return true;
    },
    async renameGroup(oldName: string, newName: string): Promise<boolean> {
      const trimmed = newName.trim();
      if (!trimmed || trimmed === oldName || this.allGroupNames.includes(trimmed)) {
        return false;
      }

      for (const session of this.sessions) {
        if (session.group === oldName) session.group = trimmed;
      }
      const customIndex = this.customGroups.indexOf(oldName);
      if (customIndex >= 0) this.customGroups.splice(customIndex, 1, trimmed);
      else if (FIXED_GROUPS.includes(oldName)) {
        if (!this.hiddenGroups.includes(oldName)) this.hiddenGroups.push(oldName);
        this.customGroups.push(trimmed);
      }
      const collapsedIndex = this.collapsedGroups.indexOf(oldName);
      if (collapsedIndex >= 0) {
        this.collapsedGroups.splice(collapsedIndex, 1, trimmed);
      }

      await this.persist();
      return true;
    },
    async deleteGroup(name: string): Promise<void> {
      const removedActive = this.sessions.some(
        (session) => session.group === name && session.id === this.activeSessionId,
      );
      this.sessions = this.sessions.filter((session) => session.group !== name);
      if (removedActive) this.activeSessionId = this.sessions[0]?.id ?? null;

      const customIndex = this.customGroups.indexOf(name);
      if (customIndex >= 0) this.customGroups.splice(customIndex, 1);
      else if (FIXED_GROUPS.includes(name) && !this.hiddenGroups.includes(name)) {
        this.hiddenGroups.push(name);
      }
      const collapsedIndex = this.collapsedGroups.indexOf(name);
      if (collapsedIndex >= 0) this.collapsedGroups.splice(collapsedIndex, 1);

      await this.persist();
    },
    async addSession(definition: SessionDef) {
      this.sessions.push(definition);
      await this.persist();
    },
    async updateSession(definition: SessionDef) {
      const index = this.sessions.findIndex(
        (session) => session.id === definition.id,
      );
      if (index < 0) return;
      this.sessions.splice(index, 1, definition);
      await this.persist();
    },
    async removeSession(id: string) {
      this.sessions = this.sessions.filter((session) => session.id !== id);
      if (this.activeSessionId === id) {
        this.activeSessionId = this.sessions[0]?.id ?? null;
      }
      await this.persist();
    },
  },
});

if (import.meta.hot) {
  import.meta.hot.accept(acceptHMRUpdate(useSessionsStore, import.meta.hot));
}
