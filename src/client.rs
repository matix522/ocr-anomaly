use std::path::PathBuf;
use structopt::StructOpt;

use reqwest::header::CONTENT_TYPE;
use serde_json::{Map, Value};

use std::error::Error;

#[derive(Debug, StructOpt)]
#[structopt(name = "ocr_anomaly", about = "call ocr_anomaly api")]
struct Opt {
    #[structopt(parse(from_os_str))]
    image: PathBuf,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::from_args();

    let base_img = image::open(opt.image)?;

    let out = std::process::Command::new("gcloud")
        .arg("auth")
        .arg("application-default")
        .arg("print-access-token")
        .output()?
        .stdout;
    let credentials = std::str::from_utf8(&out)?.into();

    let mut buf = vec![];
    base_img.write_to(&mut buf, image::ImageOutputFormat::Png)?;
    let res_base64 = base64::encode(&buf);

    let mut api_request = Map::new();
    api_request.insert("api_key".into(), Value::String(credentials));
    api_request.insert("image_data".into(), Value::String(res_base64));

    let client = reqwest::Client::new();
    let res = client
        .post("http://localhost:8000/api/v1/ocr_with_anomaly")
        .header(CONTENT_TYPE, "application/json")
        .body(Value::Object(api_request).to_string())
        .send()
        .await?;

    println!("Response {:?}", res.text().await);
    Ok(())
}
