import { defineStore } from "pinia";
import { ref } from "vue";
import {
  listSessions,
  createSession as apiCreateSession,
  deleteSession as apiDeleteSession,
} from "../lib/tauri";
import type { SessionRow } from "../lib/types";

export const useSessionStore = defineStore("session", () => {
  const sessions = ref<SessionRow[]>([]);
  const activeSessionId = ref<string | null>(null);

  async function loadSessions() {
    sessions.value = await listSessions();
  }

  async function createSession(name: string) {
    const session = await apiCreateSession(name);
    sessions.value.unshift(session);
    return session;
  }

  async function deleteSession(id: string) {
    await apiDeleteSession(id);
    sessions.value = sessions.value.filter((s) => s.id !== id);
    if (activeSessionId.value === id) {
      activeSessionId.value = sessions.value[0]?.id ?? null;
    }
  }

  function setActive(id: string | null) {
    activeSessionId.value = id;
  }

  return { sessions, activeSessionId, loadSessions, createSession, deleteSession, setActive };
});
