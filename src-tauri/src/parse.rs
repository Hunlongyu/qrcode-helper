use rqrr::PreparedImage;
use rxing;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashSet;
use tokio;

// 指定解析库来解析指定二维码图片
pub async fn parse_image(path: String, lib: String) -> String {
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
        let rqrr_result = rqrr_handle.await.unwrap();
        let rqrr_json: Value = serde_json::from_str(&rqrr_result).unwrap();
        let rxing_path = path.clone();
        let rxing_handle = tokio::task::spawn(parse_image_rxing(rxing_path));
        let rxing_result = rxing_handle.await.unwrap();
        let rxing_json: Value = serde_json::from_str(&rxing_result).unwrap();

        let arr1 = rqrr_json.as_array().unwrap();
        let arr2 = rxing_json.as_array().unwrap();

        let mut arr_set = HashSet::new();

        for item in arr1 {
            arr_set.insert(item.to_string());
        }

        for item in arr2 {
            arr_set.insert(item.to_string());
        }

        let arr: Vec<Value> = arr_set
            .into_iter()
            .map(|item| serde_json::from_str(&item).unwrap())
            .collect();
        let result = json!(arr).to_string();
        return result;
    }
    let res = json!([]).to_string();
    return res;
}

// 使用 【rqrr】 解析指定二维码图片
async fn parse_image_rqrr(path: String) -> String {
    let img = image::open(path).unwrap().to_luma8();
    let mut img = PreparedImage::prepare(img);
    let grids = img.detect_grids();
    let mut results = Vec::new();
    #[derive(Serialize, Deserialize)]
    struct Result {
        result: bool,
        content: String,
    }
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
    let str_results = serde_json::to_string(&results).unwrap();
    return str_results;
}

// 使用 【rxing】 解析指定二维码图片
async fn parse_image_rxing(path: String) -> String {
    let grids = rxing::helpers::detect_multiple_in_file(&path).expect("decodes");
    #[derive(Serialize, Deserialize)]
    struct Result {
        result: bool,
        content: String,
    }
    let mut results = Vec::new();
    for grid in grids {
        let ctx = grid.getText();
        let r = Result {
            result: true,
            content: ctx.to_string(),
        };
        results.push(r);
    }
    let str_results = serde_json::to_string(&results).unwrap();
    return str_results;
}
