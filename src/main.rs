#[macro_use] extern crate rocket;

use rocket::data::{Data, Limits, ToByteUnit};
use rocket::http::ContentType;

const CHUNK_SIZE: usize = 1_024 * 1_024; // 1MB
const CHUNKS: usize = 50; // in MB
const DOWNLOAD_BYTES: usize = CHUNKS * CHUNK_SIZE;

#[get("/empty")]
async fn empty() -> &'static str {
    ""
}

#[post("/upload", data="<data>")]
async fn upload(data: Data<'_>) -> std::io::Result<()> {
    data.open(DOWNLOAD_BYTES.bytes() + 1)
        .into_bytes()
        .await?;

    Ok(())
}

#[get("/download")]
async fn download() -> Vec<u8> {
    [1u8; DOWNLOAD_BYTES].to_vec()
}

#[get("/")]
async fn index() -> (ContentType, &'static str) {
    (ContentType::HTML, include_str!("index.html"))
}

#[launch]
fn rocket() -> _ {
    let figment = rocket::Config::figment()
        .merge(("address", "0.0.0.0"))
        .merge(("limits", Limits::new().limit("data", DOWNLOAD_BYTES.bytes())));


    rocket::custom(figment).mount("/", routes![index, empty, upload, download])
}
