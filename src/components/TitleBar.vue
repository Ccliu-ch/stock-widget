<script setup lang="ts">
import { ref } from "vue";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { invoke } from "@tauri-apps/api/core";
import { useStockStore } from "../stores/stock";

const store = useStockStore();
const showSettings = ref(false);

async function minimizeToTray() {
  await getCurrentWindow().hide();
}

async function closeApp() {
  await invoke("quit_app");
}

function toggleSettings() {
  showSettings.value = !showSettings.value;
}

function onOpacityChange(e: Event) {
  const target = e.target as HTMLInputElement;
  store.setOpacity(parseFloat(target.value));
}

function onRowSpacingChange(e: Event) {
  const target = e.target as HTMLInputElement;
  store.setRowSpacing(parseFloat(target.value));
}
</script>

<template>
  <div class="titlebar" data-tauri-drag-region>
    <div class="drag-area" data-tauri-drag-region></div>
    <div class="right">
      <button class="tb-btn" @click="toggleSettings" title="设置">
        <svg width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round">
          <circle cx="12" cy="12" r="3"/>
          <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-4 0v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1 0-4h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 4 0v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 0 4h-.09a1.65 1.65 0 0 0-1.51 1z"/>
        </svg>
      </button>
      <button class="tb-btn theme-btn" @click="store.toggleTheme" title="主题">
        <svg v-if="store.theme === 'dark'" width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round">
          <path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"/>
        </svg>
        <svg v-else width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round">
          <circle cx="12" cy="12" r="5"/>
          <line x1="12" y1="1" x2="12" y2="3"/>
          <line x1="12" y1="21" x2="12" y2="23"/>
          <line x1="4.22" y1="4.22" x2="5.64" y2="5.64"/>
          <line x1="18.36" y1="18.36" x2="19.78" y2="19.78"/>
          <line x1="1" y1="12" x2="3" y2="12"/>
          <line x1="21" y1="12" x2="23" y2="12"/>
          <line x1="4.22" y1="19.78" x2="5.64" y2="18.36"/>
          <line x1="18.36" y1="5.64" x2="19.78" y2="4.22"/>
        </svg>
      </button>
      <button class="tb-btn" @click="minimizeToTray" title="隐藏">
        <svg width="8" height="8" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3" stroke-linecap="round">
          <line x1="5" y1="12" x2="19" y2="12"/>
        </svg>
      </button>
      <button class="tb-btn close" @click="closeApp" title="退出">
        <svg width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round">
          <line x1="6" y1="6" x2="18" y2="18"/>
          <line x1="18" y1="6" x2="6" y2="18"/>
        </svg>
      </button>
    </div>

    <Transition name="slide-down">
      <div v-if="showSettings" class="settings-panel">
        <div class="setting-row">
          <span class="setting-label">透明</span>
          <input
            type="range"
            class="setting-slider"
            min="0.05"
            max="1"
            step="0.05"
            :value="store.opacity"
            @input="onOpacityChange"
          />
          <span class="setting-value">{{ Math.round(store.opacity * 100) }}%</span>
        </div>
        <div class="setting-row">
          <span class="setting-label">行距</span>
          <input
            type="range"
            class="setting-slider"
            min="0"
            max="16"
            step="1"
            :value="store.rowSpacing"
            @input="onRowSpacingChange"
          />
          <span class="setting-value">{{ store.rowSpacing }}px</span>
        </div>
      </div>
    </Transition>
  </div>
</template>

<style scoped>
.titlebar {
  height: 20px;
  display: flex;
  align-items: center;
  justify-content: flex-end;
  padding: 0 6px;
  background: transparent;
  border-bottom: none;
  flex-shrink: 0;
  user-select: none;
  position: relative;
}

.drag-area {
  flex: 1;
  height: 100%;
  -webkit-app-region: drag;
}

.right {
  display: flex;
  gap: 4px;
}

.tb-btn {
  width: 14px;
  height: 14px;
  border-radius: 3px;
  background: transparent;
  color: var(--muted2);
  display: flex;
  align-items: center;
  justify-content: center;
  opacity: 0.4;
  transition: all 0.15s;
}

.tb-btn:hover {
  opacity: 1;
  background: rgba(255, 255, 255, 0.05);
  color: var(--accent);
}

.tb-btn.close:hover {
  color: var(--up);
}

.settings-panel {
  position: absolute;
  top: 100%;
  right: 6px;
  display: flex;
  flex-direction: column;
  gap: 4px;
  padding: 6px 8px;
  background: rgba(0, 0, 0, 0.6);
  backdrop-filter: blur(10px);
  -webkit-backdrop-filter: blur(10px);
  border-radius: 4px;
  z-index: 100;
  min-width: 160px;
}

.setting-row {
  display: flex;
  align-items: center;
  gap: 6px;
}

.setting-label {
  font-size: 8px;
  color: #999;
  white-space: nowrap;
  width: 20px;
}

.setting-slider {
  flex: 1;
  height: 3px;
  -webkit-appearance: none;
  appearance: none;
  background: rgba(255, 255, 255, 0.1);
  border-radius: 2px;
  outline: none;
  cursor: pointer;
}

.setting-slider::-webkit-slider-thumb {
  -webkit-appearance: none;
  appearance: none;
  width: 10px;
  height: 10px;
  border-radius: 50%;
  background: var(--accent);
  cursor: pointer;
}

.setting-slider::-moz-range-thumb {
  width: 10px;
  height: 10px;
  border-radius: 50%;
  background: var(--accent);
  cursor: pointer;
  border: none;
}

.setting-value {
  font-size: 8px;
  color: #ccc;
  font-family: var(--font-mono);
  min-width: 24px;
  text-align: right;
}

.slide-down-enter-active,
.slide-down-leave-active {
  transition: all 0.15s ease;
}

.slide-down-enter-from,
.slide-down-leave-to {
  opacity: 0;
  transform: translateY(-4px);
}
</style>
