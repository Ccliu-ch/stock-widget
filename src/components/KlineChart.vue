<script setup lang="ts">
import { ref, onMounted, onUnmounted, watch, nextTick } from "vue";
import {
  createChart,
  ColorType,
  CrosshairMode,
  type IChartApi,
  type ISeriesApi,
} from "lightweight-charts";
import { useStockStore } from "../stores/stock";
import type { KlineBar, KlinePeriod, FuquanType, QuoteData } from "../types";

const props = defineProps<{ code: string }>();
const emit = defineEmits<{
  (e: "back"): void;
  (e: "switch-minute"): void;
}>();

const store = useStockStore();
const chartContainer = ref<HTMLElement | null>(null);
const loading = ref(true);
const errorMsg = ref("");
const stockInfo = ref<QuoteData | null>(null);

const currentPeriod = ref<KlinePeriod>("day");
const currentFq = ref<FuquanType>("qfq");
const ma5Value = ref<number | null>(null);
const ma20Value = ref<number | null>(null);

let chart: IChartApi | null = null;
let candleSeries: ISeriesApi<"Candlestick"> | null = null;
let volumeSeries: ISeriesApi<"Histogram"> | null = null;
let ma5Series: ISeriesApi<"Line"> | null = null;
let ma20Series: ISeriesApi<"Line"> | null = null;
let resizeObserver: ResizeObserver | null = null;
let klineBars: KlineBar[] = [];

function getThemeColors() {
  const style = getComputedStyle(document.documentElement);
  return {
    textColor: style.getPropertyValue("--muted").trim(),
    gridColor: style.getPropertyValue("--border").trim() || "rgba(255,255,255,0.03)",
    borderColor: style.getPropertyValue("--border2").trim() || "rgba(255,255,255,0.06)",
    up: style.getPropertyValue("--up").trim(),
    down: style.getPropertyValue("--down").trim(),
    ink: style.getPropertyValue("--ink").trim(),
  };
}

function calculateMA(data: KlineBar[], period: number) {
  const result: { time: string; value: number }[] = [];
  for (let i = period - 1; i < data.length; i++) {
    let sum = 0;
    for (let j = 0; j < period; j++) {
      sum += data[i - j].close;
    }
    result.push({ time: data[i].date, value: sum / period });
  }
  return result;
}

function initChart() {
  if (!chartContainer.value) return;

  const colors = getThemeColors();

  chart = createChart(chartContainer.value, {
    layout: {
      background: { type: ColorType.Solid, color: "transparent" },
      textColor: colors.textColor,
      fontFamily: "var(--font-sans)",
      fontSize: 10,
    },
    grid: {
      vertLines: { color: colors.gridColor },
      horzLines: { color: colors.gridColor },
    },
    crosshair: {
      mode: CrosshairMode.Normal,
      horzLine: { labelBackgroundColor: colors.up },
      vertLine: { labelBackgroundColor: colors.up },
    },
    rightPriceScale: {
      borderColor: colors.borderColor,
      scaleMargins: { top: 0.05, bottom: 0.25 },
    },
    timeScale: {
      borderColor: colors.borderColor,
      timeVisible: false,
    },
    width: chartContainer.value.clientWidth,
    height: chartContainer.value.clientHeight,
  });

  candleSeries = chart.addCandlestickSeries({
    upColor: colors.up,
    downColor: colors.down,
    borderUpColor: colors.up,
    borderDownColor: colors.down,
    wickUpColor: colors.up,
    wickDownColor: colors.down,
  });

  volumeSeries = chart.addHistogramSeries({
    priceFormat: { type: "volume" },
    priceScaleId: "vol",
  });

  chart.priceScale("vol").applyOptions({
    scaleMargins: { top: 0.8, bottom: 0 },
  });

  ma5Series = chart.addLineSeries({
    color: "#F59E0B",
    lineWidth: 1,
    priceLineVisible: false,
    lastValueVisible: false,
  });

  ma20Series = chart.addLineSeries({
    color: "#8B5CF6",
    lineWidth: 1,
    priceLineVisible: false,
    lastValueVisible: false,
  });
}

