<script setup lang="ts">
import { computed, reactive } from "vue";
import type { SessionDef } from "../types";
import { useSessionsStore } from "../stores/sessions";

const props = defineProps<{ group?: string; session?: SessionDef }>();
const emit = defineEmits<{
  (e: "close"): void;
  (e: "save", def: SessionDef, password?: string): void;
}>();

const sessions = useSessionsStore();
const groups = sessions.allGroupNames;
const isEditing = computed(() => Boolean(props.session));
type RequiredField = "name" | "host" | "user" | "keyPath";
const fieldErrors = reactive<Record<RequiredField, string>>({
  name: "",
  host: "",
  user: "",
  keyPath: "",
});

const form = reactive({
  name: props.session?.name ?? "",
  group: props.session?.group ?? props.group ?? groups[0] ?? "Production",
  cwd: props.session?.cwd ?? "",
  host: props.session?.ssh?.host ?? "",
  port: props.session?.ssh?.port ?? 22,
  user: props.session?.ssh?.user ?? "",
  authMethod:
    props.session?.ssh?.authMethod ?? (props.session?.ssh?.keyPath ? "key" : "password"),
  keyPath: props.session?.ssh?.keyPath ?? "",
  password: "",
});

function submit() {
  fieldErrors.name = form.name.trim() ? "" : "Enter a session name.";
  fieldErrors.host = form.host.trim() ? "" : "Enter a host or IP address.";
  fieldErrors.user = form.user.trim() ? "" : "Enter the login user.";
  fieldErrors.keyPath =
    form.authMethod === "key" && !form.keyPath.trim()
      ? "Enter the private key path."
      : "";
  if (Object.values(fieldErrors).some(Boolean)) return;

  const existing = props.session;
  const id = existing?.id ?? `${form.name.trim()}-${Date.now().toString(36)}`;
  const def: SessionDef = {
    id,
    name: form.name.trim(),
    kind: existing?.kind ?? "ssh",
    group: form.group,
    status: existing?.status ?? "idle",
    cwd: form.cwd.trim() || undefined,
    ssh: {
      host: form.host.trim(),
      port: Number(form.port) || 22,
      user: form.user.trim(),
      authMethod: form.authMethod,
      keyPath: form.authMethod === "key" ? form.keyPath.trim() : undefined,
      credentialId: existing?.ssh?.credentialId ?? id,
    },
  };
  emit("save", def, form.authMethod === "password" ? form.password : undefined);
}

function clearFieldError(field: RequiredField) {
  fieldErrors[field] = "";
}

function stepPort(delta: number) {
  const current = Number(form.port) || 22;
  form.port = Math.min(65535, Math.max(1, current + delta));
}
</script>

