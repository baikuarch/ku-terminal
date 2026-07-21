<script setup lang="ts">
import { computed, onMounted, onUnmounted, ref } from "vue";
import { getResourceStats } from "../lib/ipc";
import type { ResourceStats } from "../types";
import { useTabsStore } from "../stores/tabs";

const tabs = useTabsStore();
const stats = ref<ResourceStats | null>(null);
const remote = ref(false);
const connected = ref(false);
let timer: number | undefined;

function fmtBytes(n: number): string {
  if (n >= 1 << 30) return (n / (1 << 30)).toFixed(1) + " GB";
  if (n >= 1 << 20) return (n / (1 << 20)).toFixed(0) + " MB";
  if (n >= 1 << 10) return (n / (1 << 10)).toFixed(0) + " KB";
  return n + " B";
}

async function tick() {
  const active = tabs.activeTab;
  const useRemote = active?.kind === "ssh" && !!active.sessionId;
  remote.value = useRemote;
  try {
    stats.value = await getResourceStats(useRemote ? active!.sessionId : null);
    connected.value = true;
  } catch {
    connected.value = false;
  }
}

onMounted(() => {
  tick();
  timer = window.setInterval(tick, 1500);
});
onUnmounted(() => {
  if (timer) clearInterval(timer);
});

function pct(used: number, total: number): number {
  return total > 0 ? Math.round((used / total) * 100) : 0;
}

/** Color level from usage percentage. */
function level(p: number): "ok" | "warn" | "crit" {
  if (p >= 85) return "crit";
  if (p >= 60) return "warn";
  return "ok";
}

const cpu = computed(() => Math.round(stats.value?.cpu ?? 0));
const mem = computed(() =>
  stats.value ? pct(stats.value.memUsed, stats.value.memTotal) : 0,
);
const disk = computed(() =>
  stats.value ? pct(stats.value.diskUsed, stats.value.diskTotal) : 0,
);
const net = computed(() =>
  stats.value ? fmtBytes(stats.value.netRx + stats.value.netTx) + "/s" : "–",
);
</script>

<template>
  <div class="monitor">
    <div class="monitor-head">
      <div class="head-left">
        <div class="i-lucide-activity text-13px text-primary" />
        <span class="monitor-title">{{ remote ? "远端会话" : "本地会话" }}</span>
      </div>
      <div class="conn" :class="connected ? 'up' : 'down'">
        <span class="conn-dot" />
        {{ connected ? (remote ? "SSH" : "在线") : "离线" }}
      </div>
    </div>

    <div class="metric" :class="`lv-${level(cpu)}`">
      <div class="i-lucide-cpu metric-icon text-14px" />
      <div class="metric-body">
        <div class="metric-head">
          <span class="metric-label">CPU</span>
          <span class="metric-val">{{ stats ? cpu : "–" }}%</span>
        </div>
        <div class="bar">
          <div class="bar-fill" :style="{ width: cpu + '%' }" />
        </div>
      </div>
    </div>

    <div class="metric" :class="`lv-${level(mem)}`">
      <div class="i-lucide-memory-stick metric-icon text-14px" />
      <div class="metric-body">
        <div class="metric-head">
          <span class="metric-label">内存</span>
          <span class="metric-val">
            {{ stats ? mem : "–" }}%
            <span v-if="stats" class="metric-sub">
              {{ fmtBytes(stats.memUsed) }} / {{ fmtBytes(stats.memTotal) }}
            </span>
          </span>
        </div>
        <div class="bar">
          <div class="bar-fill" :style="{ width: mem + '%' }" />
        </div>
      </div>
    </div>

    <div class="mini-row">
      <div class="mini">
        <div class="i-lucide-hard-drive text-13px text-txt-muted" />
        <div class="mini-body">
          <span class="mini-label">硬盘</span>
          <span class="mini-val">{{ stats ? disk + "%" : "–" }}</span>
        </div>
      </div>
      <div class="mini">
        <div class="i-lucide-wifi text-13px text-txt-muted" />
        <div class="mini-body">
          <span class="mini-label">网络</span>
          <span class="mini-val">{{ net }}</span>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.monitor {
  padding: var(--space-3);
  border-top: 1px solid var(--border-subtle);
  background: var(--bg-surface);
  display: flex;
  flex-direction: column;
  gap: var(--space-3);
}
.monitor-head {
  display: flex;
  align-items: center;
  justify-content: space-between;
}
.head-left {
  display: flex;
  align-items: center;
  gap: var(--space-2);
}
.monitor-title {
  font-size: var(--fs-xs);
  color: var(--text-secondary);
  text-transform: uppercase;
  letter-spacing: 0.6px;
  font-weight: 600;
}
.conn {
  display: flex;
  align-items: center;
  gap: 5px;
  font-size: var(--fs-xs);
  padding: 2px 7px;
  border-radius: 10px;
}
.conn-dot {
  width: 6px;
  height: 6px;
  border-radius: 50%;
}
.conn.up {
  color: var(--status-online);
  background: rgba(152, 195, 121, 0.12);
}
.conn.up .conn-dot {
  background: var(--status-online);
  box-shadow: 0 0 6px rgba(152, 195, 121, 0.6);
}
.conn.down {
  color: var(--text-muted);
  background: var(--bg-input);
}
.conn.down .conn-dot {
  background: var(--status-idle);
}

/* ── Primary metrics (CPU / memory) ── */
.metric {
  display: flex;
  align-items: center;
  gap: var(--space-2);
}
.metric-icon {
  color: var(--text-muted);
  flex-shrink: 0;
}
.metric-body {
  flex: 1;
  min-width: 0;
}
.metric-head {
  display: flex;
  justify-content: space-between;
  align-items: baseline;
  font-size: var(--fs-sm);
  color: var(--text-secondary);
  margin-bottom: 5px;
}
.metric-val {
  color: var(--text-primary);
  font-weight: 600;
  font-variant-numeric: tabular-nums;
}
.metric-sub {
  color: var(--text-muted);
  font-weight: 400;
  font-size: var(--fs-xs);
  margin-left: 4px;
}
.bar {
  height: 6px;
  background: var(--bg-input);
  border-radius: 3px;
  overflow: hidden;
}
.bar-fill {
  height: 100%;
  border-radius: 3px;
  transition: width 0.4s ease, background 0.3s ease;
}
/* Usage-based coloring */
.lv-ok .bar-fill {
  background: linear-gradient(90deg, var(--primary-500), var(--color-primary));
}
.lv-ok .metric-icon {
  color: var(--color-primary);
}
.lv-warn .bar-fill {
  background: linear-gradient(90deg, #d9a441, var(--color-warning));
}
.lv-warn .metric-icon {
  color: var(--color-warning);
}
.lv-crit .bar-fill {
  background: linear-gradient(90deg, #c9505a, var(--color-tertiary));
}
.lv-crit .metric-icon {
  color: var(--color-tertiary);
}

/* ── Mini stats (disk / network) ── */
.mini-row {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: var(--space-2);
}
.mini {
  background: var(--bg-input);
  border: 1px solid var(--border-subtle);
  border-radius: var(--radius-md);
  padding: var(--space-2);
  display: flex;
  align-items: center;
  gap: var(--space-2);
}
.mini-body {
  display: flex;
  flex-direction: column;
  gap: 1px;
  min-width: 0;
}
.mini-label {
  font-size: var(--fs-xs);
  color: var(--text-muted);
}
.mini-val {
  font-size: var(--fs-sm);
  color: var(--text-primary);
  font-weight: 600;
  font-variant-numeric: tabular-nums;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}
</style>
