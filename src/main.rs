#[macro_use] extern crate rocket;

use rocket::response::stream::ByteStream;
use rand::{thread_rng, Rng};

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
async fn download() -> ByteStream![[u8; CHUNK_SIZE]] {
    ByteStream! {
        for _ in 0..CHUNKS {
            let mut arr = [0u8; CHUNK_SIZE];
            rand::thread_rng().fill(&mut arr);
            yield arr;
            //yield [rand::random::<u8>()];
            //yield Standard.sample(&mut rand::thread_rng());
            //yield Distribuion<NonZeroU8>.sample_iter(&mut rand::thread_rng()).into();
            //yield Alphanumeric.sample_string(&mut rand::thread_rng(), CHUNK_SIZE);
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
