#[macro_use]
extern crate rocket;

use rocket::serde::{json::Json, Deserialize};
use serde_json::{Map, Value};

use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use rocket::response::status::BadRequest;

#[derive(Deserialize)]
struct OcrImage {
    api_key: String,
    image_data: String,
}

fn make_request_body(data: &String) -> String {
    let mut image = Map::new();
    image.insert("content".into(), Value::String(data.clone()));
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

#[post(
    "/api/v1/ocr_with_anomaly",
    format = "application/json",
    data = "<image>"
)]
async fn ocr_with_anomaly(image: Json<OcrImage>) -> Result<String, BadRequest<String>> {
    let google_cloud_req = make_request_body(&image.image_data);

    let client = reqwest::Client::new();
    dbg!(&image.api_key);
    // dbg!(&image.image_data);
    let res = dbg!(client
        .post("https://vision.googleapis.com/v1/images:annotate")
        .header(AUTHORIZATION, format!("Bearer {}", &image.api_key.trim()))
        .header(CONTENT_TYPE, "application/json; charset=utf-8")
        .body(google_cloud_req)
        .send()
        .await
        .map_err(|e| BadRequest(Some(e.to_string()))))?;

    dbg!(res.text().await).map_err(|e| BadRequest(Some(e.to_string())))
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![ocr_with_anomaly])
}
