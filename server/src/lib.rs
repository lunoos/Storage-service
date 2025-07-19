use std::collections::HashMap;
use std::sync::Mutex;

/// Simple in-memory storage mapping bucket -> (key -> bytes).
pub struct MyStorage {
    map: Mutex<HashMap<String, HashMap<String, Vec<u8>>>>,
}

impl MyStorage {
    /// Create an empty storage instance.
    pub fn new() -> Self {
        Self {
            map: Mutex::new(HashMap::new()),
        }
    }
}

use futures::stream;
use hyper::body::Bytes;
use s3s::dto::{
    GetObjectOutput, GetObjectRequest, PutObjectOutput, PutObjectRequest, StreamingBlob,
};
use s3s::{S3Result, S3};

#[async_trait::async_trait]
impl S3 for MyStorage {
    async fn get_object(&self, _input: GetObjectRequest) -> S3Result<GetObjectOutput> {
        let body = StreamingBlob::wrap(stream::once(async {
            Ok::<Bytes, std::io::Error>(Bytes::new())
        }));
        let mut out = GetObjectOutput::default();
        out.body = Some(body);
        Ok(out)
    }

    async fn put_object(&self, _input: PutObjectRequest) -> S3Result<PutObjectOutput> {
        Ok(PutObjectOutput::default())
    }
}
