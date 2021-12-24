use clap::{crate_description, crate_name, crate_version, App, Arg, Values};
use std::error;
use std::fs::{metadata, read_dir};
use std::io;
use std::path::Path;
use std::{collections::HashSet, process};

pub struct Config {
    pub files: HashSet<String>,
    pub port: u16,
    pub dry_run: bool,
}

impl Config {
    pub fn new() -> Config {
        let matches = App::new(crate_name!())
            .version(crate_version!())
            .about(crate_description!())
            .arg(
                Arg::with_name("recursive")
                    .short("r")
                    .long("recursive")
                    .help("Serve directories recursively"),
            )
            .arg(
                Arg::with_name("dry run")
                    .short("D")
                    .long("dry-run")
                    .help("Print files which would be served and exit"),
            )
            .arg(
                Arg::with_name("port")
                    .short("p")
                    .long("port")
                    .help("Bind to a port")
                    .default_value("7878"),
            )
            .arg(
                Arg::with_name("files")
                    .takes_value(true)
                    .required(true)
                    .multiple(true)
                    .value_name("FILE")
                    .help("The file(s) to serve"),
            )
            .get_matches();

        let files = get_files(
            matches.values_of("files").unwrap(),
            &matches.is_present("recursive"),
        )
        .unwrap(); //TODO: error handling

        if files.is_empty() {
            eprintln!("No files to serve!\n");
            // eprintln!("{}", matches.usage());
            process::exit(1);
        }

        let port: u16 = match matches.value_of("port").unwrap().parse() {
            Ok(port) => port,
            // default should always be provided by clap
            Err(_) => unreachable!(),
        };

        let dry_run = matches.is_present("dry run");

        Config {
            files,
            port,
            dry_run,
        }
    }
}

fn get_files(paths: Values, recursive: &bool) -> io::Result<HashSet<String>> {
    let mut files = HashSet::new();

    for path in paths {
        // this uses std filesystem operations. could use tokio?
        let meta = metadata(path)?;
        if meta.is_file() {
            // could use &str?
            match get_file_path(Path::new(&path)) {
                Ok(file) => {
                    files.insert(file);
                    ()
                }
                Err(err) => eprintln!("Failed to access file: {}", err),
            }
        } else if meta.is_dir() && *recursive {
            for file in get_directory_recursive(Path::new(&path)) {
                files.insert(file);
            }
        }
    }

    Ok(files)
}

fn get_file_path(path: &Path) -> Result<String, Box<dyn error::Error>> {
    // TODO: add tests for correct path stripping
    let cwd = Path::new("./").canonicalize()?;
    let canonical = path.canonicalize()?;
    let stripped_path = canonical.strip_prefix(cwd)?.to_str();

    match stripped_path {
        Some(str) => Ok(String::from(str)),
        None => Err(Box::new(io::Error::new(
            io::ErrorKind::Other,
            "Failed to convert path to UTF-8",
        ))),
    }
}

fn get_directory_recursive(path: &Path) -> HashSet<String> {
    let mut files = HashSet::new();

    // TODO: more sophisticated error handling
    if let Ok(entries) = read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                if let Ok(meta) = entry.metadata() {
                    if meta.is_file() {
                        match get_file_path(&entry.path()) {
                            Ok(file) => {
                                files.insert(file);
                                ()
                            }
                            Err(err) => eprintln!("Failed to access file: {}", err),
                        }
                    } else if meta.is_dir() {
                        for file in get_directory_recursive(&entry.path()) {
                            files.insert(file);
                        }
                    }
                }
            }
        }
    }

    files
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex_unit() {}
}
