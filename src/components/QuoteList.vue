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

function onClickStock(stock: QuoteData) {
  emit("select", stock.code);
}
</script>

<template>
  <div class="quote-list">
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
        <div class="change" :class="stock.change >= 0 ? 'up' : 'down'">
          {{ stock.change >= 0 ? "+" : "" }}{{ stock.change_pct.toFixed(2) }}%
        </div>
      </div>

      <div v-if="quotes.length === 0" class="empty">
        <span v-if="store.errorMsg">{{ store.errorMsg }}</span>
        <span v-else>加载中...</span>
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

.list {
  flex: 1;
  overflow-y: auto;
  padding: 2px 10px 4px;
}

.stock-item {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: var(--row-padding) 4px;
  border-radius: 3px;
  cursor: pointer;
  transition: background 0.1s;
}

.stock-item:hover {
  background: transparent;
}

.info {
  flex: 0 1 auto;
  min-width: 0;
  display: flex;
  flex-direction: column;
  gap: 0;
}

.spacer {
  flex: 1;
}

.name {
  font-size: 11px;
  font-weight: 500;
  color: var(--ink);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.code {
  font-size: 8px;
  color: var(--muted2);
  font-family: var(--font-mono);
}

.price {
  width: 50px;
  text-align: right;
  font-size: 11px;
  font-weight: 600;
  font-family: var(--font-mono);
  font-variant-numeric: tabular-nums;
  flex-shrink: 0;
}

.change {
  width: 46px;
  text-align: right;
  font-size: 10px;
  font-weight: 500;
  font-family: var(--font-mono);
  font-variant-numeric: tabular-nums;
  flex-shrink: 0;
}

.empty {
  padding: 20px 12px;
  text-align: center;
  color: var(--muted2);
  font-size: 10px;
}
</style>
