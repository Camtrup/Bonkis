<script setup lang="ts">
import { ref } from "vue";
import { useSessionStore } from "../stores/session";

const sessionStore = useSessionStore();

const creating = ref(false);
const newName = ref("");
const confirmDeleteId = ref<string | null>(null);

function startCreate() {
  creating.value = true;
  newName.value = "";
}

async function submitCreate() {
  const name = newName.value.trim();
  if (!name) return;
  const session = await sessionStore.createSession(name);
  sessionStore.setActive(session.id);
  creating.value = false;
}

function cancelCreate() {
  creating.value = false;
  newName.value = "";
}

async function confirmDelete(id: string) {
  await sessionStore.deleteSession(id);
  confirmDeleteId.value = null;
}

function formatDate(ms: number): string {
  return new Date(ms).toLocaleDateString(undefined, {
    month: "short",
    day: "numeric",
    year: "numeric",
  });
}
</script>

<template>
  <aside class="sidebar">
    <div class="sidebar-header">
      <span class="sidebar-title">Sessions</span>
      <button class="new-btn" @click="startCreate" title="New session">+</button>
    </div>

    <!-- Inline create form -->
    <div v-if="creating" class="create-form">
      <input
        v-model="newName"
        class="create-input"
        placeholder="Session name…"
        autofocus
        @keydown.enter="submitCreate"
        @keydown.escape="cancelCreate"
      />
      <div class="create-actions">
        <button class="confirm-btn" @click="submitCreate">✓</button>
        <button class="cancel-btn" @click="cancelCreate">✕</button>
      </div>
    </div>

    <!-- Session list -->
    <ul class="session-list">
      <li
        v-for="session in sessionStore.sessions"
        :key="session.id"
        class="session-item"
        :class="{ active: session.id === sessionStore.activeSessionId }"
        @click="sessionStore.setActive(session.id)"
      >
        <div class="session-info">
          <span class="session-name">{{ session.name }}</span>
          <span class="session-date">{{ formatDate(session.created_at) }}</span>
        </div>

        <div class="item-actions">
          <template v-if="confirmDeleteId === session.id">
            <button class="del-confirm" @click.stop="confirmDelete(session.id)" title="Confirm delete">✓</button>
            <button class="del-cancel" @click.stop="confirmDeleteId = null" title="Cancel">✕</button>
          </template>
          <button
            v-else
            class="del-btn"
            @click.stop="confirmDeleteId = session.id"
            title="Delete session"
          >🗑</button>
        </div>
      </li>
    </ul>

    <div v-if="sessionStore.sessions.length === 0 && !creating" class="no-sessions">
      No sessions yet
    </div>
  </aside>
</template>

<style scoped>
.sidebar {
  width: 240px;
  min-width: 240px;
  background: #1a1a1a;
  border-right: 1px solid #222;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.sidebar-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 14px 16px 10px;
  border-bottom: 1px solid #222;
  flex-shrink: 0;
}

.sidebar-title {
  font-size: 11px;
  font-weight: 700;
  letter-spacing: 0.1em;
  text-transform: uppercase;
  color: #6b7280;
}

.new-btn {
  background: #252525;
  border: 1px solid #374151;
  color: #9ca3af;
  width: 24px;
  height: 24px;
  border-radius: 5px;
  cursor: pointer;
  font-size: 16px;
  line-height: 1;
  display: flex;
  align-items: center;
  justify-content: center;
}

.new-btn:hover { background: #3b82f6; color: #fff; border-color: #3b82f6; }

.create-form {
  padding: 10px 12px;
  display: flex;
  flex-direction: column;
  gap: 6px;
  border-bottom: 1px solid #222;
  background: #1e1e1e;
}

.create-input {
  background: #252525;
  border: 1px solid #3a3a3a;
  border-radius: 5px;
  color: #e2e8f0;
  padding: 6px 8px;
  font-size: 12px;
  outline: none;
}

.create-input:focus { border-color: #3b82f6; }

.create-actions {
  display: flex;
  gap: 6px;
  justify-content: flex-end;
}

.confirm-btn,
.cancel-btn {
  border: none;
  border-radius: 4px;
  width: 24px;
  height: 24px;
  cursor: pointer;
  font-size: 12px;
}

.confirm-btn { background: #166534; color: #4ade80; }
.confirm-btn:hover { background: #15803d; }
.cancel-btn { background: #374151; color: #9ca3af; }
.cancel-btn:hover { background: #4b5563; }

.session-list {
  list-style: none;
  margin: 0;
  padding: 6px 0;
  overflow-y: auto;
  flex: 1;
}

.session-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 8px 12px;
  cursor: pointer;
  border-radius: 6px;
  margin: 2px 6px;
  transition: background 0.1s;
  gap: 6px;
}

.session-item:hover { background: #252525; }

.session-item.active {
  background: #1e3a5f;
  border-left: 2px solid #3b82f6;
}

.session-info {
  display: flex;
  flex-direction: column;
  gap: 2px;
  min-width: 0;
  flex: 1;
}

.session-name {
  font-size: 13px;
  color: #e2e8f0;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.session-item.active .session-name { color: #93c5fd; }

.session-date {
  font-size: 10px;
  color: #4b5563;
}

.item-actions {
  display: flex;
  gap: 4px;
  flex-shrink: 0;
}

.del-btn {
  background: none;
  border: none;
  cursor: pointer;
  font-size: 12px;
  opacity: 0;
  transition: opacity 0.1s;
  padding: 2px;
}

.session-item:hover .del-btn { opacity: 0.5; }
.del-btn:hover { opacity: 1 !important; }

.del-confirm {
  background: #166534;
  color: #4ade80;
  border: none;
  border-radius: 3px;
  width: 20px;
  height: 20px;
  cursor: pointer;
  font-size: 10px;
}

.del-cancel {
  background: #374151;
  color: #9ca3af;
  border: none;
  border-radius: 3px;
  width: 20px;
  height: 20px;
  cursor: pointer;
  font-size: 10px;
}

.no-sessions {
  padding: 20px;
  text-align: center;
  font-size: 12px;
  color: #374151;
}
</style>
