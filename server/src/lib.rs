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

use futures::{stream, StreamExt};
use hyper::body::Bytes;
use s3s::dto::{
    DeleteObjectOutput, DeleteObjectRequest, GetObjectOutput, GetObjectRequest, PutObjectOutput,
    PutObjectRequest, StreamingBlob,
};
use s3s::{S3Result, S3};

#[async_trait::async_trait]
impl S3 for MyStorage {
    async fn get_object(&self, input: GetObjectRequest) -> S3Result<GetObjectOutput> {
        let bucket = input.bucket;
        let key = input.key;

        let map = self.map.lock().unwrap();
        let bucket_map = map
            .get(&bucket)
            .ok_or_else(|| s3s::s3_error!(NoSuchBucket, "bucket not found"))?;
        let data = bucket_map
            .get(&key)
            .ok_or_else(|| s3s::s3_error!(NoSuchKey, "key not found"))?
            .clone();

        let len = data.len() as i64;
        let body = StreamingBlob::wrap(stream::once(async move {
            Ok::<Bytes, std::io::Error>(Bytes::from(data))
        }));

        let mut out = GetObjectOutput::default();
        out.body = Some(body);
        out.content_length = len;
        Ok(out)
    }

    async fn put_object(&self, mut input: PutObjectRequest) -> S3Result<PutObjectOutput> {
        let bucket = input.bucket;
        let key = input.key;
        let body = input
            .body
            .take()
            .ok_or_else(|| s3s::s3_error!(InvalidRequest, "missing body"))?;

        let mut data = Vec::new();
        let mut stream = body.0;
        while let Some(chunk) = stream.next().await {
            let chunk = chunk.map_err(|e| s3s::S3Error::with_message(s3s::S3ErrorCode::InternalError, e.to_string()))?;
            data.extend_from_slice(&chunk);
        }

        let mut map = self.map.lock().unwrap();
        let bucket_map = map.entry(bucket).or_insert_with(HashMap::new);
        bucket_map.insert(key, data);

        Ok(PutObjectOutput::default())
    }

    async fn delete_object(&self, input: DeleteObjectRequest) -> S3Result<DeleteObjectOutput> {
        let bucket = input.bucket;
        let key = input.key;
        let mut map = self.map.lock().unwrap();
        if let Some(bucket_map) = map.get_mut(&bucket) {
            bucket_map.remove(&key);
        }
        Ok(DeleteObjectOutput::default())
    }
}
