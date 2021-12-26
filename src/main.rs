use flexi_logger;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Error, Request, Response, Server};
use log::{debug, error, info};
use simplewebserver::Config;
use std::collections::HashSet;
use std::sync::Arc;
use tokio::fs;

// could be single-threaded and still leverage tokio with rt-single-thread?
// would come with slight benefit to binary size and resource usage
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Initialize logger
    let mut logger = flexi_logger::Logger::try_with_str("info")?.start()?;

    // Get CLI information
    let conf = Config::new();

    // Print debug info and quit if it's a dry run
    if conf.dry_run {
        let mut files: Vec<&String> = conf.files.iter().collect();
        // sort for consistency in cli integration tests
        files.sort_unstable();
        println!("FILES: {:?}", files);
        return Ok(());
    }

    // Adjust logger verbosity
    // TODO: once the time crate fully resolves RUSTSEC-2020-0159 and
    // restores normal features on unix, update flexi_logger and other crates using time
    let verbosity = match conf.verbose {
        true => "debug",
        false => "info",
    };
    logger.parse_new_spec(verbosity)?;

    // Initialize server
    let served_files = Arc::new(conf.files);

    info!("Starting Server...");

    let make_svc = make_service_fn(|_conn| {
        let served_files = served_files.clone();
        async move {
            Ok::<_, Error>(service_fn(move |req| {
                let served_files = served_files.clone();
                handle_connection(served_files, req)
            }))
        }
    });

    let addr = ([127, 0, 0, 1], conf.port).into();
    let server = Server::bind(&addr).serve(make_svc);

    info!(
        "Serving {} file(s) on {} port {}",
        served_files.len(),
        "127.0.0.1",
        &conf.port
    );

    server.await?;

    Ok(())
}

async fn handle_connection(
    files: Arc<HashSet<String>>,
    req: Request<Body>,
) -> Result<Response<Body>, Box<dyn std::error::Error + Send + Sync>> {
    // TODO: error handling
    let path = req.uri().path().strip_prefix("/").unwrap();

    info!("Recived request from {} for {}", "TODO", path);

    let response_path = if files.contains(&String::from(path)) {
        path
    } else {
        "404.html"
    };

    // TODO: replace ? with match for logging
    let contents = fs::read(response_path).await?;

    Ok(Response::new(Body::from(contents)))
}