function updateChart() {
  if (!chart || !candleSeries || !volumeSeries || !ma5Series || !ma20Series || klineBars.length === 0)
    return;

  const colors = getThemeColors();

  candleSeries.applyOptions({
    upColor: colors.up,
    downColor: colors.down,
    borderUpColor: colors.up,
    borderDownColor: colors.down,
    wickUpColor: colors.up,
    wickDownColor: colors.down,
  });

  // Candlestick data
  const candleData = klineBars.map((b) => ({
    time: b.date as any,
    open: b.open,
    high: b.high,
    low: b.low,
    close: b.close,
  }));

  // Volume data
  const volData = klineBars.map((b) => ({
    time: b.date as any,
    value: b.volume,
    color: b.close >= b.open ? colors.up + "59" : colors.down + "59",
  }));

  candleSeries.setData(candleData);
  volumeSeries.setData(volData);

  // MA lines
  const ma5Data = calculateMA(klineBars, 5).map((d) => ({
    time: d.time as any,
    value: d.value,
  }));
  const ma20Data = calculateMA(klineBars, 20).map((d) => ({
    time: d.time as any,
    value: d.value,
  }));

  ma5Series.setData(ma5Data);
  ma20Series.setData(ma20Data);

  // Update MA display values
  if (klineBars.length >= 5) {
    const last5 = klineBars.slice(-5);
    ma5Value.value = last5.reduce((sum, b) => sum + b.close, 0) / 5;
  }
  if (klineBars.length >= 20) {
    const last20 = klineBars.slice(-20);
    ma20Value.value = last20.reduce((sum, b) => sum + b.close, 0) / 20;
  }

  chart.timeScale().fitContent();
}

async function loadData() {
  loading.value = true;
  errorMsg.value = "";
  try {
    const data = await store.fetchKlineData(
      props.code,
      currentPeriod.value,
      120,
      currentFq.value
    );
    klineBars = data;
    await nextTick();
    if (!chart) {
      initChart();
    }
    updateChart();
  } catch (e: any) {
    errorMsg.value = typeof e === "string" ? e : (e.message || "获取K线数据失败");
  } finally {
    loading.value = false;
  }
}

function switchPeriod(period: KlinePeriod) {
  if (currentPeriod.value === period) return;
  currentPeriod.value = period;
  loadData();
}

function switchFq(fq: FuquanType) {
  if (currentFq.value === fq) return;
  currentFq.value = fq;
  loadData();
}

// Watch theme changes to recreate chart
watch(() => store.theme, async () => {
  if (chart) {
    chart.remove();
    chart = null;
    candleSeries = null;
    volumeSeries = null;
    ma5Series = null;
    ma20Series = null;
  }
  await nextTick();
  initChart();
  updateChart();
});

// Find stock info from quotes
watch(
  () => store.quotes,
  (quotes) => {
    const found = quotes.find((q) => q.code === props.code);
    if (found) stockInfo.value = found;
  },
  { immediate: true, deep: true }
);

onMounted(() => {
  loadData();

  resizeObserver = new ResizeObserver(() => {
    if (chart && chartContainer.value) {
      chart.applyOptions({
        width: chartContainer.value.clientWidth,
        height: chartContainer.value.clientHeight,
      });
    }
  });

  if (chartContainer.value) {
    resizeObserver.observe(chartContainer.value);
  }
});

onUnmounted(() => {
  resizeObserver?.disconnect();
  if (chart) {
    chart.remove();
    chart = null;
  }
});
</script>

