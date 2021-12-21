use tokio::fs;
use clap::{crate_description, crate_version, App, Arg};
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Error, Request, Response, Server};
use std::process;
use std::sync::Arc;

// could be single-threaded and still leverage tokio with rt-single-thread?
// would come with slight benefit to binary size and cpu usage
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Clap definitions
    // TODO: define config struct, split to library
    let matches = App::new("simplewebserver")
        .version(crate_version!())
        .about(crate_description!())
        .arg(
            Arg::with_name("files")
                .takes_value(true)
                .required(true)
                .multiple(true)
                .value_name("FILE")
                .help("The file(s) to serve"),
        )
        .get_matches();

    let mut served_files = Vec::new();

    for path in matches.values_of("files").unwrap() {
        // uses std fs operations. downside to using tokio later on?
        let metadata = std::fs::metadata(path);
        if let Ok(file) = metadata {
            if file.is_file() {
                served_files.push(String::from(path))
            }
        } else {
            eprintln!("Could not find file {}", path)
        }
    }

    if served_files.is_empty() {
        eprintln!("No files to serve!\n");
        // eprintln!("{}", matches.usage());
        process::exit(1);
    }

    let served_files = Arc::new(served_files);

    // Initialize server
    let make_svc = make_service_fn(|_conn| {
        let served_files = served_files.clone();
        async move {
            Ok::<_, Error>(service_fn(move |req| {
                let served_files = served_files.clone();
                handle_connection(served_files, req)
            }))
        }
    });

    let addr = ([127, 0, 0, 1], 7878).into();
    let server = Server::bind(&addr).serve(make_svc);

    server.await?;

    Ok(())
}

async fn handle_connection(
    files: Arc<Vec<String>>,
    req: Request<Body>,
) -> Result<Response<Body>, Box<dyn std::error::Error + Send + Sync>> {
    // todo: error handling
    let path = req.uri().path().strip_prefix("/").unwrap();
    let response_path = if files.contains(&String::from(path)) {
        path
    } else {
        "404.html"
    };

    let contents = fs::read(response_path).await?;

    Ok(Response::new(Body::from(contents)))
}
