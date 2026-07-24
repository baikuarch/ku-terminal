<script setup lang="ts">
import { onMounted, onBeforeUnmount, ref, watch, nextTick } from "vue";
import { Terminal } from "@xterm/xterm";
import { FitAddon } from "@xterm/addon-fit";
import { WebLinksAddon } from "@xterm/addon-web-links";
import { Channel } from "@tauri-apps/api/core";
import { oneDarkTheme } from "../lib/theme";
import {
  createLocalSession,
  createSshSession,
  writeSession,
  resizeSession,
  closeSession,
  loadSshPassword,
  saveSshPassword,
  type OutputEvent,
} from "../lib/ipc";
import { useTabsStore } from "../stores/tabs";
import { useSessionsStore } from "../stores/sessions";
import type { TabDef } from "../stores/tabs";
import { dialogs } from "../lib/dialogs";

const props = defineProps<{ tab: TabDef }>();
const tabs = useTabsStore();
const sessions = useSessionsStore();

const host = ref<HTMLDivElement | null>(null);
let term: Terminal | null = null;
let fit: FitAddon | null = null;
let sessionId: string | null = null;
let resizeObserver: ResizeObserver | null = null;

function doFit() {
  if (!fit || !term || !host.value) return;
  // Skip while hidden (v-show sets display:none → zero size → bad fit).
  if (host.value.offsetParent === null) return;
  try {
    fit.fit();
    if (sessionId) resizeSession(sessionId, term.cols, term.rows);
  } catch {
    /* element not visible */
  }
}

onMounted(async () => {
  if (!host.value) return;

  term = new Terminal({
    theme: oneDarkTheme,
    fontFamily:
      '"Cascadia Code", "JetBrains Mono", "Fira Code", Consolas, "Courier New", monospace',
    fontSize: 13,
    cursorBlink: true,
    scrollback: 5000,
    allowProposedApi: true,
  });
  fit = new FitAddon();
  term.loadAddon(fit);
  term.loadAddon(new WebLinksAddon());
  term.open(host.value);
  // Defer initial fit until the flex layout has settled, else the first
  // fit() over-counts rows and the bottom lines overflow the clipped card.
  requestAnimationFrame(() => doFit());
  // The monospace font loads async; xterm measures cell height with the
  // fallback font first (shorter cells → too many rows), so the last row
  // overflows once the real font swaps in. Re-fit once fonts are ready.
  if (document.fonts?.ready) {
    document.fonts.ready.then(() => doFit());
  }

  const channel = new Channel<OutputEvent>();
  channel.onmessage = (msg) => {
    term?.write(new Uint8Array(msg.bytes));
  };

  try {
    if (props.tab.kind === "local") {
      sessionId = await createLocalSession(null, null, channel);
    } else {
      const def = sessions.sessions.find((s) => s.id === props.tab.sourceId);
      if (!def?.ssh) {
        throw new Error("no SSH config for this session");
      }
      term.writeln(
        `\x1b[90mConnecting to ${def.ssh.user}@${def.ssh.host}:${def.ssh.port}…\x1b[0m`,
      );
      let password: string | null = null;
      const authMethod = def.ssh.authMethod ?? (def.ssh.keyPath ? "key" : "password");
      if (authMethod === "password") {
        const credentialId = def.ssh.credentialId ?? def.id;
        password = await loadSshPassword(credentialId);
      }
      if (authMethod === "password" && password === null) {
        password = await dialogs.prompt({
          title: "SSH password",
          message: `Enter the password for ${def.ssh.user}@${def.ssh.host}.`,
          placeholder: "Password",
          confirmLabel: "Connect",
          secret: true,
        });
        if (password === null) {
          throw new Error("authentication cancelled");
        }
        void saveSshPassword(def.ssh.credentialId ?? def.id, password);
      }
      sessionId = await createSshSession(def.ssh, password, channel);
      // cd to startup directory if configured
      if (def.cwd) {
        writeSession(sessionId, `cd ${def.cwd}\n`);
      }
    }
    tabs.bindSession(props.tab.id, sessionId);
    doFit();
  } catch (e) {
    term.writeln(`\x1b[31m[ failed to start session: ${e} ]\x1b[0m`);
  }

  term.onData((data) => {
    if (sessionId) writeSession(sessionId, data);
  });

  resizeObserver = new ResizeObserver(() => doFit());
  resizeObserver.observe(host.value);

  // Also re-fit on window resize (maximize/restore/drag-edge) so the terminal
  // always tracks the window height.
  window.addEventListener("resize", onWindowResize);

  // Re-fit when this tab becomes the active one again (v-show → visible).
  watch(
    () => tabs.activeTabId === props.tab.id,
    (visible) => {
      if (visible) nextTick(() => requestAnimationFrame(() => doFit()));
    },
  );
});

function onWindowResize() {
  requestAnimationFrame(() => doFit());
}

onBeforeUnmount(() => {
  resizeObserver?.disconnect();
  window.removeEventListener("resize", onWindowResize);
  if (sessionId) closeSession(sessionId);
  term?.dispose();
});
</script>

<template>
  <div class="term-host">
    <div ref="host" class="term-mount" />
  </div>
</template>

<style scoped>
.term-host {
  width: 100%;
  height: 100%;
  padding: var(--space-2);
  background: var(--bg-app);
  overflow: hidden;
  box-sizing: border-box;
}
/* The xterm mount point carries NO padding, so FitAddon's clientHeight
   measurement matches the actual space and never over-counts rows. */
.term-mount {
  width: 100%;
  height: 100%;
  overflow: hidden;
}
:deep(.xterm) {
  width: 100%;
  height: 100%;
}
:deep(.xterm .xterm-viewport) {
  overflow-y: auto;
  /* Match the One Dark theme background instead of xterm's default black. */
  background-color: var(--bg-app) !important;
}
</style>
