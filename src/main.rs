#[macro_use] extern crate rocket;

use rocket::data::{Data, ToByteUnit};
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
    rocket::build().mount("/", routes![index, empty, upload, download])
}
