use clippers;
use fast_qr::{
    convert::{image::ImageBuilder, svg::SvgBuilder, Builder, Shape},
    QRBuilder,
};

// 生成 svg 字符串
pub async fn generate(data: String, color: String, bg_color: String) -> String {
    let qrcode = QRBuilder::new(data).build().unwrap();
    let _svg = SvgBuilder::default()
        .shape_color(Shape::Square, color)
        .background_color(bg_color)
        .to_str(&qrcode);
    return _svg;
}

// 保存 svg 文件到本地
pub async fn save_svg(data: String, color: String, bg_color: String, path: String) -> bool {
    let qrcode = QRBuilder::new(data).build().unwrap();
    let _svg = SvgBuilder::default()
        .shape_color(Shape::Square, color)
        .background_color(bg_color)
        .to_file(&qrcode, &path);
    match _svg {
        Ok(_) => true,
        Err(_) => false,
    }
}

// 保存 png 文件到本地
pub async fn save_png(
    data: String,
    color: String,
    bg_color: String,
    path: String,
    size: u32,
) -> bool {
    let _svg = QRBuilder::new(data).build().unwrap();
    let _image = ImageBuilder::default()
        .shape_color(Shape::Square, color)
        .background_color(bg_color)
        .fit_height(size)
        .to_file(&_svg, &path);
    match _image {
        Ok(_) => return true,
        Err(_) => return false,
    }
}

// 复制 png 到剪贴板
pub async fn copy_to_clipboard(data: String, color: String, bg_color: String, size: u32) -> bool {
    let _svg = QRBuilder::new(data).build().unwrap();
    let _image = ImageBuilder::default()
        .shape_color(Shape::Square, color)
        .background_color(bg_color)
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
