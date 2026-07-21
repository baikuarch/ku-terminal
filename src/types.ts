export type SessionKind = "local" | "ssh";
export type SessionStatus = "online" | "warn" | "error" | "idle";

export interface SshConfig {
  host: string;
  port: number;
  user: string;
  authMethod?: "password" | "key";
  /** absolute path to private key, if key auth */
  keyPath?: string;
  /** Identifier used by the native system credential store. */
  credentialId?: string;
}

export interface SessionDef {
  id: string;
  name: string;
  kind: SessionKind;
  group: string; // Production / Staging / Development
  status: SessionStatus;
  cwd?: string;
  shell?: string;
  ssh?: SshConfig;
}

/** Complete, portable user configuration persisted in sessions.json. */
export interface AppConfig {
  version: number;
  sessions: SessionDef[];
  customGroups: string[];
  collapsedGroups: string[];
  hiddenGroups: string[];
  sidebarCollapsed: boolean;
}

export interface ResourceStats {
  cpu: number; // percent 0-100
  memUsed: number; // bytes
  memTotal: number; // bytes
  diskUsed: number; // bytes
  diskTotal: number; // bytes
  netRx: number; // bytes/s
  netTx: number; // bytes/s
}

export interface FileEntry {
  name: string;
  path: string;
  isDir: boolean;
  size: number;
}
