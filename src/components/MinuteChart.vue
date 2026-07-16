<script setup lang="ts">
import { ref, onMounted, onUnmounted, watch, nextTick } from "vue";
import { createChart, ColorType, CrosshairMode, LineStyle, type IChartApi, type ISeriesApi, type UTCTimestamp } from "lightweight-charts";
import { useStockStore } from "../stores/stock";
import type { MinuteData, QuoteData } from "../types";

const props = defineProps<{ code: string }>();
const emit = defineEmits<{
  (e: "back"): void;
  (e: "switch-kline"): void;
}>();

const store = useStockStore();
const chartContainer = ref<HTMLElement | null>(null);
const loading = ref(true);
const errorMsg = ref("");
const stockInfo = ref<QuoteData | null>(null);

const minuteData = ref<MinuteData | null>(null);
let chart: IChartApi | null = null;
let areaSeries: ISeriesApi<"Area"> | null = null;
let volumeSeries: ISeriesApi<"Histogram"> | null = null;
let resizeObserver: ResizeObserver | null = null;

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

function timeToTimestamp(timeStr: string): UTCTimestamp {
  // "0930" -> today's date at 09:30 as UTC timestamp (seconds)
  const now = new Date();
  const h = parseInt(timeStr.substring(0, 2));
  const m = parseInt(timeStr.substring(2, 4));
  // Use a fixed date to avoid timezone issues; use current date
  const date = new Date(now.getFullYear(), now.getMonth(), now.getDate(), h, m, 0);
  return Math.floor(date.getTime() / 1000) as UTCTimestamp;
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
      timeVisible: true,
      secondsVisible: false,
    },
    width: chartContainer.value.clientWidth,
    height: chartContainer.value.clientHeight,
  });

  areaSeries = chart.addAreaSeries({
    lineColor: colors.up,
    topColor: colors.up + "26",
    bottomColor: colors.up + "00",
    lineWidth: 2,
    priceLineVisible: false,
  });

  volumeSeries = chart.addHistogramSeries({
    priceFormat: { type: "volume" },
    priceScaleId: "vol",
  });

  chart.priceScale("vol").applyOptions({
    scaleMargins: { top: 0.8, bottom: 0 },
  });
}

function updateChart() {
  if (!chart || !areaSeries || !volumeSeries || !minuteData.value) return;

  const colors = getThemeColors();
  const preClose = minuteData.value.pre_close;
  const points = minuteData.value.points;

  if (points.length === 0) return;

  const isUp = points[points.length - 1].price >= preClose;
  const lineColor = isUp ? colors.up : colors.down;

  areaSeries.applyOptions({
    lineColor: lineColor,
    topColor: lineColor + "26",
    bottomColor: lineColor + "00",
  });

  // Price data
  const priceData = points.map((p) => ({
    time: timeToTimestamp(p.time),
    value: p.price,
  }));

  // Volume data with color
  const volData = points.map((p) => ({
    time: timeToTimestamp(p.time),
    value: p.volume,
    color: p.price >= preClose ? colors.up + "66" : colors.down + "66",
  }));

  areaSeries.setData(priceData);
  volumeSeries.setData(volData);

  // Pre-close price line
  if (preClose > 0) {
    areaSeries.createPriceLine({
      price: preClose,
      color: colors.borderColor,
      lineWidth: 1,
      lineStyle: LineStyle.Dashed,
      axisLabelVisible: true,
      title: "昨收",
    });
  }

  chart.timeScale().fitContent();
}

async function loadData() {
  loading.value = true;
  errorMsg.value = "";
  try {
    const data = await store.fetchMinuteData(props.code);
    minuteData.value = data;
    await nextTick();
    if (!chart) {
      initChart();
    }
    updateChart();
  } catch (e: any) {
    errorMsg.value = typeof e === "string" ? e : (e.message || "获取分时数据失败");
  } finally {
    loading.value = false;
  }
}

// Watch theme changes to recreate chart
watch(() => store.theme, async () => {
  if (chart) {
    chart.remove();
    chart = null;
    areaSeries = null;
    volumeSeries = null;
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
  <div class="minute-view">
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
      <button class="switch-btn" @click="emit('switch-kline')">K线</button>
    </div>

    <div class="price-summary" v-if="stockInfo">
      <span class="current" :class="stockInfo.change >= 0 ? 'up' : 'down'">
        {{ stockInfo.price.toFixed(2) }}
      </span>
      <span class="change" :class="stockInfo.change >= 0 ? 'up' : 'down'">
        {{ stockInfo.change >= 0 ? "+" : "" }}{{ stockInfo.change.toFixed(2) }}
      </span>
      <span class="change-pct" :class="stockInfo.change >= 0 ? 'up-bg' : 'down-bg'">
        {{ stockInfo.change >= 0 ? "+" : "" }}{{ stockInfo.change_pct.toFixed(2) }}%
      </span>
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
.minute-view {
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

.price-summary {
  padding: 12px 16px;
  display: flex;
  align-items: baseline;
  gap: 10px;
}

.current {
  font-size: 28px;
  font-weight: 800;
  font-family: var(--font-mono);
  font-variant-numeric: tabular-nums;
}

.change {
  font-size: 13px;
  font-weight: 700;
  font-family: var(--font-mono);
}

.change-pct {
  font-size: 12px;
  font-weight: 600;
  padding: 2px 6px;
  border-radius: 4px;
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
