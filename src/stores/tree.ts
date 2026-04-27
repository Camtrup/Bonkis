import { defineStore } from "pinia";
import { ref } from "vue";
import { getSessionTree, getNodeOutput } from "../lib/tauri";
import type { NodeRow, OutputRow, NodeOutputEvent, NodeStatusEvent } from "../lib/types";

let _outputIdCounter = 0;

export const useTreeStore = defineStore("tree", () => {
  const nodes = ref<Record<string, NodeRow>>({});
  const outputBuffers = ref<Record<string, OutputRow[]>>({});

  async function loadTree(sessionId: string) {
    const rows = await getSessionTree(sessionId);
    const map: Record<string, NodeRow> = {};
    for (const n of rows) {
      map[n.id] = n;
    }
    nodes.value = map;
  }

  function upsertNode(node: NodeRow) {
    nodes.value[node.id] = { ...(nodes.value[node.id] ?? {}), ...node };
  }

  function applyStatusEvent(event: NodeStatusEvent) {
    const existing = nodes.value[event.node_id];
    if (!existing) return;
    nodes.value[event.node_id] = {
      ...existing,
      status: event.status as NodeRow["status"],
      exit_code: event.exit_code ?? existing.exit_code,
      summary: event.summary ?? existing.summary,
      finished_at: event.finished_at ?? existing.finished_at,
    };
  }

  function appendOutput(event: NodeOutputEvent) {
    if (!outputBuffers.value[event.node_id]) {
      outputBuffers.value[event.node_id] = [];
    }
    const fakeRow: OutputRow = {
      id: --_outputIdCounter,
      node_id: event.node_id,
      stream: event.stream,
      line: event.line,
      created_at: Date.now(),
    };
    outputBuffers.value[event.node_id].push(fakeRow);
  }

  async function loadOutput(nodeId: string) {
    const rows = await getNodeOutput(nodeId);
    outputBuffers.value[nodeId] = rows;
  }

  function clearTree() {
    nodes.value = {};
    outputBuffers.value = {};
  }

  function rootNodes(sessionId: string): NodeRow[] {
    return Object.values(nodes.value).filter(
      (n) => n.session_id === sessionId && !n.parent_id
    );
  }

  function childrenOf(parentId: string): NodeRow[] {
    return Object.values(nodes.value).filter((n) => n.parent_id === parentId);
  }

  return {
    nodes,
    outputBuffers,
    loadTree,
    upsertNode,
    applyStatusEvent,
    appendOutput,
    loadOutput,
    clearTree,
    rootNodes,
    childrenOf,
  };
});
