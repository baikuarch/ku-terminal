import { defineStore } from "pinia";

export interface TabDef {
  id: string; // client-side tab id
  title: string;
  kind: "local" | "ssh";
  /** backend session id, assigned once the PTY/SSH session is created */
  sessionId: string | null;
  /** for ssh tabs, the session catalog id to connect */
  sourceId?: string;
}

let counter = 0;
const nextId = () => `tab-${++counter}`;

export const useTabsStore = defineStore("tabs", {
  state: () => ({
    tabs: [] as TabDef[],
    activeTabId: null as string | null,
  }),
  getters: {
    activeTab(state): TabDef | null {
      return state.tabs.find((t) => t.id === state.activeTabId) ?? null;
    },
  },
  actions: {
    openLocal(title = "bash"): TabDef {
      const tab: TabDef = { id: nextId(), title, kind: "local", sessionId: null };
      this.tabs.push(tab);
      this.activeTabId = tab.id;
      return tab;
    },
    openSsh(title: string, sourceId: string): TabDef {
      const tab: TabDef = {
        id: nextId(),
        title,
        kind: "ssh",
        sessionId: null,
        sourceId,
      };
      this.tabs.push(tab);
      this.activeTabId = tab.id;
      return tab;
    },
    setActive(id: string) {
      this.activeTabId = id;
    },
    bindSession(tabId: string, sessionId: string) {
      const t = this.tabs.find((t) => t.id === tabId);
      if (t) t.sessionId = sessionId;
    },
    close(id: string) {
      const idx = this.tabs.findIndex((t) => t.id === id);
      if (idx < 0) return;
      this.tabs.splice(idx, 1);
      if (this.activeTabId === id) {
        const next = this.tabs[idx] ?? this.tabs[idx - 1] ?? null;
        this.activeTabId = next ? next.id : null;
      }
    },
  },
});
