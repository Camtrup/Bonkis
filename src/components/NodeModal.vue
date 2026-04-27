<script setup lang="ts">
import { ref, computed, onMounted, watch, nextTick } from "vue";
import { useTreeStore } from "../stores/tree";
import { killNode } from "../lib/tauri";
import StatusBadge from "./StatusBadge.vue";
import RunToolForm from "./RunToolForm.vue";
import type { NodeRow } from "../lib/types";

const props = defineProps<{
  nodeId: string;
  sessionId: string;
}>();

const emit = defineEmits<{
  (e: "close"): void;
}>();

const treeStore = useTreeStore();
const node = computed(() => treeStore.nodes[props.nodeId] as NodeRow | undefined);
const output = computed(() => treeStore.outputBuffers[props.nodeId] ?? []);

const showSpawnForm = ref(false);
const logEl = ref<HTMLElement | null>(null);

onMounted(async () => {
  await treeStore.loadOutput(props.nodeId);
  await nextTick();
  scrollToBottom();
});

watch(output, async () => {
  await nextTick();
  scrollToBottom();
});

function scrollToBottom() {
  if (logEl.value) {
    logEl.value.scrollTop = logEl.value.scrollHeight;
  }
}

async function kill() {
  if (!node.value) return;
  await killNode(node.value.id);
}

function formatTs(ms: number | null | undefined): string {
  if (!ms) return "—";
  return new Date(ms).toLocaleString();
}

function duration(node: NodeRow): string {
  const start = node.started_at;
  const end = node.finished_at;
  if (!start) return "—";
  const diff = ((end ?? Date.now()) - start) / 1000;
  return diff.toFixed(1) + "s";
}

function parsedArgs(node: NodeRow): string {
  try {
    return (JSON.parse(node.args) as string[]).join(" ");
  } catch {
    return node.args;
  }
}

function onOverlayKey(e: KeyboardEvent) {
  if (e.key === "Escape") emit("close");
}
</script>

<template>
  <div class="overlay" tabindex="-1" @keydown="onOverlayKey" @mousedown.self="$emit('close')">
    <div class="modal" v-if="node">
      <!-- Header -->
      <div class="modal-header">
        <div class="header-left">
          <span class="tool-name">{{ node.tool }}</span>
          <span class="args">{{ parsedArgs(node) }}</span>
          <StatusBadge :status="node.status" />
          <span v-if="node.status === 'running'" class="live-pill">● LIVE</span>
        </div>
        <button class="close-btn" @click="$emit('close')">×</button>
      </div>

      <!-- Metadata -->
      <div class="meta-grid">
        <span class="meta-label">Node ID</span>
        <span class="meta-val mono">{{ node.id.slice(0, 8) }}…</span>

        <span class="meta-label">Exit code</span>
        <span class="meta-val mono">{{ node.exit_code ?? "—" }}</span>

        <span class="meta-label">Working dir</span>
        <span class="meta-val mono">{{ node.working_dir ?? "—" }}</span>

        <span class="meta-label">Created</span>
        <span class="meta-val">{{ formatTs(node.created_at) }}</span>

        <span class="meta-label">Started</span>
        <span class="meta-val">{{ formatTs(node.started_at) }}</span>

        <span class="meta-label">Finished</span>
        <span class="meta-val">{{ formatTs(node.finished_at) }}</span>

        <span class="meta-label">Duration</span>
        <span class="meta-val">{{ duration(node) }}</span>
      </div>

      <!-- Output log -->
      <div class="log-pane" ref="logEl">
        <div v-if="output.length === 0" class="log-empty">No output yet.</div>
        <div
          v-for="row in output"
          :key="row.id"
          class="log-line"
          :class="row.stream"
        >{{ row.line }}</div>
      </div>

      <!-- Actions -->
      <div class="actions-bar">
        <button
          v-if="node.status === 'running'"
          class="btn-danger"
          @click="kill"
        >Kill</button>
        <button class="btn-secondary" @click="showSpawnForm = true">
          + Spawn child tool
        </button>
      </div>
    </div>

    <RunToolForm
      v-if="showSpawnForm && node"
      :session-id="sessionId"
      :parent-node="node"
      @close="showSpawnForm = false"
    />
  </div>
</template>

<style scoped>
.overlay {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.7);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 100;
}

.modal {
  background: #1a1a1a;
  border: 1px solid #2d2d2d;
  border-radius: 10px;
  width: 820px;
  max-width: 96vw;
  max-height: 88vh;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.modal-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 16px 20px;
  border-bottom: 1px solid #2d2d2d;
  gap: 12px;
  flex-shrink: 0;
}

.header-left {
  display: flex;
  align-items: center;
  gap: 10px;
  flex-wrap: wrap;
  min-width: 0;
}

.tool-name {
  font-family: 'Consolas', monospace;
  font-size: 15px;
  font-weight: 700;
  color: #e2e8f0;
}

.args {
  font-family: 'Consolas', monospace;
  font-size: 12px;
  color: #6b7280;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  max-width: 300px;
}

.live-pill {
  font-size: 10px;
  color: #3b82f6;
  font-weight: 700;
  letter-spacing: 0.05em;
  animation: blink 1.2s ease-in-out infinite;
}

@keyframes blink {
  0%, 100% { opacity: 1; }
  50% { opacity: 0.3; }
}

.close-btn {
  background: none;
  border: none;
  color: #6b7280;
  font-size: 24px;
  cursor: pointer;
  line-height: 1;
  padding: 0 4px;
  flex-shrink: 0;
}

.close-btn:hover { color: #e2e8f0; }

.meta-grid {
  display: grid;
  grid-template-columns: 110px 1fr;
  gap: 4px 12px;
  padding: 12px 20px;
  border-bottom: 1px solid #2d2d2d;
  font-size: 12px;
  flex-shrink: 0;
}

.meta-label {
  color: #6b7280;
  font-weight: 500;
  white-space: nowrap;
}

.meta-val {
  color: #cbd5e1;
  word-break: break-all;
}

.mono { font-family: 'Consolas', monospace; }

.log-pane {
  flex: 1;
  overflow-y: auto;
  background: #0d0d0d;
  padding: 12px 16px;
  font-family: 'Consolas', 'JetBrains Mono', monospace;
  font-size: 12px;
  line-height: 1.6;
  min-height: 200px;
}

.log-empty {
  color: #4b5563;
  font-style: italic;
}

.log-line {
  white-space: pre-wrap;
  word-break: break-all;
}

.log-line.stdout { color: #e2e8f0; }
.log-line.stderr { color: #fb923c; }

.actions-bar {
  display: flex;
  gap: 10px;
  padding: 12px 20px;
  border-top: 1px solid #2d2d2d;
  flex-shrink: 0;
}

.btn-danger,
.btn-secondary {
  padding: 7px 16px;
  border-radius: 6px;
  font-size: 12px;
  font-weight: 600;
  cursor: pointer;
  border: 1px solid transparent;
  transition: background 0.15s;
}

.btn-danger {
  background: #7f1d1d;
  color: #fca5a5;
  border-color: #ef4444;
}

.btn-danger:hover { background: #991b1b; }

.btn-secondary {
  background: #252525;
  color: #9ca3af;
  border-color: #374151;
}

.btn-secondary:hover { background: #374151; color: #e2e8f0; }
</style>
