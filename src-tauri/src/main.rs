#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use fast_qr::{
    convert::{Builder, Shape, svg::SvgBuilder},
    QRBuilder
};
use resvg::{*, usvg::TreeParsing};

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

fn save_svg_as_png( svg_string: &str, output_path: &str, size: u32) -> Result<(), Box<dyn std::error::Error>> {
    let tree = resvg::usvg::Tree::from_str(svg_string, &resvg::usvg::Options::default()).map_err(|err| format!("svg 解析错误：{}", err))?;
    let mut pixmap = tiny_skia::Pixmap::new(size, size).unwrap();
    resvg::render(&tree, FitTo::Size(size, size), tiny_skia::Transform::default(), pixmap.as_mut()).unwrap();
    pixmap.save_png(output_path).unwrap();
    Ok(())
}

#[tauri::command]
fn _save_png(data: &str, _color: &str, _bg_color: &str, path: &str, size: u32) -> bool {
    let _svg = _generate(data, _color, _bg_color);
    let is_ok = save_svg_as_png(&_svg, path, size);
    match is_ok {
        Ok(_) => return true,
        Err(_) => return false
    }
}


fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![_generate, _save_svg, _save_png])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
