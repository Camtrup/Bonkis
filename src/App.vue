<script setup lang="ts">
import { onMounted, onUnmounted } from "vue";
import type { UnlistenFn } from "@tauri-apps/api/event";
import { onNodeOutput, onNodeStatus } from "./lib/tauri";
import { useSessionStore } from "./stores/session";
import { useTreeStore } from "./stores/tree";
import SessionSidebar from "./components/SessionSidebar.vue";
import TreeCanvas from "./components/TreeCanvas.vue";

const sessionStore = useSessionStore();
const treeStore = useTreeStore();

let unlistenOutput: UnlistenFn | null = null;
let unlistenStatus: UnlistenFn | null = null;

onMounted(async () => {
  await sessionStore.loadSessions();
  if (sessionStore.sessions.length > 0) {
    sessionStore.setActive(sessionStore.sessions[0].id);
  }

  unlistenOutput = await onNodeOutput((event) => {
    treeStore.appendOutput(event);
  });

  unlistenStatus = await onNodeStatus((event) => {
    treeStore.applyStatusEvent(event);
  });
});

onUnmounted(() => {
  unlistenOutput?.();
  unlistenStatus?.();
});
</script>

<template>
  <div class="app-shell">
    <SessionSidebar />
    <TreeCanvas />
  </div>
</template>

<style>
*,
*::before,
*::after {
  box-sizing: border-box;
}

html,
body,
#app {
  height: 100%;
  margin: 0;
  padding: 0;
  background: #141414;
  color: #e2e8f0;
  font-family: Inter, system-ui, -apple-system, sans-serif;
  font-size: 14px;
  -webkit-font-smoothing: antialiased;
}

::-webkit-scrollbar {
  width: 6px;
  height: 6px;
}

::-webkit-scrollbar-track { background: #1a1a1a; }
::-webkit-scrollbar-thumb { background: #374151; border-radius: 3px; }
::-webkit-scrollbar-thumb:hover { background: #4b5563; }
</style>

<style scoped>
.app-shell {
  display: flex;
  height: 100vh;
  overflow: hidden;
}
</style>