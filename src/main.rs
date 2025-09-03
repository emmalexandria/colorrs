use clap::{Arg, Command, command, value_parser};
use rand::seq::IndexedRandom;
use std::{
    fmt::Display,
    path::{Path, PathBuf},
    process::exit,
    str::FromStr,
};

use crate::{
    files::list_dir_files,
    patterns::{get_pattern_dir, print_pattern},
};

mod files;
mod patterns;

fn main() {
    let cli = build_cli();

    let matches = cli.get_matches_from(wild::args());

    // Get the default dir, and use it if there isn't a custom dir defined
    let default_dir = get_pattern_dir().unwrap();
    let dir = matches
        .get_one::<PathBuf>("patterndir")
        .unwrap_or(&default_dir);

    if !dir.exists() {
        eprintln!("Pattern directory {} does not exist", dir.to_string_lossy());
        exit(-1);
    }

    if matches.get_flag("list") {
        list(dir);
    }

    if matches.get_flag("random") {
        random(dir);
    }

    if let Some(pattern) = matches.get_one::<String>("print") {
        print(dir, pattern);
    }
}

fn build_cli() -> Command {
    command!()
        .arg(
            Arg::new("print")
                .short('p')
                .long("print")
                .value_name("PATTERN")
                .help("Print the given pattern")
                .num_args(1)
                .conflicts_with_all(["random", "list"]),
        )
        .arg(
            Arg::new("random")
                .short('r')
                .long("random")
                .help("Choose a random pattern")
                .conflicts_with_all(["list", "print"])
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("list")
                .short('l')
                .long("list")
                .help("List all available patterns")
                .conflicts_with_all(["random", "print"])
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("patterndir")
                .short('d')
                .value_name("DIRECTORY")
                .long("dir")
                .help("Set a custom directory for pattern description files")
                .value_parser(value_parser!(PathBuf)),
        )
}

fn print(dir: &PathBuf, pattern: &String) {
    let files = list_dir_files(dir).unwrap();

    let file = files
        .iter()
        .find(|f| f.file_stem().unwrap().to_str().unwrap() == pattern);

    if let Some(f) = file {
        print_pattern(f);
    } else {
        eprintln!(
            "Pattern file for {} not found in {}",
            pattern,
            dir.to_string_lossy()
        );
    }
}

fn list(dir: &PathBuf) {
    let files = list_dir_files(dir).unwrap();

    for file in files {
        let ext = file.extension();
        let filename = file.file_stem().unwrap().to_string_lossy();

        if let Some(e) = ext {
            println!("{} ({})", filename, e.to_string_lossy());
            return;
        }

        println!("{}", filename);
    }
}

fn random(dir: &PathBuf) {
    let files = list_dir_files(dir).unwrap();
    let mut rng = rand::rng();
    let file = files.choose(&mut rng).unwrap();

    print_pattern(file);
}