<template>
  <div class="overlay">
    <div class="dialog">
      <div class="dialog-head">
        <span class="dialog-title">{{ isEditing ? "Edit Session" : "New SSH Session" }}</span>
        <button class="close" @click="emit('close')">×</button>
      </div>

      <div class="fields">
        <label :class="{ invalid: fieldErrors.name }">
          <span>Name</span>
          <input
            v-model="form.name"
            type="text"
            placeholder="my-server"
            :aria-invalid="Boolean(fieldErrors.name)"
            @input="clearFieldError('name')"
          />
          <span v-if="fieldErrors.name" class="field-error">{{ fieldErrors.name }}</span>
        </label>
        <label>
          <span>Group</span>
          <div class="select-field">
            <select v-model="form.group">
              <option v-for="g in groups" :key="g" :value="g">{{ g }}</option>
            </select>
            <div class="i-lucide-chevron-down select-chevron" />
          </div>
        </label>
        <div class="row">
          <label class="grow" :class="{ invalid: fieldErrors.host }">
            <span>Host</span>
            <input
              v-model="form.host"
              type="text"
              placeholder="10.0.0.1"
              :aria-invalid="Boolean(fieldErrors.host)"
              @input="clearFieldError('host')"
            />
            <span v-if="fieldErrors.host" class="field-error">{{ fieldErrors.host }}</span>
          </label>
          <label class="port">
            <span>Port</span>
            <div class="number-field">
              <input v-model.number="form.port" type="number" min="1" max="65535" />
              <div class="number-steppers">
                <button type="button" title="Increase port" @click="stepPort(1)">
                  <div class="i-lucide-chevron-up text-11px" />
                </button>
                <button type="button" title="Decrease port" @click="stepPort(-1)">
                  <div class="i-lucide-chevron-down text-11px" />
                </button>
              </div>
            </div>
          </label>
        </div>
        <label :class="{ invalid: fieldErrors.user }">
          <span>User</span>
          <input
            v-model="form.user"
            type="text"
            placeholder="root"
            :aria-invalid="Boolean(fieldErrors.user)"
            @input="clearFieldError('user')"
          />
          <span v-if="fieldErrors.user" class="field-error">{{ fieldErrors.user }}</span>
        </label>
        <label>
          <span>Startup Directory <em>(optional)</em></span>
          <input
            v-model="form.cwd"
            type="text"
            placeholder="/home/user or C:\Users\me"
          />
        </label>
        <div class="auth-field">
          <span>Authentication</span>
          <div class="auth-modes">
            <button
              type="button"
              :class="{ active: form.authMethod === 'password' }"
              @click="form.authMethod = 'password'; clearFieldError('keyPath')"
            >
              Password
            </button>
            <button
              type="button"
              :class="{ active: form.authMethod === 'key' }"
              @click="form.authMethod = 'key'"
            >
              Private Key
            </button>
          </div>
        </div>
        <label v-if="form.authMethod === 'password'">
          <span>Password <em>(saved in the system credential store)</em></span>
          <input
            v-model="form.password"
            type="password"
            :placeholder="isEditing ? 'Leave blank to keep existing' : 'Password'"
          />
        </label>
        <label v-else :class="{ invalid: fieldErrors.keyPath }">
          <span>Private key path</span>
          <input
            v-model="form.keyPath"
            type="text"
            placeholder="C:\\Users\\me\\.ssh\\id_ed25519"
            :aria-invalid="Boolean(fieldErrors.keyPath)"
            @input="clearFieldError('keyPath')"
          />
          <span v-if="fieldErrors.keyPath" class="field-error">{{ fieldErrors.keyPath }}</span>
        </label>
      </div>

      <div class="actions">
        <button class="btn ghost" @click="emit('close')">Cancel</button>
        <button class="btn primary" @click="submit">
          {{ isEditing ? "Save Changes" : "Add Session" }}
        </button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.overlay {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.5);
  display: grid;
  place-items: center;
  z-index: 100;
}
.dialog {
  width: 400px;
  background: var(--bg-surface);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-lg);
  box-shadow: var(--shadow-lg);
  overflow: hidden;
}
.dialog-head {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: var(--space-3) var(--space-4);
  border-bottom: 1px solid var(--border-subtle);
}
.dialog-title {
  font-size: var(--fs-lg);
  font-weight: 600;
}
.close {
  color: var(--text-muted);
  font-size: 18px;
  width: 24px;
  height: 24px;
  border-radius: var(--radius-sm);
}
.close:hover {
  background: var(--bg-hover);
  color: var(--color-tertiary);
}
.fields {
  padding: var(--space-4);
  display: flex;
  flex-direction: column;
  gap: var(--space-3);
}
label {
  display: flex;
  flex-direction: column;
  gap: var(--space-1);
  font-size: var(--fs-sm);
  color: var(--text-secondary);
}
label em {
  color: var(--text-muted);
  font-style: normal;
  font-size: var(--fs-xs);
}
input,
select {
  height: 32px;
  padding: 0 var(--space-2);
  background: var(--bg-input);
  border: 1px solid var(--border-subtle);
  border-radius: var(--radius-md);
  color: var(--text-primary);
  font-size: var(--fs-sm);
}
.select-field,
.number-field {
  position: relative;
  height: 32px;
  border: 1px solid var(--border-subtle);
  border-radius: var(--radius-md);
  background: var(--bg-input);
  transition: border-color 0.16s ease, box-shadow 0.16s ease, background 0.16s ease;
}
.select-field:hover,
.number-field:hover {
  border-color: var(--border-default);
  background: #252a31;
}
.select-field:focus-within,
.number-field:focus-within {
  border-color: var(--color-primary);
  box-shadow: 0 0 0 2px rgba(97, 175, 239, 0.12);
}
.select-field select {
  width: 100%;
  height: 100%;
  padding: 0 30px 0 var(--space-2);
  border: 0;
  border-radius: inherit;
  appearance: none;
  background: transparent;
  cursor: pointer;
}
.select-field select option {
  background: var(--bg-surface);
  color: var(--text-primary);
}
.select-chevron {
  position: absolute;
  top: 50%;
  right: 9px;
  transform: translateY(-50%);
  color: var(--text-muted);
  font-size: 14px;
  pointer-events: none;
}
.number-field input {
  width: 100%;
  height: 100%;
  padding: 0 26px 0 var(--space-2);
  border: 0;
  border-radius: inherit;
  background: transparent;
  appearance: textfield;
}
.number-field input::-webkit-inner-spin-button,
.number-field input::-webkit-outer-spin-button {
  margin: 0;
  appearance: none;
}
.number-steppers {
  position: absolute;
  top: 3px;
  right: 3px;
  bottom: 3px;
  display: grid;
  grid-template-rows: 1fr 1fr;
  width: 17px;
  overflow: hidden;
  border-left: 1px solid var(--border-subtle);
}
.number-steppers button {
  display: grid;
  place-items: center;
  color: var(--text-muted);
  line-height: 1;
}
.number-steppers button:hover {
  background: var(--bg-hover);
  color: var(--color-primary);
}
.number-steppers button:first-child {
  border-bottom: 1px solid var(--border-subtle);
}
input:focus,
select:focus {
  border-color: var(--color-primary);
}
label.invalid > span:first-child {
  color: var(--color-tertiary);
}
input[aria-invalid="true"] {
  border-color: var(--color-tertiary);
  box-shadow: 0 0 0 2px rgba(224, 108, 117, 0.12);
}
.field-error {
  color: var(--color-tertiary);
  font-size: var(--fs-xs);
  line-height: 1.2;
}
.auth-field {
  display: flex;
  flex-direction: column;
  gap: var(--space-1);
  color: var(--text-secondary);
  font-size: var(--fs-sm);
}
.auth-modes {
  display: grid;
  grid-template-columns: 1fr 1fr;
  height: 32px;
  padding: 2px;
  background: var(--bg-input);
  border: 1px solid var(--border-subtle);
  border-radius: var(--radius-md);
}
.auth-modes button {
  border-radius: 4px;
  color: var(--text-muted);
  font-size: var(--fs-sm);
}
.auth-modes button:hover { color: var(--text-secondary); }
.auth-modes button.active {
  color: var(--text-primary);
  background: var(--bg-surface-2);
  box-shadow: var(--shadow-sm);
}
.row {
  display: flex;
  gap: var(--space-2);
}
.grow {
  flex: 1;
}
.port {
  width: 88px;
}
.actions {
  display: flex;
  justify-content: flex-end;
  gap: var(--space-2);
  padding: var(--space-3) var(--space-4);
  border-top: 1px solid var(--border-subtle);
}
.btn {
  height: 32px;
  padding: 0 var(--space-4);
  border-radius: var(--radius-md);
  font-size: var(--fs-sm);
  font-weight: 500;
}
.btn.ghost {
  color: var(--text-secondary);
  border: 1px solid var(--border-default);
}
.btn.ghost:hover {
  background: var(--bg-hover);
  color: var(--text-primary);
}
.btn.primary {
  background: var(--color-primary);
  color: var(--text-inverted);
}
.btn.primary:hover {
  background: var(--color-primary-hover);
}
</style>
