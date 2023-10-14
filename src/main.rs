#![feature(iter_intersperse)]

use std::{path::PathBuf, io::{Write, Seek}, iter::once};

use clap::Parser;
use exclude::{Item, recurse_collection, CollectionMatches};
use zip::{ZipWriter, write::FileOptions};

mod args;
mod exclude;

fn file_name(path: &PathBuf) -> String {
    path.iter().filter(|p| p.to_string_lossy() != ".").map(|p| p.to_string_lossy()).intersperse("/".into()).collect()
}

fn crawler<W: Write + Seek>(path: PathBuf, include_items: Vec<Item>, exclude_items: Vec<Item>, zip: &mut ZipWriter<W>) {
    let files = match std::fs::read_dir(&path) {
        Ok(files) => files,
        Err(e) => {
            eprintln!("Failed to read file {:?} due to {}", path, e);
            return;
        },
    };
    for file in files {
        let file = match &file {
            Ok(file) => file,
            Err(e) => {
                eprintln!("Failed to read file {:?} due to {}", file, e);
                return;
            },
        };
        if [".git", ".gitignore"].contains(&&*file.file_name().to_string_lossy()) {
            continue;
        }
        let include_matches = recurse_collection(&file.file_name().to_string_lossy(), &include_items);
        let exclude_matches = recurse_collection(&file.file_name().to_string_lossy(), &exclude_items);
        if file.metadata().expect("Metadata should be available").is_dir() {
            match include_matches {
                CollectionMatches::Exact(v) | CollectionMatches::Partial(v) => {
                    let exclude_items = match exclude_matches {
                        CollectionMatches::Exact(_) => continue,
                        CollectionMatches::Partial(x) => x,
                        CollectionMatches::No => Vec::new(),
                    };
                    let name = file_name(&file.path());
                    let options = FileOptions::default().compression_method(zip::CompressionMethod::Deflated); 
                    zip.add_directory(name, options).unwrap();
                    println!("Included {}", file.file_name().to_string_lossy());
                    crawler(file.path(), v, exclude_items.clone(), zip);
                },
                _ => (),
            }
        }
        else {
            match include_matches {
                CollectionMatches::Exact(_) => {
                    match exclude_matches {
                        CollectionMatches::Exact(_) => continue,
                        _ => ()
                    }
                    let name = file_name(&file.path());
                    let options = FileOptions::default().compression_method(zip::CompressionMethod::Deflated); 
                    zip.start_file(name, options).unwrap();
                    zip.write_all(&std::fs::read(file.path()).unwrap()).unwrap();
                    println!("Included {}", file.file_name().to_string_lossy())
                },
                _ => (),
            }
        }
    }
}

fn main() {
    let args = args::Args::parse();
    let include_items: Vec<Item> = args.inputs.iter().map(|s| Item::from_string(s)).collect();
    let exclude_items: Vec<Item> = args.excludes.iter().chain(once(&args.output)).map(|s| Item::from_string(s)).collect();
    let file = std::fs::File::create(args.output).unwrap();
    let mut zip = zip::write::ZipWriter::new(file);
    crawler(".".parse().unwrap(), include_items, exclude_items, &mut zip);
    zip.finish().unwrap();
}
