#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tokio;
mod generate;
mod parse;

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

// 解析指定二维码图片
#[tauri::command]
async fn _parse_image(path: String, lib: String) -> String {
    let handle = tokio::task::spawn(parse::parse_image(path, lib));
    let result = handle.await.unwrap();
    return result;
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            _generate,
            _save_svg,
            _save_png,
            _copy_to_clipboard,
            _parse_image,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
