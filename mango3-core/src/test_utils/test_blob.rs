use std::convert::Infallible;
use std::fs::{self, File};
use std::io::Read;

use futures_util::stream::once;
use multer::bytes::Bytes;
use multer::Multipart;

use crate::config::MISC_CONFIG;
use crate::models::{Blob, User, Website};
use crate::CoreContext;

use super::insert_test_user;

pub async fn insert_test_blob(core_context: &CoreContext, user: Option<&User>, website: Option<&Website>) -> Blob {
    let user = if let Some(user) = user {
        user
    } else {
        &insert_test_user(core_context).await
    };

    let mut file = File::open("../assets/favicon.png").unwrap();
    let mut buffer = Vec::new();

    file.read_to_end(&mut buffer).unwrap();

    let payload = format!(
        "--X-BOUNDARY\r\n\
         Content-Disposition: form-data; name=\"file\"; filename=\"favicon.png\"\r\n\
         Content-Type: image/png\r\n\
         \r\n\
         {}\r\n\
         --X-BOUNDARY--\r\n",
        String::from_utf8_lossy(&buffer)
    );

    let stream = once(async move { Result::<Bytes, Infallible>::Ok(Bytes::from(payload)) });
    let mut multipart = Multipart::new(stream, "X-BOUNDARY");
    let mut field = multipart.next_field().await.unwrap().unwrap();

    fs::create_dir_all(MISC_CONFIG.storage_tmp_path()).unwrap();

    Blob::insert(core_context, &user, website, &mut field)
        .await
        .ok()
        .unwrap()
}
