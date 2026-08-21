export interface EnvVar {
  name: string;
  value: string;
  secret: boolean;
  description?: string;
}

export interface BotSpec {
  id: string;
  name: string;
  exe: string;
  args: string[];
  cwd?: string;
  env: EnvVar[];
  autostart: boolean;
}

export type BotState =
  | { kind: "stopped" }
  | { kind: "starting" }
  | { kind: "running"; pid: number; since_ms: number }
  | { kind: "backoff"; until_ms: number; attempt: number }
  | { kind: "stopping" };

export interface StateEvent {
  id: string;
  state: BotState;
  restarts: number;
}

export type LogStream = "stdout" | "stderr" | "system";

export interface LogLine {
  ts: string;
  stream: LogStream;
  line: string;
}

export interface LogEvent extends LogLine {
  id: string;
}

export function emptySpec(): BotSpec {
  return { id: "", name: "", exe: "", args: [], cwd: undefined, env: [], autostart: true };
}
