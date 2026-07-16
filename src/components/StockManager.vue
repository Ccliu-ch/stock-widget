<script setup lang="ts">
import { ref, onMounted, computed } from "vue";
import { useStockStore } from "../stores/stock";
import type { SearchResult, QuoteData } from "../types";

const store = useStockStore();
const emit = defineEmits<{
  (e: "back"): void;
}>();

const keyword = ref("");
const searchResults = ref<SearchResult[]>([]);
const searching = ref(false);
const searchError = ref("");

let searchTimer: ReturnType<typeof setTimeout> | null = null;

// 获取当前自选股的行情信息
const stockQuotes = computed(() => {
  const map = new Map<string, QuoteData>();
  for (const q of store.quotes) {
    map.set(q.code, q);
  }
  return map;
});

function onSearchInput() {
  if (searchTimer) clearTimeout(searchTimer);
  searchError.value = "";

  const kw = keyword.value.trim();
  if (!kw) {
    searchResults.value = [];
    return;
  }

  searching.value = true;
  searchTimer = setTimeout(async () => {
    try {
      const results = await store.searchStocks(kw);
      searchResults.value = results;
    } catch (e: any) {
      searchError.value = typeof e === "string" ? e : (e.message || "搜索失败");
      searchResults.value = [];
    } finally {
      searching.value = false;
    }
  }, 300);
}

function isAdded(code: string): boolean {
  return store.stockCodes.includes(code);
}

function addStock(result: SearchResult) {
  store.addStock(result.code);
}

function removeStock(code: string) {
  store.removeStock(code);
}

function moveUp(index: number) {
  store.moveStock(index, "up");
}

function moveDown(index: number) {
  store.moveStock(index, "down");
}

function clearSearch() {
  keyword.value = "";
  searchResults.value = [];
  searchError.value = "";
}

onMounted(() => {
  store.loadStockCodes();
});
</script>

<template>
  <div class="manager-view">
    <div class="detail-header">
      <button class="back-btn" @click="emit('back')">
        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round">
          <path d="m15 18-6-6 6-6"/>
        </svg>
      </button>
      <div class="detail-title">
        <div class="dt-name">我的喜欢管理</div>
        <div class="dt-code">{{ store.stockCodes.length }} 只</div>
      </div>
    </div>

    <!-- 搜索区域 -->
    <div class="search-section">
      <div class="search-box">
        <svg class="search-icon" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round">
          <circle cx="11" cy="11" r="8"/>
          <path d="m21 21-4.35-4.35"/>
        </svg>
        <input
          v-model="keyword"
          class="search-input"
          placeholder="搜索股票名称或代码..."
          @input="onSearchInput"
        />
        <button v-if="keyword" class="clear-btn" @click="clearSearch">
          <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round">
            <line x1="6" y1="6" x2="18" y2="18"/>
            <line x1="18" y1="6" x2="6" y2="18"/>
          </svg>
        </button>
      </div>

      <!-- 搜索结果 -->
      <div v-if="searching" class="search-status">搜索中...</div>
      <div v-else-if="searchError" class="search-status error">{{ searchError }}</div>
      <div v-else-if="searchResults.length > 0" class="search-results">
        <div
          v-for="result in searchResults"
          :key="result.code"
          class="search-item"
        >
          <div class="sr-info">
            <div class="sr-name">{{ result.name }}</div>
            <div class="sr-code">{{ result.code }}</div>
          </div>
          <button
            class="add-btn"
            :class="{ added: isAdded(result.code) }"
            @click="addStock(result)"
            :disabled="isAdded(result.code)"
          >
            {{ isAdded(result.code) ? "已添加" : "添加" }}
          </button>
        </div>
      </div>
    </div>

    <!-- 当前自选股列表 -->
    <div class="stock-list-section">
      <div class="section-title">
        <span>我的自选</span>
        <span class="count">{{ store.stockCodes.length }}/50</span>
      </div>
      <div class="stock-list">
        <div
          v-for="(code, index) in store.stockCodes"
          :key="code"
          class="stock-row"
        >
          <div class="order">{{ index + 1 }}</div>
          <div class="stock-info">
            <div class="stock-name">{{ stockQuotes.get(code)?.name || code }}</div>
            <div class="stock-code">{{ code }}</div>
          </div>
          <div class="stock-price" v-if="stockQuotes.get(code)">
            <span :class="stockQuotes.get(code)!.change >= 0 ? 'up' : 'down'">
              {{ stockQuotes.get(code)!.price.toFixed(2) }}
            </span>
          </div>
          <div class="actions">
            <button
              class="action-btn move-btn"
              :disabled="index === 0"
              @click="moveUp(index)"
            >
              <svg width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3" stroke-linecap="round">
                <path d="m18 15-6-6-6 6"/>
              </svg>
            </button>
            <button
              class="action-btn move-btn"
              :disabled="index === store.stockCodes.length - 1"
              @click="moveDown(index)"
            >
              <svg width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3" stroke-linecap="round">
                <path d="m6 9 6 6 6-6"/>
              </svg>
            </button>
            <button class="action-btn del-btn" @click="removeStock(code)">
              <svg width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round">
                <path d="M3 6h18M8 6V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2m3 0v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6"/>
              </svg>
            </button>
          </div>
        </div>

        <div v-if="store.stockCodes.length === 0" class="empty-list">
          暂无喜欢的股票,请在上方搜索添加
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.manager-view {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.detail-header {
  padding: 10px 16px;
  display: flex;
  align-items: center;
  gap: 10px;
  border-bottom: 1px solid var(--border);
}

.back-btn {
  width: 28px;
  height: 28px;
  border-radius: var(--radius);
  background: var(--surface2);
  color: var(--muted);
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.15s;
}

.back-btn:hover {
  background: var(--accent);
  color: #fff;
}

.detail-title {
  flex: 1;
}

.dt-name {
  font-size: 14px;
  font-weight: 700;
  color: var(--ink);
}

.dt-code {
  font-size: 10px;
  color: var(--muted);
  font-family: var(--font-mono);
}

/* 搜索区域 */
.search-section {
  padding: 12px 16px;
  border-bottom: 1px solid var(--border);
}

.search-box {
  display: flex;
  align-items: center;
  gap: 8px;
  background: var(--surface2);
  border: 1px solid var(--border2);
  border-radius: var(--radius);
  padding: 6px 10px;
  transition: border-color 0.15s;
}

.search-box:focus-within {
  border-color: var(--accent);
}

.search-icon {
  color: var(--muted2);
  flex-shrink: 0;
}

.search-input {
  flex: 1;
  background: transparent;
  border: none;
  outline: none;
  color: var(--ink);
  font-size: 12px;
  font-family: var(--font-sans);
}

.search-input::placeholder {
  color: var(--muted2);
}

.clear-btn {
  width: 18px;
  height: 18px;
  border-radius: 50%;
  background: var(--surface3);
  color: var(--muted);
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
}

.clear-btn:hover {
  background: var(--accent);
  color: #fff;
}

.search-status {
  padding: 12px 4px;
  font-size: 11px;
  color: var(--muted);
  text-align: center;
}

.search-status.error {
  color: var(--up);
}

.search-results {
  margin-top: 8px;
  max-height: 200px;
  overflow-y: auto;
}

.search-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 7px 8px;
  border-radius: 6px;
  transition: background 0.12s;
}

