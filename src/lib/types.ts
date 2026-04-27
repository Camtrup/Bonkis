export interface SessionRow {
  id: string;
  name: string;
  created_at: number;
}

export interface NodeRow {
  id: string;
  session_id: string;
  parent_id?: string | null;
  tool: string;
  args: string; // JSON-encoded string[]
  working_dir?: string | null;
  status: "queued" | "running" | "success" | "error" | "killed";
  exit_code?: number | null;
  summary?: string | null;
  created_at: number;
  started_at?: number | null;
  finished_at?: number | null;
}

export interface OutputRow {
  id: number;
  node_id: string;
  stream: "stdout" | "stderr";
  line: string;
  created_at: number;
}

export interface NodeStatusEvent {
  node_id: string;
  status: string;
  exit_code?: number | null;
  summary?: string | null;
  finished_at?: number | null;
}

export interface NodeOutputEvent {
  node_id: string;
  stream: "stdout" | "stderr";
  line: string;
}
