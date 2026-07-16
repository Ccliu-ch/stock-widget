import { defineStore } from "pinia";
import { ref, computed } from "vue";
import { invoke } from "@tauri-apps/api/core";
import type { QuoteData, MinuteData, KlineBar, Theme, KlinePeriod, FuquanType, SearchResult } from "../types";

const DEFAULT_STOCKS = [
  "sh600519", "sz000001", "sz300750", "sz002594", "sh601318",
  "sh600036", "sz000858", "sh601012", "sz002415", "sh603259",
  "sh600276", "sz300760", "sz000568", "sz000725", "sz300059",
  "sh688981", "sz002049", "sh601633", "sh600690", "sh600309",
];

export const useStockStore = defineStore("stock", () => {
  const stockCodes = ref<string[]>([...DEFAULT_STOCKS]);
  const quotes = ref<QuoteData[]>([]);
  const theme = ref<Theme>("dark");
  const opacity = ref<number>(0.55);
  const rowSpacing = ref<number>(4);
  const loading = ref(false);
  const errorMsg = ref("");

  // 从 localStorage 加载透明度
  const savedOpacity = localStorage.getItem("stock-widget-opacity");
  if (savedOpacity) {
    const val = parseFloat(savedOpacity);
    if (!isNaN(val) && val >= 0.1 && val <= 1.0) {
      opacity.value = val;
    }
  }

  // 从 localStorage 加载行间距
  const savedRowSpacing = localStorage.getItem("stock-widget-row-spacing");
  if (savedRowSpacing) {
    const val = parseFloat(savedRowSpacing);
    if (!isNaN(val) && val >= 0 && val <= 16) {
      rowSpacing.value = val;
    }
  }

  function applyOpacity(val: number) {
    document.documentElement.style.setProperty("--opacity", val.toString());
    // 动态更新 surface 透明度
    const isDark = theme.value === "dark";
    const r = isDark ? 19 : 255;
    const g = isDark ? 23 : 255;
    const b = isDark ? 34 : 255;
    document.documentElement.style.setProperty("--surface", `rgba(${r}, ${g}, ${b}, ${val})`);
    const r2 = isDark ? 28 : 241;
    const g2 = isDark ? 32 : 245;
    const b2 = isDark ? 48 : 249;
    document.documentElement.style.setProperty("--surface2", `rgba(${r2}, ${g2}, ${b2}, ${val * 0.9})`);
  }

  function setOpacity(val: number) {
    opacity.value = val;
    applyOpacity(val);
    localStorage.setItem("stock-widget-opacity", val.toString());
  }

  function applyRowSpacing(val: number) {
    document.documentElement.style.setProperty("--row-padding", `${val}px`);
  }

  function setRowSpacing(val: number) {
    rowSpacing.value = val;
    applyRowSpacing(val);
    localStorage.setItem("stock-widget-row-spacing", val.toString());
  }

  const sortedQuotes = computed(() => {
    return [...quotes.value].sort((a, b) => {
      // Keep original order based on stockCodes
      const idxA = stockCodes.value.indexOf(a.code);
      const idxB = stockCodes.value.indexOf(b.code);
      return idxA - idxB;
    });
  });

  function setQuotes(data: QuoteData[]) {
    quotes.value = data;
    loading.value = false;
    errorMsg.value = "";
  }

  function setError(msg: string) {
    errorMsg.value = msg;
  }

  function toggleTheme() {
    theme.value = theme.value === "dark" ? "light" : "dark";
    document.documentElement.setAttribute("data-theme", theme.value);
    applyOpacity(opacity.value);
  }

  function initTheme() {
    document.documentElement.setAttribute("data-theme", theme.value);
    applyOpacity(opacity.value);
    applyRowSpacing(rowSpacing.value);
  }

  async function fetchMinuteData(code: string): Promise<MinuteData> {
    return await invoke<MinuteData>("fetch_minute_data", { code });
  }

  async function fetchKlineData(
    code: string,
    period: KlinePeriod,
    count: number = 120,
    fq: FuquanType = "qfq"
  ): Promise<KlineBar[]> {
    return await invoke<KlineBar[]>("fetch_kline_data", {
      code,
      period,
      count,
      fq,
    });
  }

  function updateStockCodes(codes: string[]) {
    stockCodes.value = codes;
    invoke("update_stock_codes", { codes }).catch(console.error);
  }

  async function searchStocks(keyword: string): Promise<SearchResult[]> {
    return await invoke<SearchResult[]>("search_stocks", { keyword });
  }

  async function loadStockCodes(): Promise<void> {
    try {
      const codes = await invoke<string[]>("get_stock_codes");
      if (codes.length > 0) {
        stockCodes.value = codes;
      }
    } catch (e) {
      console.error("加载股票代码失败:", e);
    }
  }

  function addStock(code: string) {
    if (!stockCodes.value.includes(code)) {
      stockCodes.value.push(code);
      updateStockCodes(stockCodes.value);
    }
  }

  function removeStock(code: string) {
    stockCodes.value = stockCodes.value.filter((c) => c !== code);
    updateStockCodes(stockCodes.value);
  }

  function moveStock(index: number, direction: "up" | "down") {
    const target = direction === "up" ? index - 1 : index + 1;
    if (target < 0 || target >= stockCodes.value.length) return;
    const codes = [...stockCodes.value];
    [codes[index], codes[target]] = [codes[target], codes[index]];
    stockCodes.value = codes;
    updateStockCodes(stockCodes.value);
  }

  return {
    stockCodes,
    quotes,
    sortedQuotes,
    theme,
    opacity,
    rowSpacing,
    loading,
    errorMsg,
    setQuotes,
    setError,
    toggleTheme,
    initTheme,
    setOpacity,
    setRowSpacing,
    fetchMinuteData,
    fetchKlineData,
    updateStockCodes,
    searchStocks,
    loadStockCodes,
    addStock,
    removeStock,
    moveStock,
  };
});
