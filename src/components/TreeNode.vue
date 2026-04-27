<script setup lang="ts">
import { ref, computed } from "vue";
import { useTreeStore } from "../stores/tree";
import StatusBadge from "./StatusBadge.vue";
import NodeModal from "./NodeModal.vue";
import RunToolForm from "./RunToolForm.vue";
import type { NodeRow } from "../lib/types";

const props = defineProps<{
  nodeId: string;
  sessionId: string;
}>();

const treeStore = useTreeStore();

const node = computed<NodeRow | undefined>(() => treeStore.nodes[props.nodeId]);
const children = computed<NodeRow[]>(() => treeStore.childrenOf(props.nodeId));

const showModal = ref(false);
const showSpawnForm = ref(false);

function duration(n: NodeRow): string {
  const start = n.started_at;
  const end = n.finished_at;
  if (!start) return "";
  const diff = ((end ?? Date.now()) - start) / 1000;
  return diff.toFixed(1) + "s";
}

function parsedArgs(n: NodeRow): string {
  try {
    return (JSON.parse(n.args) as string[]).join(" ");
  } catch {
    return n.args;
  }
}
</script>

<template>
  <div class="subtree" v-if="node">
    <div class="row">
      <!-- Node Card -->
      <div
        class="node-card"
        :class="node.status"
        @click="showModal = true"
      >
        <div class="card-top">
          <span class="tool">{{ node.tool }}</span>
          <StatusBadge :status="node.status" />
        </div>
        <div class="args" v-if="parsedArgs(node)">{{ parsedArgs(node) }}</div>
        <div class="summary" v-if="node.summary">{{ node.summary }}</div>
        <div class="card-bottom">
          <span class="duration" v-if="duration(node)">{{ duration(node) }}</span>
          <button class="add-child-btn" @click.stop="showSpawnForm = true" title="Spawn child tool">+</button>
        </div>
      </div>

      <!-- Connector + Children column -->
      <div v-if="children.length" class="children-wrap">
        <svg class="connector" aria-hidden="true">
          <line
            v-for="child in children"
            :key="child.id"
            class="line"
          />
        </svg>
        <div class="children-col">
          <TreeNode
            v-for="child in children"
            :key="child.id"
            :node-id="child.id"
            :session-id="sessionId"
          />
        </div>
      </div>
    </div>

    <!-- Modal -->
    <NodeModal
      v-if="showModal"
      :node-id="nodeId"
      :session-id="sessionId"
      @close="showModal = false"
    />

    <!-- Spawn child form -->
    <RunToolForm
      v-if="showSpawnForm"
      :session-id="sessionId"
      :parent-node="node"
      @close="showSpawnForm = false"
    />
  </div>
</template>

<style scoped>
.subtree {
  display: flex;
  flex-direction: column;
}

.row {
  display: flex;
  flex-direction: row;
  align-items: flex-start;
  gap: 0;
}

/* Node card */
.node-card {
  width: 220px;
  min-width: 220px;
  background: #252525;
  border: 1px solid #333;
  border-radius: 8px;
  padding: 10px 12px;
  cursor: pointer;
  transition: border-color 0.15s, background 0.15s;
  position: relative;
  display: flex;
  flex-direction: column;
  gap: 5px;
  user-select: none;
  flex-shrink: 0;
}

.node-card:hover {
  background: #2d2d2d;
  border-color: #4b5563;
}

.node-card.running { border-color: #3b82f6; }
.node-card.success { border-color: #22c55e44; }
.node-card.error   { border-color: #ef444444; }
.node-card.killed  { border-color: #f59e0b44; }

.card-top {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 6px;
}

.tool {
  font-family: 'Consolas', 'JetBrains Mono', monospace;
  font-size: 13px;
  font-weight: 700;
  color: #e2e8f0;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.args {
  font-family: 'Consolas', monospace;
  font-size: 11px;
  color: #6b7280;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.summary {
  font-size: 11px;
  color: #9ca3af;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.card-bottom {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-top: 2px;
}

.duration {
  font-size: 10px;
  color: #4b5563;
  font-family: monospace;
}

.add-child-btn {
  background: #374151;
  border: 1px solid #4b5563;
  color: #9ca3af;
  border-radius: 4px;
  width: 20px;
  height: 20px;
  font-size: 14px;
  line-height: 1;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 0;
  opacity: 0;
  transition: opacity 0.1s;
}

.node-card:hover .add-child-btn {
  opacity: 1;
}

.add-child-btn:hover {
  background: #3b82f6;
  border-color: #3b82f6;
  color: #fff;
}

/* Connector lines */
.children-wrap {
  display: flex;
  flex-direction: row;
  align-items: stretch;
  position: relative;
}

.connector {
  width: 28px;
  height: 100%;
  position: absolute;
  left: 0;
  top: 0;
  overflow: visible;
  pointer-events: none;
}

.children-col {
  margin-left: 28px;
  display: flex;
  flex-direction: column;
  gap: 12px;
}
</style>
