mod stock_api;

use std::sync::{Arc, Mutex};
use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Emitter, Manager, State,
};
use tokio::time::{interval, Duration};

struct AppState {
    stock_codes: Arc<Mutex<Vec<String>>>,
}

#[tauri::command]
async fn fetch_minute_data(code: String) -> Result<serde_json::Value, String> {
    let (points, pre_close) = stock_api::fetch_minute(&code).await?;
    serde_json::to_value(serde_json::json!({
        "points": points,
        "pre_close": pre_close,
    }))
    .map_err(|e| e.to_string())
}

#[tauri::command]
async fn fetch_kline_data(
    code: String,
    period: String,
    count: u32,
    fq: String,
) -> Result<Vec<stock_api::KlineBar>, String> {
    stock_api::fetch_kline(&code, &period, count, &fq).await
}

#[tauri::command]
fn update_stock_codes(codes: Vec<String>, state: State<'_, AppState>) {
    if let Ok(mut guard) = state.stock_codes.lock() {
        *guard = codes;
    }
}

#[tauri::command]
fn quit_app(app: tauri::AppHandle) {
    app.exit(0);
}

#[tauri::command]
fn show_window(app: tauri::AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.show();
        let _ = window.set_focus();
    }
}

#[tauri::command]
async fn search_stocks(keyword: String) -> Result<Vec<stock_api::SearchResult>, String> {
    stock_api::search_stocks(&keyword).await
}

#[tauri::command]
fn get_stock_codes(state: State<'_, AppState>) -> Vec<String> {
    state.stock_codes.lock().map(|g| g.clone()).unwrap_or_default()
}

fn start_polling(app: tauri::AppHandle, state: Arc<Mutex<Vec<String>>>) {
    tauri::async_runtime::spawn(async move {
        let mut tick = interval(Duration::from_secs(2));
        tick.tick().await; // skip first immediate tick

        loop {
            tick.tick().await;

            let codes = {
                let guard = state.lock();
                match guard {
                    Ok(g) => g.clone(),
                    Err(_) => continue,
                }
            };

            if codes.is_empty() {
                continue;
            }

            // 交易时间外降频到 30 秒
            let sleep_secs = if stock_api::is_trading_time() {
                2
            } else {
                30
            };

            match stock_api::fetch_quotes(&codes).await {
                Ok(quotes) => {
                    let _ = app.emit("quote-update", &quotes);
                }
                Err(e) => {
                    eprintln!("获取行情失败: {}", e);
                    let _ = app.emit("quote-error", &e);
                }
            }

            // 动态调整间隔
            if sleep_secs != 2 {
                tokio::time::sleep(Duration::from_secs(sleep_secs - 2)).await;
            }
        }
    });
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .manage(AppState {
            stock_codes: Arc::new(Mutex::new(vec![
                "sh600519".to_string(),
                "sz000001".to_string(),
                "sz300750".to_string(),
                "sz002594".to_string(),
                "sh601318".to_string(),
                "sh600036".to_string(),
                "sz000858".to_string(),
                "sh601012".to_string(),
                "sz002415".to_string(),
                "sh603259".to_string(),
                "sh600276".to_string(),
                "sz300760".to_string(),
                "sz000568".to_string(),
                "sz000725".to_string(),
                "sz300059".to_string(),
                "sh688981".to_string(),
                "sz002049".to_string(),
                "sh601633".to_string(),
                "sh600690".to_string(),
                "sh600309".to_string(),
            ])),
        })
        .setup(|app| {
            let state = app.state::<AppState>();
            let codes_arc = state.stock_codes.clone();
            start_polling(app.handle().clone(), codes_arc);

            // 系统托盘
            let show_item = MenuItem::with_id(app, "show", "显示窗口", true, None::<&str>)?;
            let quit_item = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show_item, &quit_item])?;

            let _tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .tooltip("加油努力")
                .menu(&menu)
                .show_menu_on_left_click(false)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "show" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    "quit" => {
                        app.exit(0);
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            if window.is_visible().unwrap_or(false) {
                                let _ = window.hide();
                            } else {
                                let _ = window.show();
                                let _ = window.set_focus();
                            }
                        }
                    }
                })
                .build(app)?;

            Ok(())
        })
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                // 关闭时隐藏到托盘而非退出
                let _ = window.hide();
                api.prevent_close();
            }
        })
        .invoke_handler(tauri::generate_handler![
            fetch_minute_data,
            fetch_kline_data,
            update_stock_codes,
            search_stocks,
            get_stock_codes,
            quit_app,
            show_window,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
