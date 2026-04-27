<script setup lang="ts">
import { ref, computed, watch } from "vue";
import { useSessionStore } from "../stores/session";
import { useTreeStore } from "../stores/tree";
import TreeNode from "./TreeNode.vue";
import RunToolForm from "./RunToolForm.vue";

const sessionStore = useSessionStore();
const treeStore = useTreeStore();

const showRunForm = ref(false);

const activeSession = computed(() =>
  sessionStore.sessions.find((s) => s.id === sessionStore.activeSessionId)
);

const roots = computed(() => {
  if (!sessionStore.activeSessionId) return [];
  return treeStore.rootNodes(sessionStore.activeSessionId);
});

// Reload tree when active session changes
watch(
  () => sessionStore.activeSessionId,
  async (id) => {
    treeStore.clearTree();
    if (id) {
      await treeStore.loadTree(id);
    }
  },
  { immediate: true }
);
</script>

<template>
  <div class="canvas-wrap">
    <div v-if="!sessionStore.activeSessionId" class="empty-state">
      <div class="empty-icon">🗂</div>
      <p>Select or create a session to get started.</p>
    </div>

    <template v-else>
      <div class="canvas-toolbar">
        <span class="session-name">{{ activeSession?.name }}</span>
        <button class="add-root-btn" @click="showRunForm = true">+ Add tool</button>
      </div>

      <div class="canvas-scroll">
        <div v-if="roots.length === 0" class="empty-state canvas-empty">
          <div class="empty-icon">🔍</div>
          <p>No tools yet. Click <strong>+ Add tool</strong> to run your first scan.</p>
        </div>

        <div v-else class="tree-layout">
          <TreeNode
            v-for="root in roots"
            :key="root.id"
            :node-id="root.id"
            :session-id="sessionStore.activeSessionId!"
          />
        </div>
      </div>
    </template>

    <RunToolForm
      v-if="showRunForm && sessionStore.activeSessionId"
      :session-id="sessionStore.activeSessionId"
      :parent-node="null"
      @close="showRunForm = false"
    />
  </div>
</template>

<style scoped>
.canvas-wrap {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  background: #141414;
}

.canvas-toolbar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 10px 20px;
  border-bottom: 1px solid #222;
  flex-shrink: 0;
  background: #1a1a1a;
}

.session-name {
  font-size: 13px;
  font-weight: 600;
  color: #e2e8f0;
}

.add-root-btn {
  background: #1e3a5f;
  border: 1px solid #3b82f6;
  color: #93c5fd;
  padding: 6px 14px;
  border-radius: 6px;
  font-size: 12px;
  font-weight: 600;
  cursor: pointer;
  transition: background 0.15s;
}

.add-root-btn:hover {
  background: #2563eb;
  color: #fff;
}

.canvas-scroll {
  flex: 1;
  overflow: auto;
  padding: 24px;
}

.tree-layout {
  display: flex;
  flex-direction: column;
  gap: 24px;
  min-width: max-content;
}

.empty-state {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 12px;
  color: #4b5563;
  font-size: 14px;
  padding: 60px 0;
}

.canvas-empty {
  padding: 80px 0;
}

.empty-icon {
  font-size: 40px;
  line-height: 1;
}

.empty-state p { margin: 0; text-align: center; }
.empty-state strong { color: #6b7280; }
</style>
