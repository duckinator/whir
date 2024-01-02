#[macro_use] extern crate rocket;

use rocket::response::stream::ByteStream;

//use rocket::data::{Data, ToByteUnit};

const INDEX: &'static str = include_str!("index.html");

#[get("/empty")]
async fn empty() -> &'static str {
    ""
}

#[put("/upload", data="<data>")]
async fn upload(data: Vec<u8>) -> std::io::Result<()> {
    /*data.open(2.mebibytes())
        .into_bytes()
        .await?;*/

    drop(data);

    Ok(())
}


const DOWNLOAD_BYTES: usize = 50_000_000;
const CHUNK_SIZE: usize = 1_000;
const CHUNKS: usize = DOWNLOAD_BYTES / CHUNK_SIZE;

#[get("/download")]
async fn download() -> ByteStream![Vec<u8>] {
    ByteStream! {
        for _ in 0..CHUNKS {
            let mut chunk = Vec::new();
            for _ in 0..CHUNK_SIZE {
                chunk.push(rand::random::<u8>());
            }
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