.search-item:hover {
  background: var(--surface2);
}

.sr-info {
  display: flex;
  flex-direction: column;
  gap: 1px;
}

.sr-name {
  font-size: 12px;
  font-weight: 600;
  color: var(--ink);
}

.sr-code {
  font-size: 10px;
  color: var(--muted);
  font-family: var(--font-mono);
}

.add-btn {
  padding: 3px 10px;
  font-size: 11px;
  font-weight: 600;
  color: #fff;
  background: var(--accent);
  border-radius: 4px;
  transition: all 0.15s;
}

.add-btn.added {
  background: var(--surface3);
  color: var(--muted2);
  cursor: not-allowed;
}

/* 自选股列表 */
.stock-list-section {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.section-title {
  padding: 10px 16px 6px;
  display: flex;
  justify-content: space-between;
  align-items: center;
  font-size: 11px;
  font-weight: 700;
  color: var(--muted);
}

.count {
  font-size: 10px;
  color: var(--muted2);
  font-family: var(--font-mono);
}

.stock-list {
  flex: 1;
  overflow-y: auto;
  padding: 0 8px 8px;
}

.stock-row {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 7px 8px;
  border-radius: var(--radius);
  transition: background 0.12s;
}

.stock-row:hover {
  background: var(--surface2);
}

.order {
  width: 18px;
  text-align: center;
  font-size: 10px;
  color: var(--muted2);
  font-family: var(--font-mono);
  flex-shrink: 0;
}

.stock-info {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 1px;
}

.stock-name {
  font-size: 12px;
  font-weight: 600;
  color: var(--ink);
}

.stock-code {
  font-size: 10px;
  color: var(--muted);
  font-family: var(--font-mono);
}

.stock-price {
  width: 55px;
  text-align: right;
  font-size: 12px;
  font-weight: 700;
  font-family: var(--font-mono);
  font-variant-numeric: tabular-nums;
}

.actions {
  display: flex;
  gap: 3px;
  flex-shrink: 0;
}

.action-btn {
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

.move-btn:hover:not(:disabled) {
  background: var(--accent);
  color: #fff;
}

.del-btn:hover {
  background: var(--up);
  color: #fff;
}

.action-btn:disabled {
  opacity: 0.3;
  cursor: not-allowed;
}

.empty-list {
  padding: 40px 16px;
  text-align: center;
  color: var(--muted);
  font-size: 12px;
}
</style>
