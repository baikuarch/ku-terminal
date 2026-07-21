import { invoke, Channel } from "@tauri-apps/api/core";
import type { AppConfig, ResourceStats, FileEntry, SshConfig } from "../types";

/** Backend streams raw PTY bytes as number[] chunks over a Channel. */
export type OutputEvent = { bytes: number[] };

export async function createLocalSession(
  cwd: string | null,
  shell: string | null,
  onOutput: Channel<OutputEvent>,
): Promise<string> {
  return invoke<string>("create_local_session", { cwd, shell, onOutput });
}

export async function createSshSession(
  cfg: SshConfig,
  password: string | null,
  onOutput: Channel<OutputEvent>,
): Promise<string> {
  return invoke<string>("create_ssh_session", { cfg, password, onOutput });
}

export async function writeSession(id: string, data: string): Promise<void> {
  return invoke("write_session", { id, data });
}

export async function resizeSession(
  id: string,
  cols: number,
  rows: number,
): Promise<void> {
  return invoke("resize_session", { id, cols, rows });
}

export async function closeSession(id: string): Promise<void> {
  return invoke("close_session", { id });
}

export async function getResourceStats(
  sessionId?: string | null,
): Promise<ResourceStats> {
  return invoke<ResourceStats>("get_resource_stats", {
    sessionId: sessionId ?? null,
  });
}

export async function readDir(
  path: string,
  sessionId?: string | null,
): Promise<FileEntry[]> {
  return invoke<FileEntry[]>("read_dir", { path, sessionId: sessionId ?? null });
}

export async function homeDir(sessionId?: string | null): Promise<string> {
  return invoke<string>("home_dir", { sessionId: sessionId ?? null });
}

export async function loadConfig(): Promise<AppConfig> {
  return invoke<AppConfig>("load_config");
}

export async function saveConfig(config: AppConfig): Promise<void> {
  return invoke("save_config", { config });
}

export async function saveSshPassword(
  credentialId: string,
  password: string,
): Promise<void> {
  return invoke("save_ssh_password", { credentialId, password });
}

export async function loadSshPassword(
  credentialId: string,
): Promise<string | null> {
  return invoke<string | null>("load_ssh_password", { credentialId });
}

export async function deleteSshPassword(credentialId: string): Promise<void> {
  return invoke("delete_ssh_password", { credentialId });
}
