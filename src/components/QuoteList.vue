<script setup lang="ts">
import { computed } from "vue";
import { useStockStore } from "../stores/stock";
import type { QuoteData } from "../types";

const store = useStockStore();

const emit = defineEmits<{
  (e: "select", code: string): void;
  (e: "show-kline", code: string): void;
  (e: "manage"): void;
}>();

const quotes = computed(() => store.sortedQuotes);

function formatVolume(vol: number): string {
  if (vol >= 10000) return (vol / 10000).toFixed(1) + "万";
  return vol.toFixed(0);
}

function onClickStock(stock: QuoteData) {
  emit("select", stock.code);
}
</script>

<template>
  <div class="quote-list">
    <div class="header">
      <h2>我的喜欢</h2>
      <div class="header-right">
        <button class="manage-btn" @click="emit('manage')" title="管理我的喜欢">
          <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round">
            <path d="M12 20h9"/>
            <path d="M16.5 3.5a2.121 2.121 0 0 1 3 3L7 19l-4 1 1-4L16.5 3.5z"/>
          </svg>
        </button>
        <div class="status">
          <span class="dot" :class="{ active: quotes.length > 0 }"></span>
          <span class="time">{{ quotes.length > 0 ? "实时" : "连接中" }}</span>
        </div>
      </div>
    </div>

    <div class="col-headers">
      <span class="col-name">名称/代码</span>
      <span class="col-spacer"></span>
      <span class="col-price">现价</span>
      <span class="col-change">涨跌幅</span>
    </div>

    <div class="list">
      <div
        v-for="stock in quotes"
        :key="stock.code"
        class="stock-item"
        @click="onClickStock(stock)"
      >
        <div class="info">
          <div class="name">{{ stock.name }}</div>
          <div class="code">{{ stock.code }}</div>
        </div>
        <div class="spacer"></div>
        <div class="price" :class="stock.change >= 0 ? 'up' : 'down'">
          {{ stock.price.toFixed(2) }}
        </div>
        <div class="change" :class="stock.change >= 0 ? 'up-bg' : 'down-bg'">
          {{ stock.change >= 0 ? "+" : "" }}{{ stock.change_pct.toFixed(2) }}%
        </div>
      </div>

      <div v-if="quotes.length === 0" class="empty">
        <span v-if="store.errorMsg">{{ store.errorMsg }}</span>
        <span v-else>正在获取行情数据...</span>
      </div>
    </div>
  </div>
</template>

<style scoped>
.quote-list {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.header {
  padding: 12px 16px 8px;
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.header h2 {
  font-size: 14px;
  font-weight: 700;
  color: var(--ink);
}

.header-right {
  display: flex;
  align-items: center;
  gap: 8px;
}

.manage-btn {
  width: 22px;
  height: 22px;
  border-radius: var(--radius);
  background: var(--surface2);
  color: var(--muted);
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.15s;
}

.manage-btn:hover {
  background: var(--accent);
  color: #fff;
}

.status {
  display: flex;
  align-items: center;
  gap: 4px;
  font-size: 11px;
  color: var(--muted);
}

.dot {
  width: 6px;
  height: 6px;
  border-radius: 50%;
  background: var(--muted2);
}

.dot.active {
  background: var(--down);
  animation: pulse 1.5s infinite;
}

@keyframes pulse {
  0%, 100% { opacity: 1; }
  50% { opacity: 0.3; }
}

.col-headers {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 4px 12px 6px;
  font-size: 10px;
  color: var(--muted2);
  font-weight: 600;
}

.col-name { flex: 0 0 auto; text-align: left; }
.col-spacer { flex: 1; }
.col-price { width: 60px; text-align: right; }
.col-change { width: 54px; text-align: center; }

.list {
  flex: 1;
  overflow-y: auto;
  padding: 0 8px 8px;
}

.stock-item {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: var(--row-padding) 8px;
  border-radius: var(--radius);
  cursor: pointer;
  transition: background 0.12s;
}

.stock-item:hover {
  background: var(--surface2);
}

.info {
  flex: 0 1 auto;
  min-width: 0;
  display: flex;
  flex-direction: column;
  gap: 1px;
}

.spacer {
  flex: 1;
}

.name {
  font-size: 13px;
  font-weight: 600;
  color: var(--ink);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.code {
  font-size: 10px;
  color: var(--muted);
  font-family: var(--font-mono);
}

.price {
  width: 60px;
  text-align: right;
  font-size: 13px;
  font-weight: 700;
  font-family: var(--font-mono);
  font-variant-numeric: tabular-nums;
  flex-shrink: 0;
}

.change {
  width: 54px;
  text-align: center;
  font-size: 11px;
  font-weight: 700;
  font-family: var(--font-mono);
  padding: 3px 0;
  border-radius: 4px;
  font-variant-numeric: tabular-nums;
  flex-shrink: 0;
}

.empty {
  padding: 40px 16px;
  text-align: center;
  color: var(--muted);
  font-size: 12px;
}
</style>