<template>
  <div class="kline-view">
    <div class="detail-header">
      <button class="back-btn" @click="emit('back')">
        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round">
          <path d="m15 18-6-6 6-6"/>
        </svg>
      </button>
      <div class="detail-title">
        <div class="dt-name">{{ stockInfo?.name || code }}</div>
        <div class="dt-code">{{ code }}</div>
      </div>
      <button class="switch-btn" @click="emit('switch-minute')">分时</button>
    </div>

    <div class="kline-tabs">
      <div
        class="kline-tab"
        :class="{ active: currentPeriod === 'day' }"
        @click="switchPeriod('day')"
      >日K</div>
      <div
        class="kline-tab"
        :class="{ active: currentPeriod === 'week' }"
        @click="switchPeriod('week')"
      >周K</div>
      <div
        class="kline-tab"
        :class="{ active: currentPeriod === 'month' }"
        @click="switchPeriod('month')"
      >月K</div>
    </div>

    <div class="kline-settings">
      <select class="fq-select" v-model="currentFq" @change="loadData">
        <option value="qfq">前复权</option>
        <option value="hfq">后复权</option>
        <option value="">不复权</option>
      </select>
      <span class="ma-label">MA5: <span class="ma-val" style="color: #F59E0B">{{ ma5Value?.toFixed(2) || "--" }}</span></span>
      <span class="ma-label">MA20: <span class="ma-val" style="color: #8B5CF6">{{ ma20Value?.toFixed(2) || "--" }}</span></span>
    </div>

    <div class="chart-area">
      <div v-if="loading" class="loading">加载中...</div>
      <div v-else-if="errorMsg" class="error">{{ errorMsg }}</div>
      <div ref="chartContainer" class="chart-container"></div>
    </div>

    <div class="stats" v-if="stockInfo">
      <div class="stat"><span class="label">开盘</span><span class="value">{{ stockInfo.open.toFixed(2) }}</span></div>
      <div class="stat"><span class="label">最高</span><span class="value up">{{ stockInfo.high.toFixed(2) }}</span></div>
      <div class="stat"><span class="label">最低</span><span class="value down">{{ stockInfo.low.toFixed(2) }}</span></div>
      <div class="stat"><span class="label">昨收</span><span class="value">{{ stockInfo.pre_close.toFixed(2) }}</span></div>
    </div>
  </div>
</template>

<style scoped>
.kline-view {
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

.switch-btn {
  padding: 4px 12px;
  font-size: 11px;
  font-weight: 600;
  color: var(--accent);
  background: var(--surface2);
  border-radius: 4px;
  transition: all 0.15s;
}

.switch-btn:hover {
  background: var(--accent);
  color: #fff;
}

.kline-tabs {
  display: flex;
  padding: 0 16px;
  gap: 4px;
  border-bottom: 1px solid var(--border);
}

.kline-tab {
  padding: 8px 14px;
  font-size: 12px;
  font-weight: 600;
  color: var(--muted);
  cursor: pointer;
  border-bottom: 2px solid transparent;
  transition: all 0.15s;
}

.kline-tab:hover {
  color: var(--ink);
}

.kline-tab.active {
  color: var(--accent);
  border-bottom-color: var(--accent);
}

.kline-settings {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 6px 16px;
}

.fq-select {
  background: var(--surface2);
  border: 1px solid var(--border2);
  color: var(--muted);
  font-size: 11px;
  padding: 3px 8px;
  border-radius: 4px;
  cursor: pointer;
  outline: none;
  font-family: var(--font-sans);
}

.ma-label {
  font-size: 10px;
  color: var(--muted2);
}

.ma-val {
  font-family: var(--font-mono);
}

.chart-area {
  flex: 1;
  position: relative;
  padding: 0 8px;
  min-height: 0;
}

.chart-container {
  width: 100%;
  height: 100%;
}

.loading,
.error {
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  color: var(--muted);
  font-size: 12px;
}

.stats {
  display: flex;
  padding: 8px 16px 12px;
  gap: 0;
  border-top: 1px solid var(--border);
}

.stat {
  flex: 1;
  text-align: center;
}

.stat .label {
  display: block;
  font-size: 10px;
  color: var(--muted);
  margin-bottom: 2px;
}

.stat .value {
  font-size: 12px;
  font-weight: 600;
  font-family: var(--font-mono);
  color: var(--ink);
}
</style>
