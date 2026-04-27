<script setup lang="ts">
defineProps<{ status: string }>();
</script>

<template>
  <span class="badge" :class="status">
    <span v-if="status === 'running'" class="spinner" />
    <span v-else class="dot" />
    {{ status }}
  </span>
</template>

<style scoped>
.badge {
  display: inline-flex;
  align-items: center;
  gap: 5px;
  font-size: 11px;
  font-family: 'Consolas', monospace;
  text-transform: uppercase;
  letter-spacing: 0.04em;
  padding: 2px 6px;
  border-radius: 4px;
  font-weight: 600;
}

.dot {
  width: 7px;
  height: 7px;
  border-radius: 50%;
  background: currentColor;
  flex-shrink: 0;
}

.spinner {
  width: 9px;
  height: 9px;
  border-radius: 50%;
  border: 2px solid currentColor;
  border-top-color: transparent;
  flex-shrink: 0;
  animation: spin 0.7s linear infinite;
}

@keyframes spin {
  to { transform: rotate(360deg); }
}

.queued  { color: #6b7280; background: #1f2937; }
.running { color: #3b82f6; background: #1e3a5f; }
.success { color: #22c55e; background: #14311f; }
.error   { color: #ef4444; background: #3b1515; }
.killed  { color: #f59e0b; background: #3b2a0a; }
</style>
