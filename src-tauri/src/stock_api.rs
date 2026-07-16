use encoding_rs::GBK;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

const QUOTE_URL: &str = "http://qt.gtimg.cn/q=";
const MINUTE_URL: &str = "https://web.ifzq.gtimg.cn/appstock/app/minute/query?code=";
const KLINE_URL: &str = "https://web.ifzq.gtimg.cn/appstock/app/fqkline/get?param=";

fn build_client() -> Client {
    Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .expect("failed to build HTTP client")
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuoteData {
    pub code: String,
    pub name: String,
    pub price: f64,
    pub pre_close: f64,
    pub open: f64,
    pub volume: f64,
    pub change: f64,
    pub change_pct: f64,
    pub high: f64,
    pub low: f64,
    pub timestamp: String,
}

/// 批量获取实时行情 (GBK 编码接口)
pub async fn fetch_quotes(codes: &[String]) -> Result<Vec<QuoteData>, String> {
    let client = build_client();
    let url = format!("{}{}", QUOTE_URL, codes.join(","));

    let resp = client
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("请求失败: {}", e))?;

    let bytes = resp
        .bytes()
        .await
        .map_err(|e| format!("读取响应失败: {}", e))?;

    // GBK 解码
    let (text, _, _) = GBK.decode(&bytes);

    let mut result = Vec::new();

    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || !line.starts_with("v_") {
            continue;
        }

        // 提取引号内的内容
        let start = match line.find('"') {
            Some(s) => s + 1,
            None => continue,
        };
        let end = match line.rfind('"') {
            Some(e) => e,
            None => continue,
        };
        if start >= end {
            continue;
        }

        let content = &line[start..end];
        let fields: Vec<&str> = content.split('~').collect();
        if fields.len() < 33 {
            continue;
        }

        let parse_f64 = |idx: usize| -> f64 {
            fields
                .get(idx)
                .and_then(|s| s.parse::<f64>().ok())
                .unwrap_or(0.0)
        };

        let code = fields.get(2).unwrap_or(&"").to_string();
        let name = fields.get(1).unwrap_or(&"").to_string();
        let price = parse_f64(3);
        let pre_close = parse_f64(4);
        let open = parse_f64(5);
        let volume = parse_f64(6);
        let high = parse_f64(33);
        let low = parse_f64(34);
        let timestamp = fields.get(30).unwrap_or(&"").to_string();

        let change = price - pre_close;
        let change_pct = if pre_close != 0.0 {
            (change / pre_close) * 100.0
        } else {
            0.0
        };

        result.push(QuoteData {
            code,
            name,
            price,
            pre_close,
            open,
            volume,
            change,
            change_pct,
            high,
            low,
            timestamp,
        });
    }

    Ok(result)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinutePoint {
    pub time: String,
    pub price: f64,
    pub volume: f64,
}

/// 获取分时数据 (JSON 接口)
pub async fn fetch_minute(code: &str) -> Result<(Vec<MinutePoint>, f64), String> {
    let client = build_client();
    let url = format!("{}{}", MINUTE_URL, code);

    let resp = client
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("请求失败: {}", e))?;

    let json: serde_json::Value = resp
        .json()
        .await
        .map_err(|e| format!("解析JSON失败: {}", e))?;

    let data_arr = json
        .get("data")
        .and_then(|d| d.get(code))
        .and_then(|d| d.get("data"))
        .and_then(|d| d.get("data"))
        .and_then(|d| d.as_array())
        .ok_or("无法找到分时数据")?;

    // 尝试获取昨收价
    let pre_close = json
        .get("data")
        .and_then(|d| d.get(code))
        .and_then(|d| d.get("data"))
        .and_then(|d| d.get("pre"))
        .and_then(|p| p.as_f64())
        .unwrap_or(0.0);

    let mut points = Vec::new();

    for item in data_arr {
        if let Some(s) = item.as_str() {
            let parts: Vec<&str> = s.split_whitespace().collect();
            if parts.len() >= 2 {
                let time = parts[0].to_string();
                let price = parts[1].parse::<f64>().unwrap_or(0.0);
                let volume = if parts.len() >= 3 {
                    parts[2].parse::<f64>().unwrap_or(0.0)
                } else {
                    0.0
                };
                points.push(MinutePoint { time, price, volume });
            }
        }
    }

    Ok((points, pre_close))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KlineBar {
    pub date: String,
    pub open: f64,
    pub close: f64,
    pub high: f64,
    pub low: f64,
    pub volume: f64,
}

