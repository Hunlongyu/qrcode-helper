#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tokio;
mod generate;
mod parse;
mod settings;
use tauri::{
    CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem,
};

#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
}

// 生成 svg 字符串
#[tauri::command]
async fn _generate(data: String, color: String, bg_color: String) -> String {
    let handle = tokio::task::spawn(generate::generate(data, color, bg_color));
    let result = handle.await.unwrap();
    return result;
}

// 保存 svg 文件到本地
#[tauri::command]
async fn _save_svg(data: String, color: String, bg_color: String, path: String) -> bool {
    let handle = tokio::task::spawn(generate::save_svg(data, color, bg_color, path));
    let result = handle.await.unwrap();
    return result;
}

// 保存 png 文件到本地
#[tauri::command]
async fn _save_png(data: String, color: String, bg_color: String, path: String, size: u32) -> bool {
    let handle = tokio::task::spawn(generate::save_png(data, color, bg_color, path, size));
    let result = handle.await.unwrap();
    return result;
}

// 复制 png 到剪贴板
#[tauri::command]
async fn _copy_to_clipboard(data: String, color: String, bg_color: String, size: u32) -> bool {
    let handle = tokio::task::spawn(generate::copy_to_clipboard(data, color, bg_color, size));
    let result = handle.await.unwrap();
    return result;
}

// 解析指定路径二维码图片
#[tauri::command]
async fn _parse_images_with_paths(path: String, lib: Option<String>) -> String {
    let handle = tokio::task::spawn(parse::parse_images_with_paths(path, lib));
    let result = handle.await.unwrap();
    return result;
}

// 设置软件是否开机自启
#[tauri::command]
fn _toggle_auto_launch(enable: bool) -> bool {
    let result = settings::auto_launch(enable);
    return result;
}

fn main() {
    let quit = CustomMenuItem::new("quit".to_string(), "退出");
    let hide = CustomMenuItem::new("hide".to_string(), "隐藏");
    let scan = CustomMenuItem::new("scan".to_string(), "识别屏幕二维码");
    let screen_capture = CustomMenuItem::new("screenCapture".to_string(), "鼠标截图识别二维码");
    let tray_menu = SystemTrayMenu::new()
        .add_item(screen_capture)
        .add_item(scan)
        .add_item(hide)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);

    tauri::Builder::default()
        .system_tray(SystemTray::new().with_menu(tray_menu))
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::MenuItemClick { id, .. } => {
                let item_handle = app.tray_handle().get_item(&id);
                match id.as_str() {
                    "screenCapture" => {
                        let app_clone = app.clone();
                        let runtime = tokio::runtime::Runtime::new().unwrap();
                        runtime.block_on(async {
                            let res = parse::screen_capture().await;
                            let results_str = serde_json::to_string(&res).unwrap();
                            println!("screenCapture: {}", results_str);

                            let _ = app_clone
                                .emit_all(
                                    "scan_screen",
                                    Payload {
                                        message: results_str,
                                    },
                                )
                                .unwrap();
                            drop(app_clone);
                        });
                    }
                    "scan" => {
                        let app_clone = app.clone();
                        std::thread::spawn(move || {
                            let res = parse::scan_screen();
                            println!("scan _  {}", res);

                            let _ = app_clone
                                .emit_all("scan_screen", Payload { message: res })
                                .unwrap();
                            drop(app_clone);
                        });
                    }
                    "hide" => {
                        let window = app.get_window("main").unwrap();
                        let is_visible = window.is_visible().unwrap();
                        if is_visible {
                            window.hide().unwrap();
                            item_handle.set_title("显示").unwrap();
                        } else {
                            window.show().unwrap();
                            item_handle.set_title("隐藏").unwrap();
                        }
                    }
                    "quit" => {
                        let _ = app.get_window("main").unwrap().close();
                    }
                    _ => {}
                }
            }
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![
            _generate,
            _save_svg,
            _save_png,
            _copy_to_clipboard,
            _parse_images_with_paths,
            _toggle_auto_launch
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
