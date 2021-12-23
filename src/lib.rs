use clap::{crate_description, crate_name, crate_version, App, Arg, Values};
use std::{collections::HashSet, process, fs::read_dir};

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
                    .short("d")
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
        ).unwrap(); //TODO: error handling

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
            dry_run
        }
    }
}

fn get_files(paths: Values, recursive: &bool) -> std::io::Result<HashSet<String>> {
    let mut files = HashSet::new();

    for path in paths {
        // this uses std filesystem operations. could use tokio?
        let meta = std::fs::metadata(path)?;
        if meta.is_file() {
            // could use &str?
            files.insert(String::from(path));
        } else if meta.is_dir() && *recursive {
            // TODO: use generics to handle recursion? could implement get_files over any iterator over strings
            // alternately, could do a pass before this loop to expand directories into list of files
            for entry in read_dir(path)? {
                // TODO: test for handling nested directories
                match entry?.path().to_str() {
                    Some(file) => {files.insert(String::from(file));},
                    None => eprintln!("Failed to extract files from recursion"),
                }
            }
        }
    }

    Ok(files)
}


#[cfg(test)]
mod tests {
    use super::*;

    fn ex_unit() {

    }
}