#[macro_use] extern crate rocket;

use rocket::response::stream::ByteStream;
use rocket::data::{Data, ToByteUnit};

use rand::RngCore;

const INDEX: &'static str = include_str!("index.html");

const CHUNK_SIZE: usize = 1_024 * 1_024; // 1MB
const CHUNKS: usize = 50; // in MB
const DOWNLOAD_BYTES: usize = CHUNKS * CHUNK_SIZE;

#[get("/empty")]
async fn empty() -> &'static str {
    ""
}

#[post("/upload", data="<data>")]
async fn upload(data: Data<'_>) -> std::io::Result<()> {
    let bytes = data.open(DOWNLOAD_BYTES.bytes() + 1)
        .into_bytes()
        .await?;

    // TODO: Return bytes[bytes.len() - 1] as a confirmation that it was read.

    Ok(())
}


#[get("/download")]
async fn download() -> ByteStream![Vec<u8>] {
    let mut chunks: Vec<Vec<u8>> = Vec::new();

    for _ in 0..CHUNKS {
        let mut chunk = [0u8; CHUNK_SIZE];
        rand::thread_rng().fill_bytes(&mut chunk);
        chunks.push(chunk.to_vec());
    }

    ByteStream! {
        for chunk in chunks {
            yield chunk;
        }
    }
}

#[get("/")]
async fn index() -> &'static str {
    INDEX
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, empty, upload, download])
}
