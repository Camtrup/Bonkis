<script setup lang="ts">
import { ref, onMounted, computed } from "vue";
import { listAvailableTools, runTool } from "../lib/tauri";
import { useTreeStore } from "../stores/tree";
import type { NodeRow } from "../lib/types";

const props = defineProps<{
  sessionId: string;
  parentNode?: NodeRow | null;
}>();

const emit = defineEmits<{
  (e: "close"): void;
}>();

const treeStore = useTreeStore();

const availableTools = ref<string[]>([]);
const toolSearch = ref(props.parentNode ? "" : "nmap");
const argsInput = ref("");
const workingDir = ref("");
const submitting = ref(false);
const error = ref("");

const showDropdown = ref(false);

const DROPDOWN_HIDE_DELAY = 150;

// Filtered list for combobox
const filteredTools = computed(() => {
  const q = toolSearch.value.toLowerCase();
  if (!q) return availableTools.value.slice(0, 50);
  return availableTools.value.filter((t) => t.toLowerCase().includes(q)).slice(0, 50);
});

onMounted(async () => {
  try {
    availableTools.value = await listAvailableTools();
  } catch {
    availableTools.value = [];
  }
});

function selectTool(name: string) {
  toolSearch.value = name;
  showDropdown.value = false;
}

async function submit() {
  const tool = toolSearch.value.trim();
  if (!tool) {
    error.value = "Tool name is required.";
    return;
  }
  submitting.value = true;
  error.value = "";
  try {
    const args = argsInput.value
      .trim()
      .split(/\s+/)
      .filter((a) => a.length > 0);
    const node = await runTool({
      session_id: props.sessionId,
      parent_id: props.parentNode?.id ?? null,
      tool,
      args,
      working_dir: workingDir.value.trim() || null,
    });
    treeStore.upsertNode(node);
    emit("close");
  } catch (e: unknown) {
    error.value = String(e);
  } finally {
    submitting.value = false;
  }
}

function hideDropdown() {
  setTimeout(() => { showDropdown.value = false; }, DROPDOWN_HIDE_DELAY);
}

const parsedParentArgs = computed(() => {
  if (!props.parentNode) return "";
  try {
    return (JSON.parse(props.parentNode.args) as string[]).join(" ");
  } catch {
    return props.parentNode.args;
  }
});

function onKeydown(e: KeyboardEvent) {
  if (e.key === "Escape") emit("close");
}
</script>

<template>
  <div class="overlay" @keydown="onKeydown" @mousedown.self="$emit('close')">
    <div class="modal">
      <div class="modal-header">
        <span class="modal-title">{{ parentNode ? "Spawn child tool" : "Add root node" }}</span>
        <button class="close-btn" @click="$emit('close')">×</button>
      </div>

      <div v-if="parentNode" class="parent-info">
        <span class="label">Parent:</span>
        <span class="mono">{{ parentNode.tool }} {{ parsedParentArgs }}</span>
      </div>

      <div class="field">
        <label>Tool</label>
        <div class="combobox">
          <input
            v-model="toolSearch"
            class="input mono"
            placeholder="e.g. nmap"
            autocomplete="off"
            @focus="showDropdown = true"
            @blur="hideDropdown"
          />
          <ul v-if="showDropdown && filteredTools.length" class="dropdown">
            <li
              v-for="t in filteredTools"
              :key="t"
              @mousedown.prevent="selectTool(t)"
              :class="{ active: t === toolSearch }"
            >{{ t }}</li>
          </ul>
        </div>
      </div>

      <div class="field">
        <label>Arguments</label>
        <input
          v-model="argsInput"
          class="input mono"
          placeholder="e.g. -sV 10.0.0.1"
        />
      </div>

      <div class="field">
        <label>Working Directory <span class="optional">(optional)</span></label>
        <input
          v-model="workingDir"
          class="input mono"
          placeholder="/home/user/targets"
        />
      </div>

      <p v-if="error" class="error">{{ error }}</p>

      <div class="actions">
        <button class="btn-secondary" @click="$emit('close')">Cancel</button>
        <button class="btn-primary" :disabled="submitting" @click="submit">
          {{ submitting ? "Running…" : "Run" }}
        </button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.overlay {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.65);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 200;
}

.modal {
  background: #1e1e1e;
  border: 1px solid #333;
  border-radius: 8px;
  padding: 24px;
  width: 480px;
  max-width: 95vw;
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.modal-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.modal-title {
  font-size: 14px;
  font-weight: 600;
  color: #e2e8f0;
}

.close-btn {
  background: none;
  border: none;
  color: #9ca3af;
  font-size: 22px;
  cursor: pointer;
  line-height: 1;
  padding: 0 4px;
}

.close-btn:hover { color: #f1f5f9; }

.parent-info {
  background: #252525;
  border: 1px solid #333;
  border-radius: 6px;
  padding: 8px 12px;
  font-size: 12px;
  color: #9ca3af;
  display: flex;
  gap: 8px;
  align-items: center;
}

.label { color: #6b7280; }

.field {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.field label {
  font-size: 12px;
  color: #9ca3af;
  font-weight: 500;
}

.optional { font-weight: 400; color: #6b7280; }

.input {
  background: #252525;
  border: 1px solid #3a3a3a;
  border-radius: 6px;
  color: #e2e8f0;
  padding: 8px 10px;
  font-size: 13px;
  outline: none;
  transition: border-color 0.15s;
}

.input:focus { border-color: #3b82f6; }

.mono { font-family: 'Consolas', 'JetBrains Mono', monospace; }

.combobox { position: relative; }

.dropdown {
  position: absolute;
  top: 100%;
  left: 0;
  right: 0;
  background: #1a1a1a;
  border: 1px solid #3a3a3a;
  border-top: none;
  border-radius: 0 0 6px 6px;
  list-style: none;
  margin: 0;
  padding: 4px 0;
  max-height: 180px;
  overflow-y: auto;
  z-index: 300;
  font-family: 'Consolas', monospace;
  font-size: 12px;
}

.dropdown li {
  padding: 6px 10px;
  cursor: pointer;
  color: #cbd5e1;
}

.dropdown li:hover,
.dropdown li.active {
  background: #2d3748;
  color: #e2e8f0;
}

.error {
  color: #ef4444;
  font-size: 12px;
  margin: 0;
}

.actions {
  display: flex;
  justify-content: flex-end;
  gap: 10px;
}

.btn-primary,
.btn-secondary {
  padding: 8px 18px;
  border-radius: 6px;
  font-size: 13px;
  font-weight: 600;
  cursor: pointer;
  border: 1px solid transparent;
  transition: background 0.15s;
}

.btn-primary {
  background: #3b82f6;
  color: #fff;
}

.btn-primary:hover:not(:disabled) { background: #2563eb; }
.btn-primary:disabled { opacity: 0.5; cursor: not-allowed; }

.btn-secondary {
  background: #2d2d2d;
  color: #9ca3af;
  border-color: #3a3a3a;
}

.btn-secondary:hover { background: #374151; color: #e2e8f0; }
</style>
