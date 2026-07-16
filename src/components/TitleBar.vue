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
    <div class="left" data-tauri-drag-region>
      <div class="logo">G</div>
      <span class="title">加油努力</span>
    </div>
    <div class="right">
      <button class="tb-btn" @click="toggleSettings" title="设置">
        <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round">
          <circle cx="12" cy="12" r="3"/>
          <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-4 0v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1 0-4h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 4 0v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 0 4h-.09a1.65 1.65 0 0 0-1.51 1z"/>
        </svg>
      </button>
      <button class="tb-btn theme-btn" @click="store.toggleTheme" title="切换主题">
        <svg v-if="store.theme === 'dark'" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round">
          <path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"/>
        </svg>
        <svg v-else width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round">
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
      <button class="tb-btn" @click="minimizeToTray" title="最小化到托盘">
        <svg width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3" stroke-linecap="round">
          <line x1="5" y1="12" x2="19" y2="12"/>
        </svg>
      </button>
      <button class="tb-btn close" @click="closeApp" title="退出">
        <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round">
          <line x1="6" y1="6" x2="18" y2="18"/>
          <line x1="18" y1="6" x2="6" y2="18"/>
        </svg>
      </button>
    </div>

    <!-- 设置面板 -->
    <Transition name="slide-down">
      <div v-if="showSettings" class="settings-panel">
        <div class="setting-row">
          <span class="setting-label">透明度</span>
          <input
            type="range"
            class="setting-slider"
            min="0.1"
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
  height: 38px;
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 12px;
  background: var(--surface);
  border-bottom: 1px solid var(--border);
  flex-shrink: 0;
  user-select: none;
  position: relative;
}

.left {
  display: flex;
  align-items: center;
  gap: 8px;
}

.logo {
  width: 18px;
  height: 18px;
  border-radius: 4px;
  background: linear-gradient(135deg, var(--accent), #8B5CF6);
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 10px;
  font-weight: 800;
  color: #fff;
}

.title {
  font-size: 12px;
  font-weight: 600;
  color: var(--ink);
}

.right {
  display: flex;
  gap: 6px;
}

.tb-btn {
  width: 20px;
  height: 20px;
  border-radius: 4px;
  background: var(--surface3);
  color: var(--muted);
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.15s;
}

.tb-btn:hover {
  background: var(--accent);
  color: #fff;
}

.tb-btn.close:hover {
  background: var(--up);
}

/* 设置面板 */
.settings-panel {
  position: absolute;
  top: 100%;
  right: 12px;
  display: flex;
  flex-direction: column;
  gap: 6px;
  padding: 10px 12px;
  background: var(--surface3);
  border: 1px solid var(--border2);
  border-radius: var(--radius);
  z-index: 100;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
  min-width: 200px;
}

.setting-row {
  display: flex;
  align-items: center;
  gap: 8px;
}

.setting-label {
  font-size: 10px;
  color: var(--muted);
  white-space: nowrap;
  width: 32px;
}

.setting-slider {
  flex: 1;
  height: 4px;
  -webkit-appearance: none;
  appearance: none;
  background: var(--surface2);
  border-radius: 2px;
  outline: none;
  cursor: pointer;
}

.setting-slider::-webkit-slider-thumb {
  -webkit-appearance: none;
  appearance: none;
  width: 12px;
  height: 12px;
  border-radius: 50%;
  background: var(--accent);
  cursor: pointer;
  transition: transform 0.15s;
}

.setting-slider::-webkit-slider-thumb:hover {
  transform: scale(1.2);
}

.setting-slider::-moz-range-thumb {
  width: 12px;
  height: 12px;
  border-radius: 50%;
  background: var(--accent);
  cursor: pointer;
  border: none;
}

.setting-value {
  font-size: 10px;
  color: var(--ink);
  font-family: var(--font-mono);
  min-width: 32px;
  text-align: right;
}

/* 过渡动画 */
.slide-down-enter-active,
.slide-down-leave-active {
  transition: all 0.2s ease;
}

.slide-down-enter-from,
.slide-down-leave-to {
  opacity: 0;
  transform: translateY(-8px);
}
</style>
