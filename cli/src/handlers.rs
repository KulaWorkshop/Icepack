use crate::archive::{Archive, ArchiveType};
use crate::args::Arguments;
use crate::builder::Builder;
use crate::utils;

use colored::Colorize;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

pub fn handle_extract(arguments: &Arguments) -> Result<(), std::io::Error> {
    // validate arguments
    if arguments.files.is_empty() || arguments.files.len() > 2 {
        utils::display_error("an input archive and output path must be specified.");
    }

    // create output path
    let default = String::from(".");
    let output = Path::new(arguments.files.get(1).unwrap_or(&default));
    std::fs::create_dir_all(output)?;

    // create archive
    let path = Path::new(&arguments.files[0]);
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut archive = Archive::open(reader);

    // define name map
    let mut names: HashMap<String, u8> = HashMap::new();

    // extract archive entries
    for (i, e) in archive.entries()?.enumerate() {
        // get name
        let mut name = e.name.clone().unwrap_or(format!("FILE {}", i + 1));
        let entry = names.entry(name.clone());
        entry.and_modify(|val| *val += 1).or_insert(2);

        println!("{}{} {}", "unpacking".cyan().bold(), ":".bold(), name);
        if names.get(&name).unwrap() > &2 {
            name = format!("{}-{}", name, names.get(&name).unwrap() - 1);
            println!(
                "{}{} renaming duplicate filename...",
                "info".yellow().bold(),
                ":".bold()
            );
        }

        let path = output.join(&name);

        e.unpack(&path)?;
    }

    Ok(())
}

pub fn handle_create(arguments: &Arguments) -> Result<(), std::io::Error> {
    // validate arguments
    if arguments.files.len() < 2 {
        utils::display_error("an output path and input files must be specified.");
    }

    // get archive type
    let mut ty = ArchiveType::Pak;
    if arguments.kub {
        ty = ArchiveType::Kub;
    }

    // create archive
    let mut archive = Builder::new(ty);
    for f in arguments.files[1..].iter() {
        let path = Path::new(f);

        let name = path.file_name().unwrap().to_str().unwrap();
        println!("{}{} {}", "packing".cyan().bold(), ":".bold(), name);
        archive.add_file(path)?;
    }

    let output_path = Path::new(&arguments.files[0]);
    archive.pack(output_path)?;

    Ok(())
}
