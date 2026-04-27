import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import type {
  SessionRow,
  NodeRow,
  OutputRow,
  NodeStatusEvent,
  NodeOutputEvent,
} from "./types";

// ── Sessions ─────────────────────────────────────────────────────────────────

export function createSession(name: string): Promise<SessionRow> {
  return invoke<SessionRow>("create_session", { name });
}

export function listSessions(): Promise<SessionRow[]> {
  return invoke<SessionRow[]>("list_sessions");
}

export function deleteSession(id: string): Promise<void> {
  return invoke<void>("delete_session", { id });
}

// ── Nodes ─────────────────────────────────────────────────────────────────────

export function getSessionTree(sessionId: string): Promise<NodeRow[]> {
  return invoke<NodeRow[]>("get_session_tree", { session_id: sessionId });
}

export function getNode(id: string): Promise<NodeRow> {
  return invoke<NodeRow>("get_node", { id });
}

export function getNodeOutput(nodeId: string): Promise<OutputRow[]> {
  return invoke<OutputRow[]>("get_node_output", { node_id: nodeId });
}

export function killNode(nodeId: string): Promise<boolean> {
  return invoke<boolean>("kill_node", { node_id: nodeId });
}

// ── Runner ────────────────────────────────────────────────────────────────────

export interface RunToolParams {
  session_id: string;
  parent_id?: string | null;
  tool: string;
  args: string[];
  working_dir?: string | null;
}

export function runTool(params: RunToolParams): Promise<NodeRow> {
  return invoke<NodeRow>("run_tool", params as unknown as Record<string, unknown>);
}

// ── Tools ─────────────────────────────────────────────────────────────────────

export function listAvailableTools(): Promise<string[]> {
  return invoke<string[]>("list_available_tools");
}

// ── Events ────────────────────────────────────────────────────────────────────

export function onNodeOutput(
  cb: (event: NodeOutputEvent) => void
): Promise<UnlistenFn> {
  return listen<NodeOutputEvent>("node_output", (e) => cb(e.payload));
}

export function onNodeStatus(
  cb: (event: NodeStatusEvent) => void
): Promise<UnlistenFn> {
  return listen<NodeStatusEvent>("node_status", (e) => cb(e.payload));
}
