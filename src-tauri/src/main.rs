// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use fast_qr::{
    convert::{Builder, Shape, svg::SvgBuilder},
    QRBuilder
};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn _generate(data: &str, _color: &str, _bg_color: &str) -> String {
    let qrcode = QRBuilder::new(data).build().unwrap();

    let _svg = SvgBuilder::default()
        .shape_color(Shape::Square, [0, 0, 0, 255])
        .background_color("#FFFF0077")
        .to_str(&qrcode);

    return _svg;
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![_generate])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