/// 获取 K 线数据 (JSON 接口)
pub async fn fetch_kline(
    code: &str,
    period: &str,
    count: u32,
    fq: &str,
) -> Result<Vec<KlineBar>, String> {
    let client = build_client();
    let param = format!("{},{},,,{},{}", code, period, count, fq);
    let url = format!("{}{}", KLINE_URL, param);

    let resp = client
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("请求失败: {}", e))?;

    let json: serde_json::Value = resp
        .json()
        .await
        .map_err(|e| format!("解析JSON失败: {}", e))?;

    // 根据复权方式选择字段名
    let field_name = match fq {
        "qfq" => format!("{}{}", "qfq", period),
        "hfq" => format!("{}{}", "hfq", period),
        _ => period.to_string(),
    };

    let data_arr = json
        .get("data")
        .and_then(|d| d.get(code))
        .and_then(|d| d.get(&field_name))
        .or_else(|| {
            // 尝试直接用 period 名
            json.get("data")
                .and_then(|d| d.get(code))
                .and_then(|d| d.get(period))
        })
        .and_then(|d| d.as_array())
        .ok_or("无法找到K线数据")?;

    let mut bars = Vec::new();

    for item in data_arr {
        if let Some(arr) = item.as_array() {
            if arr.len() >= 6 {
                let date = arr[0]
                    .as_str()
                    .or_else(|| arr[0].as_str())
                    .unwrap_or("")
                    .to_string();
                let open = arr[1].as_str()
                    .and_then(|s| s.parse::<f64>().ok())
                    .or_else(|| arr[1].as_f64())
                    .unwrap_or(0.0);
                let close = arr[2].as_str()
                    .and_then(|s| s.parse::<f64>().ok())
                    .or_else(|| arr[2].as_f64())
                    .unwrap_or(0.0);
                let high = arr[3].as_str()
                    .and_then(|s| s.parse::<f64>().ok())
                    .or_else(|| arr[3].as_f64())
                    .unwrap_or(0.0);
                let low = arr[4].as_str()
                    .and_then(|s| s.parse::<f64>().ok())
                    .or_else(|| arr[4].as_f64())
                    .unwrap_or(0.0);
                let volume = arr[5].as_str()
                    .and_then(|s| s.parse::<f64>().ok())
                    .or_else(|| arr[5].as_f64())
                    .unwrap_or(0.0);

                bars.push(KlineBar {
                    date,
                    open,
                    close,
                    high,
                    low,
                    volume,
                });
            }
        }
    }

    Ok(bars)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub code: String,
    pub name: String,
    pub market: String,
}

/// 搜索股票 (腾讯智能箱接口, GBK 编码)
pub async fn search_stocks(keyword: &str) -> Result<Vec<SearchResult>, String> {
    if keyword.trim().is_empty() {
        return Ok(Vec::new());
    }

    let client = build_client();
    let url = format!("https://smartbox.gtimg.cn/s3/?q={}&t=all", keyword);

    let resp = client
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("搜索请求失败: {}", e))?;

    let bytes = resp
        .bytes()
        .await
        .map_err(|e| format!("读取响应失败: {}", e))?;

    let (text, _, _) = GBK.decode(&bytes);

    let mut results = Vec::new();

    // 响应格式: v_hint="sh600519~贵州茅台~sh~GP-A^sz000001~平安银行~sz~GP-A^..."
    for line in text.lines() {
        let line = line.trim();
        if !line.starts_with("v_hint") {
            continue;
        }

        let start = match line.find('"') {
            Some(s) => s + 1,
            None => continue,
        };
        let end = match line.rfind('"') {
            Some(e) => e,
            None => continue,
        };
        if start >= end {
            continue;
        }

        let content = &line[start..end];
        // 用 ^ 分割多个结果
        for item in content.split('^') {
            let parts: Vec<&str> = item.split('~').collect();
            // 实际格式: market~code~name~pinyin~type
            if parts.len() >= 5 {
                let market = parts[0].to_string();
                let code_num = parts[1].to_string();
                let name = parts[2].to_string();
                let stock_type = parts[4].to_string();
                let full_code = format!("{}{}", market, code_num);
                // 只保留 A 股 (GP-A / GP-S 等)
                if stock_type.starts_with("GP") && (market == "sh" || market == "sz") {
                    results.push(SearchResult {
                        code: full_code,
                        name,
                        market,
                    });
                }
            }
        }
    }

    Ok(results)
}

/// 判断当前是否为交易时间
pub fn is_trading_time() -> bool {
    use chrono::{Datelike, Local, Timelike};

    let now = Local::now();
    let weekday = now.weekday();
    let hour = now.hour();
    let minute = now.minute();

    // 周末不交易
    if weekday == chrono::Weekday::Sat || weekday == chrono::Weekday::Sun {
        return false;
    }

    let current_minutes = hour * 60 + minute;

    // 9:30 - 11:30
    let morning_start = 9 * 60 + 30;
    let morning_end = 11 * 60 + 30;
    // 13:00 - 15:00
    let afternoon_start = 13 * 60;
    let afternoon_end = 15 * 60;

    (current_minutes >= morning_start && current_minutes <= morning_end)
        || (current_minutes >= afternoon_start && current_minutes <= afternoon_end)
}
