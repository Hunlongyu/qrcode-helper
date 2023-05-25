#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use fast_qr::{
    convert::{image::ImageBuilder, svg::SvgBuilder, Builder, Shape},
    QRBuilder,
};

use clippers;

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
        Err(_) => false,
    }
}

#[tauri::command]
fn _save_png(data: &str, _color: &str, _bg_color: &str, path: &str, size: u32) -> bool {
    let _svg = QRBuilder::new(data).build().unwrap();
    let _image = ImageBuilder::default()
        .shape_color(Shape::Square, _color)
        .background_color(_bg_color)
        .fit_height(size)
        .to_file(&_svg, path);
    match _image {
        Ok(_) => return true,
        Err(_) => return false,
    }
}

#[tauri::command]
fn _copy_to_clipboard(data: &str, _color: &str, _bg_color: &str, size: u32) -> bool {
    let _svg = QRBuilder::new(data).build().unwrap();
    let _image = ImageBuilder::default()
        .shape_color(Shape::Square, _color)
        .background_color(_bg_color)
        .fit_height(size)
        .to_pixmap(&_svg);
    let mut buf = _image.encode_png().unwrap();
    let dimg = image::load_from_memory_with_format(&mut buf, image::ImageFormat::Png).unwrap();
    let rgba_img = dimg.to_rgba8();
    let mut cp = clippers::Clipboard::get();
    cp.write_image(rgba_img.width(), rgba_img.height(), &rgba_img.as_raw())
        .unwrap();
    return true;
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            _generate,
            _save_svg,
            _save_png,
            _copy_to_clipboard
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
