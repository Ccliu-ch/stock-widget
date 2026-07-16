<script setup lang="ts">
import { ref, onMounted, onUnmounted } from "vue";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import { useStockStore } from "./stores/stock";
import type { QuoteData, ViewType } from "./types";
import TitleBar from "./components/TitleBar.vue";
import QuoteList from "./components/QuoteList.vue";
import MinuteChart from "./components/MinuteChart.vue";
import KlineChart from "./components/KlineChart.vue";
import StockManager from "./components/StockManager.vue";

const store = useStockStore();
const currentView = ref<ViewType>("list");
const selectedCode = ref("");

let unlisten: UnlistenFn | null = null;
let unlistenError: UnlistenFn | null = null;

onMounted(async () => {
  store.initTheme();

  unlisten = await listen<QuoteData[]>("quote-update", (event) => {
    store.setQuotes(event.payload);
  });

  unlistenError = await listen<string>("quote-error", (event) => {
    store.setError(event.payload);
  });
});

onUnmounted(() => {
  unlisten?.();
  unlistenError?.();
});

function showMinute(code: string) {
  selectedCode.value = code;
  currentView.value = "minute";
}

function showKline(code: string) {
  selectedCode.value = code;
  currentView.value = "kline";
}

function backToList() {
  currentView.value = "list";
}

function showManager() {
  currentView.value = "manage";
}

function switchView(view: ViewType) {
  currentView.value = view;
}
</script>

<template>
  <TitleBar />
  <div class="content">
    <QuoteList
      v-if="currentView === 'list'"
      @select="showMinute"
      @show-kline="showKline"
      @manage="showManager"
    />
    <MinuteChart
      v-else-if="currentView === 'minute'"
      :code="selectedCode"
      @back="backToList"
      @switch-kline="showKline(selectedCode)"
    />
    <KlineChart
      v-else-if="currentView === 'kline'"
      :code="selectedCode"
      @back="backToList"
      @switch-minute="showMinute(selectedCode)"
    />
    <StockManager
      v-else-if="currentView === 'manage'"
      @back="backToList"
    />
  </div>
</template>

<style scoped>
.content {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}
</style>
