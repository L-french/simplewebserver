use flexi_logger;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Error, Request, Response, Server};
use log::{debug, error, info, warn};
use simplewebserver::{Config, util};
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
    let conf = Arc::new(conf);

    info!("Starting Server...");

    let make_svc = make_service_fn(|_conn| {
        let conf = conf.clone();
        async move {
            Ok::<_, Error>(service_fn(move |req| {
                let conf = conf.clone();
                handle_connection(conf, req)
            }))
        }
    });

    let addr = (conf.address, conf.port).into();
    let server = Server::bind(&addr).serve(make_svc);

    info!(
        "Serving {} file(s) on {} port {}",
        conf.files.len(),
        addr.ip(),
        addr.port(),
    );

    server.await?;

    Ok(())
}

async fn handle_connection(
    conf: Arc<Config>,
    req: Request<Body>,
) -> Result<Response<Body>, Box<dyn std::error::Error + Send + Sync>> {
    let path = match req.uri().path().strip_prefix("/") {
        Some(str) => {
            if conf.files.contains(&String::from(str)) {
                info!("Recived request for {}", str);
                Some(str)
            } else {
                warn!("Recived request for invalid file {}", str);
                None
            }
        }
        None => {
            warn!("Recievd bad request");
            None
        }
    };

    match path {
        Some(path) => {
            let contents = fs::read(path).await?;
            return Ok(Response::new(Body::from(contents)))
        },
        None => return Ok(Response::new(Body::from(util::DEFAULT_404)))
    }
}
