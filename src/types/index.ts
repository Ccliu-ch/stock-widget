export interface QuoteData {
  code: string;
  name: string;
  price: number;
  pre_close: number;
  open: number;
  volume: number;
  change: number;
  change_pct: number;
  high: number;
  low: number;
  timestamp: string;
}

export interface MinutePoint {
  time: string;
  price: number;
  volume: number;
}

export interface MinuteData {
  points: MinutePoint[];
  pre_close: number;
}

export interface KlineBar {
  date: string;
  open: number;
  close: number;
  high: number;
  low: number;
  volume: number;
}

export type ViewType = "list" | "minute" | "kline" | "manage";
export type Theme = "dark" | "light";
export type KlinePeriod = "day" | "week" | "month";
export type FuquanType = "qfq" | "hfq" | "";

export interface SearchResult {
  code: string;
  name: string;
  market: string;
}

export interface StockConfig {
  code: string;
  name: string;
}
