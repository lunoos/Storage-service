use hyper::{service::make_service_fn, Server};
use CIAOS::MyStorage;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let svc = MyStorage::new();
    let router = s3s::server::router(svc);
    Server::bind(&([0, 0, 0, 0], 8000).into())
        .serve(make_service_fn(|_| async {
            Ok::<_, hyper::Error>(router.clone())
        }))
        .await?;
    Ok(())
}
