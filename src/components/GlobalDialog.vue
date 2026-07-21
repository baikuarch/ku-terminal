<script setup lang="ts">
import { nextTick, onBeforeUnmount, onMounted, ref, watch } from "vue";
import { dialogs, dialogState } from "../lib/dialogs";

const input = ref<HTMLInputElement | null>(null);

function focusInput() {
  if (dialogState.kind === "prompt") {
    nextTick(() => input.value?.focus());
  }
}

function onKeydown(event: KeyboardEvent) {
  if (!dialogState.open || event.key !== "Escape") return;
  event.preventDefault();
  dialogs.cancel();
}

watch(() => dialogState.open, (open) => {
  if (open) focusInput();
});

onMounted(() => window.addEventListener("keydown", onKeydown));
onBeforeUnmount(() => window.removeEventListener("keydown", onKeydown));
</script>

<template>
  <Teleport to="body">
    <Transition name="dialog-fade">
      <div v-if="dialogState.open" class="dialog-overlay">
        <section class="dialog" role="dialog" aria-modal="true" :aria-labelledby="'dialog-title'">
          <header class="dialog-header">
            <div
              class="dialog-icon"
              :class="dialogState.tone === 'danger' ? 'danger' : 'default'"
            >
              <div
                :class="dialogState.tone === 'danger' ? 'i-lucide-triangle-alert' : 'i-lucide-message-square'"
                class="text-17px"
              />
            </div>
            <h2 id="dialog-title">{{ dialogState.title }}</h2>
            <button class="close" title="Close dialog" @click="dialogs.cancel()">
              <div class="i-lucide-x text-16px" />
            </button>
          </header>

          <div class="dialog-body">
            <p>{{ dialogState.message }}</p>
            <input
              v-if="dialogState.kind === 'prompt'"
              ref="input"
              v-model="dialogState.value"
              :type="dialogState.secret ? 'password' : 'text'"
              :placeholder="dialogState.placeholder"
              @keydown.enter.prevent="dialogs.accept()"
            />
          </div>

          <footer class="dialog-actions">
            <button v-if="dialogState.kind !== 'alert'" class="btn ghost" @click="dialogs.cancel()">
              Cancel
            </button>
            <button
              class="btn confirm"
              :class="{ danger: dialogState.tone === 'danger' }"
              @click="dialogs.accept()"
            >
              {{ dialogState.confirmLabel }}
            </button>
          </footer>
        </section>
      </div>
    </Transition>
  </Teleport>
</template>

<style scoped>
.dialog-overlay {
  position: fixed;
  inset: 0;
  z-index: 300;
  display: grid;
  place-items: center;
  padding: 24px;
  background: rgba(10, 12, 16, 0.62);
  backdrop-filter: blur(3px);
}
.dialog {
  width: min(420px, 100%);
  overflow: hidden;
  background: var(--bg-surface);
  border: 1px solid var(--border-default);
  border-radius: 8px;
  box-shadow: var(--shadow-lg);
}
.dialog-header {
  display: flex;
  align-items: center;
  gap: 10px;
  min-height: 58px;
  padding: 12px 14px;
  border-bottom: 1px solid var(--border-subtle);
}
.dialog-icon {
  display: grid;
  flex: 0 0 30px;
  width: 30px;
  height: 30px;
  place-items: center;
  border-radius: 5px;
}
.dialog-icon.default {
  color: var(--color-primary);
  background: rgba(97, 175, 239, 0.13);
}
.dialog-icon.danger {
  color: var(--color-tertiary);
  background: rgba(224, 108, 117, 0.13);
}
h2 {
  flex: 1;
  min-width: 0;
  color: var(--text-primary);
  font-size: var(--fs-lg);
  font-weight: 600;
}
.close {
  display: grid;
  width: 28px;
  height: 28px;
  place-items: center;
  border-radius: 4px;
  color: var(--text-muted);
}
.close:hover {
  color: var(--text-primary);
  background: var(--bg-hover);
}
.dialog-body {
  display: grid;
  gap: 14px;
  padding: 18px 16px 20px;
}
.dialog-body p {
  color: var(--text-secondary);
  font-size: var(--fs-md);
  line-height: 1.55;
  white-space: pre-wrap;
}
.dialog-body input {
  width: 100%;
  height: 34px;
  padding: 0 10px;
  color: var(--text-primary);
  font-size: var(--fs-md);
  background: var(--bg-input);
  border: 1px solid var(--border-subtle);
  border-radius: 5px;
}
.dialog-body input:focus {
  border-color: var(--color-primary);
  box-shadow: 0 0 0 2px rgba(97, 175, 239, 0.12);
}
.dialog-actions {
  display: flex;
  justify-content: flex-end;
  gap: 8px;
  padding: 12px 16px;
  border-top: 1px solid var(--border-subtle);
}
.btn {
  min-width: 74px;
  height: 32px;
  padding: 0 12px;
  border-radius: 5px;
  font-size: var(--fs-sm);
  font-weight: 600;
}
.btn.ghost {
  color: var(--text-secondary);
  border: 1px solid var(--border-default);
}
.btn.ghost:hover { background: var(--bg-hover); color: var(--text-primary); }
.btn.confirm { color: var(--text-inverted); background: var(--color-primary); }
.btn.confirm:hover { background: var(--color-primary-hover); }
.btn.confirm.danger { color: #fff; background: var(--color-tertiary); }
.btn.confirm.danger:hover { background: #e98289; }
.dialog-fade-enter-active,
.dialog-fade-leave-active { transition: opacity 0.16s ease; }
.dialog-fade-enter-active .dialog,
.dialog-fade-leave-active .dialog { transition: transform 0.16s ease, opacity 0.16s ease; }
.dialog-fade-enter-from,
.dialog-fade-leave-to { opacity: 0; }
.dialog-fade-enter-from .dialog,
.dialog-fade-leave-to .dialog { opacity: 0; transform: translateY(5px) scale(0.985); }
</style>
