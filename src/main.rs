#[macro_use]
extern crate rocket;

use std::time::Duration;

use rocket::serde::{json::Json, Deserialize};
use serde_json::{Map, Value};

use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use rocket::response::status::BadRequest;
use rand::Rng;

#[derive(Deserialize)]
struct OcrImage {
    api_key: String,
    image_data: String,
}

fn make_request_body(data: String) -> String {
    let mut image = Map::new();
    image.insert("content".into(), Value::String(data));
    let image = Value::Object(image);

    let mut feature = Map::new();
    feature.insert("type".into(), Value::String("TEXT_DETECTION".into()));
    let features = Value::Array(vec![Value::Object(feature)]);

    let mut request = Map::new();
    request.insert("image".into(), image);
    request.insert("features".into(), features);

    let requests = Value::Array(vec![Value::Object(request)]);

    let mut api_request = Map::new();
    api_request.insert("requests".into(), requests);
    Value::Object(api_request).to_string()
}

async fn anomaly() {
    let random = rand::thread_rng().gen_range(0..20);
    match random {
        20 => { tokio::time::sleep(Duration::from_secs(300)).await },
        18..=19 => { tokio::time::sleep(Duration::from_secs(5)).await },
        _ => {}
    }
}

#[post(
    "/api/v1/ocr_with_anomaly",
    format = "application/json",
    data = "<image>"
)]
async fn ocr_with_anomaly(mut image: Json<OcrImage>) -> Result<String, BadRequest<String>> {
    let mut data = String::new();
    std::mem::swap(&mut data, &mut image.image_data);
    let google_cloud_req = make_request_body(data);

    anomaly().await;

    let client = reqwest::Client::new();

    let res = client
        .post("https://vision.googleapis.com/v1/images:annotate")
        .header(AUTHORIZATION, format!("Bearer {}", &image.api_key.trim()))
        .header(CONTENT_TYPE, "application/json; charset=utf-8")
        .body(google_cloud_req)
        .send()
        .await
        .map_err(|e| BadRequest(Some(e.to_string())))?;

    res.text().await.map_err(|e| BadRequest(Some(e.to_string())))
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![ocr_with_anomaly])
}
