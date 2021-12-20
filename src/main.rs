use hyper::server::conn::Http;
use hyper::service::service_fn;
use hyper::{Request, Response, Body};
use std::error::Error;
use std::fs;
use std::thread;
use std::time::Duration;
use tokio::net::TcpListener;

// could be single-threaded and still leverage tokio with rt-single-thread?
// would come with slight benefit to binary size and cpu usage
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let listener = TcpListener::bind("127.0.0.1:7878").await?;

    loop {
        let (socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            if let Err(http_err) = Http::new()
                .http1_only(true)
                .http1_keep_alive(true)
                .serve_connection(socket, service_fn(handle_connection))
                .await
            {
                eprintln!("Error while serving HTTP connection: {}", http_err);
            }
        });
    }
}

async fn handle_connection(
    req: Request<Body>,
) -> Result<Response<Body>, Box<dyn Error + Send + Sync>> {
    let filename = match req.uri().path() {
        "/" => "test.html",
        "/sleep" => {
            thread::sleep(Duration::from_secs(5));
            "test.html"
        }
        _ => "404.html",
    };

    // use tokio's async versions of fs operations?
    let contents = fs::read_to_string(filename).unwrap();

    Ok(Response::new(Body::from(contents)))
}
