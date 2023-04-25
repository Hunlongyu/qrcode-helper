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
        .shape_color(Shape::Square, _color)
        .background_color(_bg_color)
        .to_str(&qrcode);
    return _svg;
}

#[tauri::command]
fn _save_svg(data: &str, _color: &str, _bg_color: &str, path: &str) -> bool {
    let qrcode = QRBuilder::new(data).build().unwrap();
    let _svg = SvgBuilder::default()
        .shape_color(Shape::Square, _color)
        .background_color(_bg_color)
        .to_file(&qrcode, path);
    match _svg {
        Ok(_) => true,
        Err(_) => false
    }
}


fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![_generate, _save_svg])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
