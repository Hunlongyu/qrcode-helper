use async_process::Command;
use image::load_from_memory;
use rqrr::PreparedImage;
use rxing;
use screenshots::Screen;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};
use std::{collections::HashSet, env, vec};

#[derive(Serialize, Deserialize, Clone)]
struct Result {
    result: bool,
    content: String,
}

// 解析多个文件，并可以指定 解析库 来解析二维码图片
pub async fn parse_images_with_paths(paths: String, lib: Option<String>) -> String {
    println!("paths: {}", paths);
    let paths_json: Vec<String> = serde_json::from_str(&paths).unwrap();
    let mut tasks = vec![];
    for path in paths_json {
        let lib_copy = lib.clone();
        tasks.push(tokio::task::spawn(async move {
            parse_image(path, lib_copy).await
        }))
    }
    let mut results: Vec<Result> = vec![];
    for task in tasks {
        let res = task.await.unwrap();
        results.extend(res);
    }
    let results_str = serde_json::to_string(&results).unwrap();
    return results_str;
}

// 指定解析库来解析指定二维码图片
async fn parse_image(path: String, lib: Option<String>) -> Vec<Result> {
    let lib = lib.unwrap_or_else(|| "rxing".to_string());
    if lib == "rqrr".to_string() {
        let p = path.clone();
        let handle = tokio::task::spawn(parse_image_rqrr(p));
        let result = handle.await.unwrap();
        return result;
    }
    if lib == "rxing".to_string() {
        let p = path.clone();
        let handle = tokio::task::spawn(parse_image_rxing(p));
        let result = handle.await.unwrap();
        return result;
    }

    if lib == "all".to_string() {
        let rqrr_path: String = path.clone();
        let rqrr_handle = tokio::task::spawn(parse_image_rqrr(rqrr_path));
        let rqrr_json = rqrr_handle.await.unwrap();
        let rxing_path = path.clone();
        let rxing_handle = tokio::task::spawn(parse_image_rxing(rxing_path));
        let rxing_json = rxing_handle.await.unwrap();

        let mut arr = Vec::new();
        arr.extend_from_slice(&rqrr_json);
        arr.extend_from_slice(&rxing_json);
        let mut unique = Vec::new();
        let mut seen = HashSet::new();
        for result in arr {
            if seen.contains(&result.content) {
                continue;
            }
            seen.insert(result.content.clone());
            unique.push(result);
        }
        return unique;
    }
    let res: Vec<Result> = vec![];
    return res;
}

// 使用 【rqrr】 解析指定二维码图片
async fn parse_image_rqrr(path: String) -> Vec<Result> {
    let img = image::open(path).unwrap().to_luma8();
    let mut img = PreparedImage::prepare(img);
    let grids = img.detect_grids();
    let mut results = Vec::new();
    for grid in grids {
        let ctx = grid.decode();
        match ctx {
            Ok(val) => {
                let (_, content) = val;
                let r = Result {
                    result: true,
                    content: content,
                };
                results.push(r);
            }
            Err(_) => (),
        }
    }
    return results;
}

// 使用 【rxing】 解析指定二维码图片
async fn parse_image_rxing(path: String) -> Vec<Result> {
    let grids = rxing::helpers::detect_multiple_in_file(&path).expect("decodes");
    let mut results = Vec::new();
    for grid in grids {
        let ctx = grid.getText();
        let r = Result {
            result: true,
            content: ctx.to_string(),
        };
        results.push(r);
    }
    return results;
}

// 扫描屏幕中存在的二维码
pub fn scan_screen() -> String {
    let screens = Screen::all().unwrap();
    let mut results = Vec::new();
    for screen in screens {
        let img_arr = screen.capture().unwrap().buffer().to_vec();
        let _img = load_from_memory(&img_arr).unwrap().to_luma8();
        let dia = _img.to_vec();
        let grids_arr = rxing::helpers::detect_multiple_in_luma(dia, _img.width(), _img.height());

        if let Err(_) = grids_arr {
            return serde_json::to_string(&results).unwrap();
        }
        if let Ok(grids) = grids_arr {
            for grid in grids {
                let ctx = grid.getText();
                println!("scan screen: {}", ctx);
                let r = Result {
                    result: true,
                    content: ctx.to_string(),
                };
                results.push(r);
            }
        }
    }
    let results_str = serde_json::to_string(&results).unwrap();
    return results_str;
}

// 鼠标截图保存 temp 目录下，并进行识别
pub async fn screen_capture() -> String {
    let temp_dir = env::temp_dir();
    println!("temp_dir: {:?}", temp_dir);

    let temp_dir_long = std::fs::canonicalize(&temp_dir).unwrap();
    println!("temp_dir_long: {:?}", temp_dir_long);

    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let seconds = timestamp.as_secs();

    let mut file_name = PathBuf::from(temp_dir_long);
    file_name.push("qrcode-helper");
    file_name.push(seconds.to_string());
    file_name.set_extension("png");
    println!("file_name: {:?}", file_name);

    if let Some(parent) = file_name.parent() {
        fs::create_dir_all(parent).expect("Failed to create directories");
    }

    let mut child = Command::new("E:\\qrcode-helper\\src-tauri\\libs\\qcsc.exe")
        .arg(&file_name)
        // .arg("C:\\Users\\hunlongyu\\AppData\\Local\\Temp\\012\\1619784753.png")
        .spawn()
        .unwrap();
    let _ = child.status().await.unwrap();

    return parse_images_with_paths(
        file_name.into_os_string().into_string().unwrap(),
        Some("all".to_string()),
    )
    .await;
}
